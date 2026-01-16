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
    pub struct CommissionBpsUpdated {
        #[serde(with = "pubkey_serde")]
        pub guardian_pool: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub authority: Pubkey,
        pub old_commission_bps: u16,
        pub new_commission_bps: u16,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct GuardianAuthorityUpdated {
        #[serde(with = "pubkey_serde")]
        pub guardian_pool: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub old_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub new_authority: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct GuardianCommissionClaimed {
        #[serde(with = "pubkey_serde")]
        pub guardian_pool: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cumulative_commission_per_share: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub claimed_amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct GuardianDelegationPool {
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub guardian: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub authority: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_shares: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cumulative_commission_per_share: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_share_price: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub accrued_commission: u128,
        pub commission_bps: u16,
        pub bump: u8,
        pub active: bool,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub deregistered_share_price: u128,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct GuardianDeregistered {
        #[serde(with = "pubkey_serde")]
        pub guardian_pool: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub guardian: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub deregistered_by: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct GuardianRegistered {
        #[serde(with = "pubkey_serde")]
        pub guardian_pool: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub guardian: Pubkey,
        pub commission_bps: u16,
        #[serde(with = "pubkey_serde")]
        pub registered_by: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InitializeStakeConfigArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_stake_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cooldown_seconds: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct NewStakeConfigAuthority {
        #[serde(with = "pubkey_serde")]
        pub old_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub new_authority: Pubkey,
        pub timestamp: i64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct RegisterGuardianArgs {
        pub commission_bps: u16,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct StakeArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct StakeConfig {
        pub bump: u8,
        #[serde(with = "pubkey_serde")]
        pub authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub stake_vault: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_stake_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cooldown_seconds: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub total_shares: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub share_price: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub commission_weight_sum: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cumulative_commission_per_share: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_vault_amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct StakeConfigInitialized {
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub stake_vault: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_stake_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cooldown_seconds: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct StakeConfigUpdated {
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub authority: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_stake_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cooldown_seconds: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Staked {
        #[serde(with = "pubkey_serde")]
        pub user_stake: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub guardian_pool: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub user: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub guardian: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub deposited_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub shares_minted: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub share_price: u128,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UnstakeArgs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub shares: u128,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UnstakeCancelled {
        #[serde(with = "pubkey_serde")]
        pub user_stake: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub guardian_pool: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub user: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub guardian: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub shares_restored: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub share_price: u128,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Unstaked {
        #[serde(with = "pubkey_serde")]
        pub user_stake: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub guardian_pool: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub user: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub guardian: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub shares_unstaked: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub share_price: u128,
        pub unstake_timestamp: i64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UpdateCommissionBpsArgs {
        pub new_commission_bps: u16,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UpdateGuardianAuthorityArgs {
        #[serde(with = "pubkey_serde")]
        pub new_authority: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UpdateStakeConfigArgs {
        pub min_stake_amount: Option<u64>,
        pub cooldown_seconds: Option<u64>,
        #[serde(with = "pubkey_serde_option")]
        pub authority: Option<Pubkey>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UserStake {
        pub bump: u8,
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub user: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub guardian_pool: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub shares: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cost_basis: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub cumulative_commission_before_staking: u128,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub unstaking_amount: u64,
        pub unstake_timestamp: i64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Withdrawn {
        #[serde(with = "pubkey_serde")]
        pub user_stake: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub stake_config: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub user: Pubkey,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub unstaking_amount: u64,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct CancelUnstakeAccounts {
        pub user_stake: String,
        pub stake_config: String,
        pub guardian_pool: String,
        pub user: String,
        pub stake_vault: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimGuardianCommissionAccounts {
        pub guardian_pool: String,
        pub stake_config: String,
        pub authority: String,
        pub stake_vault: String,
        pub destination: String,
        pub mint: String,
        pub token_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeregisterGuardianAccounts {
        pub guardian_pool: String,
        pub stake_config: String,
        pub authority: String,
        pub stake_vault: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeStakeConfigAccounts {
        pub stake_config: String,
        pub payer: String,
        pub authority: String,
        pub mint: String,
        pub stake_vault: String,
        pub system_program: String,
        pub token_program: String,
        pub rent: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RegisterGuardianAccounts {
        pub guardian_pool: String,
        pub stake_config: String,
        pub guardian: String,
        pub guardian_authority: String,
        pub payer: String,
        pub authority: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct StakeAccounts {
        pub user_stake: String,
        pub stake_config: String,
        pub guardian_pool: String,
        pub payer: String,
        pub user: String,
        pub user_token_account: String,
        pub stake_vault: String,
        pub mint: String,
        pub token_program: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UnstakeAccounts {
        pub user_stake: String,
        pub stake_config: String,
        pub guardian_pool: String,
        pub user: String,
        pub stake_vault: String,
        pub mint: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateCommissionBpsAccounts {
        pub guardian_pool: String,
        pub stake_config: String,
        pub authority: String,
        pub stake_vault: String,
        pub mint: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateGuardianAuthorityAccounts {
        pub guardian_pool: String,
        pub stake_config: String,
        pub authority: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateStakeConfigAccounts {
        pub stake_config: String,
        pub authority: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawAccounts {
        pub user_stake: String,
        pub stake_config: String,
        pub user: String,
        pub stake_vault: String,
        pub user_token_account: String,
        pub token_program: String,
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
    pub struct CancelUnstakeIxData {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimGuardianCommissionIxData {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeregisterGuardianIxData {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeStakeConfigIxData {
        pub args: typedefs::InitializeStakeConfigArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RegisterGuardianIxData {
        pub args: typedefs::RegisterGuardianArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct StakeIxData {
        pub args: typedefs::StakeArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnstakeIxData {
        pub args: typedefs::UnstakeArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateCommissionBpsIxData {
        pub args: typedefs::UpdateCommissionBpsArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGuardianAuthorityIxData {
        pub args: typedefs::UpdateGuardianAuthorityArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateStakeConfigIxData {
        pub args: typedefs::UpdateStakeConfigArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawIxData {}
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    CancelUnstake {
        accounts: CancelUnstakeAccounts,
        args: CancelUnstakeIxData,
    },
    ClaimGuardianCommission {
        accounts: ClaimGuardianCommissionAccounts,
        args: ClaimGuardianCommissionIxData,
    },
    DeregisterGuardian {
        accounts: DeregisterGuardianAccounts,
        args: DeregisterGuardianIxData,
    },
    InitializeStakeConfig {
        accounts: InitializeStakeConfigAccounts,
        args: InitializeStakeConfigIxData,
    },
    RegisterGuardian {
        accounts: RegisterGuardianAccounts,
        args: RegisterGuardianIxData,
    },
    Stake {
        accounts: StakeAccounts,
        args: StakeIxData,
    },
    Unstake {
        accounts: UnstakeAccounts,
        args: UnstakeIxData,
    },
    UpdateCommissionBps {
        accounts: UpdateCommissionBpsAccounts,
        args: UpdateCommissionBpsIxData,
    },
    UpdateGuardianAuthority {
        accounts: UpdateGuardianAuthorityAccounts,
        args: UpdateGuardianAuthorityIxData,
    },
    UpdateStakeConfig {
        accounts: UpdateStakeConfigAccounts,
        args: UpdateStakeConfigIxData,
    },
    Withdraw {
        accounts: WithdrawAccounts,
        args: WithdrawIxData,
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
            [64u8, 65u8, 53u8, 227u8, 125u8, 153u8, 3u8, 167u8] => {
                let mut rdr: &[u8] = rest;
                let args = CancelUnstakeIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let user_stake = keys.next().unwrap().clone();
                let stake_config = keys.next().unwrap().clone();
                let guardian_pool = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let stake_vault = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CancelUnstakeAccounts {
                    user_stake,
                    stake_config,
                    guardian_pool,
                    user,
                    stake_vault,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CancelUnstake { accounts, args });
            }
            [227u8, 163u8, 154u8, 139u8, 29u8, 195u8, 57u8, 85u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimGuardianCommissionIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let guardian_pool = keys.next().unwrap().clone();
                let stake_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let stake_vault = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimGuardianCommissionAccounts {
                    guardian_pool,
                    stake_config,
                    authority,
                    stake_vault,
                    destination,
                    mint,
                    token_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClaimGuardianCommission { accounts, args });
            }
            [93u8, 7u8, 130u8, 178u8, 136u8, 195u8, 142u8, 193u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeregisterGuardianIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let guardian_pool = keys.next().unwrap().clone();
                let stake_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let stake_vault = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeregisterGuardianAccounts {
                    guardian_pool,
                    stake_config,
                    authority,
                    stake_vault,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::DeregisterGuardian { accounts, args });
            }
            [196u8, 35u8, 131u8, 174u8, 172u8, 40u8, 233u8, 19u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeStakeConfigIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_config = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let stake_vault = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeStakeConfigAccounts {
                    stake_config,
                    payer,
                    authority,
                    mint,
                    stake_vault,
                    system_program,
                    token_program,
                    rent,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitializeStakeConfig { accounts, args });
            }
            [21u8, 206u8, 243u8, 116u8, 97u8, 100u8, 230u8, 59u8] => {
                let mut rdr: &[u8] = rest;
                let args = RegisterGuardianIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let guardian_pool = keys.next().unwrap().clone();
                let stake_config = keys.next().unwrap().clone();
                let guardian = keys.next().unwrap().clone();
                let guardian_authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RegisterGuardianAccounts {
                    guardian_pool,
                    stake_config,
                    guardian,
                    guardian_authority,
                    payer,
                    authority,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::RegisterGuardian { accounts, args });
            }
            [206u8, 176u8, 202u8, 18u8, 200u8, 209u8, 179u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let args = StakeIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let user_stake = keys.next().unwrap().clone();
                let stake_config = keys.next().unwrap().clone();
                let guardian_pool = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let user_token_account = keys.next().unwrap().clone();
                let stake_vault = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = StakeAccounts {
                    user_stake,
                    stake_config,
                    guardian_pool,
                    payer,
                    user,
                    user_token_account,
                    stake_vault,
                    mint,
                    token_program,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Stake { accounts, args });
            }
            [90u8, 95u8, 107u8, 42u8, 205u8, 124u8, 50u8, 225u8] => {
                let mut rdr: &[u8] = rest;
                let args = UnstakeIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let user_stake = keys.next().unwrap().clone();
                let stake_config = keys.next().unwrap().clone();
                let guardian_pool = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let stake_vault = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UnstakeAccounts {
                    user_stake,
                    stake_config,
                    guardian_pool,
                    user,
                    stake_vault,
                    mint,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Unstake { accounts, args });
            }
            [31u8, 213u8, 221u8, 71u8, 35u8, 97u8, 53u8, 160u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateCommissionBpsIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let guardian_pool = keys.next().unwrap().clone();
                let stake_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let stake_vault = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateCommissionBpsAccounts {
                    guardian_pool,
                    stake_config,
                    authority,
                    stake_vault,
                    mint,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdateCommissionBps { accounts, args });
            }
            [198u8, 241u8, 125u8, 25u8, 80u8, 170u8, 17u8, 3u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateGuardianAuthorityIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let guardian_pool = keys.next().unwrap().clone();
                let stake_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGuardianAuthorityAccounts {
                    guardian_pool,
                    stake_config,
                    authority,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdateGuardianAuthority { accounts, args });
            }
            [180u8, 108u8, 51u8, 29u8, 45u8, 168u8, 128u8, 134u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateStakeConfigIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateStakeConfigAccounts {
                    stake_config,
                    authority,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdateStakeConfig { accounts, args });
            }
            [183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawIxData::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let user_stake = keys.next().unwrap().clone();
                let stake_config = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let stake_vault = keys.next().unwrap().clone();
                let user_token_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawAccounts {
                    user_stake,
                    stake_config,
                    user,
                    stake_vault,
                    user_token_account,
                    token_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Withdraw { accounts, args });
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
        CommissionBpsUpdated { args: CommissionBpsUpdated },
        GuardianAuthorityUpdated { args: GuardianAuthorityUpdated },
        GuardianCommissionClaimed { args: GuardianCommissionClaimed },
        GuardianDeregistered { args: GuardianDeregistered },
        GuardianRegistered { args: GuardianRegistered },
        NewStakeConfigAuthority { args: NewStakeConfigAuthority },
        StakeConfigInitialized { args: StakeConfigInitialized },
        StakeConfigUpdated { args: StakeConfigUpdated },
        Staked { args: Staked },
        UnstakeCancelled { args: UnstakeCancelled },
        Unstaked { args: Unstaked },
        Withdrawn { args: Withdrawn },
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
                [20u8, 138u8, 78u8, 54u8, 244u8, 208u8, 239u8, 253u8] => {
                    let mut rdr = &data[16..];
                    let args = CommissionBpsUpdated::deserialize(&mut rdr)?;
                    return Ok(Event::CommissionBpsUpdated { args });
                }
                [35u8, 208u8, 55u8, 106u8, 221u8, 152u8, 200u8, 26u8] => {
                    let mut rdr = &data[16..];
                    let args = GuardianAuthorityUpdated::deserialize(&mut rdr)?;
                    return Ok(Event::GuardianAuthorityUpdated { args });
                }
                [113u8, 185u8, 128u8, 5u8, 32u8, 11u8, 110u8, 128u8] => {
                    let mut rdr = &data[16..];
                    let args = GuardianCommissionClaimed::deserialize(&mut rdr)?;
                    return Ok(Event::GuardianCommissionClaimed { args });
                }
                [165u8, 65u8, 219u8, 181u8, 219u8, 29u8, 140u8, 2u8] => {
                    let mut rdr = &data[16..];
                    let args = GuardianDeregistered::deserialize(&mut rdr)?;
                    return Ok(Event::GuardianDeregistered { args });
                }
                [255u8, 23u8, 109u8, 119u8, 11u8, 122u8, 209u8, 103u8] => {
                    let mut rdr = &data[16..];
                    let args = GuardianRegistered::deserialize(&mut rdr)?;
                    return Ok(Event::GuardianRegistered { args });
                }
                [35u8, 73u8, 90u8, 244u8, 147u8, 193u8, 193u8, 73u8] => {
                    let mut rdr = &data[16..];
                    let args = NewStakeConfigAuthority::deserialize(&mut rdr)?;
                    return Ok(Event::NewStakeConfigAuthority { args });
                }
                [48u8, 76u8, 161u8, 218u8, 120u8, 162u8, 115u8, 143u8] => {
                    let mut rdr = &data[16..];
                    let args = StakeConfigInitialized::deserialize(&mut rdr)?;
                    return Ok(Event::StakeConfigInitialized { args });
                }
                [177u8, 63u8, 174u8, 84u8, 146u8, 48u8, 110u8, 182u8] => {
                    let mut rdr = &data[16..];
                    let args = StakeConfigUpdated::deserialize(&mut rdr)?;
                    return Ok(Event::StakeConfigUpdated { args });
                }
                [11u8, 146u8, 45u8, 205u8, 230u8, 58u8, 213u8, 240u8] => {
                    let mut rdr = &data[16..];
                    let args = Staked::deserialize(&mut rdr)?;
                    return Ok(Event::Staked { args });
                }
                [102u8, 223u8, 189u8, 101u8, 201u8, 223u8, 180u8, 38u8] => {
                    let mut rdr = &data[16..];
                    let args = UnstakeCancelled::deserialize(&mut rdr)?;
                    return Ok(Event::UnstakeCancelled { args });
                }
                [27u8, 179u8, 156u8, 215u8, 47u8, 71u8, 195u8, 7u8] => {
                    let mut rdr = &data[16..];
                    let args = Unstaked::deserialize(&mut rdr)?;
                    return Ok(Event::Unstaked { args });
                }
                [20u8, 89u8, 223u8, 198u8, 194u8, 124u8, 219u8, 13u8] => {
                    let mut rdr = &data[16..];
                    let args = Withdrawn::deserialize(&mut rdr)?;
                    return Ok(Event::Withdrawn { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
