// build.rs - Comprehensive IDL parser for raydium CLMM (all 25 instructions)

use anchor_idl::{GeneratorOptions, Idl, IdlInstructionAccountItem, IdlType};
use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use serde_json::from_str;
use std::{fs, process::Command};

use syn;

fn add_serialize_to_typedefs(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    // Debug: let's see what the actual tokens look like
    let tokens_str = tokens.to_string();
    println!("Original tokens: {}", &tokens_str[..tokens_str.len().min(500)]);
    
    // Use regex for more flexible pattern matching
    let result = tokens_str
        .replace("# [derive (AnchorSerialize , AnchorDeserialize , Clone , Copy , Debug , Default)]", 
                 "# [derive (AnchorSerialize , AnchorDeserialize , Clone , Copy , Debug , Default , Serialize)]")
        .replace("# [derive (AnchorSerialize , AnchorDeserialize , Clone , Debug)]", 
                 "# [derive (AnchorSerialize , AnchorDeserialize , Clone , Debug , Serialize)]")
        .replace("# [derive (AnchorSerialize , AnchorDeserialize , Clone , Copy , Debug)]", 
                 "# [derive (AnchorSerialize , AnchorDeserialize , Clone , Copy , Debug , Serialize)]")
        .replace("# [derive (AnchorSerialize , AnchorDeserialize , Debug)]", 
                 "# [derive (AnchorSerialize , AnchorDeserialize , Debug , Serialize)]");
    
    println!("Modified tokens: {}", &result[..result.len().min(500)]);
    result.parse().unwrap_or_else(|_| tokens)
}

