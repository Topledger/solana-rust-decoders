pub use accounts_data::*;
pub use ix_data::*;
#[allow(dead_code)]
use std::convert::TryInto;
use std::mem;
pub use typedefs::*;
pub mod typedefs {
    use anchor_lang::prelude::*;
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ClaimStatus {
        pub is_claimed: bool,
        pub claimant: Pubkey,
        pub claim_status_payer: Pubkey,
        pub slot_claimed_at: u64,
        pub amount: u64,
        pub expires_at: u64,
        pub bump: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ClaimStatusClosedEvent {
        pub claim_status_payer: Pubkey,
        pub claim_status_account: Pubkey,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ClaimedEvent {
        pub tip_distribution_account: Pubkey,
        pub payer: Pubkey,
        pub claimant: Pubkey,
        pub amount: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Config {
        pub authority: Pubkey,
        pub expired_funds_account: Pubkey,
        pub num_epochs_valid: u64,
        pub max_validator_commission_bps: u16,
        pub bump: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ConfigUpdatedEvent {
        pub authority: Pubkey,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MerkleRoot {
        pub root: [u8; 32],
        pub max_total_claim: u64,
        pub max_num_nodes: u64,
        pub total_funds_claimed: u64,
        pub num_nodes_claimed: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MerkleRootUploadAuthorityUpdatedEvent {
        pub old_authority: Pubkey,
        pub new_authority: Pubkey,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MerkleRootUploadConfig {
        pub override_authority: Pubkey,
        pub original_upload_authority: Pubkey,
        pub bump: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct MerkleRootUploadedEvent {
        pub merkle_root_upload_authority: Pubkey,
        pub tip_distribution_account: Pubkey,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TipDistributionAccount {
        pub validator_vote_account: Pubkey,
        pub merkle_root_upload_authority: Pubkey,
        pub merkle_root: Option<MerkleRoot>,
        pub epoch_created_at: u64,
        pub validator_commission_bps: u16,
        pub expires_at: u64,
        pub bump: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TipDistributionAccountClosedEvent {
        pub expired_funds_account: Pubkey,
        pub tip_distribution_account: Pubkey,
        pub expired_amount: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TipDistributionAccountInitializedEvent {
        pub tip_distribution_account: Pubkey,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct ValidatorCommissionBpsUpdatedEvent {
        pub tip_distribution_account: Pubkey,
        pub old_commission_bps: u16,
        pub new_commission_bps: u16,
    }
}
pub mod accounts_data {
    #[derive(Debug)]
    pub struct ClaimAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub tip_distribution_account: anchor_lang::prelude::Pubkey,
        pub merkle_root_upload_authority: anchor_lang::prelude::Pubkey,
        pub claim_status: anchor_lang::prelude::Pubkey,
        pub claimant: anchor_lang::prelude::Pubkey,
        pub payer: anchor_lang::prelude::Pubkey,
        pub system_program: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct CloseClaimStatusAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub claim_status: anchor_lang::prelude::Pubkey,
        pub claim_status_payer: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct CloseTipDistributionAccountAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub expired_funds_account: anchor_lang::prelude::Pubkey,
        pub tip_distribution_account: anchor_lang::prelude::Pubkey,
        pub validator_vote_account: anchor_lang::prelude::Pubkey,
        pub signer: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct InitializeAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub system_program: anchor_lang::prelude::Pubkey,
        pub initializer: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct InitializeMerkleRootUploadConfigAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub merkle_root_upload_config: anchor_lang::prelude::Pubkey,
        pub authority: anchor_lang::prelude::Pubkey,
        pub payer: anchor_lang::prelude::Pubkey,
        pub system_program: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct InitializeTipDistributionAccountAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub tip_distribution_account: anchor_lang::prelude::Pubkey,
        pub validator_vote_account: anchor_lang::prelude::Pubkey,
        pub signer: anchor_lang::prelude::Pubkey,
        pub system_program: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct MigrateTdaMerkleRootUploadAuthorityAccounts {
        pub tip_distribution_account: anchor_lang::prelude::Pubkey,
        pub merkle_root_upload_config: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct UpdateConfigAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub authority: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct UpdateMerkleRootUploadConfigAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub merkle_root_upload_config: anchor_lang::prelude::Pubkey,
        pub authority: anchor_lang::prelude::Pubkey,
        pub system_program: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct UploadMerkleRootAccounts {
        pub config: anchor_lang::prelude::Pubkey,
        pub tip_distribution_account: anchor_lang::prelude::Pubkey,
        pub merkle_root_upload_authority: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
}
pub mod ix_data {
    use super::*;
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct ClaimArgs {
        pub bump: u8,
        pub amount: u64,
        pub proof: Vec<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CloseClaimStatusArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CloseTipDistributionAccountArgs {
        pub epoch: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct InitializeArgs {
        pub authority: anchor_lang::prelude::Pubkey,
        pub expired_funds_account: anchor_lang::prelude::Pubkey,
        pub num_epochs_valid: u64,
        pub max_validator_commission_bps: u16,
        pub bump: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct InitializeMerkleRootUploadConfigArgs {
        pub authority: anchor_lang::prelude::Pubkey,
        pub original_authority: anchor_lang::prelude::Pubkey,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct InitializeTipDistributionAccountArgs {
        pub merkle_root_upload_authority: anchor_lang::prelude::Pubkey,
        pub validator_commission_bps: u16,
        pub bump: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct MigrateTdaMerkleRootUploadAuthorityArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct UpdateConfigArgs {
        pub new_config: Config,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct UpdateMerkleRootUploadConfigArgs {
        pub authority: anchor_lang::prelude::Pubkey,
        pub original_authority: anchor_lang::prelude::Pubkey,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct UploadMerkleRootArgs {
        pub root: [u8; 32usize],
        pub max_total_claim: u64,
        pub max_num_nodes: u64,
    }
}
#[derive(Debug)]
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
    pub fn decode(
        account_keys: &[anchor_lang::prelude::Pubkey],
        data: &[u8],
    ) -> anyhow::Result<Self> {
        if data.len() < 8 {
            anyhow::bail!("Data too short: {}", data.len());
        }
        let (disc_slice, rest) = data.split_at(8);
        let disc: [u8; 8] = disc_slice.try_into().unwrap();
        match disc {
            [62u8, 198u8, 214u8, 193u8, 213u8, 159u8, 108u8, 210u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let tip_distribution_account = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(tip_distribution_account)
                    ))
                })?;
                let merkle_root_upload_authority = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(merkle_root_upload_authority)
                    ))
                })?;
                let claim_status = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(claim_status)))
                })?;
                let claimant = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(claimant)))
                })?;
                let payer = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(payer)))
                })?;
                let system_program = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(system_program)))
                })?;
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
                let args = CloseClaimStatusArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let claim_status = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(claim_status)))
                })?;
                let claim_status_payer = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(claim_status_payer)))
                })?;
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
                let args = CloseTipDistributionAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let expired_funds_account = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(expired_funds_account)
                    ))
                })?;
                let tip_distribution_account = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(tip_distribution_account)
                    ))
                })?;
                let validator_vote_account = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(validator_vote_account)
                    ))
                })?;
                let signer = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(signer)))
                })?;
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
                let args = InitializeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let system_program = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(system_program)))
                })?;
                let initializer = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(initializer)))
                })?;
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
                let args = InitializeMerkleRootUploadConfigArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let merkle_root_upload_config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(merkle_root_upload_config)
                    ))
                })?;
                let authority = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(authority)))
                })?;
                let payer = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(payer)))
                })?;
                let system_program = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(system_program)))
                })?;
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
                let args = InitializeTipDistributionAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let tip_distribution_account = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(tip_distribution_account)
                    ))
                })?;
                let validator_vote_account = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(validator_vote_account)
                    ))
                })?;
                let signer = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(signer)))
                })?;
                let system_program = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(system_program)))
                })?;
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
                let args = MigrateTdaMerkleRootUploadAuthorityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let tip_distribution_account = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(tip_distribution_account)
                    ))
                })?;
                let merkle_root_upload_config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(merkle_root_upload_config)
                    ))
                })?;
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
                let args = UpdateConfigArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let authority = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(authority)))
                })?;
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
                let args = UpdateMerkleRootUploadConfigArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let merkle_root_upload_config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(merkle_root_upload_config)
                    ))
                })?;
                let authority = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(authority)))
                })?;
                let system_program = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(system_program)))
                })?;
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
                let args = UploadMerkleRootArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(config)))
                })?;
                let tip_distribution_account = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(tip_distribution_account)
                    ))
                })?;
                let merkle_root_upload_authority = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(merkle_root_upload_authority)
                    ))
                })?;
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
