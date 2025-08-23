
// Import instruction processing module
include!("instruction.rs");


use bs58::decode;


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
    let data = Instruction::decode(&accounts, &decoded_bytes).unwrap();
    serde_wasm_bindgen::to_value(&data).unwrap()
}


