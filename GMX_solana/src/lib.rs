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

// Anchor-style event wrapper prefix (8 bytes)
const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29]; // e4 45 a5 2e 51 cb 9a 1d

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
    let mut h = Sha256::new();
    h.update(b"global:");
    h.update(name_snake.as_bytes());
    let digest = h.finalize();
    let mut out = [0u8; 8];
    out.copy_from_slice(&digest[..8]);
    out
}

// Return a JS object (not a JSON string)
fn to_js(v: &serde_json::Value) -> JsValue {
    serde_wasm_bindgen::to_value(v).unwrap_or_else(|_| JsValue::from_str(&v.to_string()))
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
            if let Some(darr) = ev.get("discriminator").and_then(|v| v.as_array()) {
                if darr.len() == 8 {
                    let mut disc = [0u8; 8];
                    for i in 0..8 {
                        disc[i] = darr[i].as_u64().unwrap_or(0) as u8;
                    }
                    m.insert(disc, ev);
                }
            }
        }
    }
    m
}

// === Find a struct/enum type definition by name in IDL.types ===
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

fn flatten_accounts_schema(v: &Value, prefix: &str, out: &mut Vec<AccountSpec>) {
    let Some(arr) = v.as_array() else { return; };
    for item in arr {
        let name = item.get("name").and_then(|x| x.as_str()).unwrap_or("unknown");
        let path = if prefix.is_empty() { name.to_string() } else { format!("{}.{}", prefix, name) };
        let optional = item.get("optional").and_then(|x| x.as_bool()).unwrap_or(false);

        if let Some(children) = item.get("accounts") {
            flatten_accounts_schema(children, &path, out);
        } else {
            let writable = item.get("writable").and_then(|x| x.as_bool()).unwrap_or(false);
            let signer   = item.get("signer").and_then(|x| x.as_bool()).unwrap_or(false);
            out.push(AccountSpec { path, writable, signer, optional });
        }
    }
}

// Accept accounts as simple strings OR objects with pubkey (plus optional flags)
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
enum Prim { U8, I8, U16, I16, U32, I32, U64, I64, U128, I128, Bool, Pubkey }

