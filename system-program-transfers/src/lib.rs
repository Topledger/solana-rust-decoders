
// Import instruction processing module
include!("instruction.rs");


use bs58::decode;


use serde_wasm_bindgen::{from_value, to_value};
use std::panic;
use wasm_bindgen::prelude::*;


#[derive(Serialize)]
struct ErrorObj {
    error: String,
}

#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn parse(base58_str: &str, accounts_js: JsValue) -> JsValue {
    // 1) Base58 decode
    let decoded_bytes = match decode(base58_str).into_vec() {
        Ok(b) => b,
        Err(e) => {
            return to_value(&ErrorObj {
                error: format!("base58 decode failed: {}", e),
            })
            .unwrap();
        }
    };

    // 2) Deserialize accounts array
    let accounts: Vec<String> = match from_value(accounts_js) {
        Ok(v) => v,
        Err(e) => {
            return to_value(&ErrorObj {
                error: format!("accounts deserialize failed: {}", e),
            })
            .unwrap();
        }
    };

    // 3) Decode instruction (skips unknown discriminators here)
    let inst = match Instruction::decode(&accounts, &decoded_bytes) {
        Ok(ix) => ix,
        Err(e) => {
            return to_value(&ErrorObj {
                error: format!("Instruction::decode failed: {}", e),
            })
            .unwrap();
        }
    };

    // 4) Serialize the successful Instruction back to JsValue
    to_value(&inst).unwrap_or_else(|e| {
        to_value(&ErrorObj {
            error: format!("serialize failed: {}", e),
        })
        .unwrap()
    })
}