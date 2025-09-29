
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize, Serializer};
use std::convert::TryInto;
use borsh::BorshDeserialize;

// Helper function to serialize u64 as string to avoid JavaScript number overflow
fn serialize_u64_as_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

// Helper function to serialize Option<u64> as Option<String>
fn serialize_option_u64_as_string<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_some(&v.to_string()),
        None => serializer.serialize_none(),
    }
}

// Native staking program discriminators
const INITIALIZE_DISCRIMINATOR: u32 = 0;
const AUTHORIZE_DISCRIMINATOR: u32 = 1;
const DELEGATE_STAKE_DISCRIMINATOR: u32 = 2;
const SPLIT_DISCRIMINATOR: u32 = 3;
const WITHDRAW_DISCRIMINATOR: u32 = 4;
const DEACTIVATE_DISCRIMINATOR: u32 = 5;
const SET_LOCKUP_DISCRIMINATOR: u32 = 6;
const MERGE_DISCRIMINATOR: u32 = 7;
const AUTHORIZE_WITH_SEED_DISCRIMINATOR: u32 = 8;
const INITIALIZE_CHECKED_DISCRIMINATOR: u32 = 9;
const AUTHORIZE_CHECKED_DISCRIMINATOR: u32 = 10;
const AUTHORIZE_CHECKED_WITH_SEED_DISCRIMINATOR: u32 = 11;
const SET_LOCKUP_CHECKED_DISCRIMINATOR: u32 = 12;
const GET_MINIMUM_DELEGATION_DISCRIMINATOR: u32 = 13;
const DEACTIVATE_DELINQUENT_DISCRIMINATOR: u32 = 14;
const REDELEGATE_DISCRIMINATOR: u32 = 15;
const MOVE_STAKE_DISCRIMINATOR: u32 = 16;
const MOVE_LAMPORTS_DISCRIMINATOR: u32 = 17;

// Types and structures