fn parse_prim_ty(s: &str) -> Option<Prim> {
    Some(match s {
        "u8" => Prim::U8, "i8" => Prim::I8,
        "u16" => Prim::U16, "i16" => Prim::I16,
        "u32" => Prim::U32, "i32" => Prim::I32,
        "u64" => Prim::U64, "i64" => Prim::I64,
        "u128" => Prim::U128, "i128" => Prim::I128,
        "bool" => Prim::Bool,
        "pubkey" => Prim::Pubkey,
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

// ---------- type index ----------
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

// ---------- generic type support ----------
type GenericEnv<'a> = std::collections::HashMap<String, &'a Value>;

fn resolve_generic<'a>(ty: &'a Value, genv: &GenericEnv<'a>) -> Option<&'a Value> {
    if let Some(g) = ty.get("generic").and_then(|v| v.as_str()) {
        return genv.get(g).copied();
    }
    None
}

fn type_kind<'a>(type_def: &'a Value) -> Option<&'a str> {
    type_def.get("type")?.get("kind")?.as_str()
}

fn read_enum_tag(buf: &[u8], off: &mut usize, repr: Option<&Value>) -> Option<u64> {
    // Anchor enums default to u8 tag; if repr is an object/string, we only special-case u16/u32/u64.
    if let Some(r) = repr {
        if let Some(rs) = r.as_str() {
            return match rs {
                "u16" => Some(u16::from_le_bytes(read::<2>(buf, off)?) as u64),
                "u32" => Some(u32::from_le_bytes(read::<4>(buf, off)?) as u64),
                "u64" => Some(u64::from_le_bytes(read::<8>(buf, off)?)),
                _ => Some(u8::from_le_bytes(read::<1>(buf, off)?) as u64),
            };
        }
    }
    Some(u8::from_le_bytes(read::<1>(buf, off)?) as u64)
}

// ---------- recursive decoders ----------
fn decode_type_borsh_enum(
    type_def: &Value,
    idl: &Value,
    type_index: &std::collections::HashMap<&str, (&Value, bool)>,
    buf: &[u8],
    off: &mut usize,
    genv: &GenericEnv,
) -> Option<Value> {
    let t = type_def.get("type")?;
    let variants = t.get("variants")?.as_array()?;
    let repr = t.get("repr");
    let tag = read_enum_tag(buf, off, repr)?; // advances offset

    // choose variant by explicit discriminant or index
    let mut chosen: Option<&Value> = None;
    for (i, v) in variants.iter().enumerate() {
        if let Some(disc) = v.get("discriminant").and_then(|x| x.as_u64()) {
            if disc == tag { chosen = Some(v); break; }
        } else if tag == i as u64 {
            chosen = Some(v); break;
        }
    }
    let var = chosen?;
    let vname = var.get("name")?.as_str()?.to_string();
    let fields_v = var.get("fields");

    let payload = if let Some(fa) = fields_v.and_then(|x| x.as_array()) {
        if fa.is_empty() {
            Value::Object(serde_json::Map::new())
        } else if fa[0].get("name").is_some() {
            // struct-like
            let mut m = serde_json::Map::new();
            for f in fa {
                let fname = f.get("name")?.as_str()?;
                let fty   = f.get("type").unwrap_or(&Value::Null);
                let val = decode_type_borsh(fty, idl, type_index, buf, off, genv)
                    .unwrap_or(json!({"_error":"decode failed"}));
                m.insert(fname.to_string(), val);
            }
            Value::Object(m)
        } else {
            // tuple-like
            let mut arr = Vec::with_capacity(fa.len());
            for fty in fa {
                let fty = fty.get("type").unwrap_or(fty);
                let val = decode_type_borsh(fty, idl, type_index, buf, off, genv)
                    .unwrap_or(json!({"_error":"decode failed"}));
                arr.push(val);
            }
            Value::Array(arr)
        }
    } else {
        // unit variant
        Value::Object(serde_json::Map::new())
    };

    let mut outer = serde_json::Map::new();
    outer.insert(vname, payload);
    Some(Value::Object(outer))
}

fn decode_type_borsh(
    ty: &Value,
    idl: &Value,
    type_index: &std::collections::HashMap<&str, (&Value, bool)>,
    buf: &[u8],
    off: &mut usize,
    genv: &GenericEnv,
) -> Option<Value> {
    if let Some(resolved) = resolve_generic(ty, genv) {
        return decode_type_borsh(resolved, idl, type_index, buf, off, genv);
    }

    if let Some(s) = ty.as_str() {
        if let Some(p) = parse_prim_ty(s) { return decode_prim_borsh(buf, off, p); }
        if s == "string" { return decode_borsh_string(buf, off).map(Value::String); }
        if s == "bytes"  { return decode_borsh_bytes (buf, off).map(Value::String); }
        return Some(json!({"_unsupported_prim": s}));
    }

    if let Some(o) = ty.get("option") {
        let tag = read::<1>(buf, off)?[0];
        if tag == 0 { return Some(Value::Null); }
        if tag == 1 { return decode_type_borsh(o, idl, type_index, buf, off, genv); }
        return Some(json!({"_error":"invalid option tag"}));
    }

    if let Some(vt) = ty.get("vec") {
        let len = read_le_u32(buf, off)? as usize;
        let mut arr = Vec::with_capacity(len);
        for _ in 0..len {
            arr.push(decode_type_borsh(vt, idl, type_index, buf, off, genv)?);
        }
        return Some(Value::Array(arr));
    }

    if let Some(arr) = ty.get("array").and_then(|v| v.as_array()) {
        if arr.len() != 2 { return Some(json!({"_error":"invalid array schema"})); }
        let n = arr[1].as_u64().unwrap_or(0) as usize;
        return decode_fixed_array(n, |_| decode_type_borsh(&arr[0], idl, type_index, buf, off, genv));
    }

    if let Some(def) = ty.get("defined") {
        let name = if def.is_object() {
            def.get("name").and_then(|v| v.as_str())
        } else { def.as_str() }.unwrap_or("");
        let (defn, is_bytemuck) = match type_index.get(name) {
            Some(x) => *x,
            None => return Some(json!({"_unknown_defined_type": name})),
        };

        // build generic environment for this instantiation
        let mut new_env: GenericEnv = genv.clone();
        if let Some(inst_generics) = def.get("generics").and_then(|v| v.as_array()) {
            if let Some(def_generics) = defn.get("generics").and_then(|v| v.as_array()) {
                for (i, formal) in def_generics.iter().enumerate() {
                    if let Some(fname) = formal.get("name").and_then(|v| v.as_str()) {
                        if let Some(actual) = inst_generics.get(i) {
                            // FIX: resolve pass-through generics against outer env to avoid infinite recursion
                            let mut actual_ty = actual.get("type").unwrap_or(actual);
                            if let Some(resolved) = resolve_generic(actual_ty, genv) {
                                actual_ty = resolved;
                            }
                            new_env.insert(fname.to_string(), actual_ty);
                        }
                    }
                }
            }
        }

        match type_kind(defn).unwrap_or("struct") {
            "enum" => decode_type_borsh_enum(defn, idl, type_index, buf, off, &new_env),
            _ => {
                if is_bytemuck {
                    decode_type_bytemuck(defn, idl, type_index, buf, off, &new_env)
                } else {
                    decode_type_borsh_struct(defn, idl, type_index, buf, off, &new_env)
                }
            }
        }
    } else {
        Some(json!({"_unsupported_type": ty}))
    }
}

fn decode_type_borsh_struct(
    type_def: &Value,
    _idl: &Value,
    type_index: &std::collections::HashMap<&str, (&Value, bool)>,
    buf: &[u8],
    off: &mut usize,
    genv: &GenericEnv,
) -> Option<Value> {
    let fields = type_def.get("type")?.get("fields")?.as_array()?;
    let mut out = serde_json::Map::new();
    for f in fields {
        let name = f.get("name")?.as_str()?;
        let fty  = f.get("type")?;
        let val  = decode_type_borsh(fty, _idl, type_index, buf, off, genv)
            .unwrap_or(json!({"_error":"decode failed"}));
        out.insert(name.to_string(), val);
    }
    Some(Value::Object(out))
}

// Bytemuck struct = fixed-size, C layout; only fixed-size fields
fn decode_type_bytemuck(
    type_def: &Value,
    idl: &Value,
    type_index: &std::collections::HashMap<&str, (&Value, bool)>,
    buf: &[u8],
    off: &mut usize,
    genv: &GenericEnv,
) -> Option<Value> {
    if type_kind(type_def) == Some("enum") {
        return decode_type_borsh_enum(type_def, idl, type_index, buf, off, genv);
    }

    let fields = type_def.get("type")?.get("fields")?.as_array()?;
    let mut out = serde_json::Map::new();

    fn decode_bytemuck_field(
        ty: &Value,
        idl: &Value,
        type_index: &std::collections::HashMap<&str, (&Value, bool)>,
        buf: &[u8],
        off: &mut usize,
        genv: &GenericEnv,
    ) -> Option<Value> {
        if let Some(resolved) = resolve_generic(ty, genv) {
            return decode_bytemuck_field(resolved, idl, type_index, buf, off, genv);
        }

        if let Some(s) = ty.as_str() {
            if let Some(p) = parse_prim_ty(s) {
                return decode_prim_borsh(buf, off, p);
            }
            if s == "string" || s == "bytes" {
                return Some(json!({"_unsupported_in_bytemuck": s}));
            }
            return Some(json!({"_unsupported_prim": s}));
        }

        if let Some(arr) = ty.get("array").and_then(|v| v.as_array()) {
            if arr.len() != 2 { return Some(json!({"_error":"invalid array schema"})); }
            let n = arr[1].as_u64().unwrap_or(0) as usize;
            return decode_fixed_array(n, |_| decode_bytemuck_field(&arr[0], idl, type_index, buf, off, genv));
        }

        if let Some(def) = ty.get("defined") {
            let name = if def.is_object() {
                def.get("name").and_then(|v| v.as_str())
            } else { def.as_str() }.unwrap_or("");
            let (defn, is_b) = match type_index.get(name) {
                Some(x) => *x,
                None => return Some(json!({"_unknown_defined_type": name})),
            };

            let mut new_env: GenericEnv = genv.clone();
            if let Some(inst_generics) = def.get("generics").and_then(|v| v.as_array()) {
                if let Some(def_generics) = defn.get("generics").and_then(|v| v.as_array()) {
                    for (i, formal) in def_generics.iter().enumerate() {
                        if let Some(fname) = formal.get("name").and_then(|v| v.as_str()) {
                            if let Some(actual) = inst_generics.get(i) {
                                // FIX: resolve pass-through generics against outer env
                                let mut actual_ty = actual.get("type").unwrap_or(actual);
                                if let Some(resolved) = resolve_generic(actual_ty, genv) {
                                    actual_ty = resolved;
                                }
                                new_env.insert(fname.to_string(), actual_ty);
                            }
                        }
                    }
                }
            }

            if type_kind(defn) == Some("enum") {
                return decode_type_borsh_enum(defn, idl, type_index, buf, off, &new_env);
            }

            if is_b {
                return decode_type_bytemuck(defn, idl, type_index, buf, off, &new_env);
            } else {
                return decode_type_borsh_struct(defn, idl, type_index, buf, off, &new_env);
            }
        }

        if ty.get("vec").is_some() { return Some(json!({"_unsupported_in_bytemuck_vec": true})); }
        if ty.get("option").is_some() { return Some(json!({"_unsupported_in_bytemuck_option": true})); }
        Some(json!({"_unsupported_type": ty}))
    }

    for f in fields {
        let name = f.get("name")?.as_str()?;
        let fty  = f.get("type")?;
        let val  = decode_bytemuck_field(fty, idl, type_index, buf, off, genv)
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
    let genv: GenericEnv = GenericEnv::new();

    for arg in args {
        let Some(name) = arg.get("name").and_then(|v| v.as_str()) else { continue };
        let ty = arg.get("type").cloned().unwrap_or(Value::Null);

        let val = decode_type_borsh(&ty, idl, type_index, payload, &mut off, &genv)
            .unwrap_or(json!({"_error":"not enough bytes"}));

        out.insert(name.to_string(), val);
    }

    Value::Object(out)
}

// ---------- public entry ----------
#[wasm_bindgen]
pub fn parse(data_base58: &str, account_keys: JsValue) -> JsValue {
    let data = match b58decode(data_base58).into_vec() {
        Ok(d) => d,
        Err(e) => {
            return to_js(&json!({ "error": format!("Failed to decode base58: {}", e) }));
        }
    };
    if data.len() < 8 {
        return to_js(&json!({ "error":"Instruction/event data too short", "len": data.len() }));
    }

    let Some(idl) = load_idl() else {
        return to_js(&json!({ "error":"Failed to parse embedded IDL" }));
    };
    let type_index  = build_type_index(&idl);
    let event_index = build_event_index(&idl);

    // --- Event path: [0..8)=wrapper, [8..16)=event disc, [16..]=payload
    if data.len() >= 16 {
        let wrapper: [u8; 8] = { let mut x=[0u8;8]; x.copy_from_slice(&data[..8]); x };
        if wrapper == EVENT_LOG_DISCRIMINATOR {
            let disc: [u8; 8] = { let mut x=[0u8;8]; x.copy_from_slice(&data[8..16]); x };
            let payload = &data[16..];

            if let Some(ev_node) = event_index.get(&disc) {
                let ev_name = ev_node.get("name").and_then(|v| v.as_str()).unwrap_or("UnknownEvent");
                if find_type_by_name(&idl, ev_name).is_none() {
                    return to_js(&json!({ "error": format!("event type `{}` not found in IDL.types", ev_name) }));
                }
                let defined_ev = serde_json::json!({ "defined": { "name": ev_name } });
                let mut off = 0usize;
                let genv: GenericEnv = GenericEnv::new();
                if let Some(decoded) = decode_type_borsh(&defined_ev, &idl, &type_index, payload, &mut off, &genv) {
                    return to_js(&json!({
                        "event_type": ev_name,
                        "args": decoded
                    }));
                } else {
                    return to_js(&json!({ "error":"event decode failed at offset 16" }));
                }
            } else {
                return to_js(&json!({
                    "error": "Unknown event discriminator",
                    "discriminator": disc,
                    "len": data.len()
                }));
            }
        }
    }

    // --- Instruction fallback (only when wrapper mismatches) ---
    let disc0: [u8; 8] = { let mut x=[0u8;8]; x.copy_from_slice(&data[..8]); x };
    if let Some(ix) = find_instr_by_disc(&idl, &disc0) {
        // only now deserialize account keys (avoid doing it for events)
        let accounts_in: Vec<AccountInput> =
            serde_wasm_bindgen::from_value(account_keys).unwrap_or_default();
        let accounts_raw: Vec<String> = accounts_in.iter().map(|a| a.pubkey().to_string()).collect();

        let name_snake   = ix.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
        let name_display = snake_to_pascal(name_snake);
        let args_v       = ix.get("args").cloned().unwrap_or(serde_json::Value::Null);
        let args         = decode_args_from_idl(&args_v, &idl, &type_index, &data[8..]);

        // map accounts (flattened) using provided list
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

        return to_js(&json!({
            "instruction_type": name_display,
            "accounts": serde_json::Value::Object(accounts_map),
            "args": args
        }));
    }

    // No match
    to_js(&json!({
        "error": "Unknown discriminator",
        "len": data.len()
    }))
}
