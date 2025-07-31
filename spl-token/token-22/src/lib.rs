use borsh::BorshDeserialize;
use bs58::decode;
use serde::Serialize;

// Utility function for serializing large numbers as strings
pub fn serialize_to_string<S, T>(x: &T, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: ToString,
{
    s.serialize_str(&x.to_string())
}

include!("idl.rs");

use console_error_panic_hook;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// A simple struct to serialize errors back into JS
#[derive(Serialize)]
struct ErrorObj {
    error: String,
}

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn parse(base58_str: &str, accounts_js: JsValue) -> JsValue {
    // 1) Decode base58 → Vec<u8>
    let decoded = match decode(base58_str).into_vec() {
        Ok(b) => b,
        Err(e) => {
            let err = ErrorObj {
                error: format!("base58 decode failed: {}", e),
            };
            return to_value(&err).unwrap();
        }
    };

    // 2) Parse accounts from JavaScript
    let accounts: Vec<String> = match from_value(accounts_js) {
        Ok(v) => v,
        Err(e) => {
            let err = ErrorObj {
                error: format!("accounts deserialize failed: {}", e),
            };
            return to_value(&err).unwrap();
        }
    };

    // 3) Try instruction decode - fix argument order
    match Instruction::decode(&decoded, &accounts) {
        Ok(ix) => to_value(&ix).unwrap_or_else(|e| {
            let err = ErrorObj {
                error: format!("serialize failed: {}", e),
            };
            to_value(&err).unwrap()
        }),
        Err(e) => {
            let err = ErrorObj {
                error: format!("Instruction::decode failed: {}", e),
            };
            to_value(&err).unwrap()
        }
    }
}
