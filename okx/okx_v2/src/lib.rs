use borsh::BorshDeserialize;
use bs58::decode;
use serde::Serialize;
mod pubkey_serializer;
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

    // 2) Is it an Anchor‐logged event?
    if decoded.len() >= 8 && &decoded[..8] == &events::EVENT_LOG_DISCRIMINATOR {
        match events::Event::decode(&decoded) {
            Ok(ev) => {
                return to_value(&ev).unwrap_or_else(|e| {
                    let err = ErrorObj {
                        error: format!("event serialize failed: {}", e),
                    };
                    to_value(&err).unwrap()
                })
            }
            Err(e) => {
                let err = ErrorObj {
                    error: format!("Event decode failed: {}", e),
                };
                return to_value(&err).unwrap();
            }
        }
    }

    // 3) Otherwise, treat it as an instruction
    let accounts: Vec<String> = match from_value(accounts_js) {
        Ok(v) => v,
        Err(e) => {
            let err = ErrorObj {
                error: format!("accounts deserialize failed: {}", e),
            };
            return to_value(&err).unwrap();
        }
    };

    match Instruction::decode(&accounts, &decoded) {
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


