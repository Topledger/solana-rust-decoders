use bs58::decode;
use serde::Serializer;
use anchor_lang::prelude::*;
mod pubkey_serializer;

// Utility function for serializing large numbers as strings
pub fn serialize_to_string<S, T>(x: &T, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    s.serialize_str(&x.to_string())
}

include!("idl.rs");

use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

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
            return JsValue::from_str(&format!("base58 decode failed: {}", e));
        }
    };

    // 2) Is it an Anchor‐logged event?
    if decoded.len() >= 8 && &decoded[..8] == &events::EVENT_LOG_DISCRIMINATOR {
        match events::Event::decode(&decoded) {
            Ok(ev) => {
                return JsValue::from_str(&format!("{:?}", ev));
            }
            Err(e) => {
                return JsValue::from_str(&format!("Event decode failed: {}", e));
            }
        }
    }

    // 3) Try instruction decode (accounts not used for now)
    let accounts: Vec<String> = vec![];

    match Instruction::decode(&accounts, &decoded) {
        Ok(ix) => JsValue::from_str(&format!("{:?}", ix)),
        Err(e) => JsValue::from_str(&format!("decode failed: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raydium_clmm_decode() {
        let sample_data = "Gimqm3fgf3NA3jiQkC2icmpfePo1tdWE9gPKivKqSZ83";
        
        println!("Testing raydium CLMM decoder with sample data:");
        println!("Base58 input: {}", sample_data);
        
        // Decode base58 to bytes
        let decoded_bytes = match decode(sample_data).into_vec() {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("❌ Base58 decode failed: {}", e);
                panic!("Base58 decode failed: {}", e);
            }
        };
        
        println!("✅ Base58 decode successful!");
        println!("Decoded bytes length: {}", decoded_bytes.len());
        println!("First 32 bytes (hex): {}", hex_dump(&decoded_bytes[..decoded_bytes.len().min(32)]));
        
        if decoded_bytes.len() >= 8 {
            let discriminator = &decoded_bytes[..8];
            println!("Instruction discriminator: {:?}", discriminator);
            println!("Discriminator (hex): {}", hex_dump(discriminator));
        }
        
        // Provide a large enough dummy accounts array so unwrap() never panics
        // Most Raydium instructions use <= 20 accounts.
        let dummy_accounts: Vec<String> = vec!["".to_string(); 32];
        
        // Test instruction decoding
        match Instruction::decode(&dummy_accounts, &decoded_bytes) {
            Ok(instruction) => {
                println!("✅ Instruction decode successful!");
                println!("Decoded instruction: {:#?}", instruction);
            }
            Err(e) => {
                println!("❌ Instruction decode failed: {}", e);
                // Don't panic here, just show the error
            }
        }
        
        // Test event decoding (if it looks like an event)
        if decoded_bytes.len() >= 8 && &decoded_bytes[..8] == &events::EVENT_LOG_DISCRIMINATOR {
            println!("This looks like an event, testing event decode...");
            match events::Event::decode(&decoded_bytes) {
                Ok(event) => {
                    println!("✅ Event decode successful!");
                    println!("Decoded event: {:#?}", event);
                }
                Err(e) => {
                    println!("❌ Event decode failed: {}", e);
                }
            }
        } else {
            println!("This doesn't appear to be an event (discriminator doesn't match)");
        }
    }
    
    fn hex_dump(bytes: &[u8]) -> String {
        bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }
} 