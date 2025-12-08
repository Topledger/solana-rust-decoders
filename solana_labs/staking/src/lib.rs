#![allow(unused_imports, dead_code, unused_variables)]

// Core imports for both native and WASM
use borsh::BorshDeserialize;
use serde::Serialize;

mod pubkey_serializer;

// Include generated IDL types (contains Instruction, Event, etc.)
include!("idl.rs");

// Re-export typedefs and events for convenience
pub use typedefs::*;
pub use events::*;

// WASM-specific code (only compiled when "wasm" feature is enabled)
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use serde_wasm_bindgen::{to_value, from_value};

#[cfg(feature = "wasm")]
use bs58::decode;

// A simple struct to serialize errors back into JS
#[cfg(feature = "wasm")]
#[derive(Serialize)]
struct ErrorObj {
    error: String,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn run() {
    // Install the panic hook so Rust panics show up in your console
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse(base58_str: &str, accounts_js: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue {
    // 1) Decode base58
    let decoded_bytes = match decode(base58_str).into_vec() {
        Ok(b) => b,
        Err(e) => {
            let err = ErrorObj { error: format!("base58 decode failed: {}", e) };
            return to_value(&err).unwrap();
        }
    };

    // 2) Check if data starts with EVENT_LOG_DISCRIMINATOR - if so, parse as event
    if decoded_bytes.len() >= 8 {
        let disc: [u8; 8] = decoded_bytes[0..8].try_into().unwrap();
        if disc == events::EVENT_LOG_DISCRIMINATOR {
            // Parse as event
            match events::Event::decode(&decoded_bytes) {
                Ok(event) => {
                    return to_value(&event).unwrap_or_else(|e| {
                        let err = ErrorObj { error: format!("serialize failed: {}", e) };
                        to_value(&err).unwrap()
                    });
                }
                Err(e) => {
                    let err = ErrorObj { error: format!("Event::decode failed: {}", e) };
                    return to_value(&err).unwrap();
                }
            }
        }
    }

    // 3) Not an event - parse as instruction
    // Deserialize accounts_js → Vec<String>
    let accounts: Vec<String> = match from_value(accounts_js) {
        Ok(v) => v,
        Err(e) => {
            let err = ErrorObj { error: format!("accounts deserialize failed: {}", e.to_string()) };
            return to_value(&err).unwrap();
        }
    };

    // 4) Call instruction decoder
    let ix = match Instruction::decode(&accounts, &decoded_bytes) {
        Ok(ix) => ix,
        Err(e) => {
            let err = ErrorObj { error: format!("Instruction::decode failed: {}", e) };
            return to_value(&err).unwrap();
        }
    };

    // 5) Serialize the successful result
    to_value(&ix).unwrap_or_else(|e| {
        let err = ErrorObj { error: format!("serialize failed: {}", e) };
        to_value(&err).unwrap()
    })
}
