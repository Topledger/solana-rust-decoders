use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;

// BPF Loader Upgradeable discriminators
const INITIALIZE_BUFFER_DISCRIMINATOR: u32 = 0;
const WRITE_DISCRIMINATOR: u32 = 1;
const DEPLOY_WITH_MAX_DATA_LEN_DISCRIMINATOR: u32 = 2;
const UPGRADE_DISCRIMINATOR: u32 = 3;
const SET_AUTHORITY_DISCRIMINATOR: u32 = 4;
const CLOSE_DISCRIMINATOR: u32 = 5;
const EXTEND_PROGRAM_DISCRIMINATOR: u32 = 6;
const SET_AUTHORITY_CHECKED_DISCRIMINATOR: u32 = 7;

// Account structures for each instruction

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeBufferAccounts {
    pub buffer: String,
    pub authority: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteAccounts {
    pub buffer: String,
    pub authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeployWithMaxDataLenAccounts {
    pub payer: String,
    pub program_data: String,
    pub program: String,
    pub buffer: String,
    pub rent: String,
    pub clock: String,
    pub system_program: String,
    pub authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpgradeAccounts {
    pub program_data: String,
    pub program: String,
    pub buffer: String,
    pub spill: String,
    pub rent: String,
    pub clock: String,
    pub authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAuthorityAccounts {
    pub target: String,
    pub current_authority: String,
    pub new_authority: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseAccounts {
    pub account: String,
    pub recipient: String,
    pub authority: Option<String>,
    pub program: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtendProgramAccounts {
    pub program_data: String,
    pub program: String,
    pub system_program: Option<String>,
    pub payer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAuthorityCheckedAccounts {
    pub target: String,
    pub current_authority: String,
    pub new_authority: String,
}

// Argument structures

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteArgs {
    pub offset: u32,
    pub bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeployWithMaxDataLenArgs {
    pub max_data_len: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtendProgramArgs {
    pub additional_bytes: u32,
}

// Debug information structure
#[derive(Serialize, Deserialize, Debug)]
pub struct DebugInfo {
    pub discriminator: u32,
    pub data_length: usize,
    pub raw_data_hex: String,
    pub error_message: String,
}

// Main instruction enum - FIXED: All instructions now have args and accounts fields
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitializeBuffer {
        args: HashMap<String, serde_json::Value>,
        accounts: InitializeBufferAccounts,
    },
    Write {
        args: HashMap<String, serde_json::Value>,
        accounts: WriteAccounts,
    },
    DeployWithMaxDataLen {
        args: HashMap<String, serde_json::Value>,
        accounts: DeployWithMaxDataLenAccounts,
    },
    Upgrade {
        args: HashMap<String, serde_json::Value>,
        accounts: UpgradeAccounts,
    },
    SetAuthority {
        args: HashMap<String, serde_json::Value>,
        accounts: SetAuthorityAccounts,
    },
    Close {
        args: HashMap<String, serde_json::Value>,
        accounts: CloseAccounts,
    },
    ExtendProgram {
        args: HashMap<String, serde_json::Value>,
        accounts: ExtendProgramAccounts,
    },
    SetAuthorityChecked {
        args: HashMap<String, serde_json::Value>,
        accounts: SetAuthorityCheckedAccounts,
    },
    UnknownInstruction {
        debug_info: DebugInfo,
    },
}

impl Instruction {
    // Helper function to create empty args HashMap
    fn empty_args() -> HashMap<String, serde_json::Value> {
        HashMap::new()
    }

    // Helper function to convert WriteArgs to HashMap
    fn write_args_to_map(args: &WriteArgs) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("offset".to_string(), serde_json::Value::Number(serde_json::Number::from(args.offset)));
        map.insert("bytes".to_string(), serde_json::to_value(&args.bytes).unwrap_or(serde_json::Value::Null));
        map
    }

    // Helper function to convert DeployWithMaxDataLenArgs to HashMap
    fn deploy_args_to_map(args: &DeployWithMaxDataLenArgs) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("max_data_len".to_string(), serde_json::Value::Number(serde_json::Number::from(args.max_data_len)));
        map
    }

    // Helper function to convert ExtendProgramArgs to HashMap
    fn extend_args_to_map(args: &ExtendProgramArgs) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("additional_bytes".to_string(), serde_json::Value::Number(serde_json::Number::from(args.additional_bytes)));
        map
    }

    // Helper function to convert SetAuthorityArgs to HashMap
    fn set_authority_args_to_map(authority_type: u32, new_authority: Option<String>) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("authority_type".to_string(), serde_json::Value::Number(serde_json::Number::from(authority_type)));
        if let Some(auth) = new_authority {
            map.insert("new_authority".to_string(), serde_json::Value::String(auth));
        }
        map
    }

    pub fn decode(account_keys: &[String], data: &[u8]) -> Result<Self> {
        if data.len() < 4 {
            return Err(anyhow!("instruction data too short"));
        }

        let (disc_bytes, rest) = data.split_at(4);
        let disc = u32::from_le_bytes(disc_bytes.try_into().unwrap());

        // Create debug info for troubleshooting
        let debug_info = DebugInfo {
            discriminator: disc,
            data_length: data.len(),
            raw_data_hex: hex::encode(data),
            error_message: String::new(),
        };

        match disc {
            INITIALIZE_BUFFER_DISCRIMINATOR => {
                let accounts = InitializeBufferAccounts {
                    buffer: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    authority: account_keys.get(1).map(|s| s.to_string()),
                };
                Ok(Instruction::InitializeBuffer { 
                    args: Self::empty_args(),
                    accounts: accounts,
                })
            }

            WRITE_DISCRIMINATOR => {
                // Write instruction has offset (4 bytes) + data length (4 bytes) + data
                if rest.len() < 8 {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "Write instruction expects at least 8 bytes but got {} bytes",
                        rest.len()
                    );
                    return Ok(Instruction::UnknownInstruction { debug_info: debug });
                }

                let offset = u32::from_le_bytes(rest[0..4].try_into().unwrap());
                let data_len = u32::from_le_bytes(rest[4..8].try_into().unwrap()) as usize;
                
                if rest.len() < 8 + data_len {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "Write instruction expects {} bytes for data but got {} bytes",
                        8 + data_len,
                        rest.len()
                    );
                    return Ok(Instruction::UnknownInstruction { debug_info: debug });
                }

                let bytes = rest[8..8 + data_len].to_vec();
                
                let args = WriteArgs { offset, bytes };
                let accounts = WriteAccounts {
                    buffer: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    authority: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                };
                
                Ok(Instruction::Write { 
                    args: Self::write_args_to_map(&args),
                    accounts: accounts,
                })
            }

            DEPLOY_WITH_MAX_DATA_LEN_DISCRIMINATOR => {
                // DeployWithMaxDataLen has 8 bytes for max_data_len
                if rest.len() < 8 {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "DeployWithMaxDataLen instruction expects 8 bytes but got {} bytes",
                        rest.len()
                    );
                    return Ok(Instruction::UnknownInstruction { debug_info: debug });
                }

                let max_data_len = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                let args = DeployWithMaxDataLenArgs { max_data_len };
                
                let accounts = DeployWithMaxDataLenAccounts {
                    payer: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    program_data: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    program: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    buffer: account_keys.get(3).unwrap_or(&"".to_string()).to_string(),
                    rent: account_keys.get(4).unwrap_or(&"".to_string()).to_string(),
                    clock: account_keys.get(5).unwrap_or(&"".to_string()).to_string(),
                    system_program: account_keys.get(6).unwrap_or(&"".to_string()).to_string(),
                    authority: account_keys.get(7).unwrap_or(&"".to_string()).to_string(),
                };
                
                Ok(Instruction::DeployWithMaxDataLen { 
                    args: Self::deploy_args_to_map(&args),
                    accounts: accounts,
                })
            }

            UPGRADE_DISCRIMINATOR => {
                let accounts = UpgradeAccounts {
                    program_data: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    program: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    buffer: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    spill: account_keys.get(3).unwrap_or(&"".to_string()).to_string(),
                    rent: account_keys.get(4).unwrap_or(&"".to_string()).to_string(),
                    clock: account_keys.get(5).unwrap_or(&"".to_string()).to_string(),
                    authority: account_keys.get(6).unwrap_or(&"".to_string()).to_string(),
                };
                Ok(Instruction::Upgrade { 
                    args: Self::empty_args(),
                    accounts: accounts,
                })
            }

            SET_AUTHORITY_DISCRIMINATOR => {
                // SetAuthority has 4 bytes for authority_type + optional 32 bytes for new_authority
                if rest.len() < 4 {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "SetAuthority instruction expects at least 4 bytes but got {} bytes",
                        rest.len()
                    );
                    return Ok(Instruction::UnknownInstruction { debug_info: debug });
                }

                let authority_type = u32::from_le_bytes(rest[0..4].try_into().unwrap());
                let new_authority = if rest.len() >= 36 {
                    // Has new authority (32 bytes)
                    Some(bs58::encode(&rest[4..36]).into_string())
                } else {
                    None
                };

                let accounts = SetAuthorityAccounts {
                    target: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    current_authority: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    new_authority: account_keys.get(2).map(|s| s.to_string()),
                };
                Ok(Instruction::SetAuthority { 
                    args: Self::set_authority_args_to_map(authority_type, new_authority),
                    accounts: accounts,
                })
            }

            CLOSE_DISCRIMINATOR => {
                let accounts = CloseAccounts {
                    account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    recipient: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    authority: account_keys.get(2).map(|s| s.to_string()),
                    program: account_keys.get(3).map(|s| s.to_string()),
                };
                Ok(Instruction::Close { 
                    args: Self::empty_args(),
                    accounts: accounts,
                })
            }

            EXTEND_PROGRAM_DISCRIMINATOR => {
                // ExtendProgram has 4 bytes for additional_bytes
                if rest.len() < 4 {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "ExtendProgram instruction expects 4 bytes but got {} bytes",
                        rest.len()
                    );
                    return Ok(Instruction::UnknownInstruction { debug_info: debug });
                }

                let additional_bytes = u32::from_le_bytes(rest[0..4].try_into().unwrap());
                let args = ExtendProgramArgs { additional_bytes };
                
                let accounts = ExtendProgramAccounts {
                    program_data: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    program: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    system_program: account_keys.get(2).map(|s| s.to_string()),
                    payer: account_keys.get(3).map(|s| s.to_string()),
                };
                
                Ok(Instruction::ExtendProgram { 
                    args: Self::extend_args_to_map(&args),
                    accounts: accounts,
                })
            }

            SET_AUTHORITY_CHECKED_DISCRIMINATOR => {
                // SetAuthorityChecked has 4 bytes for authority_type
                if rest.len() < 4 {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "SetAuthorityChecked instruction expects 4 bytes but got {} bytes",
                        rest.len()
                    );
                    return Ok(Instruction::UnknownInstruction { debug_info: debug });
                }

                let authority_type = u32::from_le_bytes(rest[0..4].try_into().unwrap());

                let accounts = SetAuthorityCheckedAccounts {
                    target: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    current_authority: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    new_authority: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                };
                Ok(Instruction::SetAuthorityChecked { 
                    args: Self::set_authority_args_to_map(authority_type, None),
                    accounts: accounts,
                })
            }

            _ => {
                let mut debug = debug_info;
                debug.error_message = format!("Unknown discriminator: {}. Not a valid BPF Loader instruction.", disc);
                Ok(Instruction::UnknownInstruction { debug_info: debug })
            }
        }
    }
}
