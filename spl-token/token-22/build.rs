use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct CodemaIdl {
    program: CodemaProgram,
}

#[derive(Debug, Deserialize)]
struct CodemaProgram {
    instructions: Vec<CodemaInstruction>,
    // Skip definedTypes for now to avoid parsing complexity
}

#[derive(Debug, Deserialize)]
struct CodemaInstruction {
    name: String,
    accounts: Vec<CodemaAccount>,
    arguments: Vec<CodemaArgument>,
    // Skip discriminators and docs for simplicity
}

#[derive(Debug, Deserialize)]
struct CodemaAccount {
    name: String,
    #[serde(rename = "isOptional")]
    is_optional: bool,
    // Skip other fields we don't need
}

#[derive(Debug, Deserialize)]
struct CodemaArgument {
    name: String,
    #[serde(rename = "type")]
    arg_type: CodemaType,
    #[serde(rename = "defaultValue")]
    default_value: Option<CodemaDefaultValue>,
    // Skip other fields we don't need
}

#[derive(Debug, Deserialize)]
struct CodemaDefaultValue {
    number: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct CodemaType {
    kind: String,
    format: Option<String>,
    item: Option<Box<CodemaType>>,
    // Handle both integer sizes and complex size objects
    size: Option<serde_json::Value>,
    #[serde(rename = "type")]
    inner_type: Option<Box<CodemaType>>,
}

fn map_codama_type(codama_type: &CodemaType) -> TokenStream {
    match codama_type.kind.as_str() {
        "numberTypeNode" => {
            match codama_type.format.as_deref().unwrap_or("u8") {
                "u8" => quote! { u8 },
                "u16" => quote! { u16 },
                "u32" => quote! { u32 },
                "u64" => quote! { u64 },
                "u128" => quote! { u128 },
                "i8" => quote! { i8 },
                "i16" => quote! { i16 },
                "i32" => quote! { i32 },
                "i64" => quote! { i64 },
                "i128" => quote! { i128 },
                _ => quote! { u8 }, // fallback
            }
        }
        "publicKeyTypeNode" => quote! { [u8; 32usize] },
        "booleanTypeNode" => quote! { bool },
        "bytesTypeNode" => quote! { Vec<u8> },
        "fixedSizeTypeNode" => {
            // Handle fixed size types (like [u8; N])
            if let Some(size_val) = &codama_type.size {
                if let Some(size_num) = size_val.as_u64() {
                    if let Some(ref inner) = codama_type.inner_type {
                        let inner_mapped = map_codama_type(inner);
                        // If inner type is bytes, make it a byte array
                        if inner.kind == "bytesTypeNode" {
                            quote! { [u8; #size_num] }
                        } else {
                            quote! { [#inner_mapped; #size_num] }
                        }
                    } else {
                        quote! { [u8; #size_num] }
                    }
                } else {
                    quote! { Vec<u8> } // fallback
                }
            } else {
                quote! { Vec<u8> } // fallback
            }
        }
        "optionTypeNode" => {
            if let Some(ref item) = codama_type.item {
                let inner_type = map_codama_type(item);
                quote! { Option<#inner_type> }
            } else {
                quote! { Option<u8> }
            }
        }
        "definedTypeLinkNode" => {
            // For now, we'll just use u8 as placeholder for defined types like accountState
            quote! { u8 }
        }
        _ => {
            eprintln!("Unknown Codama type: {}", codama_type.kind);
            quote! { u8 } // fallback
        }
    }
}

fn extract_discriminator(instruction: &CodemaInstruction) -> u8 {
    // Find the discriminator argument
    instruction
        .arguments
        .iter()
        .find(|arg| arg.name == "discriminator")
        .and_then(|arg| arg.default_value.as_ref())
        .and_then(|val| val.number)
        .unwrap_or(0) as u8
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=idls/spl_token_22.json");

    // Read the IDL
    let idl_content = fs::read_to_string("idls/spl_token_22.json")?;
    let idl: CodemaIdl = serde_json::from_str(&idl_content)?;

    let mut args_structs = Vec::new();
    let mut accounts_structs = Vec::new();
    let mut arms = Vec::new();

    println!("Processing {} instructions", idl.program.instructions.len());

    // Process each instruction
    for instruction in &idl.program.instructions {
        let pascal_name = instruction.name.to_upper_camel_case();
        let variant = format_ident!("{}", pascal_name);
        let args_ty = format_ident!("{}Arguments", pascal_name);
        let accounts_ty = format_ident!("{}Accounts", pascal_name);

        // Extract discriminator
        let discriminator = extract_discriminator(instruction);
        println!(
            "Processing instruction: {} (discriminator: {})",
            instruction.name, discriminator
        );

        // Generate arguments struct
        let arg_fields = instruction
            .arguments
            .iter()
            .filter(|arg| arg.name != "discriminator") // Skip discriminator field
            .map(|arg| {
                let field_name = format_ident!("{}", arg.name.to_snake_case());
                let field_type = map_codama_type(&arg.arg_type);

                // Add serde attributes for special types
                match arg.arg_type.kind.as_str() {
                    "publicKeyTypeNode" => {
                        quote! {
                            #[serde(with = "pubkey_serde")]
                            pub #field_name: [u8; 32usize],
                        }
                    }
                    "numberTypeNode"
                        if matches!(arg.arg_type.format.as_deref(), Some("u64") | Some("u128")) =>
                    {
                        quote! {
                            #[serde(serialize_with = "crate::serialize_to_string")]
                            pub #field_name: #field_type,
                        }
                    }
                    _ => {
                        quote! {
                            pub #field_name: #field_type,
                        }
                    }
                }
            });

        args_structs.push(quote! {
            #[derive(::borsh::BorshDeserialize, Debug, Serialize)]
            pub struct #args_ty {
                #(#arg_fields)*
            }
        });

        // Generate accounts struct
        let account_fields = instruction.accounts.iter().map(|acc| {
            let field_name = format_ident!("{}", acc.name.to_snake_case());
            if acc.is_optional {
                quote! {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    pub #field_name: Option<String>,
                }
            } else {
                quote! {
                    pub #field_name: String,
                }
            }
        });

        accounts_structs.push(quote! {
            #[derive(Debug, Serialize)]
            pub struct #accounts_ty {
                #(#account_fields)*
                pub remaining: Vec<String>,
            }
        });

        // Generate parsing logic for arguments
        let mut parse_args = Vec::new();
        let mut arg_idents = Vec::new();

        for arg in &instruction.arguments {
            if arg.name == "discriminator" {
                continue; // Skip discriminator in parsing
            }

            let field_name = format_ident!("{}", arg.name.to_snake_case());
            let field_type = map_codama_type(&arg.arg_type);

            let parse_snippet = quote! {
                let #field_name: #field_type = <#field_type as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(#field_name), e))?;
            };

            parse_args.push(parse_snippet);
            arg_idents.push(field_name);
        }

        // Generate account extraction logic
        let mut extract_accounts = Vec::new();
        let mut account_idents = Vec::new();

        // Count mandatory accounts
        let mandatory_count = instruction
            .accounts
            .iter()
            .filter(|acc| !acc.is_optional)
            .count();

        extract_accounts.push(quote! {
            let mut keys = account_keys.iter();
            let mut opt_budget = account_keys.len().saturating_sub(#mandatory_count);
        });

        for acc in &instruction.accounts {
            let acc_name = format_ident!("{}", acc.name.to_snake_case());

            if acc.is_optional {
                extract_accounts.push(quote! {
                    let #acc_name = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                });
            } else {
                extract_accounts.push(quote! {
                    let #acc_name = keys.next().unwrap().clone();
                });
            }

            account_idents.push(acc_name);
        }

        // Generate match arm
        arms.push(quote! {
            [#discriminator] => {
                let mut rdr: &[u8] = rest;
                #(#parse_args)*
                let args = #args_ty { #(#arg_idents),* };
                #(#extract_accounts)*
                let remaining = keys.cloned().collect();
                let accounts = #accounts_ty {
                    remaining,
                    #(#account_idents),*
                };
                return Ok(Instruction::#variant { accounts, args });
            }
        });
    }

    // Generate the main instruction enum
    let variants = idl.program.instructions.iter().map(|ix| {
        let pascal = ix.name.to_upper_camel_case();
        let var = format_ident!("{}", pascal);
        let args_ty = format_ident!("{}Arguments", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        quote! { #var { accounts: #accounts_ty, args: #args_ty }, }
    });

    let instruction_enum = quote! {
        #[derive(Debug, Serialize)]
        #[serde(tag = "instruction_type")]
        pub enum Instruction {
            #(#variants)*
        }
    };

    // Generate the decode implementation
    let decode_impl = quote! {
        impl Instruction {
            pub fn decode(data: &[u8], account_keys: &[String]) -> Result<Self, String> {
                if data.is_empty() {
                    return Err("Empty instruction data".to_string());
                }

                let (discriminator, rest) = data.split_at(1);

                match discriminator {
                    #(#arms)*
                    _ => Err(format!("Unknown instruction discriminator: {:?}", discriminator)),
                }
            }
        }
    };

    // Generate the complete output
    let out = quote! {
        use anyhow::Result;

        mod pubkey_serde {
            use bs58;
            use serde::{Serializer};

            pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let s = bs58::encode(bytes).into_string();
                serializer.serialize_str(&s)
            }
        }

        #(#args_structs)*
        #(#accounts_structs)*
        #instruction_enum
        #decode_impl
    };

    // Write the generated code
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("idl.rs");
    fs::write(&dest_path, out.to_string())?;

    // Also write to src/idl.rs for development
    fs::write("src/idl.rs", out.to_string())?;
    Command::new("rustfmt").arg("src/idl.rs").status()?;

    println!("Successfully generated SPL Token-22 decoder!");
    Ok(())
}
