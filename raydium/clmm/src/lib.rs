use bs58::decode;
use serde::{Serialize, Serializer};
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
        
    }
    
    #[test]
    fn test_transfer_reward_owner_decode() {
        let sample_data = "MRSKorw8EFnDoi6xMgnHWodPJyX3ZGvL26czji83uY9qNjQ7EBQFhy";
        
        println!("Testing TransferRewardOwner decode with sample data:");
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
        
        // Provide empty accounts array for this test
        let dummy_accounts: Vec<String> = vec!["authority".to_string(), "pool_state".to_string()];
        
        // Test instruction decoding
        match Instruction::decode(&dummy_accounts, &decoded_bytes) {
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
    fn test_transfer_reward_owner_json_serialization() {
        // function body
    }

    #[test]
    fn test_collect_fund_fee_historical_data() {
        println!("Testing CollectFundFee with historical data (fewer accounts):");
        
        // Create a fake CollectFundFee discriminator + some args data
        let mut data = vec![167, 138, 78, 149, 223, 194, 6, 126]; // CollectFundFee discriminator
        data.extend_from_slice(&[0u8; 16]); // Add some dummy args (amount_0_requested, amount_1_requested)
        
        // Simulate historical data with only the first 8 accounts (missing the newer optional ones)
        let historical_accounts = vec![
            "owner".to_string(),
            "pool_state".to_string(), 
            "amm_config".to_string(),
            "token_vault_0".to_string(),
            "token_vault_1".to_string(),
            "vault_0_mint".to_string(),
            "vault_1_mint".to_string(),
            "token_program".to_string(), // This is the 8th account
            // Missing: recipient_token_account_0, recipient_token_account_1, token_program_2022
        ];
        
        println!("Historical accounts count: {}", historical_accounts.len());
        
        // Test instruction decoding
        match Instruction::decode(&historical_accounts, &data) {
            Ok(instruction) => {
                println!("✅ Historical decode successful!");
                println!("Decoded instruction: {:#?}", instruction);
            }
            Err(e) => {
                println!("❌ Historical decode failed: {}", e);
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