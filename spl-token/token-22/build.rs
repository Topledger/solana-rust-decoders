use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::fs;
use std::process::Command;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct CodemaIdl {
    program: CodemaProgram,
}

#[derive(Debug, Deserialize)]
struct CodemaProgram {
    instructions: Vec<CodemaInstruction>,
    #[serde(rename = "definedTypes")]
    defined_types: Vec<CodemaDefinedType>,
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
    r#type: CodemaType,
    #[serde(rename = "defaultValue")]
    default_value: Option<CodemaDefaultValue>,
    // Skip other fields we don't need
}

#[derive(Debug, Deserialize)]
struct CodemaDefaultValue {
    number: Option<u64>,
    data: Option<String>, // Added for hex data
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
    name: Option<String>, // Added for defined type names
}

#[derive(Debug, Deserialize)]
struct CodemaDefinedType {
    name: String,
    // We'll keep this simple for now and only handle what we need
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
        "zeroableOptionTypeNode" => {
            if let Some(ref item) = codama_type.item {
                let inner_type = map_codama_type(item);
                quote! { Option<#inner_type> }
            } else {
                quote! { Option<u8> }
            }
        }
        "definedTypeLinkNode" => {
            // Use the actual defined type name
            if let Some(ref name) = codama_type.name {
                let type_name = format_ident!("{}", name.to_upper_camel_case());
                quote! { #type_name }
            } else {
                quote! { u8 } // fallback
            }
        }
        "sizePrefixTypeNode" => {
            // Handle size-prefixed strings (u32 length + string data)
            quote! { String }
        }
        _ => {
            eprintln!("Unknown Codama type: {}", codama_type.kind);
            quote! { u8 } // fallback
        }
    }
}

fn extract_discriminator(instruction: &CodemaInstruction) -> Vec<u8> {
    let mut discriminators = Vec::new();
    
    for arg in &instruction.arguments {
        if let Some(ref default_value) = arg.default_value {
            if let Some(number) = default_value.number {
                discriminators.push(number as u8);
            } else if let Some(ref data) = default_value.data {
                // Handle hex discriminators
                if let Ok(bytes) = hex::decode(data) {
                    discriminators.extend(bytes);
                    return discriminators;
                }
            }
        }
    }
    
    // If we found multiple discriminator fields, use them all
    if discriminators.len() >= 2 {
        return discriminators;
    }
    
    // Default to [0] if no discriminator found
    if discriminators.is_empty() {
        vec![0]
    } else {
        discriminators
    }
}

// Function to extract extension type name from discriminator field name
fn extract_extension_type_name(discriminator_field_name: &str) -> Option<String> {
    if discriminator_field_name.ends_with("Discriminator") && discriminator_field_name != "discriminator" {
        let extension_name = discriminator_field_name
            .strip_suffix("Discriminator")
            .unwrap()
            .to_upper_camel_case();
        Some(extension_name)
    } else {
        None
    }
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=idls/spl_token_22.json");

    // Read the IDL
    let idl_content = fs::read_to_string("idls/spl_token_22.json")?;
    let idl: CodemaIdl = serde_json::from_str(&idl_content)?;

    let mut args_structs = Vec::new();
    let mut accounts_structs = Vec::new();
    let mut defined_types = Vec::new();
    let mut arms = Vec::new();

    // Process defined types first (enums, structs, etc.)
    for defined_type in &idl.program.defined_types {
        if defined_type.name == "tokenMetadataField" {
            // Generate the TokenMetadataField enum
            let enum_name = format_ident!("TokenMetadataField");
            let variants = quote! {
                #[derive(Debug, Serialize)]
                pub enum #enum_name {
                    Name,
                    Symbol, 
                    Uri,
                    Key(String),
                }
                
                impl ::borsh::BorshDeserialize for #enum_name {
                    fn deserialize(buf: &mut &[u8]) -> ::borsh::maybestd::io::Result<Self> {
                        let field_discriminator: u8 = ::borsh::BorshDeserialize::deserialize(buf)?;
                        match field_discriminator {
                            0 => Ok(#enum_name::Name),
                            1 => Ok(#enum_name::Symbol),
                            2 => Ok(#enum_name::Uri),
                            3 => {
                                // Read the key string for Key variant
                                let key_len: u32 = ::borsh::BorshDeserialize::deserialize(buf)?;
                                let mut key_bytes = vec![0u8; key_len as usize];
                                use std::io::Read;
                                buf.read_exact(&mut key_bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                                let key_string = String::from_utf8(key_bytes)
                                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                                Ok(#enum_name::Key(key_string))
                            }
                            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Unknown TokenMetadataField discriminator: {}", field_discriminator))),
                        }
                    }
                    
                    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> ::borsh::maybestd::io::Result<Self> {
                        let field_discriminator: u8 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;
                        match field_discriminator {
                            0 => Ok(#enum_name::Name),
                            1 => Ok(#enum_name::Symbol),
                            2 => Ok(#enum_name::Uri),
                            3 => {
                                // Read the key string for Key variant
                                let key_len: u32 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;
                                let mut key_bytes = vec![0u8; key_len as usize];
                                reader.read_exact(&mut key_bytes)?;
                                let key_string = String::from_utf8(key_bytes)
                                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                                Ok(#enum_name::Key(key_string))
                            }
                            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Unknown TokenMetadataField discriminator: {}", field_discriminator))),
                        }
                    }
                }
            };
            defined_types.push(variants);
        }
    }
    
    // Add placeholder types for missing defined types
    defined_types.push(quote! {
        #[derive(Debug, Serialize, ::borsh::BorshDeserialize)]
        pub struct AuthorityType(pub u8);
        
        #[derive(Debug, Serialize, ::borsh::BorshDeserialize)]
        pub struct DecryptableBalance(pub Vec<u8>);
        
        #[derive(Debug, Serialize, ::borsh::BorshDeserialize)]
        pub struct AccountState(pub u8);
    });

    println!("Processing {} instructions", idl.program.instructions.len());

    // Process each instruction
    for instruction in &idl.program.instructions {
        let has_extension = instruction.arguments.iter().any(|arg| {
            arg.name.ends_with("Discriminator") && arg.name != "discriminator"
        });

        let pascal_name = instruction.name.to_upper_camel_case();
        let variant = format_ident!("{}", pascal_name);
        let args_ty = format_ident!("{}Arguments", pascal_name);
        let accounts_ty = format_ident!("{}Accounts", pascal_name);

        // Extract discriminator
        let discriminator = extract_discriminator(instruction);
        println!(
            "Processing instruction: {} (discriminator: {:?})",
            instruction.name, discriminator
        );

        // Generate arguments struct
        let arg_fields = instruction.arguments.iter().filter_map(|arg| {
            if arg.name == "discriminator" {
                return None; // Skip main discriminator
            }

            // Skip any discriminator field that's part of a compound discriminator
            if discriminator.len() > 1 && arg.name.ends_with("Discriminator") && arg.name != "discriminator" {
                return None; // Skip this field as it's part of the discriminator
            }

            let field_name = format_ident!("{}", arg.name.to_snake_case());
            let field_type = map_codama_type(&arg.r#type);

            // Add serde attributes for special types
            let mut attributes = Vec::new();
            
            if matches!(arg.r#type.kind.as_str(), "publicKeyTypeNode") {
                attributes.push(quote! { #[serde(with = "pubkey_serde")] });
            }
            
            if matches!(arg.r#type.kind.as_str(), "numberTypeNode") {
                if let Some(format) = arg.r#type.format.as_ref() {
                    if format == "u64" || format == "u128" {
                        attributes.push(quote! { #[serde(serialize_with = "crate::serialize_to_string")] });
                    }
                }
            }

            if matches!(arg.r#type.kind.as_str(), "zeroableOptionTypeNode") {
                if let Some(item) = &arg.r#type.item {
                    if matches!(item.kind.as_str(), "publicKeyTypeNode") {
                        attributes.push(quote! { #[serde(with = "pubkey_serde_option")] });
                    }
                }
            }

            Some(quote! {
                #(#attributes)*
                pub #field_name: #field_type,
            })
        }).collect::<Vec<_>>();

        // Check if this instruction has extension discriminators and add extension_type field
        let extension_type_field = if discriminator.len() > 1 {
            vec![quote! {
                pub extension_type: String,
            }]
        } else {
            vec![]
        };

        args_structs.push(quote! {
            #[derive(::borsh::BorshDeserialize, Debug, Serialize)]
            pub struct #args_ty {
                #(#arg_fields)*
                #(#extension_type_field)*
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
                pub remaining: Vec<String>,
                #(#account_fields)*
            }
        });

        // Generate parsing logic for each argument
        let parse_args = instruction.arguments.iter().filter_map(|arg| {
            if arg.name == "discriminator" {
                return None; // Skip main discriminator
            }

            // Skip any discriminator field that's part of a compound discriminator
            if discriminator.len() > 1 && arg.name.ends_with("Discriminator") && arg.name != "discriminator" {
                return None; // Skip this field as it's already consumed by the match pattern
            }

            let field_name = format_ident!("{}", arg.name.to_snake_case());
            let field_type = map_codama_type(&arg.r#type);

            let parse_snippet = match arg.r#type.kind.as_str() {
                "sizePrefixTypeNode" => {
                    quote! {
                        let len = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(#field_name), e))?;
                        let mut bytes = vec![0u8; len as usize];
                        rdr.read_exact(&mut bytes)
                            .map_err(|e| format!("Failed to read {} bytes for {}: {}", len, stringify!(#field_name), e))?;
                        let #field_name = String::from_utf8(bytes)
                            .map_err(|e| format!("Failed to convert {} to string: {}", stringify!(#field_name), e))?;
                    }
                }
                "zeroableOptionTypeNode" => {
                    if let Some(ref item) = arg.r#type.item {
                        if item.kind == "publicKeyTypeNode" {
                            quote! {
                                let mut bytes = [0u8; 32];
                                rdr.read_exact(&mut bytes)
                                    .map_err(|e| format!("Failed to read pubkey bytes for {}: {}", stringify!(#field_name), e))?;
                                let #field_name = if bytes == [0u8; 32] {
                                    None
                                } else {
                                    Some(bytes)
                                };
                            }
                        } else {
                            quote! {
                                let #field_name: #field_type =
                                    <#field_type as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                                        format!(
                                            "Failed to deserialize {}: {}",
                                            stringify!(#field_name), e
                                        )
                                    })?;
                            }
                        }
                    } else {
                        quote! {
                            let #field_name: #field_type =
                                <#field_type as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                                    format!(
                                        "Failed to deserialize {}: {}",
                                        stringify!(#field_name), e
                                    )
                                })?;
                        }
                    }
                }
                _ => {
                    quote! {
                        let #field_name: #field_type =
                            <#field_type as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(#field_name), e
                                )
                            })?;
                    }
                }
            };

            Some(parse_snippet)
        }).collect::<Vec<_>>();

        // Get extension type name if this instruction has extension discriminators
        let extension_type_assignment = if discriminator.len() > 1 {
            // Use the full instruction name from the IDL as the extension_type
            let instruction_name = &instruction.name;
            Some(quote! {
                extension_type: #instruction_name.to_string(),
            })
        } else {
            None
        };

        // Collect field names for struct instantiation
        let arg_idents = instruction.arguments.iter().filter_map(|arg| {
            if arg.name == "discriminator" {
                return None;
            }
            
            // Skip any discriminator field that's part of a compound discriminator
            if discriminator.len() > 1 && arg.name.ends_with("Discriminator") && arg.name != "discriminator" {
                return None;
            }
            
            Some(format_ident!("{}", arg.name.to_snake_case()))
        }).collect::<Vec<_>>();

        // Generate account extraction logic
        let account_extractions = instruction.accounts.iter().enumerate().map(|(i, account)| {
            let account_name = format_ident!("{}", account.name.to_snake_case());
            let account_name_str = &account.name;
            quote! {
                let #account_name = account_keys.get(#i).ok_or_else(|| {
                    format!("Missing account at index {}: {}", #i, #account_name_str)
                })?;
            }
        }).collect::<Vec<_>>();

        let accounts_instantiation_fields = instruction.accounts.iter().map(|account| {
            let account_name = format_ident!("{}", account.name.to_snake_case());
            if account.is_optional {
                quote! {
                    #account_name: Some(#account_name.clone()),
                }
            } else {
                quote! {
                    #account_name: #account_name.clone(),
                }
            }
        }).collect::<Vec<_>>();

        // Create struct instantiation with all fields
        let struct_fields = arg_idents.iter().map(|ident| {
            quote! { #ident, }
        }).collect::<Vec<_>>();

        let args_instantiation = if discriminator.len() > 1 {
            quote! {
                let args = #args_ty {
                    #(#struct_fields)*
                    #extension_type_assignment
                };
            }
        } else {
            quote! {
                let args = #args_ty {
                    #(#struct_fields)*
                };
            }
        };

        // Generate match arm
        if discriminator.len() > 1 {
            // Multi-byte discriminator
            let disc_bytes = discriminator.iter().map(|&b| quote! { #b });
            arms.push(quote! {
                [#(#disc_bytes),*] => {
                    let mut rdr: &[u8] = rest;
                    #(#parse_args)*
                    #args_instantiation
                    #(#account_extractions)*
                    let accounts = #accounts_ty {
                        remaining: vec![],
                        #(#accounts_instantiation_fields)*
                    };
                    return Ok(Instruction::#variant { accounts, args });
                }
            });
        } else {
            // Single-byte discriminator
            let disc_byte = discriminator[0];
            arms.push(quote! {
                [#disc_byte] => {
                    let mut rdr: &[u8] = rest;
                    #(#parse_args)*
                    #args_instantiation
                    #(#account_extractions)*
                    let accounts = #accounts_ty {
                        remaining: vec![],
                        #(#accounts_instantiation_fields)*
                    };
                    return Ok(Instruction::#variant { accounts, args });
                }
            });
        }
    }

    // Generate the main instruction enum
    let mut instruction_variants = Vec::new();
    for ix in &idl.program.instructions {
        let pascal = ix.name.to_upper_camel_case();
        let var = format_ident!("{}", pascal);
        let args_ty = format_ident!("{}Arguments", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);

        // For extension-based instructions, we'll use a custom instruction_type
        let instruction_type_override = if ix.arguments.iter().any(|arg| {
            arg.name.ends_with("Discriminator") && arg.name != "discriminator"
        }) {
            if let Some(ext_arg) = ix.arguments.iter().find(|arg| {
                arg.name.ends_with("Discriminator") && arg.name != "discriminator"
            }) {
                extract_extension_type_name(&ext_arg.name)
            } else {
                None
            }
        } else {
            None
        };

        // Generate instruction enum variant with custom rename for extensions
        let variant_definition = if let Some(ref custom_type) = instruction_type_override {
            quote! {
                #[serde(rename = #custom_type)]
                #var { accounts: #accounts_ty, args: #args_ty },
            }
        } else {
            quote! {
                #var { accounts: #accounts_ty, args: #args_ty },
            }
        };

        instruction_variants.push(variant_definition);
    }

    let instruction_enum = quote! {
        #[derive(Debug, Serialize)]
        #[serde(tag = "instruction_type")]
        pub enum Instruction {
            #(#instruction_variants)*
        }
    };

    // Generate the decode implementation
    let decode_impl = quote! {
        impl Instruction {
            pub fn decode(data: &[u8], account_keys: &[String]) -> Result<Self, String> {
                if data.is_empty() {
                    return Err("Empty instruction data".to_string());
                }

                // Try 8-byte discriminators first (for multi-byte discriminators)
                if data.len() >= 8 {
                    let (discriminator, rest) = data.split_at(8);
                    match discriminator {
                        #(#arms)*
                        _ => {} // Fall through to try shorter discriminators
                    }
                }

                // Try 2-byte discriminators (for compound discriminators like transferFee instructions)
                if data.len() >= 2 {
                    let (discriminator, rest) = data.split_at(2);
                    match discriminator {
                        #(#arms)*
                        _ => {} // Fall through to try 1-byte discriminator
                    }
                }

                // Try 1-byte discriminators (for single-byte discriminators) 
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
        use std::io::Read;

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

        mod pubkey_serde_option {
            use bs58;
            use serde::{Serializer};

            pub fn serialize<S>(bytes: &Option<[u8; 32]>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match bytes {
                    Some(bytes) => {
                        let s = bs58::encode(bytes).into_string();
                        serializer.serialize_some(&s)
                    }
                    None => serializer.serialize_none(),
                }
            }
        }

        #(#args_structs)*
        #(#accounts_structs)*
        #(#defined_types)*
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