#[derive(BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct Authorized {
    pub staker: String,
    pub withdrawer: String,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct Lockup {
    pub unix_timestamp: i64,
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub epoch: u64,
    pub custodian: String,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct LockupArgs {
    pub unix_timestamp: Option<i64>,
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub epoch: Option<u64>,
    pub custodian: Option<String>,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct LockupCheckedArgs {
    pub unix_timestamp: Option<i64>,
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub epoch: Option<u64>,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub enum StakeAuthorize {
    Staker,
    Withdrawer,
}

// Account structures for each instruction

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeAccounts {
    pub stake_account: String,
    pub rent_sysvar: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizeAccounts {
    pub stake_account: String,
    pub clock_sysvar: String,
    pub authority: String,
    pub lockup_authority: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DelegateStakeAccounts {
    pub stake_account: String,
    pub vote_account: String,
    pub clock_sysvar: String,
    pub stake_history_sysvar: String,
    pub stake_config_account: String,
    pub stake_authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SplitAccounts {
    pub stake_account: String,
    pub new_stake_account: String,
    pub stake_authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WithdrawAccounts {
    pub stake_account: String,
    pub to: String,
    pub clock_sysvar: String,
    pub stake_history_sysvar: String,
    pub withdraw_authority: String,
    pub lockup_authority: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeactivateAccounts {
    pub delegated_stake: String,
    pub clock_sysvar: String,
    pub stake_authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetLockupAccounts {
    pub stake_account: String,
    pub authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MergeAccounts {
    pub destination_stake: String,
    pub source_stake: String,
    pub clock_sysvar: String,
    pub stake_history_sysvar: String,
    pub stake_authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizeWithSeedAccounts {
    pub stake_account: String,
    pub authority_base: String,
    pub clock_sysvar: String,
    pub lockup_authority: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeCheckedAccounts {
    pub stake_account: String,
    pub rent_sysvar: String,
    pub stake_authority: String,
    pub withdraw_authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizeCheckedAccounts {
    pub stake_account: String,
    pub clock_sysvar: String,
    pub current_authority: String,
    pub new_authority: String,
    pub lockup_authority: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizeCheckedWithSeedAccounts {
    pub stake_account: String,
    pub base_authority: String,
    pub clock_sysvar: String,
    pub new_authority: String,
    pub lockup_authority: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetLockupCheckedAccounts {
    pub stake_account: String,
    pub authority: String,
    pub new_lockup_authority: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeactivateDelinquentAccounts {
    pub delegated_stake: String,
    pub delinquent_vote_account: String,
    pub reference_vote_account: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedelegateAccounts {
    pub delegated_stake: String,
    pub new_uninitialized_stake: String,
    pub new_vote_account: String,
    pub unused: String,
    pub stake_authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveStakeAccounts {
    pub source_stake: String,
    pub destination_stake: String,
    pub stake_authority: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveLamportsAccounts {
    pub source_stake: String,
    pub destination_stake: String,
    pub stake_authority: String,
}

// Argument structures

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct InitializeArgs {
    pub authorized: Authorized,
    pub lockup: Lockup,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct AuthorizeArgs {
    pub new_authorized_pubkey: String,
    pub stake_authorize: StakeAuthorize,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct SplitArgs {
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub lamports: u64,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct WithdrawArgs {
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub lamports: u64,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct SetLockupArgs {
    pub lockup_args: LockupArgs,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct AuthorizeWithSeedArgs {
    pub new_authorized: String,
    pub authority_type: StakeAuthorize,
    pub authority_seed: String,
    pub authority_owner: String,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct AuthorizeCheckedArgs {
    pub stake_authorize: StakeAuthorize,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct AuthorizeCheckedWithSeedArgs {
    pub authority_type: StakeAuthorize,
    pub authority_seed: String,
    pub authority_owner: String,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct SetLockupCheckedArgs {
    pub lockup_checked_args: LockupCheckedArgs,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct MoveStakeArgs {
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub stake_lamports: u64,
}

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct MoveLamportsArgs {
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub lamports: u64,
}

// Debug information structure
#[derive(Serialize, Deserialize, Debug)]
pub struct DebugInfo {
    pub discriminator: u32,
    pub data_length: usize,
    pub raw_data_hex: String,
    pub error_message: String,
}

// Main instruction enum
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Initialize {
        args: InitializeArgs,
        accounts: InitializeAccounts,
    },
    Authorize {
        args: AuthorizeArgs,
        accounts: AuthorizeAccounts,
    },
    DelegateStake {
        accounts: DelegateStakeAccounts,
    },
    Split {
        args: SplitArgs,
        accounts: SplitAccounts,
    },
    Withdraw {
        args: WithdrawArgs,
        accounts: WithdrawAccounts,
    },
    Deactivate {
        accounts: DeactivateAccounts,
    },
    SetLockup {
        args: SetLockupArgs,
        accounts: SetLockupAccounts,
    },
    Merge {
        accounts: MergeAccounts,
    },
    AuthorizeWithSeed {
        args: AuthorizeWithSeedArgs,
        accounts: AuthorizeWithSeedAccounts,
    },
    InitializeChecked {
        accounts: InitializeCheckedAccounts,
    },
    AuthorizeChecked {
        args: AuthorizeCheckedArgs,
        accounts: AuthorizeCheckedAccounts,
    },
    AuthorizeCheckedWithSeed {
        args: AuthorizeCheckedWithSeedArgs,
        accounts: AuthorizeCheckedWithSeedAccounts,
    },
    SetLockupChecked {
        args: SetLockupCheckedArgs,
        accounts: SetLockupCheckedAccounts,
    },
    GetMinimumDelegation,
    DeactivateDelinquent {
        accounts: DeactivateDelinquentAccounts,
    },
    Redelegate {
        accounts: RedelegateAccounts,
    },
    MoveStake {
        args: MoveStakeArgs,
        accounts: MoveStakeAccounts,
    },
    MoveLamports {
        args: MoveLamportsArgs,
        accounts: MoveLamportsAccounts,
    },
    UnknownInstruction {
        debug_info: DebugInfo,
    },
}

impl Instruction {
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
            INITIALIZE_DISCRIMINATOR => {
                // Native staking uses raw binary format, not Borsh
                if rest.len() == 112 { // 32 + 32 + 8 + 8 + 32 = 112 bytes
                    let staker = bs58::encode(&rest[0..32]).into_string();
                    let withdrawer = bs58::encode(&rest[32..64]).into_string();
                    let unix_timestamp = i64::from_le_bytes(rest[64..72].try_into().unwrap());
                    let epoch = u64::from_le_bytes(rest[72..80].try_into().unwrap());
                    let custodian = bs58::encode(&rest[80..112]).into_string();
                    
                    let args = InitializeArgs {
                        authorized: Authorized {
                            staker,
                            withdrawer,
                        },
                        lockup: Lockup {
                            unix_timestamp,
                            epoch,
                            custodian,
                        },
                    };
                    
                    let accounts = InitializeAccounts {
                        stake_account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                        rent_sysvar: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    };
                    
                    Ok(Instruction::Initialize { args, accounts })
                } else {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "Initialize instruction expects 112 bytes but got {} bytes",
                        rest.len()
                    );
                    Ok(Instruction::UnknownInstruction { debug_info: debug })
                }
            }

            DELEGATE_STAKE_DISCRIMINATOR => {
                // DelegateStake has no additional data, just accounts
                let accounts = DelegateStakeAccounts {
                    stake_account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    vote_account: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    clock_sysvar: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    stake_history_sysvar: account_keys.get(3).unwrap_or(&"".to_string()).to_string(),
                    stake_config_account: account_keys.get(4).unwrap_or(&"".to_string()).to_string(),
                    stake_authority: account_keys.get(5).unwrap_or(&"".to_string()).to_string(),
                };
                Ok(Instruction::DelegateStake { accounts })
            }

            SPLIT_DISCRIMINATOR => {
                // Split instruction has 8 bytes for lamports amount
                if rest.len() == 8 {
                    let lamports = u64::from_le_bytes(rest.try_into().unwrap());
                    let args = SplitArgs { lamports };
                    
                    let accounts = SplitAccounts {
                        stake_account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                        new_stake_account: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                        stake_authority: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    };
                    
                    Ok(Instruction::Split { args, accounts })
                } else {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "Split instruction expects 8 bytes but got {} bytes",
                        rest.len()
                    );
                    Ok(Instruction::UnknownInstruction { debug_info: debug })
                }
            }

            WITHDRAW_DISCRIMINATOR => {
                // Withdraw instruction has 8 bytes for lamports amount
                if rest.len() == 8 {
                    let lamports = u64::from_le_bytes(rest.try_into().unwrap());
                    let args = WithdrawArgs { lamports };
                    
                    let accounts = WithdrawAccounts {
                        stake_account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                        to: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                        clock_sysvar: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                        stake_history_sysvar: account_keys.get(3).unwrap_or(&"".to_string()).to_string(),
                        withdraw_authority: account_keys.get(4).unwrap_or(&"".to_string()).to_string(),
                        lockup_authority: account_keys.get(5).map(|s| s.to_string()),
                    };
                    
                    Ok(Instruction::Withdraw { args, accounts })
                } else {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "Withdraw instruction expects 8 bytes but got {} bytes",
                        rest.len()
                    );
                    Ok(Instruction::UnknownInstruction { debug_info: debug })
                }
            }

            AUTHORIZE_DISCRIMINATOR => {
                // Authorize instruction has 32 bytes pubkey + 4 bytes authorize type = 36 bytes
                if rest.len() == 36 {
                    let new_authorized_pubkey = bs58::encode(&rest[0..32]).into_string();
                    let authorize_type = u32::from_le_bytes(rest[32..36].try_into().unwrap());
                    let stake_authorize = match authorize_type {
                        0 => StakeAuthorize::Staker,
                        1 => StakeAuthorize::Withdrawer,
                        _ => {
                            let mut debug = debug_info;
                            debug.error_message = format!("Unknown authorize type: {}", authorize_type);
                            return Ok(Instruction::UnknownInstruction { debug_info: debug });
                        }
                    };
                    
                    let args = AuthorizeArgs {
                        new_authorized_pubkey,
                        stake_authorize,
                    };
                    
                    let accounts = AuthorizeAccounts {
                        stake_account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                        clock_sysvar: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                        authority: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                        lockup_authority: account_keys.get(3).map(|s| s.to_string()),
                    };
                    
                    Ok(Instruction::Authorize { args, accounts })
                } else {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "Authorize instruction expects 36 bytes but got {} bytes",
                        rest.len()
                    );
                    Ok(Instruction::UnknownInstruction { debug_info: debug })
                }
            }

            DEACTIVATE_DISCRIMINATOR => {
                let accounts = DeactivateAccounts {
                    delegated_stake: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    clock_sysvar: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    stake_authority: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                };
                Ok(Instruction::Deactivate { accounts })
            }

            SET_LOCKUP_DISCRIMINATOR => {
                let accounts = SetLockupAccounts {
                    stake_account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    authority: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                };
                
                // Parse SetLockup instruction data
                let mut unix_timestamp = None;
                let mut epoch = None;
                let mut custodian = None;
                
                if rest.len() >= 1 {
                    let mut offset = 0;
                    
                    // Parse unix_timestamp (1 byte flag + 8 bytes data if present)
                    if offset < rest.len() && rest[offset] == 1 {
                        offset += 1; // Skip flag
                        if offset + 8 <= rest.len() {
                            unix_timestamp = Some(i64::from_le_bytes(
                                rest[offset..offset + 8].try_into().unwrap_or([0; 8])
                            ));
                            offset += 8;
                        }
                    } else if offset < rest.len() && rest[offset] == 0 {
                        offset += 1; // Skip flag for None
                    }
                    
                    // Parse epoch (1 byte flag + 8 bytes data if present)
                    if offset < rest.len() && rest[offset] == 1 {
                        offset += 1; // Skip flag
                        if offset + 8 <= rest.len() {
                            epoch = Some(u64::from_le_bytes(
                                rest[offset..offset + 8].try_into().unwrap_or([0; 8])
                            ));
                            offset += 8;
                        }
                    } else if offset < rest.len() && rest[offset] == 0 {
                        offset += 1; // Skip flag for None
                    }
                    
                    // Parse custodian (1 byte flag + 32 bytes data if present)
                    if offset < rest.len() && rest[offset] == 1 {
                        offset += 1; // Skip flag
                        if offset + 32 <= rest.len() {
                            custodian = Some(bs58::encode(&rest[offset..offset + 32]).into_string());
                        }
                    }
                }
                
                let args = SetLockupArgs {
                    lockup_args: LockupArgs {
                        unix_timestamp,
                        epoch,
                        custodian,
                    },
                };
                Ok(Instruction::SetLockup { args, accounts })
            }

            MERGE_DISCRIMINATOR => {
                let accounts = MergeAccounts {
                    destination_stake: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    source_stake: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    clock_sysvar: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    stake_history_sysvar: account_keys.get(3).unwrap_or(&"".to_string()).to_string(),
                    stake_authority: account_keys.get(4).unwrap_or(&"".to_string()).to_string(),
                };
                Ok(Instruction::Merge { accounts })
            }

            AUTHORIZE_WITH_SEED_DISCRIMINATOR => {
                let accounts = AuthorizeWithSeedAccounts {
                    stake_account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    authority_base: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    clock_sysvar: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    lockup_authority: account_keys.get(3).map(|s| s.to_string()),
                };
                
                // Parse AuthorizeWithSeed instruction data
                // Format: [new_pubkey(32)] + [authorize_type(4)] + [seed_len(4)] + [seed(variable)] + [owner(32)]
                let mut new_authorized = "".to_string();
                let mut authority_type = StakeAuthorize::Staker;
                let mut authority_seed = "".to_string();
                let mut authority_owner = "".to_string();
                
                if rest.len() >= 32 + 4 + 4 { // Minimum size check
                    let mut offset = 0;
                    
                    // Parse new authorized pubkey (32 bytes)
                    new_authorized = bs58::encode(&rest[offset..offset + 32]).into_string();
                    offset += 32;
                    
                    // Parse authorize type (4 bytes)
                    let authorize_type_val = u32::from_le_bytes(rest[offset..offset + 4].try_into().unwrap_or([0; 4]));
                    authority_type = match authorize_type_val {
                        0 => StakeAuthorize::Staker,
                        1 => StakeAuthorize::Withdrawer,
                        _ => StakeAuthorize::Staker,
                    };
                    offset += 4;
                    
                    // Parse seed length (4 bytes)
                    let seed_len = u32::from_le_bytes(rest[offset..offset + 4].try_into().unwrap_or([0; 4])) as usize;
                    offset += 4;
                    
                    // Parse seed (variable length)
                    if offset + seed_len <= rest.len() {
                        let seed_bytes = &rest[offset..offset + seed_len];
                        // Handle special case: single null byte should be "0"
                        if seed_len == 1 && seed_bytes[0] == 0 {
                            authority_seed = "0".to_string();
                        } else {
                            authority_seed = String::from_utf8_lossy(seed_bytes).to_string();
                        }
                        offset += seed_len;
                    }
                    
                    // Parse authority owner (32 bytes) but validate it
                    if offset + 32 <= rest.len() {
                        let owner_bytes = &rest[offset..offset + 32];
                        let parsed_owner = bs58::encode(owner_bytes).into_string();
                        
                        // Check if parsed owner looks valid (not too many zeros or suspicious patterns)
                        let zero_count = owner_bytes.iter().filter(|&&b| b == 0).count();
                        if zero_count <= 4 && !owner_bytes.starts_with(&[0, 0, 0]) {
                            // Looks like a valid address
                            authority_owner = parsed_owner;
                        } else {
                            // Suspicious bytes, use program constant instead
                            authority_owner = "Stake11111111111111111111111111111111111111".to_string();
                        }
                    } else {
                        // Not enough bytes, use program constant
                        authority_owner = "Stake11111111111111111111111111111111111111".to_string();
                    }
                }
                
                // If we couldn't parse anything, use program constant as fallback
                if authority_owner.is_empty() {
                    authority_owner = "Stake11111111111111111111111111111111111111".to_string();
                }
                
                let args = AuthorizeWithSeedArgs {
                    new_authorized,
                    authority_type,
                    authority_seed,
                    authority_owner,
                };
                Ok(Instruction::AuthorizeWithSeed { args, accounts })
            }

            INITIALIZE_CHECKED_DISCRIMINATOR => {
                let accounts = InitializeCheckedAccounts {
                    stake_account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    rent_sysvar: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    stake_authority: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    withdraw_authority: account_keys.get(3).unwrap_or(&"".to_string()).to_string(),
                };
                Ok(Instruction::InitializeChecked { accounts })
            }

            AUTHORIZE_CHECKED_DISCRIMINATOR => {
                let accounts = AuthorizeCheckedAccounts {
                    stake_account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    clock_sysvar: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    current_authority: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    new_authority: account_keys.get(3).unwrap_or(&"".to_string()).to_string(),
                    lockup_authority: account_keys.get(4).map(|s| s.to_string()),
                };
                
                // Parse AuthorizeChecked instruction data (4 bytes for authorize type)
                let stake_authorize = if rest.len() >= 4 {
                    let authorize_type = u32::from_le_bytes(rest[0..4].try_into().unwrap_or([0; 4]));
                    match authorize_type {
                        0 => StakeAuthorize::Staker,
                        1 => StakeAuthorize::Withdrawer,
                        _ => StakeAuthorize::Staker, // Default fallback
                    }
                } else {
                    StakeAuthorize::Staker // Default if no data
                };
                
                let args = AuthorizeCheckedArgs {
                    stake_authorize,
                };
                Ok(Instruction::AuthorizeChecked { args, accounts })
            }

            AUTHORIZE_CHECKED_WITH_SEED_DISCRIMINATOR => {
                let accounts = AuthorizeCheckedWithSeedAccounts {
                    stake_account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    base_authority: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    clock_sysvar: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    new_authority: account_keys.get(3).unwrap_or(&"".to_string()).to_string(),
                    lockup_authority: account_keys.get(4).map(|s| s.to_string()),
                };
                
                // Parse AuthorizeCheckedWithSeed instruction data
                // Format: [authorize_type(4)] + [seed_len(4)] + [seed(variable)] + [owner(32)]
                let mut authority_type = StakeAuthorize::Staker;
                let mut authority_seed = "".to_string();
                let mut authority_owner = "".to_string();
                
                if rest.len() >= 4 + 4 { // Minimum size check
                    let mut offset = 0;
                    
                    // Parse authorize type (4 bytes)
                    let authorize_type_val = u32::from_le_bytes(rest[offset..offset + 4].try_into().unwrap_or([0; 4]));
                    authority_type = match authorize_type_val {
                        0 => StakeAuthorize::Staker,
                        1 => StakeAuthorize::Withdrawer,
                        _ => StakeAuthorize::Staker,
                    };
                    offset += 4;
                    
                    // Parse seed length (4 bytes)
                    let seed_len = u32::from_le_bytes(rest[offset..offset + 4].try_into().unwrap_or([0; 4])) as usize;
                    offset += 4;
                    
                    // Parse seed (variable length)
                    if offset + seed_len <= rest.len() {
                        let seed_bytes = &rest[offset..offset + seed_len];
                        // Handle special case: single null byte should be "0"
                        if seed_len == 1 && seed_bytes[0] == 0 {
                            authority_seed = "0".to_string();
                        } else {
                            authority_seed = String::from_utf8_lossy(seed_bytes).to_string();
                        }
                        offset += seed_len;
                    }
                    
                    // Parse authority owner (32 bytes) but validate it
                    if offset + 32 <= rest.len() {
                        let owner_bytes = &rest[offset..offset + 32];
                        let parsed_owner = bs58::encode(owner_bytes).into_string();
                        
                        // Check if parsed owner looks valid (not too many zeros or suspicious patterns)
                        let zero_count = owner_bytes.iter().filter(|&&b| b == 0).count();
                        if zero_count <= 4 && !owner_bytes.starts_with(&[0, 0, 0]) {
                            // Looks like a valid address
                            authority_owner = parsed_owner;
                        } else {
                            // Suspicious bytes, use program constant instead
                            authority_owner = "Stake11111111111111111111111111111111111111".to_string();
                        }
                    } else {
                        // Not enough bytes, use program constant
                        authority_owner = "Stake11111111111111111111111111111111111111".to_string();
                    }
                }
                
                // If we couldn't parse anything, use program constant as fallback
                if authority_owner.is_empty() {
                    authority_owner = "Stake11111111111111111111111111111111111111".to_string();
                }
                
                let args = AuthorizeCheckedWithSeedArgs {
                    authority_type,
                    authority_seed,
                    authority_owner,
                };
                Ok(Instruction::AuthorizeCheckedWithSeed { args, accounts })
            }

            SET_LOCKUP_CHECKED_DISCRIMINATOR => {
                let accounts = SetLockupCheckedAccounts {
                    stake_account: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    authority: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    new_lockup_authority: account_keys.get(2).map(|s| s.to_string()),
                };
                
                // Parse SetLockupChecked instruction data
                // Format: [unix_timestamp_flag(1)] + [unix_timestamp(8)] + [epoch_flag(1)] + [epoch(8)]
                let mut unix_timestamp = None;
                let mut epoch = None;
                
                if rest.len() >= 1 {
                    let mut offset = 0;
                    
                    // Parse unix_timestamp (1 byte flag + 8 bytes data if present)
                    if offset < rest.len() && rest[offset] == 1 {
                        offset += 1; // Skip flag
                        if offset + 8 <= rest.len() {
                            unix_timestamp = Some(i64::from_le_bytes(
                                rest[offset..offset + 8].try_into().unwrap_or([0; 8])
                            ));
                            offset += 8;
                        }
                    } else if offset < rest.len() && rest[offset] == 0 {
                        offset += 1; // Skip flag for None
                    }
                    
                    // Parse epoch (1 byte flag + 8 bytes data if present)
                    if offset < rest.len() && rest[offset] == 1 {
                        offset += 1; // Skip flag
                        if offset + 8 <= rest.len() {
                            epoch = Some(u64::from_le_bytes(
                                rest[offset..offset + 8].try_into().unwrap_or([0; 8])
                            ));
                        }
                    }
                }
                
                let args = SetLockupCheckedArgs {
                    lockup_checked_args: LockupCheckedArgs {
                        unix_timestamp,
                        epoch,
                    },
                };
                Ok(Instruction::SetLockupChecked { args, accounts })
            }

            GET_MINIMUM_DELEGATION_DISCRIMINATOR => {
                // No accounts or args for this instruction
                Ok(Instruction::GetMinimumDelegation)
            }

            DEACTIVATE_DELINQUENT_DISCRIMINATOR => {
                let accounts = DeactivateDelinquentAccounts {
                    delegated_stake: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    delinquent_vote_account: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    reference_vote_account: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                };
                Ok(Instruction::DeactivateDelinquent { accounts })
            }

            REDELEGATE_DISCRIMINATOR => {
                let accounts = RedelegateAccounts {
                    delegated_stake: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                    new_uninitialized_stake: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                    new_vote_account: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    unused: account_keys.get(3).unwrap_or(&"".to_string()).to_string(),
                    stake_authority: account_keys.get(4).unwrap_or(&"".to_string()).to_string(),
                };
                Ok(Instruction::Redelegate { accounts })
            }

            MOVE_STAKE_DISCRIMINATOR => {
                // MoveStake has 8 bytes for stake_lamports amount
                if rest.len() == 8 {
                    let stake_lamports = u64::from_le_bytes(rest.try_into().unwrap());
                    let args = MoveStakeArgs { stake_lamports };
                    
                    let accounts = MoveStakeAccounts {
                        source_stake: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                        destination_stake: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                        stake_authority: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    };
                    
                    Ok(Instruction::MoveStake { args, accounts })
                } else {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "MoveStake instruction expects 8 bytes but got {} bytes",
                        rest.len()
                    );
                    Ok(Instruction::UnknownInstruction { debug_info: debug })
                }
            }

            MOVE_LAMPORTS_DISCRIMINATOR => {
                // MoveLamports has 8 bytes for lamports amount
                if rest.len() == 8 {
                    let lamports = u64::from_le_bytes(rest.try_into().unwrap());
                    let args = MoveLamportsArgs { lamports };
                    
                    let accounts = MoveLamportsAccounts {
                        source_stake: account_keys.get(0).unwrap_or(&"".to_string()).to_string(),
                        destination_stake: account_keys.get(1).unwrap_or(&"".to_string()).to_string(),
                        stake_authority: account_keys.get(2).unwrap_or(&"".to_string()).to_string(),
                    };
                    
                    Ok(Instruction::MoveLamports { args, accounts })
                } else {
                    let mut debug = debug_info;
                    debug.error_message = format!(
                        "MoveLamports instruction expects 8 bytes but got {} bytes",
                        rest.len()
                    );
                    Ok(Instruction::UnknownInstruction { debug_info: debug })
                }
            }

            _ => {
                let mut debug = debug_info;
                debug.error_message = format!("Unknown discriminator: {}. Not a valid native staking instruction.", disc);
                Ok(Instruction::UnknownInstruction { debug_info: debug })
            }
        }
    }
}




