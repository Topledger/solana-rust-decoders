// build.rs for Raydium CPMM decoder
use anchor_idl::{GeneratorOptions, Idl, IdlInstructionAccountItem, IdlType};
use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use serde_json::from_str;
use std::{fs, process::Command};

fn main() -> Result<()> {
    let path = "idls/raydium_cpmm.json";
    let idl_json = fs::read_to_string(path)?;
    let idl: Idl = from_str(&idl_json)?;

    println!(
        "cargo:warning=Building raydium CPMM parser with {} instructions, {} types",
        idl.instructions.len(),
        idl.types.len()
    );

    // Generate typedefs using built-in anchor-idl generator
    let mut opts = GeneratorOptions::default();
    opts.idl_path = path.into();
    let gen = opts.to_generator();
    let typedefs_tokens = anchor_idl::generate_typedefs(&gen.idl.types, &gen.struct_opts);

    // Instruction codegen with serde support
    let (args_structs, accounts_structs, variants, arms) = generate_instructions(&idl)?;

    let out = quote! {
        extern crate serde;
        #[allow(dead_code)]
        use ::borsh::BorshDeserialize;
        use anchor_lang::prelude::*;

        pub use ix_data::*;
        pub use accounts_data::*;
        pub use typedefs::*;

        pub mod typedefs {
            use anchor_lang::prelude::*;
            use serde::Serialize;
            use crate::pubkey_serializer::pubkey_serde;
            use crate::pubkey_serializer::pubkey_serde_option;
            
            #typedefs_tokens
        }

        pub mod accounts_data {
            use serde::Serialize;
            #(#accounts_structs)*
        }

        pub mod ix_data {
            use super::*;
            use crate::pubkey_serializer::pubkey_serde;
            use crate::pubkey_serializer::pubkey_serde_option;
            #(#args_structs)*
        }

        #[derive(Debug, Serialize)]
        #[serde(tag = "instruction_type")]
        pub enum Instruction { 
            #(#variants)* 
        }

        impl Instruction {
            pub fn decode(account_keys: &[String], data: &[u8]) -> anyhow::Result<Self> {
                if data.len() < 8 { 
                    anyhow::bail!("Data too short: {}", data.len()); 
                }
                let (disc, rest) = data.split_at(8);
                let disc: [u8;8] = disc.try_into().unwrap();
                match disc {
                    #(#arms)*
                    _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
                }
            }
        }
    };

    fs::create_dir_all("src").ok();
    let generated_code = out.to_string();
    fs::write("src/idl.rs", generated_code)?;
    
    // Post-process the generated file to add Serialize to typedefs
    let idl_content = fs::read_to_string("src/idl.rs")?;
    
    // Simple approach: find any line with AnchorSerialize and add Serialize to it
    let lines: Vec<&str> = idl_content.lines().collect();
    let mut processed_lines = Vec::new();
    
    for line in lines {
        if line.contains("derive(AnchorSerialize") && !line.contains("Serialize") {
            // Add Serialize before the closing parenthesis
            let new_line = line.replace(")]", ", Serialize)]");
            processed_lines.push(new_line);
        } else {
            processed_lines.push(line.to_string());
        }
    }
    
    let processed_content = processed_lines.join("\n");
    fs::write("src/idl.rs", processed_content)?;
    
    Command::new("rustfmt").arg("src/idl.rs").status()?;
    println!("cargo:warning=Generated comprehensive parser for all {} raydium CPMM instructions!", idl.instructions.len());
    Ok(())
}

