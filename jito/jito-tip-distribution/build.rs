// build.rs

use anchor_idl::{GeneratorOptions, Idl, IdlArrayLen, IdlInstructionAccountItem, IdlType};
use anyhow::Result;
use heck::{ ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use serde_json::from_str;
use sha2::{Digest, Sha256};
use std::{fs, process::Command};

fn main() -> Result<()> {
    let path = "idls/jito_tip_distribution.json";
    let raw = fs::read_to_string(path)?;
    let normalized = raw.replace(r#"\"writable\""#, r#"\"mut\""#);
    let idl: Idl = from_str(&normalized)?;

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
        let var = format_ident!("{}", pascal);

        // compute discriminator
        let mut hasher = Sha256::new();
        hasher.update(b"global:");
        hasher.update(name.to_snake_case());
        let hash = hasher.finalize();
        let disc_bytes: [u8; 8] = hash[0..8].try_into().unwrap();
        let disc_tokens = disc_bytes.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

        // build Args struct
        let fields = ix.args.iter().map(|arg| {
            let f = format_ident!("{}", arg.name.to_snake_case());
            let ty_ts = map_idl_type(&arg.ty);
            quote! { pub #f: #ty_ts, }
        });
        args_structs.push(quote! {
            #[derive(::borsh::BorshDeserialize, Debug, Serialize)]
            pub struct #args_ty { #(#fields)* }
        });

        // build Accounts struct
        let flat_accounts = flatten_accounts(&ix.accounts);
        let acc_fields = flat_accounts.iter().map(|acc| {
            let f = format_ident!("{}", &acc.name);
            quote! { pub #f: String, }
        });
        accounts_structs.push(quote! {
            #[derive(Debug, Serialize)]
            pub struct #accounts_ty {
                #(#acc_fields)*
                pub remaining: Vec<String>,
            }
        });

        // build match arm
        let acc_idents: Vec<_> = flat_accounts.iter().map(|acc| format_ident!("{}", &acc.name)).collect();
        let extract_accounts = acc_idents.iter().map(|ident| {
            quote! {
                let #ident = keys.next().unwrap().clone();
            }
        });

        arms.push(quote! {
            [#(#disc_tokens),*] => {
                // deserialize args from rest of data
                let mut rdr: &[u8] = rest;
                let args = #args_ty::deserialize(&mut rdr)?;
                // consume accounts
                let mut keys = account_keys.iter();
                #(#extract_accounts)*
                let remaining = keys.cloned().collect();
                let accounts = #accounts_ty { #(#acc_idents),*, remaining };
                return Ok(Instruction::#var { accounts, args });
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

        // for splitting slices and converting sizes
        use std::convert::TryInto;

        // re-export
        pub use ix_data::*;
        pub use typedefs::*;
        pub use accounts_data::*;

        pub mod typedefs {
            use anchor_lang::prelude::*;
            use serde::Serialize;
            #typedefs_with_serde
        }

        pub mod accounts_data {
            use serde::Serialize;
            #(#accounts_structs)*
        }

        pub mod ix_data {
            use super::*;
            use serde::Serialize;
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
        IdlType::Pubkey => quote! { String },
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