fn main() -> Result<()> {
    let path = "idls/raydium_clmm.json";
    let idl_json = fs::read_to_string(path)?;
    let idl: Idl = from_str(&idl_json)?;

    println!("cargo:warning=Building comprehensive raydium CLMM parser with {} instructions, {} types (events skipped)", 
        idl.instructions.len(), idl.types.len());

    // Generate typedefs using built-in anchor-idl generator
    let mut opts = GeneratorOptions::default();
    opts.idl_path = path.into();
    let gen = opts.to_generator();
    let typedefs_tokens = anchor_idl::generate_typedefs(&gen.idl.types, &gen.struct_opts);

    // Generate typedefs with Serialize support for JSON output
    println!("cargo:warning=About to process typedefs for serialization");
    let mut file = syn::parse2::<syn::File>(typedefs_tokens)?;
    for item in &mut file.items {
        if let syn::Item::Struct(s) = item {
            // ensure Serialize derive
            let derive_attr: syn::Attribute = syn::parse_quote!(#[derive(Serialize)]);
            s.attrs.insert(0, derive_attr);
            // annotate Pubkey and Option<Pubkey> fields
            for field in &mut s.fields {
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
                    syn::Type::Array(arr) => {
                        // skip serializing very large fixed arrays (len > 32) to avoid serde limitations
                        if let syn::Expr::Lit(expr_lit) = &arr.len {
                            if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                                if let Ok(len_val) = lit_int.base10_parse::<usize>() {
                                    if len_val > 32 {
                                        let attr: syn::Attribute = syn::parse_quote!(#[serde(skip_serializing)]);
                                        field.attrs.insert(0, attr);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    syn::Type::Path(path) if {
                        let id = &path.path.segments.last().unwrap().ident;
                        id == "u64" || id == "u128"
                    } => {
                        let attr: syn::Attribute = syn::parse_quote!(#[serde(serialize_with = "crate::serialize_to_string")]);
                        field.attrs.insert(0, attr);
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
    println!("cargo:warning=Finished processing typedefs");

    // Generate all instruction structs and decoders
    let (args_structs, accounts_structs, instruction_variants, decode_arms) = generate_all_instructions(&idl)?;

    // Build the comprehensive output
    let out = quote! {
        extern crate serde;
        // @generated by build.rs — DO NOT EDIT
        #[allow(dead_code)]

        use ::borsh::BorshDeserialize;
        use anchor_lang::prelude::*;

        pub use ix_data::*;
        pub use typedefs::*;
        pub use accounts_data::*;

        pub mod typedefs {
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
            use super::*;
            use crate::pubkey_serializer::pubkey_serde;
            use crate::pubkey_serializer::pubkey_serde_option;
            #(#args_structs)*
        }

        #[derive(Debug, Serialize)]
        #[serde(tag = "instruction_type")]
        pub enum Instruction {
            #(#instruction_variants)*
        }

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
                    #(#decode_arms)*
                    _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
                }
            }
        }


    };

    let generated_code = out.to_string();
    fs::write("src/idl.rs", generated_code)?;
    
    // Post-process the generated file to add Serialize to typedefs
    let idl_content = fs::read_to_string("src/idl.rs")?;
    
    // Count how many replacements we're making for debugging
    let pattern = "    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]";
    let count = idl_content.matches(pattern).count();
    println!("cargo:warning=Found {} instances of AnchorSerialize pattern to replace", count);
    
    // Debug: show a snippet around InitializeRewardParam
    if let Some(pos) = idl_content.find("InitializeRewardParam") {
        let start = pos.saturating_sub(100);
        let end = (pos + 100).min(idl_content.len());
        let snippet = &idl_content[start..end];
        println!("cargo:warning=Context around InitializeRewardParam: {}", snippet.replace('\n', " "));
    }
    
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
    
    let new_count = processed_content.matches("Serialize)]").count();
    println!("cargo:warning=After processing, found {} instances with Serialize", new_count);
    
    fs::write("src/idl.rs", processed_content)?;
    Command::new("rustfmt").arg("src/idl.rs").status()?;
    println!("cargo:warning=Generated comprehensive parser for all {} raydium CLMM instructions!", idl.instructions.len());
    Ok(())
}



fn generate_all_instructions(idl: &Idl) -> Result<(Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>)> {
    let mut args_structs = Vec::new();
    let mut accounts_structs = Vec::new();
    let mut instruction_variants = Vec::new();
    let mut decode_arms = Vec::new();

    for ix in &idl.instructions {
        let pascal = ix.name.to_upper_camel_case();
        let args_ty = format_ident!("{}Arguments", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        let variant = format_ident!("{}", pascal);

        // Get discriminator from IDL
        let disc_bytes: [u8; 8] = ix.discriminator.clone().try_into()
            .map_err(|_| anyhow::anyhow!("Invalid discriminator length for instruction {}", ix.name))?;
        let disc_tokens = disc_bytes.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

        // Generate args struct with serde annotations
        let args_fields: Vec<_> = ix.args.iter().map(|arg| {
            let field_ident = format_ident!("{}", arg.name.to_snake_case());
            let ty = map_idl_type(&arg.ty);

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

        args_structs.push(quote! {
            #[derive(::borsh::BorshDeserialize, Debug, Serialize)]
            pub struct #args_ty {
                #(#args_fields)*
            }
        });

        // Generate accounts struct using flatten pattern
        let flat = flatten_accounts(&ix.accounts);
        let acc_fields = flat.iter().map(|acc| {
            let f = format_ident!("{}", acc.name.as_str());
            
            // Check if account is marked as optional in IDL
            let is_optional = acc.optional;
            
            if is_optional {
                quote! { pub #f: Option<String>, }
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

        // Generate instruction variant
        instruction_variants.push(quote! {
            #variant { accounts: #accounts_ty, args: #args_ty },
        });

        // Generate decode arm with simple two-pass account mapping
        let required_count = flat.iter().filter(|acc| !acc.optional).count();
        let optional_count = flat.iter().filter(|acc| acc.optional).count();
        let total_count = flat.len();
        
        let idents: Vec<_> = flat
            .iter()
            .map(|acc| format_ident!("{}", acc.name.as_str()))
            .collect();
        
        // Separate required and optional account extractions
        let required_extracts: Vec<_> = flat.iter().zip(&idents).filter_map(|(acc, ident)| {
            if !acc.optional {
                Some(quote! { let #ident = required_iter.next().unwrap().clone(); })
            } else {
                None
            }
        }).collect();
        
        let optional_extracts: Vec<_> = flat.iter().zip(&idents).filter_map(|(acc, ident)| {
            if acc.optional {
                Some(quote! { let #ident = optional_iter.next().map(|s| s.clone()); })
            } else {
                None
            }
        }).collect();

        // Special handling for instructions with incomplete historical data
        let args_deserialization = if ix.name == "create_pool" {
            quote! {
                let args = if rest.len() >= 24 {
                    // Complete data: deserialize normally
                    let mut rdr: &[u8] = rest;
                    #args_ty::deserialize(&mut rdr)?
                } else if rest.len() >= 16 {
                    // Incomplete data: only sqrt_price_x64, no open_time
                    let mut rdr: &[u8] = rest;
                    let sqrt_price_x64 = u128::deserialize(&mut rdr)?;
                    #args_ty {
                        sqrt_price_x64,
                        open_time: None,
                    }
                } else {
                    anyhow::bail!("Insufficient data for create_pool: got {} bytes, need at least 16", rest.len());
                };
            }
        } else if ix.name == "create_amm_config" {
            quote! {
                let args = if rest.len() >= 16 {
                    // Complete data: manually deserialize all fields as non-optional
                    let mut rdr: &[u8] = rest;
                    let index = u16::deserialize(&mut rdr)?;
                    let tick_spacing = u16::deserialize(&mut rdr)?;
                    let trade_fee_rate = u32::deserialize(&mut rdr)?;
                    let protocol_fee_rate = u32::deserialize(&mut rdr)?;
                    let fund_fee_rate = u32::deserialize(&mut rdr)?;
                    #args_ty {
                        index,
                        tick_spacing,
                        trade_fee_rate,
                        protocol_fee_rate,
                        fund_fee_rate: Some(fund_fee_rate),
                    }
                } else if rest.len() >= 12 {
                    // Incomplete data: missing fund_fee_rate
                    let mut rdr: &[u8] = rest;
                    let index = u16::deserialize(&mut rdr)?;
                    let tick_spacing = u16::deserialize(&mut rdr)?;
                    let trade_fee_rate = u32::deserialize(&mut rdr)?;
                    let protocol_fee_rate = u32::deserialize(&mut rdr)?;
                    #args_ty {
                        index,
                        tick_spacing,
                        trade_fee_rate,
                        protocol_fee_rate,
                        fund_fee_rate: None,
                    }
                } else {
                    anyhow::bail!("Insufficient data for create_amm_config: got {} bytes, need at least 12", rest.len());
                };
            }
        } else {
            quote! {
                let mut rdr: &[u8] = rest;
                let args = #args_ty::deserialize(&mut rdr)?;
            }
        };

        decode_arms.push(quote! {
            [#(#disc_tokens),*] => {
                #args_deserialization
                
                // Check minimum required accounts
                if account_keys.len() < #required_count {
                    anyhow::bail!("Insufficient accounts: got {}, need at least {} for required accounts", 
                        account_keys.len(), #required_count);
                }
                
                // First pass: extract required accounts
                let mut required_iter = account_keys.iter().take(#required_count);
                #(#required_extracts)*
                
                // Second pass: extract optional accounts from remaining
                let mut optional_iter = account_keys.iter().skip(#required_count);
                #(#optional_extracts)*
                
                let remaining = if account_keys.len() > (#required_count + #optional_count) {
                    account_keys[(#required_count + #optional_count)..].to_vec()
                } else {
                    Vec::new()
                };
                let accounts = #accounts_ty { #(#idents),*, remaining };
                return Ok(Instruction::#variant { accounts, args });
            }
        });
    }

    Ok((args_structs, accounts_structs, instruction_variants, decode_arms))
}

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
        IdlType::Pubkey => quote! { Pubkey },
        IdlType::Vec(inner) => {
            let i = map_idl_type(inner);
            quote! { Vec<#i> }
        }
        IdlType::Array(inner, len) => {
            let i = map_idl_type(inner);
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
            quote! { #id }
        }
        IdlType::Option(inner) => {
            match &**inner {
                IdlType::Bool => quote! { bool },
                _ => {
                    let i = map_idl_type(inner);
                    quote! { Option<#i> }
                }
            }
        }
        _ => {
            // For any unsupported types, fall back to Vec<u8>
            quote! { Vec<u8> }
        }
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