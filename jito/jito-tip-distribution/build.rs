// build.rs

use anchor_idl::{GeneratorOptions, Idl, IdlArrayLen, IdlInstructionAccountItem, IdlType};
use anyhow::Result;
use heck::{ ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use serde_json::from_str;
use sha2::{Digest, Sha256};
use std::{fs, process::Command};
use serde_json::Value;

fn main() -> Result<()> {
    let path = "idls/jito_tip_distribution.json";
    let raw = fs::read_to_string(path)?;
    let normalized = raw.replace(r#"\"writable\""#, r#"\"mut\""#);
    let idl: Idl = from_str(&normalized)?;
    let raw: Value = from_str(&normalized)?;

    let mut opts = GeneratorOptions::default();
    opts.idl_path = path.into();
    let gen = opts.to_generator();
    let typedefs_ts = anchor_idl::generate_typedefs(&gen.idl.types, &gen.struct_opts);

    // 3) build Args and Accounts structs and match arms
    let mut args_structs = Vec::new();
    let mut accounts_structs = Vec::new();
    let mut arms = Vec::new();

    for ix in &idl.instructions {
        let name = &ix.name;
        let pascal = name.to_upper_camel_case();

        let args_ty = format_ident!("{}Args", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        let variant = format_ident!("{}", pascal);
        

        // compute discriminator
        let mut hasher = Sha256::new();
        hasher.update(b"global:");
        hasher.update(name.to_snake_case());
        let hash = hasher.finalize();
        let disc_bytes: [u8; 8] = hash[0..8].try_into().unwrap();
        let disc_tokens = disc_bytes.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

        // build Args struct
        let args_fields = ix.args.iter().map(|arg| {
            let field_ident = format_ident!("{}", arg.name.to_snake_case());
            let ty = map_idl_type(&arg.ty);
            let field_tokens = match &arg.ty {
                anchor_idl::IdlType::U64 | anchor_idl::IdlType::U128 => {
                    quote! {
                        #[serde(serialize_with = "crate::serialize_to_string")]
                        pub #field_ident: #ty,
                    }
                }
                anchor_idl::IdlType::Pubkey => {
                    quote! {
                        #[serde(with = "pubkey_serde")]
                        pub #field_ident: [u8; 32usize],
                    }
                }
                // Option<PublicKey>
                anchor_idl::IdlType::Option(inner)
                    if matches!(**inner, anchor_idl::IdlType::Pubkey) =>
                {
                    quote! {
                        #[serde(with = "pubkey_serde_option")]
                        pub #field_ident: Option<[u8; 32usize]>,
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

        // build match arm
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
        let mut arg_idents= Vec::new();
        for arg in &ix.args {
            let fname = format_ident!("{}", arg.name.to_snake_case());
            let fty   = map_idl_type(&arg.ty);
            let snippet = match &arg.ty {
                anchor_idl::IdlType::Option(inner) => {
                    let inner_ty = map_idl_type(inner);
                    quote! {
                        let #fname: Option<#inner_ty> = parse_option(&mut rdr)?;
                    }
                }
                _ => quote! {
                    let #fname: #fty = <#fty as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                }
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
        extract.push(quote! { let mut keys = account_keys.iter(); });
        // true if this invocation has the extra slot
        extract.push(quote! { let has_extra = account_keys.len() > #mandatory_count; });

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
                    let #name = if has_extra {
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
                let accounts = #accounts_ty { #(#idents),*, remaining };
                return Ok(Instruction::#variant { accounts, args });
            }
        });
    
    }

    // 4) Instruction enum
    let variants = idl.instructions.iter().map(|ix| {
        let pascal = ix.name.to_upper_camel_case();
        let var = format_ident!("{}", pascal);
        let args_ty = format_ident!("{}Args", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        quote! { #var { accounts: #accounts_ty, args: #args_ty }, }
    });
    let decoded_enum = quote! {
        #[derive(Debug, Serialize)]
        #[serde(tag = "instruction_type")]
        pub enum Instruction { #(#variants)* }
    };

    // 5) decode implementation
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

    let mut file = syn::parse2::<syn::File>(typedefs_ts)?;

    for item in &mut file.items {

        if let syn::Item::Struct(s) = item {

            // prepend a #[derive(Serialize, Deserialize)] attr

            let derive_attr: syn::Attribute = syn::parse_quote!(

                #[derive(Serialize)]

            );

            s.attrs.insert(0, derive_attr);

        }

    }

    // then turn `file` back into tokens:

    let typedefs_with_serde = quote! { #file };

    // 2) generate typedefs


    // 6) glue & write
    let out = quote! {
        // @generated by build.rs — DO NOT EDIT
        #[allow(dead_code)]
        use serde::Serializer;
        use std::convert::TryInto;

        fn serialize_to_string<S, T>(x: &T, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
            T: ToString,
        {
            s.serialize_str(&x.to_string())
        }
        #parse_option_fn

        // for splitting slices and converting sizes
       

        // re-export
        pub use ix_data::*;
        pub use typedefs::*;
        pub use accounts_data::*;

        pub mod typedefs {
            use ::borsh::{BorshSerialize, BorshDeserialize};
            use anchor_lang::prelude::*;
            use serde::Serialize;
            use crate::pubkey_serializer::pubkey_serde;
            use crate::pubkey_serializer::pubkey_serde_option;
            #typedefs_with_serde
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
            #(#args_structs)*
        }

        #decoded_enum
        #decode_impl
    };

    fs::write("src/idl.rs", out.to_string())?;
    Command::new("rustfmt").arg("src/idl.rs").status()?;
    Ok(())
}

// helper to map IdlType → Rust TokenStream
fn map_idl_type(ty: &IdlType) -> proc_macro2::TokenStream {
    match ty {
        IdlType::Bool => quote! { bool },
        IdlType::U8 => quote! { u8 },
        IdlType::I8 => quote! { i8 },
        IdlType::U16 => quote! { u16 },
        IdlType::I16 => quote! { i16 },
        IdlType::U32 => quote! { u32 },
        IdlType::I32 => quote! { i32 },
        IdlType::U64 => quote! { u64 },
        IdlType::I64 => quote! { i64 },
        IdlType::U128 => quote! { u128 },
        IdlType::I128 => quote! { i128 },
        IdlType::Bytes => quote! { Vec<u8> },
        IdlType::String => quote! { String },
        IdlType::Pubkey => quote! { [u8; 32usize] },
        IdlType::Vec(inner) => {
            let inner_ts = map_idl_type(inner);
            quote! { Vec<#inner_ts> }
        }
        IdlType::Array(inner, len) => {
            let inner_ts = map_idl_type(inner);
            let len_val = match len {
                IdlArrayLen::Value(n) => *n as usize,
                _ => panic!("unsupported array length: {:?}", len),
            };
            quote! { [#inner_ts; #len_val] }
        }
        IdlType::Defined { name, generics: _ } => {
            let ident = format_ident!("{}", name);
            quote! { #ident }
        }
        IdlType::Option(inner) => {
            let inner_ts = map_idl_type(inner);
            quote! { Option<#inner_ts> }
        }
        other => panic!("unsupported IDL type: {:?}", other),
    }
}


fn flatten_accounts<'a>(items: &'a [IdlInstructionAccountItem]) -> Vec<&'a anchor_idl::IdlInstructionAccount> {
    let mut out = Vec::new();
    for item in items {
        match item {
            IdlInstructionAccountItem::Single(acc) => out.push(acc),
            IdlInstructionAccountItem::Composite(group) => {
                out.extend(flatten_accounts(&group.accounts));
            }
        }
    }
    out
}