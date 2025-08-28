// src/lib.rs
use bs58::decode as b58decode;
use console_error_panic_hook;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// === Compile-time embedded IDL (adjust path if needed) ===
const GMX_IDL: &str = include_str!("../idls/gmx_solana.json");

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
}

// ---------- small utils ----------
fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .filter(|p| !p.is_empty())
        .map(|w| {
            let mut it = w.chars();
            match it.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + &it.as_str().to_lowercase(),
            }
        })
        .collect::<String>()
}

fn load_idl() -> Option<Value> {
    serde_json::from_str::<Value>(GMX_IDL).ok()
}

fn anchor_disc_for(name_snake: &str) -> [u8; 8] {
    // Anchor discriminator: first 8 bytes of sha256("global:" + name)
    let mut h = Sha256::new();
    h.update(b"global:");
    h.update(name_snake.as_bytes());
    let digest = h.finalize();
    let mut out = [0u8; 8];
    out.copy_from_slice(&digest[..8]);
    out
}

fn find_instr_by_disc<'a>(idl: &'a Value, disc: &[u8]) -> Option<&'a Value> {
    let arr = idl.get("instructions")?.as_array()?;
    for ix in arr {
        let mut cand = [0u8; 8];
        if let Some(d) = ix.get("discriminator").and_then(|v| v.as_array()) {
            if d.len() == 8 {
                for i in 0..8 {
                    cand[i] = d[i].as_u64().unwrap_or(0) as u8;
                }
            } else {
                let nm = ix.get("name")?.as_str()?;
                cand = anchor_disc_for(nm);
            }
        } else {
            let nm = ix.get("name")?.as_str()?;
            cand = anchor_disc_for(nm);
        }
        if cand == disc {
            return Some(ix);
        }
    }
    None
}
// === Events index: discriminator ([u8;8]) -> event JSON node from IDL ===
fn build_event_index<'a>(idl: &'a Value) -> std::collections::HashMap<[u8; 8], &'a Value> {
    let mut m = std::collections::HashMap::new();
    if let Some(arr) = idl.get("events").and_then(|v| v.as_array()) {
        for ev in arr {
            if let (Some(name), Some(darr)) = (
                ev.get("name").and_then(|v| v.as_str()),
                ev.get("discriminator").and_then(|v| v.as_array()),
            ) {
                if darr.len() == 8 {
                    let mut disc = [0u8; 8];
                    for i in 0..8 {
                        disc[i] = darr[i].as_u64().unwrap_or(0) as u8;
                    }
                    // name is not used here, we just map disc -> the event node
                    m.insert(disc, ev);
                }
            }
        }
    }
    m
}

// === Find a struct type definition by name in IDL.types ===
fn find_type_by_name<'a>(idl: &'a Value, name: &str) -> Option<&'a Value> {
    let arr = idl.get("types")?.as_array()?;
    arr.iter()
        .find(|t| t.get("name").and_then(|v| v.as_str()) == Some(name))
}

// ---------- account mapping (IDL → named accounts) ----------
#[derive(Clone, Debug)]
struct AccountSpec {
    path: String,
    writable: bool,
    signer: bool,
    optional: bool,
}

// IDL accounts can nest via "accounts": [...]. We flatten to leaves in order. (IDL uses "writable"/"signer") :contentReference[oaicite:3]{index=3}
fn flatten_accounts_schema(v: &Value, prefix: &str, out: &mut Vec<AccountSpec>) {
    let Some(arr) = v.as_array() else { return; };
    for item in arr {
        let name = item.get("name").and_then(|x| x.as_str()).unwrap_or("unknown");
        let path = if prefix.is_empty() { name.to_string() } else { format!("{}.{}", prefix, name) };
        let optional = item.get("optional").and_then(|x| x.as_bool()).unwrap_or(false);

        if let Some(children) = item.get("accounts") {
            // group node → recurse
            flatten_accounts_schema(children, &path, out);
        } else {
            // leaf node
            let writable = item.get("writable").and_then(|x| x.as_bool()).unwrap_or(false);
            let signer   = item.get("signer").and_then(|x| x.as_bool()).unwrap_or(false);
            out.push(AccountSpec { path, writable, signer, optional });
        }
    }
}