fn generate_instructions(idl: &Idl) -> Result<(
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
)> {
    let mut args = vec![];
    let mut accs = vec![];
    let mut variants = vec![];
    let mut arms = vec![];

    for ix in &idl.instructions {
        let pascal = ix.name.to_upper_camel_case();
        let var = format_ident!("{}", pascal);
        let args_ty = format_ident!("{}Arguments", pascal);
        let acc_ty = format_ident!("{}Accounts", pascal);

        // discriminator from IDL
        let disc: [u8;8] = ix.discriminator.clone().try_into().unwrap();
        let disc_tokens = disc.iter().map(|b| quote!{ #b }).collect::<Vec<_>>();

        // Generate args struct with serde annotations
        let fields: Vec<_> = ix.args.iter().map(|arg| {
            let field_ident = format_ident!("{}", arg.name.to_snake_case());
            let ty = map_type(&arg.ty);

            match &arg.ty {
                anchor_idl::IdlType::U64 | anchor_idl::IdlType::U128 => {
                    quote! {
                        #[serde(serialize_with = "crate::serialize_to_string")]
                        pub #field_ident: #ty,
                    }
                }
                anchor_idl::IdlType::Pubkey => {
                    quote! {
                        #[serde(with = "pubkey_serde")]
                        pub #field_ident: #ty,
                    }
                }
                anchor_idl::IdlType::Option(inner) if matches!(**inner, anchor_idl::IdlType::Pubkey) => {
                    quote! {
                        #[serde(with = "pubkey_serde_option")]
                        pub #field_ident: #ty,
                    }
                }
                _ => {
                    quote! {
                        pub #field_ident: #ty,
                    }
                }
            }
        }).collect();

        args.push(quote! {
            #[derive(::borsh::BorshDeserialize, Debug, Serialize)]
            pub struct #args_ty { 
                #(#fields)* 
            }
        });

        // accounts struct (flatten) with Serialize
        let flat = flatten(&ix.accounts);
        let acc_fields: Vec<_> = flat.iter().map(|a| {
            let id = format_ident!("{}", a.name.as_str());
            quote! { pub #id: String, }
        }).collect();
        accs.push(quote! {
            #[derive(Debug, Serialize)]
            pub struct #acc_ty {
                #(#acc_fields)*
                pub remaining: Vec<String>,
            }
        });

        variants.push(quote! { #var { accounts: #acc_ty, args: #args_ty }, });

        let idents: Vec<_> = flat.iter().map(|a| format_ident!("{}", a.name.as_str())).collect();
        let extract = idents.iter().map(|i| quote! { let #i = keys.next().unwrap().clone(); });
        arms.push(quote! {
            [#(#disc_tokens),*] => {
                let mut rdr: &[u8] = rest;
                let args = #args_ty::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                #(#extract)*
                let remaining = keys.cloned().collect();
                let accounts = #acc_ty { #(#idents),*, remaining };
                return Ok(Instruction::#var { accounts, args });
            }
        });
    }
    Ok((args, accs, variants, arms))
}

fn map_type(t: &IdlType) -> proc_macro2::TokenStream {
    match t {
        IdlType::Bool => quote!{ bool },
        IdlType::U8 => quote!{ u8 },
        IdlType::I8 => quote!{ i8 },
        IdlType::U16 => quote!{ u16 },
        IdlType::I16 => quote!{ i16 },
        IdlType::U32 => quote!{ u32 },
        IdlType::I32 => quote!{ i32 },
        IdlType::U64 => quote!{ u64 },
        IdlType::I64 => quote!{ i64 },
        IdlType::U128 => quote!{ u128 },
        IdlType::I128 => quote!{ i128 },
        IdlType::Bytes => quote!{ Vec<u8> },
        IdlType::String => quote!{ String },
        IdlType::Pubkey => quote!{ Pubkey },  // Changed from [u8;32] to Pubkey
        IdlType::Vec(inner) => { 
            let i = map_type(inner); 
            quote!{ Vec<#i> } 
        },
        IdlType::Array(inner, len) => {
            let i = map_type(inner);
            match len {
                anchor_idl::IdlArrayLen::Generic(_) => {
                    // For generic array length, fall back to Vec
                    quote! { Vec<#i> }
                }
                anchor_idl::IdlArrayLen::Value(size) => {
                    let l = *size;
                    quote! { [#i; #l] }
                }
            }
        }
        IdlType::Defined { name, .. } => { 
            let id = format_ident!("{}", name.to_upper_camel_case()); 
            quote!{ #id } 
        },
        IdlType::Option(inner) => {
            match &**inner {
                IdlType::Bool => quote! { bool },
                _ => {
                    let i = map_type(inner);
                    quote! { Option<#i> }
                }
            }
        }
        _ => quote!{ Vec<u8> },
    }
}

fn flatten<'a>(items: &'a [IdlInstructionAccountItem]) -> Vec<&'a anchor_idl::IdlInstructionAccount> {
    let mut v = vec![];
    for it in items {
        match it {
            IdlInstructionAccountItem::Single(a) => v.push(a),
            IdlInstructionAccountItem::Composite(c) => v.extend(flatten(&c.accounts)),
        }
    }
    v
} 