// build.rs
use anyhow::{Context, Result};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs, process::Command};

fn main() -> Result<()> {
    // IDL path (env var or default)
    let path = "idls/pump_swap.json";

    // Load raw JSON
    let idl_src = fs::read_to_string(&path)
        .with_context(|| format!("failed to read IDL at {}", path))?;
    let idl: Value = serde_json::from_str(&idl_src)?;

    // Index types by name
    let types_arr = idl.get("types").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let mut type_by_name: HashMap<String, Value> = HashMap::new();
    for t in &types_arr {
        if let Some(name) = t.get("name").and_then(|x| x.as_str()) {
            type_by_name.insert(name.to_string(), t.clone());
        }
    }

    // ---------- typedefs (structs/enums), incl. tuple-structs ----------
    let mut typedefs_ts = Vec::new();
    for t in &types_arr {
        let Some(name) = t.get("name").and_then(|x| x.as_str()) else { continue };
        let name_ident = format_ident!("{}", name.to_upper_camel_case());
        let Some(ty) = t.get("type").and_then(|x| x.as_object()) else { continue };
        let kind = ty.get("kind").and_then(|x| x.as_str()).unwrap_or_default();

        match kind {
            "struct" => {
                let fields = ty.get("fields").cloned().unwrap_or(Value::Null);
                if let Some(arr) = fields.as_array() {
                    // tuple struct if elements are NOT {name, type} objects
                    let is_tuple = arr.iter().all(|el| !el.is_object() || !el.get("name").is_some());
                    if is_tuple {
                        let items = arr.iter().map(|v| map_json_type(v));
                        typedefs_ts.push(quote! {
                            #[derive(::borsh::BorshSerialize, ::borsh::BorshDeserialize, Clone, Debug, Serialize)]
                            pub struct #name_ident( #( #items ),* );
                        });
                    } else {
                        let named = arr.iter().map(|f| {
                            let fname = f.get("name").and_then(|x| x.as_str()).unwrap();
                            let f_ident = format_ident!("{}", fname.to_snake_case());
                            let fty = map_json_type(f.get("type").unwrap());
                            quote! { pub #f_ident: #fty, }
                        });
                        typedefs_ts.push(quote! {
                            #[derive(::borsh::BorshSerialize, ::borsh::BorshDeserialize, Clone, Debug, Serialize)]
                            pub struct #name_ident { #(#named)* }
                        });
                    }
                }
            }
            "enum" => {
                let variants = ty.get("variants").and_then(|v| v.as_array()).cloned().unwrap_or_default();
                let vdefs = variants.iter().map(|v| {
                    let vname = v.get("name").and_then(|x| x.as_str()).unwrap();
                    let v_ident = format_ident!("{}", vname.to_upper_camel_case());
                    match v.get("fields") {
                        None | Some(Value::Null) => quote! { #v_ident, },
                        Some(Value::Array(arr)) if arr.iter().all(|el| el.get("name").is_some()) => {
                            // named fields
                            let named = arr.iter().map(|f| {
                                let fname = f.get("name").and_then(|x| x.as_str()).unwrap();
                                let f_ident = format_ident!("{}", fname.to_snake_case());
                                let fty = map_json_type(f.get("type").unwrap());
                                quote! { #f_ident: #fty, }
                            });
                            quote! { #v_ident { #(#named)* }, }
                        }
                        Some(Value::Array(arr)) => {
                            // tuple fields
                            let tuple = arr.iter().map(|el| map_json_type(el));
                            quote! { #v_ident( #(#tuple)* ), }
                        }
                        _ => quote! { #v_ident, },
                    }
                });
                typedefs_ts.push(quote! {
                    #[derive(::borsh::BorshSerialize, ::borsh::BorshDeserialize, Clone, Debug, Serialize)]
                    pub enum #name_ident { #(#vdefs)* }
                });
            }
            _ => {}
        }
    }
    let typedefs_rs = quote! { #(#typedefs_ts)* };

    // ---------- instructions: args, accounts, decode arms ----------
    let mut args_structs = Vec::new();
    let mut accounts_structs = Vec::new();
    let mut arms = Vec::new();

    let instructions = idl.get("instructions").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    for ix in &instructions {
        let iname = ix.get("name").and_then(|x| x.as_str()).unwrap();
        let pascal = iname.to_upper_camel_case();
        let args_ty = format_ident!("{}Arguments", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        let variant = format_ident!("{}", pascal);

        // discriminator: from JSON or compute
        let disc = ix.get("discriminator")
            .and_then(|a| a.as_array())
            .and_then(|arr| {
                if arr.len() == 8 { Some(arr.iter().map(|n| n.as_u64().unwrap() as u8).collect::<Vec<_>>()) } else { None }
            })
            .unwrap_or_else(|| compute_ix_disc(iname));
        let disc_tokens = disc.iter().map(|b| quote! { #b });

        // args
        let args = ix.get("args").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let fields = args.iter().map(|a| {
            let aname = a.get("name").and_then(|x| x.as_str()).unwrap();
            let ident = format_ident!("{}", aname.to_snake_case());
            let tyv = a.get("type").unwrap();
            let ty = map_json_type(tyv);
            if is_u64_or_u128(tyv) {
                quote! {
                    #[serde(serialize_with = "super::serialize_to_string")]
                    pub #ident: #ty,
                }
            } else {
                quote! { pub #ident: #ty, }
            }
        });
        args_structs.push(quote! {
            #[derive(::borsh::BorshDeserialize, Debug, Serialize)]
            pub struct #args_ty { #(#fields)* }
        });

        // accounts (flatten)
        let accs = ix.get("accounts").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let flat_names = flatten_accounts_json(&accs);
        let acc_fields = flat_names.iter().map(|n| {
            let f = format_ident!("{}", n);
            quote! { pub #f: String, }
        });
        accounts_structs.push(quote! {
            #[derive(Debug, Serialize)]
            pub struct #accounts_ty { #(#acc_fields)* pub remaining: Vec<String>, }
        });

        // decode arm
        let idents = flat_names.iter().map(|n| format_ident!("{}", n)).collect::<Vec<_>>();
        let extract = idents.iter().map(|id| quote! { let #id = keys.next().unwrap().clone(); });
        arms.push(quote! {
            [#(#disc_tokens),*] => {
                let mut rdr: &[u8] = rest;
                let args = #args_ty::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                #(#extract)*
                let remaining = keys.cloned().collect();
                let accounts = #accounts_ty { #(#idents),*, remaining };
                return Ok(Instruction::#variant { accounts, args });
            }
        });
    }

    let instr_variants = instructions.iter().map(|ix| {
        let pascal = ix.get("name").and_then(|x| x.as_str()).unwrap().to_upper_camel_case();
        let var = format_ident!("{}", pascal);
        let args_ty = format_ident!("{}Arguments", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        quote! { #var { accounts: #accounts_ty, args: #args_ty }, }
    });
    let instruction_enum = quote! {
        #[derive(Debug, Serialize)]
        #[serde(tag = "instruction_type")]
        pub enum Instruction { #(#instr_variants)* }
    };
    let instruction_impl = quote! {
        impl Instruction {
            pub fn decode(account_keys: &[String], data: &[u8]) -> anyhow::Result<Self> {
                if data.len() < 8 { anyhow::bail!("Data too short: {}", data.len()); }
                let (disc_slice, rest) = data.split_at(8);
                let disc: [u8; 8] = disc_slice.try_into().unwrap();
                match disc {
                    #(#arms)*
                    _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
                }
            }
        }
    };

    // ---------- events: structs from types + enum + decoder ----------
    let mut event_structs = Vec::new();
    let mut event_arms = Vec::new();

    let events = idl.get("events").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    for ev in &events {
        let name = ev.get("name").and_then(|x| x.as_str()).unwrap();
        let pascal = format_ident!("{}", name.to_upper_camel_case());

        // find matching struct type definition
        let Some(tdef) = type_by_name.get(name) else { continue };
        let ty = tdef.get("type").and_then(|x| x.as_object()).unwrap();
        let fields = ty.get("fields").and_then(|x| x.as_array()).cloned().unwrap_or_default();

        // fields with pubkey serde helpers when appropriate
        let mut field_ts: Vec<TokenStream> = Vec::new();
        for f in &fields {
            let fname = f.get("name").and_then(|x| x.as_str()).unwrap();
            let f_ident = format_ident!("{}", fname.to_snake_case());
            let fty_v = f.get("type").unwrap();

            let tokens = if is_public_key(fty_v) {
                quote! {
                    #[serde(with = "pubkey_serde")]
                    pub #f_ident: [u8; 32usize],
                }
            } else if is_option_public_key(fty_v) {
                quote! {
                    #[serde(with = "pubkey_serde_option")]
                    pub #f_ident: Option<[u8; 32usize]>,
                }
            } else {
                let fty = map_json_type(fty_v);
                quote! { pub #f_ident: #fty, }
            };
            field_ts.push(tokens);
        }

        event_structs.push(quote! {
            #[derive(::borsh::BorshDeserialize, Debug, Serialize)]
            pub struct #pascal { #(#field_ts)* }
        });

        // event discriminator
        let disc = ev.get("discriminator")
            .and_then(|a| a.as_array())
            .and_then(|arr| {
                if arr.len() == 8 { Some(arr.iter().map(|n| n.as_u64().unwrap() as u8).collect::<Vec<_>>()) } else { None }
            })
            .unwrap_or_else(|| compute_event_disc(name));
        let disc_tokens = disc.iter().map(|b| quote! { #b });

        event_arms.push(quote! {
            [#(#disc_tokens),*] => {
                let mut rdr = &payload[..];
                let args = #pascal::deserialize(&mut rdr)?;
                return Ok(Event::#pascal{ args });
            }
        });
    }

    let decoded_events = if !event_structs.is_empty() {
        let variants = events.iter()
            .filter_map(|ev| ev.get("name").and_then(|x| x.as_str()))
            .map(|n| {
                let pascal = format_ident!("{}", n.to_upper_camel_case());
                quote! { #pascal { args: #pascal }, }
            });
        Some(quote! {
            #[derive(Debug, Serialize)]
            #[serde(tag = "event_type")]
            pub enum Event { #(#variants)* }
        })
    } else { None };

    let event_decode_impl = if !event_arms.is_empty() {
        Some(quote! {
            pub const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
            impl Event {
                pub fn decode(data: &[u8]) -> anyhow::Result<Self> {
                    if data.len() < 16 { anyhow::bail!("Event log too short: {}", data.len()); }
                    let (wrapper, rest) = data.split_at(8);
                    if wrapper != EVENT_LOG_DISCRIMINATOR {
                        anyhow::bail!("Missing event log prefix");
                    }
                    let (disc_slice, payload) = rest.split_at(8);
                    let disc: [u8;8] = disc_slice.try_into().unwrap();
                    match disc {
                        #(#event_arms)*
                        _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
                    }
                }
            }
        })
    } else { None };

    // ---------- write src/idl.rs ----------
    let out = quote! {
        // @generated by build.rs — DO NOT EDIT
        #[allow(dead_code)]
        use std::convert::TryInto;
        use serde::Serializer;

        // match your lib.rs layout:
        use crate::pubkey_serializer::pubkey_serde;
        use crate::pubkey_serializer::pubkey_serde_option;

        // helper for big ints-as-strings in JSON
        fn serialize_to_string<S, T>(x: &T, s: S) -> Result<S::Ok, S::Error>
        where S: Serializer, T: ToString {
            s.serialize_str(&x.to_string())
        }

        pub use ix_data::*;
        pub use typedefs::*;
        pub use accounts_data::*;

        pub mod typedefs {
            use serde::Serialize;
            #typedefs_rs
        }

        pub mod accounts_data {
            use serde::Serialize;
            #(#accounts_structs)*
        }

        pub mod ix_data {
            use serde::Serialize;
            use super::*;
            #(#args_structs)*
        }

        #instruction_enum
        #instruction_impl

        pub mod events {
            use super::*;
            use serde::Serialize;
            #(#event_structs)*
            #decoded_events
            #event_decode_impl
        }
    };

    fs::write("src/idl.rs", out.to_string())
        .context("failed to write src/idl.rs")?;
    let _ = Command::new("rustfmt").arg("src/idl.rs").status();
    Ok(())
}

// ---------------- helpers ----------------

fn compute_ix_disc(name_snake: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(b"global:");
    hasher.update(name_snake.to_snake_case());
    hasher.finalize()[0..8].to_vec()
}

fn compute_event_disc(name: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(b"event:");
    hasher.update(name.as_bytes());
    hasher.finalize()[0..8].to_vec()
}

fn is_u64_or_u128(ty: &Value) -> bool {
    match ty {
        Value::String(s) => s == "u64" || s == "u128",
        Value::Object(o) => o.get("defined")
            .and_then(|d| d.as_str())
            .map(|s| s == "u64" || s == "u128")
            .unwrap_or(false),
        _ => false,
    }
}

fn is_public_key(ty: &Value) -> bool {
    match ty {
        Value::String(s) => s == "publicKey" || s == "pubkey",
        Value::Object(_) => false,
        _ => false,
    }
}

fn is_option_public_key(ty: &Value) -> bool {
    if let Value::Object(o) = ty {
        if let Some(inner) = o.get("option") {
            return is_public_key(inner);
        }
    }
    false
}

// map JSON IDL type → Rust tokens
fn map_json_type(v: &Value) -> TokenStream {
    if let Some(s) = v.as_str() {
        return match s {
            "bool" => quote! { bool },
            "u8" => quote! { u8 },
            "i8" => quote! { i8 },
            "u16" => quote! { u16 },
            "i16" => quote! { i16 },
            "u32" => quote! { u32 },
            "i32" => quote! { i32 },
            "u64" => quote! { u64 },
            "i64" => quote! { i64 },
            "u128" => quote! { u128 },
            "i128" => quote! { i128 },
            "bytes" => quote! { Vec<u8> },
            "string" => quote! { String },
            "publicKey" | "pubkey" => quote! { [u8; 32usize] },
            other => {
                let id = format_ident!("{}", other);
                quote! { #id }
            }
        };
    }
    if let Some(obj) = v.as_object() {
        if let Some(def) = obj.get("defined") {
            if let Some(s) = def.as_str() {
                let id = format_ident!("{}", s);
                return quote! { #id };
            }
            if let Some(o) = def.as_object() {
                if let Some(n) = o.get("name").and_then(|x| x.as_str()) {
                    let id = format_ident!("{}", n);
                    return quote! { #id };
                }
            }
        }
        if let Some(inner) = obj.get("option") {
            let t = map_json_type(inner);
            return quote! { Option<#t> };
        }
        if let Some(inner) = obj.get("vec") {
            let t = map_json_type(inner);
            return quote! { Vec<#t> };
        }
        if let Some(arr) = obj.get("array").and_then(|x| x.as_array()) {
            if arr.len() == 2 {
                let t = map_json_type(&arr[0]);
                let len = arr[1].as_u64().unwrap_or(0) as usize;
                return quote! { [#t; #len] };
            }
        }
    }
    panic!("unsupported JSON IDL type: {:?}", v);
}

// flatten instruction accounts (handles groups)
fn flatten_accounts_json(items: &[Value]) -> Vec<String> {
    let mut out = Vec::new();
    fn rec(list: &[Value], out: &mut Vec<String>) {
        for item in list {
            if let Some(sub) = item.get("accounts").and_then(|x| x.as_array()) {
                rec(sub, out);
            } else if let Some(name) = item.get("name").and_then(|x| x.as_str()) {
                out.push(name.to_string());
            }
        }
    }
    rec(items, &mut out);
    out
}
