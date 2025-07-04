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
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use anchor_lang::prelude::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde::Serialize;
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ClaimStatus {
        pub is_claimed: bool,
        pub claimant: Pubkey,
        pub claim_status_payer: Pubkey,
        pub slot_claimed_at: u64,
        pub amount: u64,
        pub expires_at: u64,
        pub bump: u8,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ClaimStatusClosedEvent {
        pub claim_status_payer: Pubkey,
        pub claim_status_account: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ClaimedEvent {
        pub tip_distribution_account: Pubkey,
        pub payer: Pubkey,
        pub claimant: Pubkey,
        pub amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Config {
        pub authority: Pubkey,
        pub expired_funds_account: Pubkey,
        pub num_epochs_valid: u64,
        pub max_validator_commission_bps: u16,
        pub bump: u8,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ConfigUpdatedEvent {
        pub authority: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MerkleRoot {
        pub root: [u8; 32],
        pub max_total_claim: u64,
        pub max_num_nodes: u64,
        pub total_funds_claimed: u64,
        pub num_nodes_claimed: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MerkleRootUploadAuthorityUpdatedEvent {
        pub old_authority: Pubkey,
        pub new_authority: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MerkleRootUploadConfig {
        pub override_authority: Pubkey,
        pub original_upload_authority: Pubkey,
        pub bump: u8,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MerkleRootUploadedEvent {
        pub merkle_root_upload_authority: Pubkey,
        pub tip_distribution_account: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TipDistributionAccount {
        pub validator_vote_account: Pubkey,
        pub merkle_root_upload_authority: Pubkey,
        pub merkle_root: Option<MerkleRoot>,
        pub epoch_created_at: u64,
        pub validator_commission_bps: u16,
        pub expires_at: u64,
        pub bump: u8,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TipDistributionAccountClosedEvent {
        pub expired_funds_account: Pubkey,
        pub tip_distribution_account: Pubkey,
        pub expired_amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TipDistributionAccountInitializedEvent {
        pub tip_distribution_account: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ValidatorCommissionBpsUpdatedEvent {
        pub tip_distribution_account: Pubkey,
        pub old_commission_bps: u16,
        pub new_commission_bps: u16,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct ClaimAccounts {
        pub config: String,
        pub tip_distribution_account: String,
        pub merkle_root_upload_authority: String,
        pub claim_status: String,
        pub claimant: String,
        pub payer: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub system_program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseClaimStatusAccounts {
        pub config: String,
        pub claim_status: String,
        pub claim_status_payer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseTipDistributionAccountAccounts {
        pub config: String,
        pub expired_funds_account: String,
        pub tip_distribution_account: String,
        pub validator_vote_account: String,
        pub signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeAccounts {
        pub config: String,
        pub system_program: String,
        pub initializer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeMerkleRootUploadConfigAccounts {
        pub config: String,
        pub merkle_root_upload_config: String,
        pub authority: String,
        pub payer: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeTipDistributionAccountAccounts {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub config: Option<String>,
        pub tip_distribution_account: String,
        pub validator_vote_account: String,
        pub signer: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MigrateTdaMerkleRootUploadAuthorityAccounts {
        pub tip_distribution_account: String,
        pub merkle_root_upload_config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateConfigAccounts {
        pub config: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateMerkleRootUploadConfigAccounts {
        pub config: String,
        pub merkle_root_upload_config: String,
        pub authority: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UploadMerkleRootAccounts {
        pub config: String,
        pub tip_distribution_account: String,
        pub merkle_root_upload_authority: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimArgs {
        pub bump: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        pub proof: Vec<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseClaimStatusArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseTipDistributionAccountArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub epoch: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeArgs {
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub expired_funds_account: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub num_epochs_valid: u64,
        pub max_validator_commission_bps: u16,
        pub bump: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeMerkleRootUploadConfigArgs {
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub original_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeTipDistributionAccountArgs {
        #[serde(with = "pubkey_serde")]
        pub merkle_root_upload_authority: [u8; 32usize],
        pub validator_commission_bps: u16,
        pub bump: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MigrateTdaMerkleRootUploadAuthorityArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateConfigArgs {
        pub new_config: Config,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateMerkleRootUploadConfigArgs {
        #[serde(with = "pubkey_serde")]
        pub authority: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub original_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UploadMerkleRootArgs {
        pub root: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_total_claim: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_num_nodes: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Claim {
        accounts: ClaimAccounts,
        args: ClaimArgs,
    },
    CloseClaimStatus {
        accounts: CloseClaimStatusAccounts,
        args: CloseClaimStatusArgs,
    },
    CloseTipDistributionAccount {
        accounts: CloseTipDistributionAccountAccounts,
        args: CloseTipDistributionAccountArgs,
    },
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArgs,
    },
    InitializeMerkleRootUploadConfig {
        accounts: InitializeMerkleRootUploadConfigAccounts,
        args: InitializeMerkleRootUploadConfigArgs,
    },
    InitializeTipDistributionAccount {
        accounts: InitializeTipDistributionAccountAccounts,
        args: InitializeTipDistributionAccountArgs,
    },
    MigrateTdaMerkleRootUploadAuthority {
        accounts: MigrateTdaMerkleRootUploadAuthorityAccounts,
        args: MigrateTdaMerkleRootUploadAuthorityArgs,
    },
    UpdateConfig {
        accounts: UpdateConfigAccounts,
        args: UpdateConfigArgs,
    },
    UpdateMerkleRootUploadConfig {
        accounts: UpdateMerkleRootUploadConfigAccounts,
        args: UpdateMerkleRootUploadConfigArgs,
    },
    UploadMerkleRoot {
        accounts: UploadMerkleRootAccounts,
        args: UploadMerkleRootArgs,
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
            [62u8, 198u8, 214u8, 193u8, 213u8, 159u8, 108u8, 210u8] => {
                let mut rdr: &[u8] = rest;
                let bump: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let proof: Vec<[u8; 32usize]> =
                    <Vec<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = ClaimArgs {
                    bump,
                    amount,
                    proof,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 6usize;
                let config = keys.next().unwrap().clone();
                let tip_distribution_account = keys.next().unwrap().clone();
                let merkle_root_upload_authority = keys.next().unwrap().clone();
                let claim_status = keys.next().unwrap().clone();
                let claimant = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let system_program = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = ClaimAccounts {
                    config,
                    tip_distribution_account,
                    merkle_root_upload_authority,
                    claim_status,
                    claimant,
                    payer,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::Claim { accounts, args });
            }
            [163u8, 214u8, 191u8, 165u8, 245u8, 188u8, 17u8, 185u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseClaimStatusArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config = keys.next().unwrap().clone();
                let claim_status = keys.next().unwrap().clone();
                let claim_status_payer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseClaimStatusAccounts {
                    config,
                    claim_status,
                    claim_status_payer,
                    remaining,
                };
                return Ok(Instruction::CloseClaimStatus { accounts, args });
            }
            [47u8, 136u8, 208u8, 190u8, 125u8, 243u8, 74u8, 227u8] => {
                let mut rdr: &[u8] = rest;
                let epoch: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = CloseTipDistributionAccountArgs { epoch };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let config = keys.next().unwrap().clone();
                let expired_funds_account = keys.next().unwrap().clone();
                let tip_distribution_account = keys.next().unwrap().clone();
                let validator_vote_account = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseTipDistributionAccountAccounts {
                    config,
                    expired_funds_account,
                    tip_distribution_account,
                    validator_vote_account,
                    signer,
                    remaining,
                };
                return Ok(Instruction::CloseTipDistributionAccount { accounts, args });
            }
            [175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let authority: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let expired_funds_account: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let num_epochs_valid: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_validator_commission_bps: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let bump: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeArgs {
                    authority,
                    expired_funds_account,
                    num_epochs_valid,
                    max_validator_commission_bps,
                    bump,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let initializer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    config,
                    system_program,
                    initializer,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            [232u8, 87u8, 72u8, 14u8, 89u8, 40u8, 40u8, 27u8] => {
                let mut rdr: &[u8] = rest;
                let authority: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let original_authority: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeMerkleRootUploadConfigArgs {
                    authority,
                    original_authority,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 5usize;
                let config = keys.next().unwrap().clone();
                let merkle_root_upload_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeMerkleRootUploadConfigAccounts {
                    config,
                    merkle_root_upload_config,
                    authority,
                    payer,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::InitializeMerkleRootUploadConfig { accounts, args });
            }
            [120u8, 191u8, 25u8, 182u8, 111u8, 49u8, 179u8, 55u8] => {
                let mut rdr: &[u8] = rest;
                let merkle_root_upload_authority: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let validator_commission_bps: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let bump: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = InitializeTipDistributionAccountArgs {
                    merkle_root_upload_authority,
                    validator_commission_bps,
                    bump,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let config = if has_extra {
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let tip_distribution_account = keys.next().unwrap().clone();
                let validator_vote_account = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeTipDistributionAccountAccounts {
                    config,
                    tip_distribution_account,
                    validator_vote_account,
                    signer,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::InitializeTipDistributionAccount { accounts, args });
            }
            [13u8, 226u8, 163u8, 144u8, 56u8, 202u8, 214u8, 23u8] => {
                let mut rdr: &[u8] = rest;
                let args = MigrateTdaMerkleRootUploadAuthorityArgs {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let tip_distribution_account = keys.next().unwrap().clone();
                let merkle_root_upload_config = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MigrateTdaMerkleRootUploadAuthorityAccounts {
                    tip_distribution_account,
                    merkle_root_upload_config,
                    remaining,
                };
                return Ok(Instruction::MigrateTdaMerkleRootUploadAuthority { accounts, args });
            }
            [29u8, 158u8, 252u8, 191u8, 10u8, 83u8, 219u8, 99u8] => {
                let mut rdr: &[u8] = rest;
                let new_config: Config =
                    <Config as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateConfigArgs { new_config };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 2usize;
                let config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateConfigAccounts {
                    config,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdateConfig { accounts, args });
            }
            [128u8, 227u8, 159u8, 139u8, 176u8, 128u8, 118u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let authority: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let original_authority: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateMerkleRootUploadConfigArgs {
                    authority,
                    original_authority,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 4usize;
                let config = keys.next().unwrap().clone();
                let merkle_root_upload_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateMerkleRootUploadConfigAccounts {
                    config,
                    merkle_root_upload_config,
                    authority,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::UpdateMerkleRootUploadConfig { accounts, args });
            }
            [70u8, 3u8, 110u8, 29u8, 199u8, 190u8, 205u8, 176u8] => {
                let mut rdr: &[u8] = rest;
                let root: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_total_claim: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let max_num_nodes: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UploadMerkleRootArgs {
                    root,
                    max_total_claim,
                    max_num_nodes,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let has_extra = account_keys.len() > 3usize;
                let config = keys.next().unwrap().clone();
                let tip_distribution_account = keys.next().unwrap().clone();
                let merkle_root_upload_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UploadMerkleRootAccounts {
                    config,
                    tip_distribution_account,
                    merkle_root_upload_authority,
                    remaining,
                };
                return Ok(Instruction::UploadMerkleRoot { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
