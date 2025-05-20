
// Import instruction processing module

use bs58::decode;

use std::str::FromStr;
include!("idl.rs");

use serde_wasm_bindgen;
use std::panic;
use wasm_bindgen::prelude::*;
use anyhow::Result;


#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn parse(base58_str: &str, accounts_js: JsValue) -> JsValue {
    // Decode the instruction data from base58:
    let decoded_bytes = decode(base58_str).into_vec().unwrap();

    // Deserialize the JS array of base58-strings into Vec<String>:
    let accounts: Vec<String> = serde_wasm_bindgen::from_value(accounts_js).unwrap();

    // Hand the raw strings directly to decode:
    let ix = Instruction::decode(&accounts, &decoded_bytes).unwrap();

    // Serialize back to JS:
    serde_wasm_bindgen::to_value(&ix).unwrap()
}


// fn str_to_pubkey(s: &str) -> anyhow::Result<Pubkey> {
//     // FromStr is implemented for Pubkey, and errors if the string
//     // isn’t valid Base58 or isn’t 32 bytes.
//     let pk =
//         Pubkey::from_str(s).map_err(|e| anyhow::anyhow!("Invalid pubkey string {}: {}", s, e))?;
//     Ok(pk)
// }

