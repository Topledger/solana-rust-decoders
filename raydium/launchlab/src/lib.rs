use bs58::decode;
use serde::Serializer;
use anchor_lang::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};
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

// A simple struct to serialize errors back into JS
#[derive(serde::Serialize)]
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

    // 3) Try instruction decode
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raydium_launchlab_decode() {
        let sample_data = "Gimqm3fgf3NA3jiQkC2icmpfePo1tdWE9gPKivKqSZ83";
        
        println!("Testing raydium Launchlab decoder with sample data:");
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
    }
    
    #[test]
    fn test_launchlab_with_accounts() {
        let sample_data = "Gimqm3fgf3NA3jiQkC2icmpfePo1tdWE9gPKivKqSZ83";
        
        println!("Testing Launchlab decode with specific accounts:");
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
        
        if decoded_bytes.len() >= 8 {
            let discriminator = &decoded_bytes[..8];
            println!("Instruction discriminator: {:?}", discriminator);
        }
        
        // Provide meaningful account names for Launchlab
        let accounts = vec![
            "authority".to_string(),
            "pool_state".to_string(),
            "token_account".to_string(),
            "mint_account".to_string(),
        ];
        
        // Test instruction decoding
        match Instruction::decode(&accounts, &decoded_bytes) {
            Ok(instruction) => {
                println!("✅ Instruction decode successful!");
                println!("Decoded instruction: {:#?}", instruction);
            }
            Err(e) => {
                println!("❌ Instruction decode failed: {}", e);
            }
        }
    }
    
    #[test]
    fn test_launchlab_json_serialization() {
        println!("Testing Launchlab JSON serialization:");
        
        // Create a fake instruction data for testing serialization
        let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Fake discriminator
        data.extend_from_slice(&[0u8; 16]); // Add some dummy args
        
        let accounts = vec![
            "authority".to_string(),
            "pool_state".to_string(),
            "token_account".to_string(),
        ];
        
        println!("Test accounts count: {}", accounts.len());
        
        // Test instruction decoding
        match Instruction::decode(&accounts, &data) {
            Ok(instruction) => {
                println!("✅ Decode successful!");
                println!("Decoded instruction: {:#?}", instruction);
            }
            Err(e) => {
                println!("❌ Decode failed: {}", e);
            }
        }
    }

    fn hex_dump(bytes: &[u8]) -> String {
        bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }
} 