// Accept accounts as simple strings OR objects with pubkey (plus optional flags, ignored here)
#[derive(Deserialize)]
#[serde(untagged)]
enum AccountInput {
    Str(String),
    Obj {
        pubkey: String,
        #[serde(default)] is_signer: bool,
        #[serde(default)] is_writable: bool,
    }
}
impl AccountInput {
    fn pubkey(&self) -> &str {
        match self {
            AccountInput::Str(s) => s,
            AccountInput::Obj { pubkey, .. } => pubkey,
        }
    }
}

// ---------- primitive kinds ----------
#[derive(Clone, Copy, Debug)]
enum Prim { U8, I8, U16, I16, U32, I32, U64, I64, U128, I128, Bool, Pubkey } // "pubkey" is 32 bytes

fn parse_prim_ty(s: &str) -> Option<Prim> {
    Some(match s {
        "u8" => Prim::U8, "i8" => Prim::I8,
        "u16" => Prim::U16, "i16" => Prim::I16,
        "u32" => Prim::U32, "i32" => Prim::I32,
        "u64" => Prim::U64, "i64" => Prim::I64,
        "u128" => Prim::U128, "i128" => Prim::I128,
        "bool" => Prim::Bool,
        "pubkey" => Prim::Pubkey, // your IDL uses "pubkey" for 32B keys :contentReference[oaicite:4]{index=4}
        _ => return None,
    })
}

// ---------- byte readers ----------
fn read<const N: usize>(buf: &[u8], off: &mut usize) -> Option<[u8; N]> {
    if *off + N > buf.len() { return None; }
    let mut out = [0u8; N];
    out.copy_from_slice(&buf[*off..*off + N]);
    *off += N;
    Some(out)
}
fn read_le_u32(buf: &[u8], off: &mut usize) -> Option<u32> { Some(u32::from_le_bytes(read::<4>(buf, off)?)) }

// ---------- borsh primitives / composites ----------
fn decode_prim_borsh(buf: &[u8], off: &mut usize, p: Prim) -> Option<Value> {
    Some(match p {
        Prim::U8  => json!(u8 ::from_le_bytes(read::<1>(buf, off)?)),
        Prim::I8  => json!(i8 ::from_le_bytes(read::<1>(buf, off)?)),
        Prim::U16 => json!(u16::from_le_bytes(read::<2>(buf, off)?)),
        Prim::I16 => json!(i16::from_le_bytes(read::<2>(buf, off)?)),
        Prim::U32 => json!(u32::from_le_bytes(read::<4>(buf, off)?)),
        Prim::I32 => json!(i32::from_le_bytes(read::<4>(buf, off)?)),
        Prim::U64 => json!(u64::from_le_bytes(read::<8>(buf, off)?).to_string()),
        Prim::I64 => json!(i64::from_le_bytes(read::<8>(buf, off)?).to_string()),
        Prim::U128 => json!(u128::from_le_bytes(read::<16>(buf, off)?).to_string()),
        Prim::I128 => json!(i128::from_le_bytes(read::<16>(buf, off)?).to_string()),
        Prim::Bool => json!(read::<1>(buf, off)?[0] != 0),
        Prim::Pubkey => {
            let pk = read::<32>(buf, off)?;
            json!(bs58::encode(pk).into_string())
        }
    })
}

fn decode_borsh_string(buf: &[u8], off: &mut usize) -> Option<String> {
    let len = read_le_u32(buf, off)? as usize;
    if *off + len > buf.len() { return None; }
    let s = std::str::from_utf8(&buf[*off..*off + len]).ok()?.to_string();
    *off += len;
    Some(s)
}

fn decode_borsh_bytes(buf: &[u8], off: &mut usize) -> Option<String> {
    let len = read_le_u32(buf, off)? as usize;
    if *off + len > buf.len() { return None; }
    let out = bs58::encode(&buf[*off..*off + len]).into_string();
    *off += len;
    Some(out)
}

fn decode_fixed_array<F>(n: usize, mut f: F) -> Option<Value>
where F: FnMut(usize) -> Option<Value> {
    let mut vec = Vec::with_capacity(n);
    for i in 0..n { vec.push(f(i)?); }
    Some(Value::Array(vec))
}

