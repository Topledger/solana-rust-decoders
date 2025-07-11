// build.rs

use anchor_idl::{GeneratorOptions, Idl, IdlAccountItem};
use anyhow::Result;
use heck::{ ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use serde_json::from_str;
use sha2::{Digest, Sha256};
use std::{fs, process::Command};



fn main() -> Result<()> {
    let path = "idls/jito_tip_payment.json";
    // 1) load the IDL JSON
    let idl_json = fs::read_to_string(path)?;
    let idl: Idl = from_str(&idl_json)?;

    // 2) generate typedefs
    let mut opts = GeneratorOptions::default();
    opts.idl_path = path.into();
    let gen = opts.to_generator();
    let typedefs_ts = anchor_idl::generate_typedefs(&gen.idl.types, &gen.struct_opts);

    // 3) build each Args and Accounts struct
    let mut args_structs = Vec::new();
    let mut accounts_structs = Vec::new();
    // also collect (literal discriminator array, variant ident, args_ty ident)
    let mut arms = Vec::new();

    for ix in &idl.instructions {
        let name = &ix.name;
        // PascalCase: uppercase first character
        let pascal = name.to_upper_camel_case();
           
        let args_ty = format_ident!("{}Args", pascal);
        let accounts_ty = format_ident!("{}Accounts", pascal);
        let var = format_ident!("{}", pascal);

        // 3a) compute Anchor discriminator = first 8 bytes of sha256("global:ix_name")
        let mut hasher = Sha256::new();
        hasher.update(b"global:");
        hasher.update(name.to_snake_case());
        let hash = hasher.finalize();
        let disc_bytes: [u8; 8] = hash[0..8].try_into().unwrap();
        let disc_tokens = disc_bytes.iter().map(|b| quote! { #b }).collect::<Vec<_>>();

        // 3b) match arm: split off only the bytes needed for this Args struct
        // arms.push(quote! {
        //     [#(#disc_tokens),*] => {
        //         // split exactly the number of bytes for this arguments struct
        //         let (args_bytes, _remaining) = rest.split_at(mem::size_of::<#args_ty>());
        //         return Ok(Instruction::#var(
        //             #args_ty::try_from_slice(args_bytes)?
        //         ));
        //     }
        // });

        // 3c) build the Args struct fields
        let fields = ix.args.iter().map(|arg| {
            let f = format_ident!("{}", &arg.name.to_snake_case());
            let ty_ts = map_idl_type(&arg.ty);
            quote! { pub #f: #ty_ts, }
        });

        args_structs.push(quote! {
            #[derive(::borsh::BorshDeserialize, Debug, Serialize)]
            pub struct #args_ty {
                #(#fields)*
            }
        });

        let flat_accounts = flatten_accounts(&ix.accounts);

        // build Accounts struct
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

         // build match arm for decode_with_accounts
         let acc_idents: Vec<_> = flat_accounts.iter().map(|acc| format_ident!("{}", &acc.name)).collect();
         let extract_accounts = acc_idents.iter().map(|ident| {
             quote! {
                 let #ident = keys.next().unwrap().clone();
             }
         });

         arms.push(quote! {
            [#(#disc_tokens),*] => {
                // decode args
                let (args_bytes, _tail) = rest.split_at(mem::size_of::<#args_ty>());
                let args = #args_ty::try_from_slice(args_bytes)?;
                // consume accounts in order
                let mut keys = account_keys.iter();
                #(#extract_accounts)*
                let remaining = keys.cloned().collect();
                let accounts = #accounts_ty { #(#acc_idents),*, remaining };
                return Ok(Instruction::#var { accounts, args });
            }
        });
    }

    // 4) build the Instruction enum (no derives)
    let variants = idl.instructions.iter().map(|ix| {
        let name = &ix.name;
        let pascal = name.to_upper_camel_case();
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

    // 5) implement decode()
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

    // 6) glue & write
    let out = quote! {
        // @generated by build.rs — DO NOT EDIT
        #[allow(dead_code)]

        // for splitting slices and converting sizes
        use std::convert::TryInto;
        use std::mem;

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
            use super::*;
            #(#accounts_structs)*
        }

        pub mod ix_data {
            use serde::Serialize;
            use super::*;
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
        PublicKey => quote! { String },
        Vec(inner) => {
            let inner = map_idl_type(inner);
            quote! { Vec<#inner> }
        }
        Array(inner, len) => {
            let inner = map_idl_type(inner);
            let l = *len;
            quote! { [#inner; #l] }
        }
        Defined(n) => {
            let ident = format_ident!("{}", n);
            quote! { #ident }
        }
        Option(inner) => {
            let inner = map_idl_type(inner);
            quote! { Option<#inner> }
        }
        _ => panic!("unsupported IDL type: {:?}", ty),
    }
}


fn flatten_accounts<'a>(items: &'a [IdlAccountItem]) -> Vec<&'a anchor_idl::IdlAccount> {
    let mut out = Vec::new();
    for item in items {
        match item {
            IdlAccountItem::IdlAccount(acc) => out.push(acc),
            IdlAccountItem::IdlAccounts(group) => {
                out.extend(flatten_accounts(&group.accounts));
            }
        }
    }
    out
}