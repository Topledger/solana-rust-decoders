#[allow(dead_code)]
use serde::Serializer;
use std::convert::TryInto;
fn serialize_to_string<S, T>(x: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    s.serialize_str(&x.to_string())
}
fn serialize_big_array<S, T, const N: usize>(arr: &[T; N], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: serde::Serialize,
{
    use serde::ser::SerializeTuple;
    let mut tup = s.serialize_tuple(N)?;
    for v in arr.iter() {
        tup.serialize_element(v)?;
    }
    tup.end()
}
#[doc = r" Parse an Option<T> in either old‑IDL (no tag) or new‑IDL (0x00/0x01 prefix) form"]
fn parse_option<T: ::borsh::BorshDeserialize>(rdr: &mut &[u8]) -> anyhow::Result<Option<T>> {
    if rdr.is_empty() {
        return Ok(None);
    }
    let tag = rdr[0];
    if tag == 0 {
        *rdr = &rdr[1..];
        return Ok(None);
    } else if tag == 1 {
        *rdr = &rdr[1..];
        let v = T::deserialize(rdr)?;
        return Ok(Some(v));
    }
    let v = T::deserialize(rdr)?;
    Ok(Some(v))
}
pub use accounts_data::*;
pub use ix_data::*;
pub use typedefs::*;
pub mod typedefs {
    use anchor_lang::prelude::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde::Serialize;
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct CircBuf {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub idx: u64,
        pub is_empty: u8,
        pub padding: [u8; 7],
        #[serde(serialize_with = "crate::serialize_big_array")]
        pub arr: [ValidatorHistoryEntry; 512],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct CircBufCluster {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub idx: u64,
        pub is_empty: u8,
        pub padding: [u8; 7],
        #[serde(serialize_with = "crate::serialize_big_array")]
        pub arr: [ClusterHistoryEntry; 512],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ClientVersion {
        pub major: u8,
        pub minor: u8,
        pub patch: u16,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct ClusterHistory {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub struct_version: u64,
        pub bump: u8,
        pub padding0: [u8; 7],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cluster_history_last_update_slot: u64,
        #[serde(serialize_with = "crate::serialize_big_array")]
        pub padding1: [u8; 232],
        pub history: CircBufCluster,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct ClusterHistoryEntry {
        pub total_blocks: u32,
        pub epoch: u16,
        pub padding0: [u8; 2],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch_start_timestamp: u64,
        #[serde(serialize_with = "crate::serialize_big_array")]
        pub padding: [u8; 240],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct Config {
        #[serde(with = "crate::pubkey_serializer::pubkey_serde_anchor")]
        pub tip_distribution_program: Pubkey,
        #[serde(with = "crate::pubkey_serializer::pubkey_serde_anchor")]
        pub admin: Pubkey,
        #[serde(with = "crate::pubkey_serializer::pubkey_serde_anchor")]
        pub oracle_authority: Pubkey,
        pub counter: u32,
        pub bump: u8,
        pub padding0: [u8; 3],
        #[serde(with = "crate::pubkey_serializer::pubkey_serde_anchor")]
        pub priority_fee_distribution_program: Pubkey,
        #[serde(with = "crate::pubkey_serializer::pubkey_serde_anchor")]
        pub priority_fee_oracle_authority: Pubkey,
        #[serde(serialize_with = "crate::serialize_big_array")]
        pub reserve: [u8; 224],
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum MerkleRootUploadAuthority {
        Unset,
        Empty,
        Other,
        OldJitoLabs,
        TipRouter,
    }
    impl Default for MerkleRootUploadAuthority {
        fn default() -> Self {
            Self::Unset
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct ValidatorHistory {
        pub struct_version: u32,
        #[serde(with = "crate::pubkey_serializer::pubkey_serde_anchor")]
        pub vote_account: Pubkey,
        pub index: u32,
        pub bump: u8,
        pub padding0: [u8; 7],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_ip_timestamp: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_version_timestamp: u64,
        pub validator_age: u32,
        pub validator_age_last_updated_epoch: u16,
        #[serde(serialize_with = "crate::serialize_big_array")]
        pub padding1: [u8; 226],
        pub history: CircBuf,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct ValidatorHistoryEntry {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub activated_stake_lamports: u64,
        pub epoch: u16,
        pub mev_commission: u16,
        pub epoch_credits: u32,
        pub commission: u8,
        pub client_type: u8,
        pub version: ClientVersion,
        pub ip: [u8; 4],
        pub merkle_root_upload_authority: MerkleRootUploadAuthority,
        pub is_superminority: u8,
        pub rank: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub vote_account_last_update_slot: u64,
        pub mev_earned: u32,
        pub priority_fee_commission: u16,
        pub padding0: [u8; 2],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub priority_fee_tips: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_priority_fees: u64,
        pub total_leader_slots: u32,
        pub blocks_produced: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub block_data_updated_at_slot: u64,
        pub priority_fee_merkle_root_upload_authority: MerkleRootUploadAuthority,
        #[serde(serialize_with = "crate::serialize_big_array")]
        pub padding1: [u8; 47],
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct BackfillTotalBlocksAccounts {
        pub cluster_history_account: String,
        pub config: String,
        pub oracle_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CopyClusterInfoAccounts {
        pub cluster_history_account: String,
        pub slot_history: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CopyGossipContactInfoAccounts {
        pub validator_history_account: String,
        pub vote_account: String,
        pub instructions: String,
        pub config: String,
        pub oracle_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CopyPriorityFeeDistributionAccounts {
        pub validator_history_account: String,
        pub vote_account: String,
        pub config: String,
        pub distribution_account: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CopyTipDistributionAccountAccounts {
        pub validator_history_account: String,
        pub vote_account: String,
        pub config: String,
        pub tip_distribution_account: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CopyVoteAccountAccounts {
        pub validator_history_account: String,
        pub vote_account: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeClusterHistoryAccountAccounts {
        pub cluster_history_account: String,
        pub system_program: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeConfigAccounts {
        pub config: String,
        pub system_program: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeValidatorHistoryAccountAccounts {
        pub validator_history_account: String,
        pub vote_account: String,
        pub system_program: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ReallocClusterHistoryAccountAccounts {
        pub cluster_history_account: String,
        pub system_program: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ReallocConfigAccountAccounts {
        pub config_account: String,
        pub system_program: String,
        pub payer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ReallocValidatorHistoryAccountAccounts {
        pub validator_history_account: String,
        pub config: String,
        pub vote_account: String,
        pub system_program: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetNewAdminAccounts {
        pub config: String,
        pub new_admin: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetNewOracleAuthorityAccounts {
        pub config: String,
        pub new_oracle_authority: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetNewPriorityFeeDistributionProgramAccounts {
        pub config: String,
        pub new_priority_fee_distribution_program: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetNewPriorityFeeOracleAuthorityAccounts {
        pub config: String,
        pub new_priority_fee_oracle_authority: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetNewTipDistributionProgramAccounts {
        pub config: String,
        pub new_tip_distribution_program: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePriorityFeeHistoryAccounts {
        pub validator_history_account: String,
        pub vote_account: String,
        pub config: String,
        pub priority_fee_oracle_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateStakeHistoryAccounts {
        pub validator_history_account: String,
        pub vote_account: String,
        pub config: String,
        pub oracle_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UploadValidatorAgeAccounts {
        pub validator_history_account: String,
        pub vote_account: String,
        pub config: String,
        pub oracle_authority: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BackfillTotalBlocksArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
        pub blocks_in_epoch: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CopyClusterInfoArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CopyGossipContactInfoArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CopyPriorityFeeDistributionArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CopyTipDistributionAccountArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CopyVoteAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeClusterHistoryAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeConfigArgs {
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeValidatorHistoryAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ReallocClusterHistoryAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ReallocConfigAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ReallocValidatorHistoryAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetNewAdminArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetNewOracleAuthorityArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetNewPriorityFeeDistributionProgramArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetNewPriorityFeeOracleAuthorityArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetNewTipDistributionProgramArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePriorityFeeHistoryArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_priority_fees: u64,
        pub total_leader_slots: u32,
        pub blocks_produced: u32,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub highest_oracle_recorded_slot: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateStakeHistoryArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports: u64,
        pub rank: u32,
        pub is_superminority: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UploadValidatorAgeArgs {
        pub validator_age: u32,
        pub validator_age_last_updated_epoch: u16,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    BackfillTotalBlocks {
        accounts: BackfillTotalBlocksAccounts,
        args: BackfillTotalBlocksArgs,
    },
    CopyClusterInfo {
        accounts: CopyClusterInfoAccounts,
        args: CopyClusterInfoArgs,
    },
    CopyGossipContactInfo {
        accounts: CopyGossipContactInfoAccounts,
        args: CopyGossipContactInfoArgs,
    },
    CopyPriorityFeeDistribution {
        accounts: CopyPriorityFeeDistributionAccounts,
        args: CopyPriorityFeeDistributionArgs,
    },
    CopyTipDistributionAccount {
        accounts: CopyTipDistributionAccountAccounts,
        args: CopyTipDistributionAccountArgs,
    },
    CopyVoteAccount {
        accounts: CopyVoteAccountAccounts,
        args: CopyVoteAccountArgs,
    },
    InitializeClusterHistoryAccount {
        accounts: InitializeClusterHistoryAccountAccounts,
        args: InitializeClusterHistoryAccountArgs,
    },
    InitializeConfig {
        accounts: InitializeConfigAccounts,
        args: InitializeConfigArgs,
    },
    InitializeValidatorHistoryAccount {
        accounts: InitializeValidatorHistoryAccountAccounts,
        args: InitializeValidatorHistoryAccountArgs,
    },
    ReallocClusterHistoryAccount {
        accounts: ReallocClusterHistoryAccountAccounts,
        args: ReallocClusterHistoryAccountArgs,
    },
    ReallocConfigAccount {
        accounts: ReallocConfigAccountAccounts,
        args: ReallocConfigAccountArgs,
    },
    ReallocValidatorHistoryAccount {
        accounts: ReallocValidatorHistoryAccountAccounts,
        args: ReallocValidatorHistoryAccountArgs,
    },
    SetNewAdmin {
        accounts: SetNewAdminAccounts,
        args: SetNewAdminArgs,
    },
    SetNewOracleAuthority {
        accounts: SetNewOracleAuthorityAccounts,
        args: SetNewOracleAuthorityArgs,
    },
    SetNewPriorityFeeDistributionProgram {
        accounts: SetNewPriorityFeeDistributionProgramAccounts,
        args: SetNewPriorityFeeDistributionProgramArgs,
    },
    SetNewPriorityFeeOracleAuthority {
        accounts: SetNewPriorityFeeOracleAuthorityAccounts,
        args: SetNewPriorityFeeOracleAuthorityArgs,
    },
    SetNewTipDistributionProgram {
        accounts: SetNewTipDistributionProgramAccounts,
        args: SetNewTipDistributionProgramArgs,
    },
    UpdatePriorityFeeHistory {
        accounts: UpdatePriorityFeeHistoryAccounts,
        args: UpdatePriorityFeeHistoryArgs,
    },
    UpdateStakeHistory {
        accounts: UpdateStakeHistoryAccounts,
        args: UpdateStakeHistoryArgs,
    },
    UploadValidatorAge {
        accounts: UploadValidatorAgeAccounts,
        args: UploadValidatorAgeArgs,
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
            [223u8, 59u8, 120u8, 117u8, 34u8, 248u8, 117u8, 220u8] => {
                let mut rdr: &[u8] = rest;
                let epoch: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let blocks_in_epoch: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = BackfillTotalBlocksArgs {
                    epoch,
                    blocks_in_epoch,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let cluster_history_account = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let oracle_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BackfillTotalBlocksAccounts {
                    cluster_history_account,
                    config,
                    oracle_authority,
                    remaining,
                };
                return Ok(Instruction::BackfillTotalBlocks { accounts, args });
            }
            [124u8, 126u8, 139u8, 134u8, 126u8, 230u8, 100u8, 37u8] => {
                let mut rdr: &[u8] = rest;
                let args = CopyClusterInfoArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let cluster_history_account = keys.next().unwrap().clone();
                let slot_history = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CopyClusterInfoAccounts {
                    cluster_history_account,
                    slot_history,
                    signer,
                    remaining,
                };
                return Ok(Instruction::CopyClusterInfo { accounts, args });
            }
            [246u8, 174u8, 228u8, 249u8, 28u8, 209u8, 69u8, 85u8] => {
                let mut rdr: &[u8] = rest;
                let args = CopyGossipContactInfoArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let validator_history_account = keys.next().unwrap().clone();
                let vote_account = keys.next().unwrap().clone();
                let instructions = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let oracle_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CopyGossipContactInfoAccounts {
                    validator_history_account,
                    vote_account,
                    instructions,
                    config,
                    oracle_authority,
                    remaining,
                };
                return Ok(Instruction::CopyGossipContactInfo { accounts, args });
            }
            [152u8, 174u8, 178u8, 77u8, 92u8, 83u8, 93u8, 33u8] => {
                let mut rdr: &[u8] = rest;
                let epoch: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = CopyPriorityFeeDistributionArgs { epoch };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let validator_history_account = keys.next().unwrap().clone();
                let vote_account = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let distribution_account = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CopyPriorityFeeDistributionAccounts {
                    validator_history_account,
                    vote_account,
                    config,
                    distribution_account,
                    signer,
                    remaining,
                };
                return Ok(Instruction::CopyPriorityFeeDistribution { accounts, args });
            }
            [208u8, 213u8, 185u8, 210u8, 103u8, 124u8, 128u8, 173u8] => {
                let mut rdr: &[u8] = rest;
                let epoch: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = CopyTipDistributionAccountArgs { epoch };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let validator_history_account = keys.next().unwrap().clone();
                let vote_account = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let tip_distribution_account = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CopyTipDistributionAccountAccounts {
                    validator_history_account,
                    vote_account,
                    config,
                    tip_distribution_account,
                    signer,
                    remaining,
                };
                return Ok(Instruction::CopyTipDistributionAccount { accounts, args });
            }
            [171u8, 204u8, 73u8, 59u8, 129u8, 63u8, 134u8, 61u8] => {
                let mut rdr: &[u8] = rest;
                let args = CopyVoteAccountArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let validator_history_account = keys.next().unwrap().clone();
                let vote_account = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CopyVoteAccountAccounts {
                    validator_history_account,
                    vote_account,
                    signer,
                    remaining,
                };
                return Ok(Instruction::CopyVoteAccount { accounts, args });
            }
            [8u8, 204u8, 21u8, 27u8, 253u8, 9u8, 10u8, 78u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeClusterHistoryAccountArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let cluster_history_account = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeClusterHistoryAccountAccounts {
                    cluster_history_account,
                    system_program,
                    signer,
                    remaining,
                };
                return Ok(Instruction::InitializeClusterHistoryAccount { accounts, args });
            }
            [208u8, 127u8, 21u8, 1u8, 194u8, 190u8, 196u8, 70u8] => {
                let mut rdr: &[u8] = rest;
                let authority: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeConfigArgs { authority };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeConfigAccounts {
                    config,
                    system_program,
                    signer,
                    remaining,
                };
                return Ok(Instruction::InitializeConfig { accounts, args });
            }
            [61u8, 152u8, 10u8, 77u8, 196u8, 242u8, 89u8, 36u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeValidatorHistoryAccountArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let validator_history_account = keys.next().unwrap().clone();
                let vote_account = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeValidatorHistoryAccountAccounts {
                    validator_history_account,
                    vote_account,
                    system_program,
                    signer,
                    remaining,
                };
                return Ok(Instruction::InitializeValidatorHistoryAccount { accounts, args });
            }
            [249u8, 51u8, 161u8, 22u8, 107u8, 40u8, 129u8, 104u8] => {
                let mut rdr: &[u8] = rest;
                let args = ReallocClusterHistoryAccountArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let cluster_history_account = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ReallocClusterHistoryAccountAccounts {
                    cluster_history_account,
                    system_program,
                    signer,
                    remaining,
                };
                return Ok(Instruction::ReallocClusterHistoryAccount { accounts, args });
            }
            [55u8, 69u8, 118u8, 225u8, 163u8, 54u8, 79u8, 101u8] => {
                let mut rdr: &[u8] = rest;
                let args = ReallocConfigAccountArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config_account = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ReallocConfigAccountAccounts {
                    config_account,
                    system_program,
                    payer,
                    remaining,
                };
                return Ok(Instruction::ReallocConfigAccount { accounts, args });
            }
            [196u8, 17u8, 33u8, 140u8, 174u8, 130u8, 33u8, 12u8] => {
                let mut rdr: &[u8] = rest;
                let args = ReallocValidatorHistoryAccountArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let validator_history_account = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let vote_account = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ReallocValidatorHistoryAccountAccounts {
                    validator_history_account,
                    config,
                    vote_account,
                    system_program,
                    signer,
                    remaining,
                };
                return Ok(Instruction::ReallocValidatorHistoryAccount { accounts, args });
            }
            [62u8, 156u8, 4u8, 148u8, 79u8, 162u8, 148u8, 252u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetNewAdminArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config = keys.next().unwrap().clone();
                let new_admin = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetNewAdminAccounts {
                    config,
                    new_admin,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetNewAdmin { accounts, args });
            }
            [70u8, 73u8, 21u8, 170u8, 82u8, 128u8, 144u8, 56u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetNewOracleAuthorityArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config = keys.next().unwrap().clone();
                let new_oracle_authority = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetNewOracleAuthorityAccounts {
                    config,
                    new_oracle_authority,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetNewOracleAuthority { accounts, args });
            }
            [219u8, 234u8, 129u8, 224u8, 128u8, 162u8, 61u8, 174u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetNewPriorityFeeDistributionProgramArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config = keys.next().unwrap().clone();
                let new_priority_fee_distribution_program = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetNewPriorityFeeDistributionProgramAccounts {
                    config,
                    new_priority_fee_distribution_program,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetNewPriorityFeeDistributionProgram { accounts, args });
            }
            [140u8, 74u8, 47u8, 78u8, 73u8, 165u8, 143u8, 92u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetNewPriorityFeeOracleAuthorityArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config = keys.next().unwrap().clone();
                let new_priority_fee_oracle_authority = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetNewPriorityFeeOracleAuthorityAccounts {
                    config,
                    new_priority_fee_oracle_authority,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetNewPriorityFeeOracleAuthority { accounts, args });
            }
            [200u8, 69u8, 235u8, 182u8, 178u8, 62u8, 130u8, 217u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetNewTipDistributionProgramArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config = keys.next().unwrap().clone();
                let new_tip_distribution_program = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetNewTipDistributionProgramAccounts {
                    config,
                    new_tip_distribution_program,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetNewTipDistributionProgram { accounts, args });
            }
            [238u8, 213u8, 116u8, 176u8, 1u8, 211u8, 52u8, 214u8] => {
                let mut rdr: &[u8] = rest;
                let epoch: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let total_priority_fees: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let total_leader_slots: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let blocks_produced: u32 =
                    <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let highest_oracle_recorded_slot: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdatePriorityFeeHistoryArgs {
                    epoch,
                    total_priority_fees,
                    total_leader_slots,
                    blocks_produced,
                    highest_oracle_recorded_slot,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let validator_history_account = keys.next().unwrap().clone();
                let vote_account = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let priority_fee_oracle_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePriorityFeeHistoryAccounts {
                    validator_history_account,
                    vote_account,
                    config,
                    priority_fee_oracle_authority,
                    remaining,
                };
                return Ok(Instruction::UpdatePriorityFeeHistory { accounts, args });
            }
            [213u8, 212u8, 103u8, 144u8, 8u8, 212u8, 173u8, 232u8] => {
                let mut rdr: &[u8] = rest;
                let epoch: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let lamports: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let rank: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let is_superminority: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateStakeHistoryArgs {
                    epoch,
                    lamports,
                    rank,
                    is_superminority,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let validator_history_account = keys.next().unwrap().clone();
                let vote_account = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let oracle_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateStakeHistoryAccounts {
                    validator_history_account,
                    vote_account,
                    config,
                    oracle_authority,
                    remaining,
                };
                return Ok(Instruction::UpdateStakeHistory { accounts, args });
            }
            [52u8, 37u8, 147u8, 127u8, 255u8, 138u8, 32u8, 94u8] => {
                let mut rdr: &[u8] = rest;
                let validator_age: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let validator_age_last_updated_epoch: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UploadValidatorAgeArgs {
                    validator_age,
                    validator_age_last_updated_epoch,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let validator_history_account = keys.next().unwrap().clone();
                let vote_account = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let oracle_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UploadValidatorAgeAccounts {
                    validator_history_account,
                    vote_account,
                    config,
                    oracle_authority,
                    remaining,
                };
                return Ok(Instruction::UploadValidatorAge { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