// ---------- type index (to know which "defined" types are bytemuck) ----------
fn build_type_index<'a>(idl: &'a Value) -> std::collections::HashMap<&'a str, (&'a Value, bool)> {
    let mut m = std::collections::HashMap::new();
    if let Some(types) = idl.get("types").and_then(|v| v.as_array()) {
        for t in types {
            if let Some(name) = t.get("name").and_then(|v| v.as_str()) {
                let is_bytemuck = t.get("serialization")
                    .and_then(|v| v.as_str())
                    .map(|s| s.eq_ignore_ascii_case("bytemuck"))
                    .unwrap_or(false);
                m.insert(name, (t, is_bytemuck));
            }
        }
    }
    m
}

// ---------- recursive decoders ----------
fn decode_type_borsh(
    ty: &Value,
    idl: &Value,
    type_index: &std::collections::HashMap<&str, (&Value, bool)>,
    buf: &[u8],
    off: &mut usize,
) -> Option<Value> {
    // literal primitive?
    if let Some(s) = ty.as_str() {
        if let Some(p) = parse_prim_ty(s) { return decode_prim_borsh(buf, off, p); }
        if s == "string" { return decode_borsh_string(buf, off).map(Value::String); }
        if s == "bytes"  { return decode_borsh_bytes (buf, off).map(Value::String); }
        return Some(json!({"_unsupported_prim": s}));
    }

    // Option<T>
    if let Some(o) = ty.get("option") {
        let tag = read::<1>(buf, off)?[0];
        if tag == 0 { return Some(Value::Null); }
        if tag == 1 { return decode_type_borsh(o, idl, type_index, buf, off); }
        return Some(json!({"_error":"invalid option tag"}));
    }

    // Vec<T>
    if let Some(vt) = ty.get("vec") {
        let len = read_le_u32(buf, off)? as usize;
        let mut arr = Vec::with_capacity(len);
        for _ in 0..len {
            arr.push(decode_type_borsh(vt, idl, type_index, buf, off)?);
        }
        return Some(Value::Array(arr));
    }

    // Array [T, N]
    if let Some(arr) = ty.get("array").and_then(|v| v.as_array()) {
        if arr.len() != 2 { return Some(json!({"_error":"invalid array schema"})); }
        let n = arr[1].as_u64().unwrap_or(0) as usize;
        return decode_fixed_array(n, |_| decode_type_borsh(&arr[0], idl, type_index, buf, off));
    }

    // Defined type
    if let Some(def) = ty.get("defined") {
        let name = if def.is_object() {
            def.get("name").and_then(|v| v.as_str())
        } else { def.as_str() }.unwrap_or("");
        let (defn, is_bytemuck) = match type_index.get(name) {
            Some(x) => *x,
            None => return Some(json!({"_unknown_defined_type": name})),
        };
        if is_bytemuck {
            return decode_type_bytemuck(defn, idl, type_index, buf, off);
        } else {
            return decode_type_borsh_struct(defn, idl, type_index, buf, off);
        }
    }

    Some(json!({"_unsupported_type": ty}))
}

// Borsh struct (fields may be primitives / options / arrays / nested defined)
fn decode_type_borsh_struct(
    type_def: &Value,
    idl: &Value,
    type_index: &std::collections::HashMap<&str, (&Value, bool)>,
    buf: &[u8],
    off: &mut usize,
) -> Option<Value> {
    let fields = type_def.get("type")?.get("fields")?.as_array()?;
    let mut out = serde_json::Map::new();
    for f in fields {
        let name = f.get("name")?.as_str()?;
        let fty  = f.get("type")?;
        let val  = decode_type_borsh(fty, idl, type_index, buf, off)
            .unwrap_or(json!({"_error":"decode failed"}));
        out.insert(name.to_string(), val);
    }
    Some(Value::Object(out))
}

