use serde::Serializer;
#[allow(unused_imports, dead_code, unused_variables)]
use std::convert::TryInto;
fn serialize_to_string<S, T>(x: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    s.serialize_str(&x.to_string())
}
pub use accounts_data::*;
pub use ix_data::*;
pub use typedefs::*;
pub mod typedefs {
    #![allow(unused_imports)]
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use anchor_lang::prelude::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde::Serialize;
    serde_big_array::big_array! { BigArray ; 76 }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ClaimStatus {
        #[serde(with = "pubkey_serde")]
        pub claimant: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub locked_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub locked_amount_withdrawn: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub unlocked_amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MerkleDistributor {
        pub bump: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub version: u64,
        pub root: [u8; 32],
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub token_vault: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_total_claim: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_num_nodes: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_amount_claimed: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub num_nodes_claimed: u64,
        pub start_ts: i64,
        pub end_ts: i64,
        pub clawback_start_ts: i64,
        #[serde(with = "pubkey_serde")]
        pub clawback_receiver: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub admin: Pubkey,
        pub clawed_back: bool,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct NewClaimEvent {
        #[serde(with = "pubkey_serde")]
        pub claimant: Pubkey,
        pub timestamp: i64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ClaimedEvent {
        #[serde(with = "pubkey_serde")]
        pub claimant: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct NewDistributorAccounts {
        pub distributor: String,
        pub clawback_receiver: String,
        pub mint: String,
        pub token_vault: String,
        pub admin: String,
        pub system_program: String,
        pub associated_token_program: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct NewClaimAccounts {
        pub distributor: String,
        pub claim_status: String,
        pub from: String,
        pub to: String,
        pub claimant: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimLockedAccounts {
        pub distributor: String,
        pub claim_status: String,
        pub from: String,
        pub to: String,
        pub claimant: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClawbackAccounts {
        pub distributor: String,
        pub from: String,
        pub to: String,
        pub claimant: String,
        pub system_program: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetClawbackReceiverAccounts {
        pub distributor: String,
        pub new_clawback_account: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetAdminAccounts {
        pub distributor: String,
        pub admin: String,
        pub new_admin: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    #![allow(unused_imports)]
    use super::*;
    use crate::pubkey_serializer::pubkey_serde_option;
    use crate::pubkey_serializer::pubkey_serde_u32;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct NewDistributorIxData {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub version: u64,
        pub root: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_total_claim: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_num_nodes: u64,
        pub start_vesting_ts: i64,
        pub end_vesting_ts: i64,
        pub clawback_start_ts: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct NewClaimIxData {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_unlocked: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_locked: u64,
        pub proof: Vec<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimLockedIxData {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClawbackIxData {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetClawbackReceiverIxData {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetAdminIxData {}
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    NewDistributor {
        accounts: NewDistributorAccounts,
        args: NewDistributorIxData,
    },
    NewClaim {
        accounts: NewClaimAccounts,
        args: NewClaimIxData,
    },
    ClaimLocked {
        accounts: ClaimLockedAccounts,
        args: ClaimLockedIxData,
    },
    Clawback {
        accounts: ClawbackAccounts,
        args: ClawbackIxData,
    },
    SetClawbackReceiver {
        accounts: SetClawbackReceiverAccounts,
        args: SetClawbackReceiverIxData,
    },
    SetAdmin {
        accounts: SetAdminAccounts,
        args: SetAdminIxData,
    },
}
impl Instruction {
    pub fn decode(account_keys: &[String], data: &[u8]) -> anyhow::Result<Self> {
        if data.len() < 8 {
            anyhow::bail!("Data too short: {}", data.len());
        }
        let (disc_slice, rest) = data.split_at(8);
        let disc: [u8; 8] = disc_slice.try_into().unwrap();
        match disc {
            [32u8, 139u8, 112u8, 171u8, 0u8, 2u8, 225u8, 155u8] => {
                let mut rdr: &[u8] = rest;
                let args = NewDistributorIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let distributor = keys.next().unwrap().clone();
                let clawback_receiver = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token_vault = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = NewDistributorAccounts {
                    distributor,
                    clawback_receiver,
                    mint,
                    token_vault,
                    admin,
                    system_program,
                    associated_token_program,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::NewDistributor { accounts, args });
            }
            [78u8, 177u8, 98u8, 123u8, 210u8, 21u8, 187u8, 83u8] => {
                let mut rdr: &[u8] = rest;
                let args = NewClaimIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let distributor = keys.next().unwrap().clone();
                let claim_status = keys.next().unwrap().clone();
                let from = keys.next().unwrap().clone();
                let to = keys.next().unwrap().clone();
                let claimant = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = NewClaimAccounts {
                    distributor,
                    claim_status,
                    from,
                    to,
                    claimant,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::NewClaim { accounts, args });
            }
            [34u8, 206u8, 181u8, 23u8, 11u8, 207u8, 147u8, 90u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimLockedIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let distributor = keys.next().unwrap().clone();
                let claim_status = keys.next().unwrap().clone();
                let from = keys.next().unwrap().clone();
                let to = keys.next().unwrap().clone();
                let claimant = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimLockedAccounts {
                    distributor,
                    claim_status,
                    from,
                    to,
                    claimant,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::ClaimLocked { accounts, args });
            }
            [111u8, 92u8, 142u8, 79u8, 33u8, 234u8, 82u8, 27u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClawbackIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let distributor = keys.next().unwrap().clone();
                let from = keys.next().unwrap().clone();
                let to = keys.next().unwrap().clone();
                let claimant = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClawbackAccounts {
                    distributor,
                    from,
                    to,
                    claimant,
                    system_program,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::Clawback { accounts, args });
            }
            [153u8, 217u8, 34u8, 20u8, 19u8, 29u8, 229u8, 75u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetClawbackReceiverIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let distributor = keys.next().unwrap().clone();
                let new_clawback_account = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetClawbackReceiverAccounts {
                    distributor,
                    new_clawback_account,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetClawbackReceiver { accounts, args });
            }
            [251u8, 163u8, 0u8, 52u8, 91u8, 194u8, 187u8, 92u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetAdminIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let distributor = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let new_admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetAdminAccounts {
                    distributor,
                    admin,
                    new_admin,
                    remaining,
                };
                return Ok(Instruction::SetAdmin { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    use borsh::BorshDeserialize;
    use serde::Serialize;
    pub use typedefs::*;
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        NewClaimEvent { args: NewClaimEvent },
        ClaimedEvent { args: ClaimedEvent },
    }
    pub const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
    impl Event {
        pub fn decode(data: &[u8]) -> anyhow::Result<Self> {
            if data.len() < 16 {
                anyhow::bail!("Event log too short");
            }
            let wrapper: [u8; 8] = data[0..8].try_into().unwrap();
            if wrapper != EVENT_LOG_DISCRIMINATOR {
                anyhow::bail!("Invalid event wrapper");
            }
            let disc: [u8; 8] = data[8..16].try_into().unwrap();
            let _payload = &data[16..];
            match disc {
                [224u8, 129u8, 92u8, 29u8, 61u8, 179u8, 171u8, 201u8] => {
                    let mut rdr = &data[16..];
                    let args = NewClaimEvent::deserialize(&mut rdr)?;
                    return Ok(Event::NewClaimEvent { args });
                }
                [198u8, 196u8, 91u8, 135u8, 134u8, 156u8, 125u8, 60u8] => {
                    let mut rdr = &data[16..];
                    let args = ClaimedEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ClaimedEvent { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
