// build.rs

use anchor_idl::{GeneratorOptions, Idl, IdlArrayLen, IdlInstructionAccountItem, IdlType};
use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use serde_json::from_str;
use sha2::{Digest, Sha256};
use std::{fs, process::Command};

fn main() -> Result<()> {
    // load and normalize IDL
    let path = "idls/Pyth_staking.json";
    let raw = fs::read_to_string(path)?;
    let normalized = raw.replace(r#"\"writable\""#, r#"\"mut\""#);
    let idl: Idl = from_str(&normalized)?;

    // generate typedefs (structs & enums)
    let mut opts = GeneratorOptions::default();
    opts.idl_path = path.into();
    let gen = opts.to_generator();
    let typedefs_ts = anchor_idl::generate_typedefs(&gen.idl.types, &gen.struct_opts);

    // prepare Args, Accounts and instruction arms
    let mut args_structs = Vec::new();
    let mut accounts_structs = Vec::new();
    let mut arms = Vec::new();

    for ix in &idl.instructions {
        let name = &ix.name;
        let pascal = name.to_upper_camel_case();
        let args_ty = format_ident!("{}Args", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        let var = format_ident!("{}", pascal);

        // 8-byte discriminator
        let mut hasher = Sha256::new();
        hasher.update(b"global:");
        hasher.update(name.to_snake_case().as_bytes());
        let hash = hasher.finalize();
        let disc: [u8; 8] = hash[0..8].try_into().unwrap();
        let disc_tokens = disc.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

        // Args struct
        let args_fields = ix.args.iter().map(|arg| {
            let field_ident = format_ident!("{}", arg.name.to_snake_case());
            let ty = map_idl_type(&arg.ty);
            // check for U64 / U128
            let field_tokens = match &arg.ty {
                IdlType::U64 | IdlType::U128 => {
                    quote! {
                        #[serde(serialize_with = "crate::serialize_to_string")]
                        pub #field_ident: #ty,
                    }
                }

                IdlType::Pubkey => {
                    quote! {
                        #[serde(with = "pubkey_serde_u32")]
                        pub #field_ident: [u8; 32usize],
                    }
                }
                // Option<PublicKey> → Option<[u8;32]> + its serde
                IdlType::Option(inner)
                    if matches!(inner.as_ref(),IdlType::Pubkey) =>
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

        // Accounts struct
        let flat = flatten_accounts(&ix.accounts);
        let acc_fields = flat.iter().map(|acc| {
            let f = format_ident!("{}", acc.name.as_str());
            quote! { pub #f: String, }
        });
        accounts_structs.push(quote! {
            #[derive(Debug, Serialize)]
            pub struct #accounts_ty {
                #(#acc_fields)*
                pub remaining: Vec<String>,
            }
        });

        // decoder arm
        let idents: Vec<_> = flat
            .iter()
            .map(|acc| format_ident!("{}", acc.name.as_str()))
            .collect();
        let extract = idents.iter().map(|ident| {
            quote! {
                let #ident = keys.next().unwrap().clone();
            }
        });
        arms.push(quote! {
            [#(#disc_tokens),*] => {
                let mut rdr: &[u8] = rest;
                let args = #args_ty::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                #(#extract)*
                let remaining = keys.cloned().collect();
                let accounts = #accounts_ty { #(#idents),*, remaining };
                return Ok(Instruction::#var { accounts, args });
            }
        });
    }

    // Instruction enum & decode impl
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

        // 5) Build Event decode arms (IDL.events is always Vec<IdlEvent>)
        let mut event_arms = Vec::new();
        for ev in &idl.events {
            let pascal = format_ident!("{}", ev.name.to_upper_camel_case());
            let disc_tokens = ev.discriminator.iter().map(|b| quote! {#b}).collect::<Vec<_>>();
            event_arms.push(quote! {
                [#(#disc_tokens),*] => {
                    let mut rdr = &data[16..];
                    let args = #pascal::deserialize(&mut rdr)?;
                    return Ok(Event::#pascal{args});
                }
            });
        }
        let decoded_events = if !idl.events.is_empty() {
            let ev_variants = idl.events.iter().map(|ev| {
                let pascal = format_ident!("{}", ev.name.to_upper_camel_case());
                quote! { #pascal{ args: #pascal}, }
            });
            Some(quote! {
                #[derive(Debug, Serialize)]
                #[serde(tag = "event_type")]
                pub enum Event { #(#ev_variants)* }
            })
        } else { None };
        let event_impl = if !idl.events.is_empty() {
            Some(quote! {
                pub const EVENT_LOG_DISCRIMINATOR: [u8;8] = [228, 69, 165, 46, 81, 203, 154, 29];
                impl Event {
                    pub fn decode(data: &[u8]) -> anyhow::Result<Self> {
                        if data.len() < 16 { anyhow::bail!("Event log too short"); }
                        let wrapper: [u8;8] = data[0..8].try_into().unwrap();
                        if wrapper != EVENT_LOG_DISCRIMINATOR {
                            anyhow::bail!("Invalid event wrapper");
                        }
                        let disc: [u8;8] = data[8..16].try_into().unwrap();
                        let payload = &data[16..];
                        match disc {
                            #(#event_arms)*
                            _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
                        }
                    }
                }
            })
        } else { None };

    let mut file = syn::parse2::<syn::File>(typedefs_ts.clone())?;
    for item in &mut file.items {
        if let syn::Item::Struct(s) = item {
            // ensure Serialize derive
            let derive_attr: syn::Attribute = syn::parse_quote!(#[derive(Serialize)]);
            s.attrs.insert(0, derive_attr);
            // annotate Pubkey and Option<Pubkey> fields
            for field in &mut s.fields {
                if let syn::Type::Array(type_array) = &field.ty {
                    // get the literal length N
                    if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(len_lit), .. }) = &type_array.len {
                        let len = len_lit.base10_parse::<usize>().unwrap();
                        if len > 32 {
                            // inject #[serde(with = "BigArray")]
                            let ba_attr: syn::Attribute = syn::parse_quote!(#[serde(with = "BigArray")]);
                            field.attrs.insert(0, ba_attr);
                        }
                    }
                }
                match &field.ty {
                    // plain Pubkey
                    syn::Type::Path(path) if path.path.segments.last().unwrap().ident == "Pubkey" => {
                        let attr: syn::Attribute = syn::parse_quote!(#[serde(with = "pubkey_serde")]);
                        field.attrs.insert(0, attr);
                    }
                    // Option<Pubkey>
                    syn::Type::Path(path) if path.path.segments.len() == 1 && path.path.segments[0].ident == "Option" => {
                        if let syn::PathArguments::AngleBracketed(ab) = &path.path.segments[0].arguments {
                            if let Some(syn::GenericArgument::Type(inner_ty)) = ab.args.first() {
                                if let syn::Type::Path(inner_path) = inner_ty {
                                    if inner_path.path.segments.last().unwrap().ident == "Pubkey" {
                                        let attr: syn::Attribute = syn::parse_quote!(#[serde(with = "pubkey_serde_option")]);
                                        field.attrs.insert(0, attr);
                                    }
                                }
                            }
                        }
                    }
                    
                    _ => {}
                }
            }
        }
        if let syn::Item::Enum(e) = item {
            // ensure Serialize derive on enums too
            let derive_attr: syn::Attribute = syn::parse_quote!(#[derive(Serialize)]);
            e.attrs.insert(0, derive_attr);
        }
    }
    let typedefs_with_serde = quote! { #file };
    

    // write out
    let out = quote! {
        // @generated by build.rs — DO NOT EDIT
        #[allow(dead_code)]
        use std::convert::TryInto;
        use serde::Serializer;
    
        fn serialize_to_string<S, T>(x: &T, s: S) -> Result<S::Ok, S::Error>
        where S: Serializer, T: ToString {
            s.serialize_str(&x.to_string())
        }

        pub use ix_data::*;
        pub use typedefs::*;
        pub use accounts_data::*;

        pub mod typedefs {
            use anchor_lang::prelude::*;
            use ::borsh::{BorshSerialize, BorshDeserialize};
            use serde::Serialize;
            use crate::pubkey_serializer::pubkey_serde;
            use crate::pubkey_serializer::pubkey_serde_option;
           
            // generate impls for fixed-size arrays up to length 64
            serde_big_array::big_array! { BigArray; 64 }
            #typedefs_with_serde
        }

        pub mod accounts_data {
            use serde::Serialize;
            #(#accounts_structs)*
        }

        pub mod ix_data {
            use serde::Serialize;
            use crate::pubkey_serializer::pubkey_serde_u32;
            use crate::pubkey_serializer::pubkey_serde_option;
            use super::*;
            #(#args_structs)*
        }

        #decoded_enum
        #decode_impl

        pub mod events {
            pub use typedefs::*;
            use super::*;
            use ::borsh::BorshDeserialize;
            use serde::Serialize;
            #decoded_events
            #event_impl
        }
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

fn flatten_accounts<'a>(
    items: &'a [IdlInstructionAccountItem],
) -> Vec<&'a anchor_idl::IdlInstructionAccount> {
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