// Bytemuck struct = fixed-size, C layout; only fixed-size fields (no string/vec/option tags)
fn decode_type_bytemuck(
    type_def: &Value,
    idl: &Value,
    type_index: &std::collections::HashMap<&str, (&Value, bool)>,
    buf: &[u8],
    off: &mut usize,
) -> Option<Value> {
    let fields = type_def.get("type")?.get("fields")?.as_array()?;
    let mut out = serde_json::Map::new();

    fn decode_bytemuck_field(
        ty: &Value,
        idl: &Value,
        type_index: &std::collections::HashMap<&str, (&Value, bool)>,
        buf: &[u8],
        off: &mut usize,
    ) -> Option<Value> {
        if let Some(s) = ty.as_str() {
            if let Some(p) = parse_prim_ty(s) {
                // fixed-size primitives
                return decode_prim_borsh(buf, off, p);
            }
            // strings/bytes are not valid bytemuck fields
            if s == "string" || s == "bytes" {
                return Some(json!({"_unsupported_in_bytemuck": s}));
            }
            return Some(json!({"_unsupported_prim": s}));
        }

        if let Some(arr) = ty.get("array").and_then(|v| v.as_array()) {
            if arr.len() != 2 { return Some(json!({"_error":"invalid array schema"})); }
            let n = arr[1].as_u64().unwrap_or(0) as usize;
            return decode_fixed_array(n, |_| decode_bytemuck_field(&arr[0], idl, type_index, buf, off));
        }

        if let Some(def) = ty.get("defined") {
            let name = if def.is_object() {
                def.get("name").and_then(|v| v.as_str())
            } else { def.as_str() }.unwrap_or("");
            let (defn, is_b) = match type_index.get(name) {
                Some(x) => *x,
                None => return Some(json!({"_unknown_defined_type": name})),
            };
            if is_b {
                return decode_type_bytemuck(defn, idl, type_index, buf, off);
            } else {
                // Allow non-bytemuck struct inside bytemuck as a best-effort (rare)
                return decode_type_borsh_struct(defn, idl, type_index, buf, off);
            }
        }

        if ty.get("vec").is_some() { return Some(json!({"_unsupported_in_bytemuck_vec": true})); }
        if ty.get("option").is_some() { return Some(json!({"_unsupported_in_bytemuck_option": true})); }
        Some(json!({"_unsupported_type": ty}))
    }

    for f in fields {
        let name = f.get("name")?.as_str()?;
        let fty  = f.get("type")?;
        let val  = decode_bytemuck_field(fty, idl, type_index, buf, off)
            .unwrap_or(json!({"_error":"decode failed"}));
        out.insert(name.to_string(), val);
    }

    Some(Value::Object(out))
}

// ---------- top-level arg decoding using instruction.args ----------
fn decode_args_from_idl(
    args_v: &Value,
    idl: &Value,
    type_index: &std::collections::HashMap<&str, (&Value, bool)>,
    payload: &[u8],
) -> Value {
    let Some(args) = args_v.as_array() else { return Value::Object(serde_json::Map::new()) };
    let mut off = 0usize;
    let mut out = serde_json::Map::new();

    for arg in args {
        let Some(name) = arg.get("name").and_then(|v| v.as_str()) else { continue };
        let ty = arg.get("type").cloned().unwrap_or(Value::Null);

        let val = decode_type_borsh(&ty, idl, type_index, payload, &mut off)
            .unwrap_or(json!({"_error":"not enough bytes"}));

        out.insert(name.to_string(), val);
    }

    Value::Object(out)
}

