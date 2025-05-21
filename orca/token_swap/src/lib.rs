use bs58::decode;
include!("idl.rs");

use serde_wasm_bindgen::{to_value, from_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use console_error_panic_hook;

// A simple struct to serialize errors back into JS
#[derive(Serialize)]
struct ErrorObj {
    error: String,
}

#[wasm_bindgen(start)]
pub fn run() {
    // Install the panic hook so Rust panics show up in your console
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn parse(base58_str: &str, accounts_js: JsValue) -> JsValue {
    // 1) Decode base58
    let decoded_bytes = match decode(base58_str).into_vec() {
        Ok(b) => b,
        Err(e) => {
            let err = ErrorObj { error: format!("base58 decode failed: {}", e) };
            return to_value(&err).unwrap();
        }
    };

    // 2) Deserialize accounts_js → Vec<String>
    let accounts: Vec<String> = match from_value(accounts_js) {
        Ok(v) => v,
        Err(e) => {
            let err = ErrorObj { error: format!("accounts deserialize failed: {}", e.to_string()) };
            return to_value(&err).unwrap();
        }
    };

    // 3) Call your decoder
    let ix = match Instruction::decode(&accounts, &decoded_bytes) {
        Ok(ix) => ix,
        Err(e) => {
            let err = ErrorObj { error: format!("Instruction::decode failed: {}", e) };
            return to_value(&err).unwrap();
        }
    };

    // 4) Serialize the successful result, or catch that error too
    to_value(&ix).unwrap_or_else(|e| {
        let err = ErrorObj { error: format!("serialize failed: {}", e) };
        to_value(&err).unwrap()
    })
}
