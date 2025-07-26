// build.rs

use anchor_idl::{EnumFields, GeneratorOptions, Idl, IdlAccountItem, IdlTypeDefinitionTy};
use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::Literal;
use quote::{format_ident, quote};
use serde_json::from_str;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, process::Command};

fn main() -> Result<()> {
    let path = "idls/kamino_farm.json";

    // 1) load the IDL JSON
    let idl_json = fs::read_to_string(path)?;
    let idl: Idl = from_str(&idl_json)?;
    let raw: Value = from_str(&idl_json)?;

    // 2) hand‐roll typedefs so that enums with fields work correctly
    let typedefs_rs = {
        let mut tts = Vec::new();

        for def in &idl.types {
            let name = format_ident!("{}", def.name.to_upper_camel_case());
            match &def.ty {
                // Structs are unchanged
                IdlTypeDefinitionTy::Struct { fields } => {
                    let fields_ts = fields.iter().map(|f| {
                        let f_ident = format_ident!("{}", f.name.to_snake_case());
                        match &f.ty {

                            anchor_idl::IdlType::U64 | anchor_idl::IdlType::U128 => {
                                let ty = map_idl_type(&f.ty);
                                quote! {
                                    #[serde(serialize_with = "crate::serialize_to_string")]
                                    pub #f_ident: #ty,
                                }
                            }
                            // a bare PublicKey
                            anchor_idl::IdlType::PublicKey => {
                                quote! {
                                    #[serde(with = "pubkey_serde")]
                                    pub #f_ident: [u8; 32usize],
                                }
                            }
                            // Option<PublicKey>
                            anchor_idl::IdlType::Option(inner)
                                if matches!(**inner, anchor_idl::IdlType::PublicKey) =>
                            {
                                quote! {
                                    #[serde(with = "pubkey_serde_option")]
                                    pub #f_ident: Option<[u8; 32usize]>,
                                }
                            }
                            anchor_idl::IdlType::Array(inner, len) => {
                                let l = *len as usize;
                                let inner_type_ts = map_idl_type(inner);

                                if l > 32 {
                                    quote! {
                                        #[serde(with = "BigArray")]
                                        pub #f_ident: [#inner_type_ts; #l],
                                    }
                                } else {
                                    quote! {
                                        pub #f_ident: [#inner_type_ts; #l],
                                    }
                                }
                            }
                            // everything else
                            _ => {
                                let ty = map_idl_type(&f.ty);
                                quote! { pub #f_ident: #ty, }
                            }
                        }
                    });
                    tts.push(quote! {
                        #[derive(::borsh::BorshSerialize, ::borsh::BorshDeserialize, Clone, Debug, Serialize)]
                        pub struct #name {
                            #(#fields_ts)*
                        }
                    });
                }
                // Enums: support unit, named and tuple variants
                IdlTypeDefinitionTy::Enum { variants } => {
                    // Build each variant’s definition
                    let variant_defs = variants
                        .iter()
                        .map(|v| {
                            let v_ident = format_ident!("{}", v.name.to_upper_camel_case());
                            match &v.fields {
                                None => {
                                    // unit variant
                                    quote! { #v_ident, }
                                }
                                Some(EnumFields::Named(fields)) => {
                                    // named‐field variant
                                    let named = fields.iter().map(|f| {
                                        let f_ident = format_ident!("{}", f.name.to_snake_case());
                                        match &f.ty {

                                            anchor_idl::IdlType::U64 | anchor_idl::IdlType::U128 => {
                                                let ty = map_idl_type(&f.ty);
                                                quote! {
                                                    #[serde(serialize_with = "crate::serialize_to_string")]
                                                    pub #f_ident: #ty
                                                }
                                            }
                                            // a bare PublicKey
                                            anchor_idl::IdlType::PublicKey => {
                                                quote! {
                                                    #[serde(with = "pubkey_serde")]
                                                     #f_ident: [u8; 32usize],
                                                }
                                            }
                                            // Option<PublicKey>
                                            anchor_idl::IdlType::Option(inner)
                                                if matches!(
                                                    **inner,
                                                    anchor_idl::IdlType::PublicKey
                                                ) =>
                                            {
                                                quote! {
                                                    #[serde(with = "pubkey_serde_option")]
                                                    #f_ident: Option<[u8; 32usize]>,
                                                }
                                            }
                                            anchor_idl::IdlType::Array(inner, len) => {
                                                let l = *len as usize;
                                                let inner_type_ts = map_idl_type(inner);

                                                if l > 32 {
                                                    quote! {
                                                        #[serde(with = "BigArray")]
                                                        pub #f_ident: [#inner_type_ts; #l],
                                                    }
                                                } else {
                                                    quote! {
                                                        pub #f_ident: [#inner_type_ts; #l],
                                                    }
                                                }
                                            }
                                            // everything else
                                            _ => {
                                                let ty = map_idl_type(&f.ty);
                                                quote! { #f_ident: #ty, }
                                            }
                                        }
                                    });
                                    quote! { #v_ident { #(#named)* }, }
                                }
                                Some(EnumFields::Tuple(types)) => {
                                    // tuple‐field variant
                                    let tuple = types.iter().map(|t| {
                                        let ty_ts = map_idl_type(t);
                                        quote! { #ty_ts, }
                                    });
                                    quote! { #v_ident( #(#tuple)* ), }
                                }
                            }
                        })
                        .collect::<Vec<_>>(); // Pick the first variant as default

                    tts.push(quote! {
                        #[derive(::borsh::BorshSerialize, ::borsh::BorshDeserialize, Clone, Debug, Serialize)]
                        pub enum #name {
                            #(#variant_defs)*
                        }
                    });
                }
            }
        }

        quote! { #(#tts)* }
    };

    // 3) build each Args and Accounts struct + decoder arms (unchanged)
    let mut args_structs = Vec::new();
    let mut accounts_structs = Vec::new();
    let mut arms = Vec::new();

    for ix in &idl.instructions {
        let pascal = ix.name.to_upper_camel_case();
        let args_ty = format_ident!("{}Arguments", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        let variant = format_ident!("{}", pascal);

        // 3a) 8‐byte discriminator
        let mut hasher = Sha256::new();
        hasher.update(b"global:");
        hasher.update(ix.name.to_snake_case());
        let hash = hasher.finalize();
        let disc: [u8; 8] = hash[0..8].try_into().unwrap();
        let disc_tokens = disc.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

        // 3b) Args struct
        let args_fields = ix.args.iter().map(|arg| {
            let field_ident = format_ident!("{}", arg.name.to_snake_case());
            let ty = map_idl_type(&arg.ty);
            // check for U64 / U128
            let field_tokens = match &arg.ty {
                anchor_idl::IdlType::U64 | anchor_idl::IdlType::U128 => {
                    quote! {
                        #[serde(serialize_with = "crate::serialize_to_string")]
                        pub #field_ident: #ty,
                    }
                }
                &anchor_idl::IdlType::PublicKey => {
                    quote! {
                        #[serde(with = "pubkey_serde")]
                        pub #field_ident: [u8; 32usize],
                    }
                }
                // Option<PublicKey> → Option<[u8;32]> + its serde
                anchor_idl::IdlType::Option(inner)
                    if matches!(inner.as_ref(), &anchor_idl::IdlType::PublicKey) =>
                {
                    quote! {
                        #[serde(with = "pubkey_serde_option")]
                        pub #field_ident: Option<[u8; 32usize]>,
                    }
                }
                anchor_idl::IdlType::Array(inner, len) => {
                    let l = *len as usize;
                    let inner_type_ts = map_idl_type(inner);

                    if l > 32 {
                        quote! {
                            #[serde(with = "BigArray")]
                            pub #field_ident: [#inner_type_ts; #l],
                        }
                    } else {
                        quote! {
                            pub #field_ident: [#inner_type_ts; #l],
                        }
                    }
                }
                _ => {
                    quote! {
                        pub #field_ident: #ty,
                    }
                }
            };

            field_tokens
        });
        args_structs.push(quote! {
            #[derive(::borsh::BorshDeserialize, Debug, Serialize)]
            pub struct #args_ty {
                #(#args_fields)*
            }
        });

        // 3c) Accounts struct
        let flat = flatten_accounts(&ix.accounts);
        let acc_fields = flat.iter().map(|acc| {
            let is_optional = raw["instructions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|instr| instr["name"] == ix.name)
                .and_then(|instr| {
                    instr["accounts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|a| a["name"] == acc.name)
                        .and_then(|a| a["optional"].as_bool())
                })
                .unwrap_or(false);

            let f = format_ident!("{}", acc.name);
            if is_optional {
                quote! {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub #f: Option<String>,
                }
            } else {
                quote! { pub #f: String, }
            }
        });
        accounts_structs.push(quote! {
            #[derive(Debug, Serialize)]
            pub struct #accounts_ty {
                #(#acc_fields)*
                pub remaining: Vec<String>,
            }
        });

        let mut parse_args = Vec::new();
        let mut arg_idents = Vec::new();
        for arg in &ix.args {
            let fname = format_ident!("{}", arg.name.to_snake_case());
            let fty = map_idl_type(&arg.ty);
            let snippet = match &arg.ty {
                anchor_idl::IdlType::Option(inner) => {
                    let inner_ty = map_idl_type(inner);
                    quote! {
                        let #fname: Option<#inner_ty> = parse_option(&mut rdr)?;
                    }
                }
                anchor_idl::IdlType::Bytes => quote! {
                    let #fname: Vec<u8> = rdr.to_vec();
                },
                _ => quote! {
                    let #fname: #fty = <#fty as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                },
            };
            parse_args.push(snippet);
            arg_idents.push(fname);
        }

        // 3d) decoder arm
        let mut extract = Vec::new();
        let mandatory_count = raw["instructions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|instr| instr["name"] == ix.name)
            .and_then(|instr| instr["accounts"].as_array())
            .unwrap()
            .iter()
            .filter(|a| !a.get("optional").and_then(|b| b.as_bool()).unwrap_or(false))
            .count();

        // iterate the keys once
        extract.push(quote! {
            let mut keys = account_keys.iter();
            // How many optional slots were actually supplied?
            let mut opt_budget = account_keys.len().saturating_sub(#mandatory_count);
        });

        for acc in &flat {
            let name = format_ident!("{}", acc.name);
            // check optional flag again
            let is_opt = raw["instructions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|instr| instr["name"] == ix.name)
                .and_then(|instr| instr["accounts"].as_array())
                .unwrap()
                .iter()
                .find(|a| a["name"] == acc.name)
                .and_then(|a| a["optional"].as_bool())
                .unwrap_or(false);

            if is_opt {
                extract.push(quote! {
                    let #name = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                });
            } else {
                extract.push(quote! {
                    let #name = keys.next().unwrap().clone();
                });
            }
        }

        let idents = flat
            .iter()
            .map(|acc| format_ident!("{}", acc.name))
            .collect::<Vec<_>>();

        arms.push(quote! {
            [#(#disc_tokens),*] => {
                let mut rdr: &[u8] = rest;
                #(#parse_args)*
                let args = #args_ty { #(#arg_idents),* };
                let mut keys = account_keys.iter();
                #(#extract)*
                let remaining = keys.cloned().collect();
                let accounts = #accounts_ty { remaining, #(#idents),* };
                return Ok(Instruction::#variant { accounts, args });
            }
        });
    }

    // 4) enum Instruction
    let variants = idl.instructions.iter().map(|ix| {
        let pascal = ix.name.to_upper_camel_case();
        let var = format_ident!("{}", pascal);
        let args_ty = format_ident!("{}Arguments", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        quote! { #var { accounts: #accounts_ty, args: #args_ty }, }
    });
    let decoded_enum = quote! {
        #[derive(Debug, Serialize)]
        #[serde(tag = "instruction_type")]
        pub enum Instruction {
            #(#variants)*
        }
    };

    // 5) impl decode()
    let decode_impl = quote! {
        impl Instruction {
            pub fn decode(
                account_keys: &[String],
                data: &[u8],
            ) -> anyhow::Result<Self> {
                if data.len() < 8 {
                    anyhow::bail!("Data too short: {}", data.len());
                }
                let (disc_slice, rest) = data.split_at(8);
                let disc: [u8; 8] = disc_slice.try_into().unwrap();
                match disc {
                    #(#arms)*
                    _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
                }
            }
        }
    };

    // 1) prepare vectors for the generated structs & match arms
    let mut event_structs = Vec::new();
    let mut event_arms = Vec::new();

    // 2) iterate only if the IDL actually has events
    if let Some(events) = &idl.events {
        for ev in events {
            // A) PascalCase name + struct fields
            let pascal = format_ident!("{}", ev.name.to_upper_camel_case());
            let mut fields = Vec::new();
            for f in &ev.fields {
                let fld = format_ident!("{}", f.name.to_snake_case());

                // pick the right tokens based on your IDL type
                let field_tokens = match &f.ty {

                    anchor_idl::IdlType::U64 | anchor_idl::IdlType::U128 => {
                        let ty = map_idl_type(&f.ty);
                        quote! {
                            #[serde(serialize_with = "crate::serialize_to_string")]
                            pub #fld: #ty,
                        }
                    }
                    // plain PublicKey → [u8;32] + base58 serde
                    &anchor_idl::IdlType::PublicKey => {
                        quote! {
                            #[serde(with = "pubkey_serde")]
                            pub #fld: [u8; 32usize],
                        }
                    }
                    // Option<PublicKey> → Option<[u8;32]> + its serde
                    anchor_idl::IdlType::Option(inner)
                        if matches!(inner.as_ref(), &anchor_idl::IdlType::PublicKey) =>
                    {
                        quote! {
                            #[serde(with = "pubkey_serde_option")]
                            pub #fld: Option<[u8; 32usize]>,
                        }
                    }
                    // everything else: fall back to your existing logic
                    _ => {
                        let ty = map_idl_type(&f.ty);
                        quote! {
                            pub #fld: #ty,
                        }
                    }
                };

                fields.push(field_tokens);
            }

            // B) push the `[derive(BorshDeserialize, Debug, Serialize)] pub struct …`
            event_structs.push(quote! {
                #[derive(::borsh::BorshDeserialize, Debug, Serialize)]
                pub struct #pascal {
                    #(#fields)*
                }
            });

            // C) compute the 8-byte event discriminator (same as instructions)
            let mut hasher = Sha256::new();
            hasher.update(b"event:");
            hasher.update(ev.name.as_bytes());
            let hash = hasher.finalize();
            let disc: [u8; 8] = hash[0..8].try_into().unwrap();
            let disc_tokens = disc.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

            // D) push the match‐arm for this event
            event_arms.push(quote! {
                [#(#disc_tokens),*] => {
                    let mut rdr = &payload[..];
                    let args = #pascal::deserialize(&mut rdr)?;
                    return Ok(Event::#pascal{ args });
                }
            });
        }
    }

    // 3) build the Event enum if we saw any structs
    let decoded_events = if !event_structs.is_empty() {
        let variants = idl.events.as_ref().unwrap().iter().map(|ev| {
            let pascal = format_ident!("{}", ev.name.to_upper_camel_case());
            quote! { #pascal{ args: #pascal}, }
        });
        Some(quote! {
            #[derive(Debug, Serialize)]
            #[serde(tag = "event_type")]
            pub enum Event {
                #(#variants)*
            }
        })
    } else {
        None
    };

    // 4) emit the `impl Event::decode` (with wrapper + inner discriminator)
    let event_decode_impl = if !event_arms.is_empty() {
        Some(quote! {
            // Anchor’s fixed “log prefix” for events
            pub const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];

            impl Event {
                /// Decode a raw Anchor‐logged event:
                ///  [ EVENT_LOG_DISCRIMINATOR (8) ]
                ///  [ REAL_EVENT_DISCRIMINATOR (8) ]
                ///  [ Borsh payload …           ]
                pub fn decode(data: &[u8]) -> anyhow::Result<Self> {
                    // need at least wrapper + real disc
                    if data.len() < 16 {
                        anyhow::bail!("Event log too short: {}", data.len());
                    }
                    // 1) strip off the 8-byte wrapper
                    let (wrapper, rest) = data.split_at(8);
                    if wrapper != EVENT_LOG_DISCRIMINATOR {
                        anyhow::bail!(
                            "Missing event log prefix: expected {:x?}, got {:x?}",
                            EVENT_LOG_DISCRIMINATOR,
                            wrapper
                        );
                    }
                    // 2) next 8 bytes = real discriminator
                    let (disc_slice, payload) = rest.split_at(8);
                    let disc: [u8;8] = disc_slice.try_into().unwrap();
                    // 3) dispatch and Borsh‐deserialize
                    match disc {
                        #(#event_arms)*
                        _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
                    }
                }
            }
        })
    } else {
        None
    };

    let parse_option_fn = quote! {
        /// Parse an Option<T> in either old‑IDL (no tag) or new‑IDL (0x00/0x01 prefix) form
        fn parse_option<T: ::borsh::BorshDeserialize>(
            rdr: &mut &[u8],
        ) -> anyhow::Result<Option<T>> {
            if rdr.is_empty() {
                return Ok(None);
            }
            let tag = rdr[0];
            if tag == 0 {
                *rdr = &rdr[1..];
                return Ok(None);
            } else if tag == 1 {
                *rdr = &rdr[1..];
                let v = T::deserialize(rdr)?;
                return Ok(Some(v));
            }
            let v = T::deserialize(rdr)?;
            Ok(Some(v))
        }
    };

    // 6) write out
    let out = quote! {
        // @generated by build.rs — DO NOT EDIT
        #[allow(dead_code)]
        use std::convert::TryInto;
        use serde::Serializer;
        fn serialize_to_string<S, T>(x: &T, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
            T: ToString,
        {
            s.serialize_str(&x.to_string())
        }

        #parse_option_fn

        pub use ix_data::*;
        pub use typedefs::*;
        pub use accounts_data::*;

        pub mod typedefs {
            use ::borsh::{BorshSerialize, BorshDeserialize};
            use anchor_lang::prelude::*;
            use crate::pubkey_serializer::pubkey_serde;
            use crate::pubkey_serializer::pubkey_serde_option;
            use serde::Serialize;
            serde_big_array::big_array! { BigArray; 64, 51, 72, 128, 256 /*, …other sizes…*/ }
            #typedefs_rs
        }

        pub mod accounts_data {
            use serde::Serialize;
            #(#accounts_structs)*
        }

        pub mod ix_data {
            use serde::Serialize;
            use super::*;
            use crate::pubkey_serializer::pubkey_serde;
            use crate::pubkey_serializer::pubkey_serde_option;
            serde_big_array::big_array! { BigArray; 64, 51, 72, 128, 256 /*, …other sizes…*/ }
            #(#args_structs)*
        }

        #decoded_enum
        #decode_impl


        pub mod events {
            use super::*;
            pub use typedefs::*;
            use ::borsh::BorshDeserialize;
            use serde::Serialize;
            use crate::pubkey_serializer::pubkey_serde;
            use crate::pubkey_serializer::pubkey_serde_option;

            #(#event_structs)*

            #decoded_events
            #event_decode_impl
        }
    };

    fs::write("src/idl.rs", out.to_string())?;
    Command::new("rustfmt").arg("src/idl.rs").status()?;
    Ok(())
}

// --- helper functions unchanged ---

fn map_idl_type(ty: &anchor_idl::IdlType) -> proc_macro2::TokenStream {
    use anchor_idl::IdlType::*;
    match ty {
        Bool => quote! { bool },
        U8 => quote! { u8 },
        I8 => quote! { i8 },
        U16 => quote! { u16 },
        I16 => quote! { i16 },
        U32 => quote! { u32 },
        I32 => quote! { i32 },
        U64 => quote! { u64 },
        I64 => quote! { i64 },
        U128 => quote! { u128 },
        I128 => quote! { i128 },
        Bytes => quote! { Vec<u8> }, // This might need to be handled as raw bytes without length prefix
        String => quote! { String },
        PublicKey => quote! { [u8; 32usize] },
        Vec(inner) => {
            let i = map_idl_type(inner);
            quote! { Vec<#i> }
        }
        Array(inner, len) => {
            let i = map_idl_type(inner);
            let l = *len;
            quote! { [#i; #l] }
        }
        Defined(name) => {
            let id = format_ident!("{}", name);
            quote! { #id }
        }
        Option(inner) => {
            let i = map_idl_type(inner);
            quote! { Option<#i> }
        }
        other => panic!("unsupported IDL type: {:?}", other),
    }
}

fn flatten_accounts<'a>(items: &'a [IdlAccountItem]) -> Vec<&'a anchor_idl::IdlAccount> {
    let mut out = Vec::new();
    for item in items {
        match item {
            IdlAccountItem::IdlAccount(acc) => out.push(acc),
            IdlAccountItem::IdlAccounts(group) => out.extend(flatten_accounts(&group.accounts)),
        }
    }
    out
}
