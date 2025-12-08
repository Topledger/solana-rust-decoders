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
    pub struct InflationConfig {
        pub bump: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub initial_supply: u64,
        pub initial_rate_bps: u32,
        pub terminal_rate_bps: u32,
        pub decay_rate_bps: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub seconds_per_period: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub inflation_start_timestamp: u64,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub destination: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub authority: Pubkey,
        pub paused: bool,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub pause_start_timestamp: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub skipped_seconds: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_minted_period: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_minted: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InflationConfigInitialized {
        #[serde(with = "pubkey_serde")]
        pub inflation_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub destination: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub initial_supply: u64,
        pub initial_rate_bps: u32,
        pub terminal_rate_bps: u32,
        pub decay_rate_bps: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub seconds_per_period: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub inflation_start_timestamp: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InflationConfigUpdated {
        #[serde(with = "pubkey_serde")]
        pub inflation_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub old_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub new_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub old_destination: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub new_destination: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub updated_by: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InflationParametersUpdated {
        #[serde(with = "pubkey_serde")]
        pub inflation_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub updated_by: Pubkey,
        pub old_initial_rate_bps: u32,
        pub new_initial_rate_bps: u32,
        pub old_terminal_rate_bps: u32,
        pub new_terminal_rate_bps: u32,
        pub old_decay_rate_bps: u32,
        pub new_decay_rate_bps: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub old_seconds_per_period: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub new_seconds_per_period: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InflationPaused {
        #[serde(with = "pubkey_serde")]
        pub inflation_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub paused_by: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub timestamp: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InflationResumed {
        #[serde(with = "pubkey_serde")]
        pub inflation_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub resumed_by: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub timestamp: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub elapsed_seconds: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_skipped_seconds: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InflationTriggered {
        #[serde(with = "pubkey_serde")]
        pub inflation_config: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub period: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub tokens_minted: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub current_rate_bps: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub rate_per_period_bps: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_minted: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub timestamp: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InitializeInflationConfigArgs {
        pub initial_rate_bps: u32,
        pub terminal_rate_bps: u32,
        pub decay_rate_bps: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub seconds_per_period: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MintAuthorityTransferred {
        #[serde(with = "pubkey_serde")]
        pub inflation_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        #[serde(with = "pubkey_serde_option")]
        pub old_authority: Option<Pubkey>,
        #[serde(with = "pubkey_serde_option")]
        pub new_authority: Option<Pubkey>,
        #[serde(with = "pubkey_serde")]
        pub transferred_by: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TransferMintAuthorityArgs {
        #[serde(with = "pubkey_serde_option")]
        pub new_authority: Option<Pubkey>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UpdateConfigArgs {
        #[serde(with = "pubkey_serde_option")]
        pub new_authority: Option<Pubkey>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UpdateInflationParametersArgs {
        pub new_initial_rate_bps: Option<u32>,
        pub new_terminal_rate_bps: Option<u32>,
        pub new_decay_rate_bps: Option<u32>,
        pub new_seconds_per_period: Option<u64>,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitializeInflationConfigAccounts {
        pub payer: String,
        pub mint_authority: String,
        pub inflation_config: String,
        pub mint: String,
        pub destination: String,
        pub token_program: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PauseInflationAccounts {
        pub inflation_config: String,
        pub authority: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ResumeInflationAccounts {
        pub inflation_config: String,
        pub authority: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferMintAuthorityAccounts {
        pub inflation_config: String,
        pub authority: String,
        pub mint: String,
        pub token_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TriggerInflationAccounts {
        pub inflation_config: String,
        pub mint: String,
        pub destination: String,
        pub token_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateConfigAccounts {
        pub inflation_config: String,
        pub authority: String,
        pub new_destination: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateInflationParametersAccounts {
        pub inflation_config: String,
        pub authority: String,
        pub event_authority: String,
        pub program: String,
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
    pub struct InitializeInflationConfigIxData {
        pub args: typedefs::InitializeInflationConfigArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PauseInflationIxData {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ResumeInflationIxData {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferMintAuthorityIxData {
        pub args: typedefs::TransferMintAuthorityArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TriggerInflationIxData {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateConfigIxData {
        pub args: typedefs::UpdateConfigArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateInflationParametersIxData {
        pub args: typedefs::UpdateInflationParametersArgs,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitializeInflationConfig {
        accounts: InitializeInflationConfigAccounts,
        args: InitializeInflationConfigIxData,
    },
    PauseInflation {
        accounts: PauseInflationAccounts,
        args: PauseInflationIxData,
    },
    ResumeInflation {
        accounts: ResumeInflationAccounts,
        args: ResumeInflationIxData,
    },
    TransferMintAuthority {
        accounts: TransferMintAuthorityAccounts,
        args: TransferMintAuthorityIxData,
    },
    TriggerInflation {
        accounts: TriggerInflationAccounts,
        args: TriggerInflationIxData,
    },
    UpdateConfig {
        accounts: UpdateConfigAccounts,
        args: UpdateConfigIxData,
    },
    UpdateInflationParameters {
        accounts: UpdateInflationParametersAccounts,
        args: UpdateInflationParametersIxData,
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
            [150u8, 128u8, 12u8, 95u8, 160u8, 154u8, 90u8, 24u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeInflationConfigIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let mint_authority = keys.next().unwrap().clone();
                let inflation_config = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeInflationConfigAccounts {
                    payer,
                    mint_authority,
                    inflation_config,
                    mint,
                    destination,
                    token_program,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitializeInflationConfig { accounts, args });
            }
            [26u8, 209u8, 144u8, 252u8, 107u8, 160u8, 134u8, 107u8] => {
                let mut rdr: &[u8] = rest;
                let args = PauseInflationIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let inflation_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PauseInflationAccounts {
                    inflation_config,
                    authority,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::PauseInflation { accounts, args });
            }
            [58u8, 153u8, 30u8, 219u8, 223u8, 122u8, 205u8, 184u8] => {
                let mut rdr: &[u8] = rest;
                let args = ResumeInflationIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let inflation_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ResumeInflationAccounts {
                    inflation_config,
                    authority,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ResumeInflation { accounts, args });
            }
            [87u8, 237u8, 187u8, 84u8, 168u8, 175u8, 241u8, 75u8] => {
                let mut rdr: &[u8] = rest;
                let args = TransferMintAuthorityIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let inflation_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferMintAuthorityAccounts {
                    inflation_config,
                    authority,
                    mint,
                    token_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::TransferMintAuthority { accounts, args });
            }
            [98u8, 169u8, 27u8, 83u8, 188u8, 9u8, 120u8, 199u8] => {
                let mut rdr: &[u8] = rest;
                let args = TriggerInflationIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let inflation_config = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TriggerInflationAccounts {
                    inflation_config,
                    mint,
                    destination,
                    token_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::TriggerInflation { accounts, args });
            }
            [29u8, 158u8, 252u8, 191u8, 10u8, 83u8, 219u8, 99u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateConfigIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let inflation_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let new_destination = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateConfigAccounts {
                    inflation_config,
                    authority,
                    new_destination,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdateConfig { accounts, args });
            }
            [142u8, 21u8, 95u8, 55u8, 169u8, 161u8, 28u8, 213u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateInflationParametersIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let inflation_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateInflationParametersAccounts {
                    inflation_config,
                    authority,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdateInflationParameters { accounts, args });
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
        InflationConfigInitialized { args: InflationConfigInitialized },
        InflationConfigUpdated { args: InflationConfigUpdated },
        InflationParametersUpdated { args: InflationParametersUpdated },
        InflationPaused { args: InflationPaused },
        InflationResumed { args: InflationResumed },
        InflationTriggered { args: InflationTriggered },
        MintAuthorityTransferred { args: MintAuthorityTransferred },
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
                [63u8, 27u8, 168u8, 47u8, 67u8, 145u8, 234u8, 45u8] => {
                    let mut rdr = &data[16..];
                    let args = InflationConfigInitialized::deserialize(&mut rdr)?;
                    return Ok(Event::InflationConfigInitialized { args });
                }
                [209u8, 186u8, 199u8, 8u8, 27u8, 64u8, 190u8, 133u8] => {
                    let mut rdr = &data[16..];
                    let args = InflationConfigUpdated::deserialize(&mut rdr)?;
                    return Ok(Event::InflationConfigUpdated { args });
                }
                [24u8, 162u8, 14u8, 158u8, 173u8, 149u8, 32u8, 244u8] => {
                    let mut rdr = &data[16..];
                    let args = InflationParametersUpdated::deserialize(&mut rdr)?;
                    return Ok(Event::InflationParametersUpdated { args });
                }
                [99u8, 2u8, 99u8, 182u8, 168u8, 117u8, 22u8, 165u8] => {
                    let mut rdr = &data[16..];
                    let args = InflationPaused::deserialize(&mut rdr)?;
                    return Ok(Event::InflationPaused { args });
                }
                [203u8, 219u8, 167u8, 95u8, 154u8, 239u8, 153u8, 209u8] => {
                    let mut rdr = &data[16..];
                    let args = InflationResumed::deserialize(&mut rdr)?;
                    return Ok(Event::InflationResumed { args });
                }
                [226u8, 7u8, 68u8, 231u8, 129u8, 19u8, 29u8, 20u8] => {
                    let mut rdr = &data[16..];
                    let args = InflationTriggered::deserialize(&mut rdr)?;
                    return Ok(Event::InflationTriggered { args });
                }
                [23u8, 136u8, 155u8, 223u8, 27u8, 166u8, 51u8, 85u8] => {
                    let mut rdr = &data[16..];
                    let args = MintAuthorityTransferred::deserialize(&mut rdr)?;
                    return Ok(Event::MintAuthorityTransferred { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
