pub mod pb;

// Import instruction processing module
pub mod instructions;

pub mod prepare_input_accounts;
use bs58::decode;
use pb::sf::solana::orca_whirlpool::v1::JsonMeta;

use serde_wasm_bindgen;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn parse(base58_str: &str, accounts_js: JsValue) -> JsValue {
    let accounts: Vec<String> = serde_wasm_bindgen::from_value(accounts_js).unwrap();
    let decoded_bytes = decode(base58_str).into_vec().unwrap();
    let data = instructions::process_instruction(&decoded_bytes, &accounts).unwrap();
    let json_meta: JsonMeta = data.into();
    serde_wasm_bindgen::to_value(&json_meta).unwrap()
}


