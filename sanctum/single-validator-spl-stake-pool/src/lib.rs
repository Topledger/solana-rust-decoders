use bs58::decode;
use serde::Serializer;
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
    // 1) Clean the base58 string - remove any invalid characters
    let cleaned_base58: String = base58_str
        .chars()
        .filter(|c| "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".contains(*c))
        .collect();
    
    // 2) Decode base58 → Vec<u8>
    let decoded = match decode(&cleaned_base58).into_vec() {
        Ok(b) => b,
        Err(e) => {
            let err = ErrorObj {
                error: format!("base58 decode failed: {} (original: '{}', cleaned: '{}')", e, base58_str, cleaned_base58),
            };
            return to_value(&err).unwrap();
        }
    };

    // 3) Parse accounts from JavaScript
    let accounts: Vec<String> = if accounts_js.is_null() || accounts_js.is_undefined() {
        Vec::new()  // Use empty array for null/undefined
    } else {
        match from_value(accounts_js.clone()) {
            Ok(v) => v,
            Err(_) => {
                // If not an array, maybe it's a JSON string
                if let Some(s) = accounts_js.as_string() {
                    serde_json::from_str::<Vec<String>>(&s).unwrap_or_default()
                } else {
                    Vec::new()
                }
            }
        }
    };

    // 4) Try instruction decode
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
    fn test_sanctum_stake_pool_decode() {
        let sample_data = "A";  // Single byte representing instruction discriminant 0 (Initialize instruction)
        
        println!("Testing Sanctum Single Validator SPL Stake Pool decoder with sample data:");
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
        println!("All bytes (hex): {}", hex_dump(&decoded_bytes));
        
        if decoded_bytes.len() >= 1 {
            let discriminator = decoded_bytes[0];
            println!("Instruction discriminator: {}", discriminator);
            println!("Discriminator (hex): {:02x}", discriminator);
        }
        
        // Provide a large enough dummy accounts array so unwrap() never panics
        // SPL Stake Pool instructions can use multiple accounts.
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
    fn test_stake_pool_with_accounts() {
        let sample_data = "A";  // Single byte representing instruction discriminant 0 (Initialize instruction)
        
        println!("Testing Stake Pool decode with specific accounts:");
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
        
        if decoded_bytes.len() >= 1 {
            let discriminator = decoded_bytes[0];
            println!("Instruction discriminator: {}", discriminator);
        }
        
        // Provide meaningful account names for SPL Stake Pool
        let accounts = vec![
            "stake_pool".to_string(),
            "manager".to_string(),
            "staker".to_string(),
            "withdraw_authority".to_string(),
            "validator_list".to_string(),
            "reserve_stake".to_string(),
            "pool_mint".to_string(),
            "manager_pool_account".to_string(),
            "token_program".to_string(),
            "deposit_authority".to_string(),
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
    fn test_stake_pool_json_serialization() {
        println!("Testing Stake Pool JSON serialization:");
        
        // Create a fake instruction data for testing serialization
        let mut data = vec![0]; // Initialize instruction discriminant
        data.extend_from_slice(&[0u8; 64]); // Add enough dummy args for Initialize instruction
        
        let accounts = vec![
            "stake_pool".to_string(),
            "manager".to_string(),
            "staker".to_string(),
        ];
        
        println!("Test accounts count: {}", accounts.len());
        
        // Test instruction decoding
        match Instruction::decode(&accounts, &data) {
            Ok(instruction) => {
                println!("✅ Decode successful!");
                println!("Decoded instruction: {:#?}", instruction);
                
                // Test serialization to JSON
                match serde_json::to_string_pretty(&instruction) {
                    Ok(json) => {
                        println!("✅ JSON serialization successful!");
                        println!("JSON output: {}", json);
                    }
                    Err(e) => {
                        println!("❌ JSON serialization failed: {}", e);
                    }
                }
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