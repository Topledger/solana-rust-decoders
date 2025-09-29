
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
    // Better error handling to avoid panics
    let accounts: Vec<String> = match serde_wasm_bindgen::from_value(accounts_js) {
        Ok(acc) => acc,
        Err(e) => {
            let error_info = DebugInfo {
                discriminator: 0,
                data_length: 0,
                raw_data_hex: String::new(),
                error_message: format!("Failed to parse accounts: {}", e),
            };
            let error_instruction = Instruction::UnknownInstruction { debug_info: error_info };
            return serde_wasm_bindgen::to_value(&error_instruction).unwrap_or(JsValue::NULL);
        }
    };

    let decoded_bytes = match decode(base58_str).into_vec() {
        Ok(bytes) => bytes,
        Err(e) => {
            let error_info = DebugInfo {
                discriminator: 0,
                data_length: 0,
                raw_data_hex: base58_str.to_string(),
                error_message: format!("Failed to decode base58: {}", e),
            };
            let error_instruction = Instruction::UnknownInstruction { debug_info: error_info };
            return serde_wasm_bindgen::to_value(&error_instruction).unwrap_or(JsValue::NULL);
        }
    };

    let data = match Instruction::decode(&accounts, &decoded_bytes) {
        Ok(instruction) => instruction,
        Err(e) => {
            let error_info = DebugInfo {
                discriminator: 0,
                data_length: decoded_bytes.len(),
                raw_data_hex: hex::encode(&decoded_bytes),
                error_message: format!("Failed to decode instruction: {}", e),
            };
            Instruction::UnknownInstruction { debug_info: error_info }
        }
    };

    match serde_wasm_bindgen::to_value(&data) {
        Ok(js_value) => js_value,
        Err(e) => {
            // If serialization fails, create a simple error object
            let error_msg = format!("Serialization failed: {}", e);
            serde_wasm_bindgen::to_value(&serde_json::json!({
                "error": error_msg,
                "instruction_type": "UnknownInstruction"
            })).unwrap_or(JsValue::NULL)
        }
    }
}