// ---------- public entry ----------
#[wasm_bindgen]
pub fn parse(data_base58: &str, account_keys: JsValue) -> JsValue {
    // Accept accounts as strings or objects {pubkey, ...}
    let accounts_in: Vec<AccountInput> =
        serde_wasm_bindgen::from_value(account_keys).unwrap_or_default();
    let accounts_raw: Vec<String> = accounts_in.iter().map(|a| a.pubkey().to_string()).collect();

    // base58 → bytes
    let data = match b58decode(data_base58).into_vec() {
        Ok(d) => d,
        Err(e) => {
            return JsValue::from_str(
                &serde_json::json!({ "error": format!("Failed to decode base58: {}", e) }).to_string(),
            )
        }
    };
    if data.len() < 8 {
        return JsValue::from_str(
            &serde_json::json!({ "error":"Instruction/event data too short", "len": data.len() }).to_string(),
        );
    }

    // Load IDL + indices
    let Some(idl) = load_idl() else {
        return JsValue::from_str(
            &serde_json::json!({ "error":"Failed to parse embedded IDL" }).to_string(),
        );
    };
    let type_index  = build_type_index(&idl);
    let event_index = build_event_index(&idl);

    // tiny util
    fn to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("")
    }

    // ─── Discriminator @ 0 ───
    let disc0: [u8; 8] = { let mut x=[0u8;8]; x.copy_from_slice(&data[..8]); x };
    let payload0 = &data[8..];

    // 1) INSTRUCTION first (primary path)
    if let Some(ix) = find_instr_by_disc(&idl, &disc0) {
        let name_snake   = ix.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
        let name_display = snake_to_pascal(name_snake);
        let args_v       = ix.get("args").cloned().unwrap_or(serde_json::Value::Null);
        let args         = decode_args_from_idl(&args_v, &idl, &type_index, payload0);

        // map accounts (flattened)
        let mut specs: Vec<AccountSpec> = Vec::new();
        if let Some(ix_accounts) = ix.get("accounts") {
            flatten_accounts_schema(ix_accounts, "", &mut specs);
        }
        let mut accounts_map = serde_json::Map::new();
        let min_len = std::cmp::min(specs.len(), accounts_raw.len());
        for i in 0..min_len {
            accounts_map.insert(specs[i].path.clone(), serde_json::Value::String(accounts_raw[i].clone()));
        }
        if accounts_raw.len() < specs.len() {
            for s in &specs[accounts_raw.len()..] {
                accounts_map.insert(s.path.clone(), serde_json::Value::Null);
            }
        }

        let resp = serde_json::json!({
            "instruction_type": name_display,
            "accounts": serde_json::Value::Object(accounts_map),
            "args": args
        });
        return JsValue::from_str(&resp.to_string());
    }

    // 2) EVENT at offset 0 (raw log payloads)
    if let Some(ev_node) = event_index.get(&disc0) {
        let ev_name = ev_node.get("name").and_then(|v| v.as_str()).unwrap_or("UnknownEvent");

        if find_type_by_name(&idl, ev_name).is_none() {
            return JsValue::from_str(
                &serde_json::json!({ "error": format!("event type `{}` not found in IDL.types", ev_name) }).to_string(),
            );
        }
        let defined_ev = serde_json::json!({ "defined": { "name": ev_name } });
        let mut off = 0usize;
        if let Some(decoded) = decode_type_borsh(&defined_ev, &idl, &type_index, payload0, &mut off) {
            let resp = serde_json::json!({
                "event_type": ev_name,
                "args": decoded
            });
            return JsValue::from_str(&resp.to_string());
        } else {
            return JsValue::from_str(&serde_json::json!({ "error":"event decode failed at offset 0" }).to_string());
        }
    }

    // 3) (Optional) EVENT embedded at offset +8
    if data.len() >= 16 {
        let disc1: [u8; 8] = { let mut x=[0u8;8]; x.copy_from_slice(&data[8..16]); x };
        let payload1 = &data[16..];

        if let Some(ev_node) = event_index.get(&disc1) {
            let ev_name = ev_node.get("name").and_then(|v| v.as_str()).unwrap_or("UnknownEvent");
            if find_type_by_name(&idl, ev_name).is_none() {
                return JsValue::from_str(
                    &serde_json::json!({ "error": format!("event type `{}` not found in IDL.types", ev_name) }).to_string(),
                );
            }
            let defined_ev = serde_json::json!({ "defined": { "name": ev_name } });
            let mut off = 0usize;
            if let Some(decoded) = decode_type_borsh(&defined_ev, &idl, &type_index, payload1, &mut off) {
                let resp = serde_json::json!({
                    "event_type": ev_name,
                    "args": decoded,
                    "embedded_event": true,
                    "outer_discriminator_hex": to_hex(&disc0)
                });
                return JsValue::from_str(&resp.to_string());
            } else {
                return JsValue::from_str(&serde_json::json!({
                    "error":"event decode failed at offset 8",
                    "outer_discriminator_hex": to_hex(&disc0)
                }).to_string());
            }
        }
    }

    // 4) no match
    let resp = serde_json::json!({
        "error": "Unknown discriminator (no instruction at 0, no event at 0 or 8)",
        "disc0_hex": to_hex(&disc0),
        "len": data.len()
    });
    JsValue::from_str(&resp.to_string())
}
