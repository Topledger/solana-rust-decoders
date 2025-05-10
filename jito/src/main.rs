use anyhow::Result;
use borsh::BorshDeserialize;
use bs58::decode;
include!("idl.rs");
use anchor_lang::prelude::Pubkey;
use std::str::FromStr;

fn str_to_pubkey(s: &str) -> anyhow::Result<Pubkey> {
    // FromStr is implemented for Pubkey, and errors if the string
    // isn’t valid Base58 or isn’t 32 bytes.
    let pk = Pubkey::from_str(s)
        .map_err(|e| anyhow::anyhow!("Invalid pubkey string {}: {}", s, e))?;
    Ok(pk)
}

fn main() -> Result<()> {
    // decode the 8-byte discriminator + Borsh args in one go:
    let data_base58 = "Cc9CdHiv1Kc";
    let data = decode(data_base58).into_vec()?;
    let accounts = ["94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB", 
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB", 
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB",
                                "94GevKgX3yofzQgRrNfdyVwJUBWLBbxgAxpT5SRfw7nB"];
    let pubkeys: Result<Vec<Pubkey>> = accounts
    .iter()
    .map(|s| str_to_pubkey(s))
    .collect();
let pubkeys = pubkeys?;
    let ix = Instruction::decode(&pubkeys, &data)?;
    println!("{:#?}", ix);

    Ok(())
}