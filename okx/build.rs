// build.rs - Generate idl.rs from okx_v2.json

use anchor_idl::{GeneratorOptions, Idl, IdlArrayLen, IdlInstructionAccountItem, IdlType};
use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use serde_json::from_str;
use std::{fs, process::Command};

fn main() -> Result<()> {
    // Load OKX IDL
    let path = "idl/okx_v2.json";
    let raw = fs::read_to_string(path)?;
    let idl: Idl = from_str(&raw)?;

    // Generate typedefs (structs & enums) from IDL types
    let mut opts = GeneratorOptions::default();
    opts.idl_path = path.into();
    let gen = opts.to_generator();
    let typedefs_ts = anchor_idl::generate_typedefs(&gen.idl.types, &gen.struct_opts);

    // Prepare Args, Accounts and instruction decode arms
    let mut args_structs = Vec::new();
    let mut accounts_structs = Vec::new();
    let mut arms = Vec::new();

    for ix in &idl.instructions {
        let name = &ix.name;
        let pascal = name.to_upper_camel_case();
        let args_ty = format_ident!("{}InstructionArgs", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        let var = format_ident!("{}", pascal);

        // Use discriminator from IDL (OKX IDL already has discriminators)
        let disc = &ix.discriminator;
        let disc_tokens = disc.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

        // Args struct - only create if there are args
        if !ix.args.is_empty() {
            let fields = ix.args.iter().map(|arg| {
                let f = format_ident!("{}", arg.name.to_snake_case());
                let ty_ts = map_idl_type(&arg.ty);
                // Add serde serialization for u64 fields to avoid JS overflow
                let serde_attr = if matches!(&arg.ty, IdlType::U64) {
                    quote! { #[serde(serialize_with = "serialize_u64_as_string")] }
                } else {
                    quote! {}
                };
                quote! { #serde_attr pub #f: #ty_ts, }
            });
            args_structs.push(quote! {
                #[derive(::borsh::BorshDeserialize, Debug, Serialize)]
                pub struct #args_ty { #(#fields)* }
            });
        } else {
            // Empty args struct for instructions with no args
            args_structs.push(quote! {
                #[derive(::borsh::BorshDeserialize, Debug, Serialize, Default)]
                pub struct #args_ty {}
            });
        }

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

        // Decoder arm
        let idents: Vec<_> = flat
            .iter()
            .map(|acc| format_ident!("{}", acc.name.as_str()))
            .collect();
        let extract = idents.iter().map(|ident| {
            quote! {
                let #ident = keys.next().unwrap_or(&String::new()).clone();
            }
        });
        arms.push(quote! {
            [#(#disc_tokens),*] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::#args_ty::deserialize(&mut rdr)?;
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
        let args_ty = format_ident!("{}InstructionArgs", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        quote! { #var { accounts: #accounts_ty, args: ix_data::#args_ty }, }
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

    // Build Event decode arms
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
                    match disc {
                        #(#event_arms)*
                        _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
                    }
                }
            }
        })
    } else { None };

    // Parse typedef tokens and add Serialize derive + pubkey serialization + u64 string serialization
    let mut file = syn::parse2::<syn::File>(typedefs_ts.clone())?;
    for item in &mut file.items {
        if let syn::Item::Struct(s) = item {
            // Add Serialize derive
            let derive_attr: syn::Attribute = syn::parse_quote!(#[derive(Serialize)]);
            s.attrs.insert(0, derive_attr);
            // Annotate Pubkey, Option<Pubkey>, and u64 fields
            for field in &mut s.fields {
                match &field.ty {
                    // plain Pubkey
                    syn::Type::Path(path) if path.path.segments.last().unwrap().ident == "Pubkey" => {
                        let attr: syn::Attribute = syn::parse_quote!(#[serde(with = "pubkey_serde")]);
                        field.attrs.insert(0, attr);
                    }
                    // u64 - serialize as string to avoid JS overflow
                    syn::Type::Path(path) if path.path.segments.last().unwrap().ident == "u64" => {
                        let attr: syn::Attribute = syn::parse_quote!(#[serde(serialize_with = "serialize_u64_as_string")]);
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
            // Add Serialize derive on enums
            let derive_attr: syn::Attribute = syn::parse_quote!(#[derive(Serialize)]);
            e.attrs.insert(0, derive_attr);
        }
    }
    let typedefs_with_serde = quote! { #file };

    // Write out the generated code
    let out = quote! {
        // @generated by build.rs from okx_v2.json — DO NOT EDIT
        #[allow(dead_code)]
        use std::convert::TryInto;
        use serde::Serializer;
    
        fn serialize_to_string<S, T>(x: &T, s: S) -> Result<S::Ok, S::Error>
        where S: Serializer, T: ToString {
            s.serialize_str(&x.to_string())
        }

        // Helper to serialize u64 as string to avoid JavaScript number overflow
        fn serialize_u64_as_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
            serializer.serialize_str(&value.to_string())
        }

        pub use typedefs::*;
        pub use accounts_data::*;

        pub mod typedefs {
            use anchor_lang::prelude::*;
            use ::borsh::{BorshSerialize, BorshDeserialize};
            use serde::Serialize;
            use serde::Serializer;
            use crate::pubkey_serializer::pubkey_serde;
            use crate::pubkey_serializer::pubkey_serde_option;
            use super::serialize_u64_as_string;
            #typedefs_with_serde
        }

        pub mod accounts_data {
            use serde::Serialize;
            #(#accounts_structs)*
        }

        pub mod ix_data {
            use serde::Serialize;
            use serde::Serializer;
            use super::typedefs::*;
            use super::serialize_u64_as_string;
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

// Map IDL type to Rust TokenStream
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

