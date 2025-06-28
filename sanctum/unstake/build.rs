// build.rs

use anchor_idl::{EnumFields, GeneratorOptions, Idl, IdlAccountItem, IdlTypeDefinitionTy};
use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use serde_json::from_str;
use sha2::{Digest, Sha256};
use std::{fs, process::Command};

fn main() -> Result<()> {
    let path = "idls/unstake_program.json";

    // 1) load the IDL JSON
    let idl_json = fs::read_to_string(path)?;
    let idl: Idl = from_str(&idl_json)?;

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
                            // a bare PublicKey
                            anchor_idl::IdlType::PublicKey => {
                                quote! {
                                    #[serde(with = "pubkey_serde")]
                                    pub #f_ident: [u8; 32usize],
                                }
                            }
                            anchor_idl::IdlType::U64 | anchor_idl::IdlType::U128 => {
                                let ty = map_idl_type(&f.ty);
                                quote! {
                                    #[serde(serialize_with = "crate::serialize_to_string")]
                                    pub #f_ident: #ty,
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
                                        let ty_ts = map_idl_type(&f.ty);
                                        quote! { #f_ident: #ty_ts, }
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
                        .collect::<Vec<_>>();

                    // Pick the first variant as default
                    let default_ident = format_ident!("{}", variants[0].name.to_upper_camel_case());

                    tts.push(quote! {
                        #[derive(::borsh::BorshSerialize, ::borsh::BorshDeserialize, Clone, Debug, Serialize)]
                        pub enum #name {
                            #(#variant_defs)*
                        }

                        // impl Default for #name {
                        //     fn default() -> Self {
                        //         Self::#default_ident
                        //     }
                        // }
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
        let args_ty = format_ident!("{}Args", pascal);
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
                anchor_idl::IdlType::PublicKey => {
                    quote! {
                        #[serde(with = "pubkey_serde")]
                        pub #field_ident: [u8; 32usize],
                    }
                }
                // Option<PublicKey>
                anchor_idl::IdlType::Option(inner)
                    if matches!(**inner, anchor_idl::IdlType::PublicKey) =>
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

        // 3c) Accounts struct
        let flat = flatten_accounts(&ix.accounts);
        let acc_fields = flat.iter().map(|acc| {
            let f = format_ident!("{}", acc.name);
            quote! { pub #f: String, }
        });
        accounts_structs.push(quote! {
            #[derive(Debug, Serialize)]
            pub struct #accounts_ty {
                #(#acc_fields)*
                pub remaining: Vec<String>,
            }
        });

        // 3d) decoder arm
        let idents = flat
            .iter()
            .map(|acc| format_ident!("{}", acc.name))
            .collect::<Vec<_>>();
        let extract = idents.iter().map(|id| {
            quote! {
                let #id = keys.next().unwrap().clone();
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
                return Ok(Instruction::#variant { accounts, args });
            }
        });
    }

    // 4) enum Instruction
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

        pub use ix_data::*;
        pub use typedefs::*;
        pub use accounts_data::*;

        pub mod typedefs {
            use ::borsh::{BorshSerialize, BorshDeserialize};
            use anchor_lang::prelude::*;
            use serde::Serialize;
            use crate::pubkey_serializer::pubkey_serde;
            use crate::pubkey_serializer::pubkey_serde_option;
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
            #(#args_structs)*
        }

        #decoded_enum
        #decode_impl
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
        Bytes => quote! { Vec<u8> },
        String => quote! { String },
        PublicKey => quote! { [u8; 32usize] },
        Vec(inner) => {
            let i = map_idl_type(inner);
            quote! { Vec<#i> }
        }
        Array(inner, _len) => {
            let i = map_idl_type(inner);
            quote! { Vec<#i> }
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
