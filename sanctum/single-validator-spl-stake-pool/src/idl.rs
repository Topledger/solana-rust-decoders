extern crate serde;
pub use accounts_data::*;
#[allow(dead_code)]
use borsh::BorshDeserialize;
pub use ix_data::*;
use serde::Serialize;
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, serde :: Serialize)]
    pub struct InitializeAccounts {
        pub stake_pool: String,
        pub manager: String,
        pub staker: String,
        pub stake_pool_withdraw_authority: String,
        pub validator_list: String,
        pub reserve_stake: String,
        pub pool_mint: String,
        pub manager_pool_account: String,
        pub token_program: String,
        pub deposit_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct AddValidatorToPoolAccounts {
        pub stake_pool: String,
        pub staker: String,
        pub reserve_stake_account: String,
        pub stake_pool_withdraw: String,
        pub validator_list: String,
        pub stake: String,
        pub validator: String,
        pub sysvar_rent: String,
        pub sysvar_clock: String,
        pub sysvar_stake_history: String,
        pub sysvar_stake_config: String,
        pub system_program: String,
        pub stake_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct RemoveValidatorFromPoolAccounts {
        pub stake_pool: String,
        pub staker: String,
        pub stake_pool_withdraw: String,
        pub validator_list: String,
        pub stake_account: String,
        pub transient_stake_account: String,
        pub sysvar_clock: String,
        pub stake_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DecreaseValidatorStakeAccounts {
        pub stake_pool: String,
        pub staker: String,
        pub stake_pool_withdraw_authority: String,
        pub validator_list: String,
        pub canonical_stake_account: String,
        pub transient_stake_account: String,
        pub sysvar_clock: String,
        pub sysvar_rent: String,
        pub system_program: String,
        pub stake_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct IncreaseValidatorStakeAccounts {
        pub stake_pool: String,
        pub staker: String,
        pub stake_pool_withdraw_authority: String,
        pub validator_list: String,
        pub stake_pool_reserve_stake: String,
        pub transient_stake_account: String,
        pub validator_stake_account: String,
        pub validator_vote_account_to_delegate_to: String,
        pub sysvar_clock: String,
        pub sysvar_rent: String,
        pub stake_history_sysvar: String,
        pub stake_config_sysvar: String,
        pub system_program: String,
        pub stake_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SetPreferredValidatorAccounts {
        pub stake_pool_address: String,
        pub staker: String,
        pub validator_list: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateValidatorListBalanceAccounts {
        pub stake_pool: String,
        pub stake_pool_withdraw_authority: String,
        pub storage_account: String,
        pub reserve_stake_account: String,
        pub sysvar_clock: String,
        pub sysvar_stake_history: String,
        pub stake_program: String,
        pub validator_stake_account: String,
        pub transient_stake_account: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateStakePoolBalanceAccounts {
        pub stake_pool: String,
        pub stake_pool_withdraw_authority: String,
        pub validator_stake_list: String,
        pub reserve_stake_account: String,
        pub account_to_receive_pool_fee_tokens: String,
        pub pool_mint_account: String,
        pub pool_token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CleanupRemovedValidatorEntriesAccounts {
        pub stake_pool: String,
        pub validator_list_storage: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DepositStakeAccounts {
        pub stake_pool: String,
        pub validator_stake_list: String,
        pub stake_pool_deposit_authority: String,
        pub stake_pool_withdraw_authority: String,
        pub stake_account: String,
        pub validator_stake_account: String,
        pub reserve_stake_account: String,
        pub user_account: String,
        pub pool_tokens_amount: String,
        pub pool_fees_amount: String,
        pub pool_token_mint_account: String,
        pub sysvar_clock_account: String,
        pub sysvar_stake_history_account: String,
        pub pool_token_program_id: String,
        pub stake_program_id: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct WithdrawStakeAccounts {
        pub stake_pool: String,
        pub validator_stake_list: String,
        pub stake_pool_withdraw_authority: String,
        pub validator_account: String,
        pub uninitialized_stake_account: String,
        pub user_account: String,
        pub user_transfer_authority: String,
        pub user_account_with_pool_tokens_to_burn_from: String,
        pub account_to_receive_pool_fee_tokens: String,
        pub pool_token_mint_account: String,
        pub sysvar_clock_account: String,
        pub pool_token_program_id: String,
        pub stake_program_id: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SetManagerAccounts {
        pub stake_pool: String,
        pub manager: String,
        pub new_manager: String,
        pub new_fee_receiver: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SetFeeAccounts {
        pub stake_pool: String,
        pub manager: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SetStakerAccounts {
        pub stake_pool: String,
        pub current_staker: String,
        pub new_staker: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DepositSolAccounts {
        pub stake_pool: String,
        pub stake_pool_withdraw_authority: String,
        pub reserve_stake_account: String,
        pub depositer: String,
        pub user_account: String,
        pub fee_account: String,
        pub referral_fee_account: String,
        pub pool_token_mint: String,
        pub system_program: String,
        pub token_program_id: String,
        pub deposit_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct SetFundingAuthorityAccounts {
        pub stake_pool: String,
        pub manager: String,
        pub new_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct WithdrawSolAccounts {
        pub stake_pool: String,
        pub withdraw_authority: String,
        pub transfer_authority: String,
        pub burn_pool_tokens: String,
        pub reserve_stake_account: String,
        pub withdraw_account: String,
        pub fee_token_account: String,
        pub pool_token_mint: String,
        pub sysvar_clock: String,
        pub sysvar_stake_history: String,
        pub stake_program: String,
        pub token_program: String,
        pub sol_withdraw_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct CreateTokenMetadataAccounts {
        pub stake_pool: String,
        pub manager: String,
        pub stake_pool_withdraw_authority: String,
        pub pool_mint: String,
        pub payer: String,
        pub token_metadata: String,
        pub mpl_token_metadata: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct UpdateTokenMetadataAccounts {
        pub stake_pool: String,
        pub manager: String,
        pub stake_pool_withdraw_authority: String,
        pub token_metadata: String,
        pub mpl_token_metadata: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct IncreaseAdditionalValidatorStakeAccounts {
        pub stake_pool: String,
        pub staker: String,
        pub withdraw_authority: String,
        pub validator_list: String,
        pub reserve_stake_account: String,
        pub uninitialized_ephemeral_stake_account: String,
        pub transient_stake_account: String,
        pub validator_stake_account: String,
        pub validator_vote_account: String,
        pub clock_sysvar: String,
        pub stake_history_sysvar: String,
        pub stake_config_sysvar: String,
        pub system_program: String,
        pub stake_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DecreaseAdditionalValidatorStakeAccounts {
        pub stake_pool: String,
        pub staker: String,
        pub withdraw_authority: String,
        pub validator_list: String,
        pub source_canonical_stake_account: String,
        pub uninitialized_ephemeral_stake_account: String,
        pub transient_stake_account: String,
        pub clock_sysvar: String,
        pub stake_history_sysvar: String,
        pub system_program: String,
        pub stake_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct RedelegateAccounts {
        pub stake_pool: String,
        pub staker: String,
        pub withdraw_authority: String,
        pub validator_list: String,
        pub source_canonical_stake_account: String,
        pub source_transient_stake_account: String,
        pub uninitialized_ephemeral_stake_account: String,
        pub destination_transient_stake_account: String,
        pub destination_stake_account: String,
        pub destination_validator_vote_account: String,
        pub clock_sysvar: String,
        pub stake_history_sysvar: String,
        pub stake_config_sysvar: String,
        pub system_program: String,
        pub stake_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DepositStakeWithSlippageAccounts {
        pub stake_pool: String,
        pub validator_list: String,
        pub deposit_authority: String,
        pub withdraw_authority: String,
        pub stake_account: String,
        pub validator_stake_account: String,
        pub reserve_stake_account: String,
        pub pool_token_account: String,
        pub fee_token_account: String,
        pub referral_fee_account: String,
        pub pool_token_mint_account: String,
        pub sysvar_clock_account: String,
        pub sysvar_stake_history: String,
        pub pool_token_program: String,
        pub stake_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct WithdrawStakeWithSlippageAccounts {
        pub stake_pool: String,
        pub validator_list: String,
        pub withdraw_authority: String,
        pub stake_account: String,
        pub withdrawal_account: String,
        pub new_withdraw_authority: String,
        pub user_transfer_authority: String,
        pub burn_pool_tokens: String,
        pub fee_token_account: String,
        pub pool_token_mint: String,
        pub sysvar_clock_account: String,
        pub pool_token_program: String,
        pub stake_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct DepositSolWithSlippageAccounts {
        pub stake_pool: String,
        pub withdraw_authority: String,
        pub reserve_stake_account: String,
        pub deposit_account: String,
        pub pool_token_account: String,
        pub fee_token_account: String,
        pub referral_fee_account: String,
        pub pool_token_mint: String,
        pub system_program: String,
        pub token_program: String,
        pub sol_deposit_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, serde :: Serialize)]
    pub struct WithdrawSolWithSlippageAccounts {
        pub stake_pool: String,
        pub withdraw_authority: String,
        pub transfer_authority: String,
        pub pool_token_burn: String,
        pub reserve_stake_account: String,
        pub withdraw_account: String,
        pub fee_token_account: String,
        pub pool_token_mint: String,
        pub sysvar_clock: String,
        pub sysvar_stake_history: String,
        pub stake_program: String,
        pub token_program: String,
        pub sol_withdraw_authority: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct InitializeArguments {
        pub fee: Vec<u8>,
        pub withdrawal_fee: Vec<u8>,
        pub deposit_fee: Vec<u8>,
        pub referral_fee: u8,
        pub max_validators: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct AddValidatorToPoolArguments {
        pub seed: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct RemoveValidatorFromPoolArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct DecreaseValidatorStakeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub transient_stake_seed: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct IncreaseValidatorStakeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub transient_stake_seed: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct SetPreferredValidatorArguments {
        pub validator_type: Vec<u8>,
        pub validator_vote_address: Option<Vec<u8>>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct UpdateValidatorListBalanceArguments {
        pub start_index: u32,
        pub no_merge: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct UpdateStakePoolBalanceArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct CleanupRemovedValidatorEntriesArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct DepositStakeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct WithdrawStakeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct SetManagerArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct SetFeeArguments {
        pub fee: Vec<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct SetStakerArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct DepositSolArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub deposit_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct SetFundingAuthorityArguments {
        pub funding_type: Vec<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct WithdrawSolArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub arg: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct CreateTokenMetadataArguments {
        pub name: Vec<u8>,
        pub symbol: Vec<u8>,
        pub uri: Vec<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct UpdateTokenMetadataArguments {
        pub name: Vec<u8>,
        pub symbol: Vec<u8>,
        pub uri: Vec<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct IncreaseAdditionalValidatorStakeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub transient_stake_seed: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub ephemeral_stake_seed: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct DecreaseAdditionalValidatorStakeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub transient_stake_seed: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub ephemeral_stake_seed: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct RedelegateArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub source_transient_stake_seed: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub ephemeral_stake_seed: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub destination_transient_stake_seed: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct DepositStakeWithSlippageArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_pool_tokens_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct WithdrawStakeWithSlippageArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub pool_tokens_in: u64,
        pub minimum_lamports_out: Vec<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct DepositSolWithSlippageArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lamports_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_pools_tokens_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, serde :: Serialize)]
    pub struct WithdrawSolWithSlippageArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub pool_tokens_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_lamports_out: u64,
    }
}
#[derive(Debug, serde :: Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArguments,
    },
    AddValidatorToPool {
        accounts: AddValidatorToPoolAccounts,
        args: AddValidatorToPoolArguments,
    },
    RemoveValidatorFromPool {
        accounts: RemoveValidatorFromPoolAccounts,
        args: RemoveValidatorFromPoolArguments,
    },
    DecreaseValidatorStake {
        accounts: DecreaseValidatorStakeAccounts,
        args: DecreaseValidatorStakeArguments,
    },
    IncreaseValidatorStake {
        accounts: IncreaseValidatorStakeAccounts,
        args: IncreaseValidatorStakeArguments,
    },
    SetPreferredValidator {
        accounts: SetPreferredValidatorAccounts,
        args: SetPreferredValidatorArguments,
    },
    UpdateValidatorListBalance {
        accounts: UpdateValidatorListBalanceAccounts,
        args: UpdateValidatorListBalanceArguments,
    },
    UpdateStakePoolBalance {
        accounts: UpdateStakePoolBalanceAccounts,
        args: UpdateStakePoolBalanceArguments,
    },
    CleanupRemovedValidatorEntries {
        accounts: CleanupRemovedValidatorEntriesAccounts,
        args: CleanupRemovedValidatorEntriesArguments,
    },
    DepositStake {
        accounts: DepositStakeAccounts,
        args: DepositStakeArguments,
    },
    WithdrawStake {
        accounts: WithdrawStakeAccounts,
        args: WithdrawStakeArguments,
    },
    SetManager {
        accounts: SetManagerAccounts,
        args: SetManagerArguments,
    },
    SetFee {
        accounts: SetFeeAccounts,
        args: SetFeeArguments,
    },
    SetStaker {
        accounts: SetStakerAccounts,
        args: SetStakerArguments,
    },
    DepositSol {
        accounts: DepositSolAccounts,
        args: DepositSolArguments,
    },
    SetFundingAuthority {
        accounts: SetFundingAuthorityAccounts,
        args: SetFundingAuthorityArguments,
    },
    WithdrawSol {
        accounts: WithdrawSolAccounts,
        args: WithdrawSolArguments,
    },
    CreateTokenMetadata {
        accounts: CreateTokenMetadataAccounts,
        args: CreateTokenMetadataArguments,
    },
    UpdateTokenMetadata {
        accounts: UpdateTokenMetadataAccounts,
        args: UpdateTokenMetadataArguments,
    },
    IncreaseAdditionalValidatorStake {
        accounts: IncreaseAdditionalValidatorStakeAccounts,
        args: IncreaseAdditionalValidatorStakeArguments,
    },
    DecreaseAdditionalValidatorStake {
        accounts: DecreaseAdditionalValidatorStakeAccounts,
        args: DecreaseAdditionalValidatorStakeArguments,
    },
    Redelegate {
        accounts: RedelegateAccounts,
        args: RedelegateArguments,
    },
    DepositStakeWithSlippage {
        accounts: DepositStakeWithSlippageAccounts,
        args: DepositStakeWithSlippageArguments,
    },
    WithdrawStakeWithSlippage {
        accounts: WithdrawStakeWithSlippageAccounts,
        args: WithdrawStakeWithSlippageArguments,
    },
    DepositSolWithSlippage {
        accounts: DepositSolWithSlippageAccounts,
        args: DepositSolWithSlippageArguments,
    },
    WithdrawSolWithSlippage {
        accounts: WithdrawSolWithSlippageAccounts,
        args: WithdrawSolWithSlippageArguments,
    },
}
impl Instruction {
    pub fn decode(account_keys: &[String], data: &[u8]) -> anyhow::Result<Self> {
        if data.is_empty() {
            anyhow::bail!("Data is empty");
        }
        let (tag_byte, rest) = data.split_at(1);
        let tag = tag_byte[0];
        match tag {
            0u8 => {
                let mut rdr: &[u8] = rest;
                let args = InitializeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let manager = keys.next().unwrap_or(&"".to_string()).clone();
                let staker = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list = keys.next().unwrap_or(&"".to_string()).clone();
                let reserve_stake = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let manager_pool_account = keys.next().unwrap_or(&"".to_string()).clone();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let deposit_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    stake_pool,
                    manager,
                    staker,
                    stake_pool_withdraw_authority,
                    validator_list,
                    reserve_stake,
                    pool_mint,
                    manager_pool_account,
                    token_program,
                    deposit_authority,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            1u8 => {
                let mut rdr: &[u8] = rest;
                let args = AddValidatorToPoolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let staker = keys.next().unwrap_or(&"".to_string()).clone();
                let reserve_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list = keys.next().unwrap_or(&"".to_string()).clone();
                let stake = keys.next().unwrap_or(&"".to_string()).clone();
                let validator = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_rent = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_clock = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_stake_history = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_stake_config = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = AddValidatorToPoolAccounts {
                    stake_pool,
                    staker,
                    reserve_stake_account,
                    stake_pool_withdraw,
                    validator_list,
                    stake,
                    validator,
                    sysvar_rent,
                    sysvar_clock,
                    sysvar_stake_history,
                    sysvar_stake_config,
                    system_program,
                    stake_program,
                    remaining,
                };
                return Ok(Instruction::AddValidatorToPool { accounts, args });
            }
            2u8 => {
                let mut rdr: &[u8] = rest;
                let args = RemoveValidatorFromPoolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let staker = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let transient_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_clock = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveValidatorFromPoolAccounts {
                    stake_pool,
                    staker,
                    stake_pool_withdraw,
                    validator_list,
                    stake_account,
                    transient_stake_account,
                    sysvar_clock,
                    stake_program,
                    remaining,
                };
                return Ok(Instruction::RemoveValidatorFromPool { accounts, args });
            }
            3u8 => {
                let mut rdr: &[u8] = rest;
                let args = DecreaseValidatorStakeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let staker = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list = keys.next().unwrap_or(&"".to_string()).clone();
                let canonical_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let transient_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_clock = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_rent = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = DecreaseValidatorStakeAccounts {
                    stake_pool,
                    staker,
                    stake_pool_withdraw_authority,
                    validator_list,
                    canonical_stake_account,
                    transient_stake_account,
                    sysvar_clock,
                    sysvar_rent,
                    system_program,
                    stake_program,
                    remaining,
                };
                return Ok(Instruction::DecreaseValidatorStake { accounts, args });
            }
            4u8 => {
                let mut rdr: &[u8] = rest;
                let args = IncreaseValidatorStakeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let staker = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_reserve_stake = keys.next().unwrap_or(&"".to_string()).clone();
                let transient_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_vote_account_to_delegate_to =
                    keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_clock = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_rent = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_history_sysvar = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_config_sysvar = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = IncreaseValidatorStakeAccounts {
                    stake_pool,
                    staker,
                    stake_pool_withdraw_authority,
                    validator_list,
                    stake_pool_reserve_stake,
                    transient_stake_account,
                    validator_stake_account,
                    validator_vote_account_to_delegate_to,
                    sysvar_clock,
                    sysvar_rent,
                    stake_history_sysvar,
                    stake_config_sysvar,
                    system_program,
                    stake_program,
                    remaining,
                };
                return Ok(Instruction::IncreaseValidatorStake { accounts, args });
            }
            5u8 => {
                let mut rdr: &[u8] = rest;
                let args = SetPreferredValidatorArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool_address = keys.next().unwrap_or(&"".to_string()).clone();
                let staker = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SetPreferredValidatorAccounts {
                    stake_pool_address,
                    staker,
                    validator_list,
                    remaining,
                };
                return Ok(Instruction::SetPreferredValidator { accounts, args });
            }
            6u8 => {
                let mut rdr: &[u8] = rest;
                let args = UpdateValidatorListBalanceArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let storage_account = keys.next().unwrap_or(&"".to_string()).clone();
                let reserve_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_clock = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_stake_history = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let transient_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateValidatorListBalanceAccounts {
                    stake_pool,
                    stake_pool_withdraw_authority,
                    storage_account,
                    reserve_stake_account,
                    sysvar_clock,
                    sysvar_stake_history,
                    stake_program,
                    validator_stake_account,
                    transient_stake_account,
                    remaining,
                };
                return Ok(Instruction::UpdateValidatorListBalance { accounts, args });
            }
            7u8 => {
                let mut rdr: &[u8] = rest;
                let args = UpdateStakePoolBalanceArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_stake_list = keys.next().unwrap_or(&"".to_string()).clone();
                let reserve_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let account_to_receive_pool_fee_tokens =
                    keys.next().unwrap_or(&"".to_string()).clone();
                let pool_mint_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateStakePoolBalanceAccounts {
                    stake_pool,
                    stake_pool_withdraw_authority,
                    validator_stake_list,
                    reserve_stake_account,
                    account_to_receive_pool_fee_tokens,
                    pool_mint_account,
                    pool_token_program,
                    remaining,
                };
                return Ok(Instruction::UpdateStakePoolBalance { accounts, args });
            }
            8u8 => {
                let mut rdr: &[u8] = rest;
                let args = CleanupRemovedValidatorEntriesArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list_storage = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CleanupRemovedValidatorEntriesAccounts {
                    stake_pool,
                    validator_list_storage,
                    remaining,
                };
                return Ok(Instruction::CleanupRemovedValidatorEntries { accounts, args });
            }
            9u8 => {
                let mut rdr: &[u8] = rest;
                let args = DepositStakeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_stake_list = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_deposit_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let reserve_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let user_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_tokens_amount = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_fees_amount = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_mint_account = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_clock_account = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_stake_history_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_program_id = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program_id = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositStakeAccounts {
                    stake_pool,
                    validator_stake_list,
                    stake_pool_deposit_authority,
                    stake_pool_withdraw_authority,
                    stake_account,
                    validator_stake_account,
                    reserve_stake_account,
                    user_account,
                    pool_tokens_amount,
                    pool_fees_amount,
                    pool_token_mint_account,
                    sysvar_clock_account,
                    sysvar_stake_history_account,
                    pool_token_program_id,
                    stake_program_id,
                    remaining,
                };
                return Ok(Instruction::DepositStake { accounts, args });
            }
            10u8 => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawStakeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_stake_list = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_account = keys.next().unwrap_or(&"".to_string()).clone();
                let uninitialized_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let user_account = keys.next().unwrap_or(&"".to_string()).clone();
                let user_transfer_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let user_account_with_pool_tokens_to_burn_from =
                    keys.next().unwrap_or(&"".to_string()).clone();
                let account_to_receive_pool_fee_tokens =
                    keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_mint_account = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_clock_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_program_id = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program_id = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawStakeAccounts {
                    stake_pool,
                    validator_stake_list,
                    stake_pool_withdraw_authority,
                    validator_account,
                    uninitialized_stake_account,
                    user_account,
                    user_transfer_authority,
                    user_account_with_pool_tokens_to_burn_from,
                    account_to_receive_pool_fee_tokens,
                    pool_token_mint_account,
                    sysvar_clock_account,
                    pool_token_program_id,
                    stake_program_id,
                    remaining,
                };
                return Ok(Instruction::WithdrawStake { accounts, args });
            }
            11u8 => {
                let mut rdr: &[u8] = rest;
                let args = SetManagerArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let manager = keys.next().unwrap_or(&"".to_string()).clone();
                let new_manager = keys.next().unwrap_or(&"".to_string()).clone();
                let new_fee_receiver = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SetManagerAccounts {
                    stake_pool,
                    manager,
                    new_manager,
                    new_fee_receiver,
                    remaining,
                };
                return Ok(Instruction::SetManager { accounts, args });
            }
            12u8 => {
                let mut rdr: &[u8] = rest;
                let args = SetFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let manager = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SetFeeAccounts {
                    stake_pool,
                    manager,
                    remaining,
                };
                return Ok(Instruction::SetFee { accounts, args });
            }
            13u8 => {
                let mut rdr: &[u8] = rest;
                let args = SetStakerArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let current_staker = keys.next().unwrap_or(&"".to_string()).clone();
                let new_staker = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SetStakerAccounts {
                    stake_pool,
                    current_staker,
                    new_staker,
                    remaining,
                };
                return Ok(Instruction::SetStaker { accounts, args });
            }
            14u8 => {
                let mut rdr: &[u8] = rest;
                let args = DepositSolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let reserve_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let depositer = keys.next().unwrap_or(&"".to_string()).clone();
                let user_account = keys.next().unwrap_or(&"".to_string()).clone();
                let fee_account = keys.next().unwrap_or(&"".to_string()).clone();
                let referral_fee_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let token_program_id = keys.next().unwrap_or(&"".to_string()).clone();
                let deposit_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositSolAccounts {
                    stake_pool,
                    stake_pool_withdraw_authority,
                    reserve_stake_account,
                    depositer,
                    user_account,
                    fee_account,
                    referral_fee_account,
                    pool_token_mint,
                    system_program,
                    token_program_id,
                    deposit_authority,
                    remaining,
                };
                return Ok(Instruction::DepositSol { accounts, args });
            }
            15u8 => {
                let mut rdr: &[u8] = rest;
                let args = SetFundingAuthorityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let manager = keys.next().unwrap_or(&"".to_string()).clone();
                let new_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SetFundingAuthorityAccounts {
                    stake_pool,
                    manager,
                    new_authority,
                    remaining,
                };
                return Ok(Instruction::SetFundingAuthority { accounts, args });
            }
            16u8 => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawSolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let transfer_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let burn_pool_tokens = keys.next().unwrap_or(&"".to_string()).clone();
                let reserve_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let withdraw_account = keys.next().unwrap_or(&"".to_string()).clone();
                let fee_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_clock = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_stake_history = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let sol_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawSolAccounts {
                    stake_pool,
                    withdraw_authority,
                    transfer_authority,
                    burn_pool_tokens,
                    reserve_stake_account,
                    withdraw_account,
                    fee_token_account,
                    pool_token_mint,
                    sysvar_clock,
                    sysvar_stake_history,
                    stake_program,
                    token_program,
                    sol_withdraw_authority,
                    remaining,
                };
                return Ok(Instruction::WithdrawSol { accounts, args });
            }
            17u8 => {
                let mut rdr: &[u8] = rest;
                let args = CreateTokenMetadataArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let manager = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let payer = keys.next().unwrap_or(&"".to_string()).clone();
                let token_metadata = keys.next().unwrap_or(&"".to_string()).clone();
                let mpl_token_metadata = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateTokenMetadataAccounts {
                    stake_pool,
                    manager,
                    stake_pool_withdraw_authority,
                    pool_mint,
                    payer,
                    token_metadata,
                    mpl_token_metadata,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateTokenMetadata { accounts, args });
            }
            18u8 => {
                let mut rdr: &[u8] = rest;
                let args = UpdateTokenMetadataArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let manager = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_pool_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let token_metadata = keys.next().unwrap_or(&"".to_string()).clone();
                let mpl_token_metadata = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateTokenMetadataAccounts {
                    stake_pool,
                    manager,
                    stake_pool_withdraw_authority,
                    token_metadata,
                    mpl_token_metadata,
                    remaining,
                };
                return Ok(Instruction::UpdateTokenMetadata { accounts, args });
            }
            19u8 => {
                let mut rdr: &[u8] = rest;
                let args = IncreaseAdditionalValidatorStakeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let staker = keys.next().unwrap_or(&"".to_string()).clone();
                let withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list = keys.next().unwrap_or(&"".to_string()).clone();
                let reserve_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let uninitialized_ephemeral_stake_account =
                    keys.next().unwrap_or(&"".to_string()).clone();
                let transient_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_vote_account = keys.next().unwrap_or(&"".to_string()).clone();
                let clock_sysvar = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_history_sysvar = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_config_sysvar = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = IncreaseAdditionalValidatorStakeAccounts {
                    stake_pool,
                    staker,
                    withdraw_authority,
                    validator_list,
                    reserve_stake_account,
                    uninitialized_ephemeral_stake_account,
                    transient_stake_account,
                    validator_stake_account,
                    validator_vote_account,
                    clock_sysvar,
                    stake_history_sysvar,
                    stake_config_sysvar,
                    system_program,
                    stake_program,
                    remaining,
                };
                return Ok(Instruction::IncreaseAdditionalValidatorStake { accounts, args });
            }
            20u8 => {
                let mut rdr: &[u8] = rest;
                let args = DecreaseAdditionalValidatorStakeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let staker = keys.next().unwrap_or(&"".to_string()).clone();
                let withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list = keys.next().unwrap_or(&"".to_string()).clone();
                let source_canonical_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let uninitialized_ephemeral_stake_account =
                    keys.next().unwrap_or(&"".to_string()).clone();
                let transient_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let clock_sysvar = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_history_sysvar = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = DecreaseAdditionalValidatorStakeAccounts {
                    stake_pool,
                    staker,
                    withdraw_authority,
                    validator_list,
                    source_canonical_stake_account,
                    uninitialized_ephemeral_stake_account,
                    transient_stake_account,
                    clock_sysvar,
                    stake_history_sysvar,
                    system_program,
                    stake_program,
                    remaining,
                };
                return Ok(Instruction::DecreaseAdditionalValidatorStake { accounts, args });
            }
            21u8 => {
                let mut rdr: &[u8] = rest;
                let args = RedelegateArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let staker = keys.next().unwrap_or(&"".to_string()).clone();
                let withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list = keys.next().unwrap_or(&"".to_string()).clone();
                let source_canonical_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let source_transient_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let uninitialized_ephemeral_stake_account =
                    keys.next().unwrap_or(&"".to_string()).clone();
                let destination_transient_stake_account =
                    keys.next().unwrap_or(&"".to_string()).clone();
                let destination_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let destination_validator_vote_account =
                    keys.next().unwrap_or(&"".to_string()).clone();
                let clock_sysvar = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_history_sysvar = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_config_sysvar = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = RedelegateAccounts {
                    stake_pool,
                    staker,
                    withdraw_authority,
                    validator_list,
                    source_canonical_stake_account,
                    source_transient_stake_account,
                    uninitialized_ephemeral_stake_account,
                    destination_transient_stake_account,
                    destination_stake_account,
                    destination_validator_vote_account,
                    clock_sysvar,
                    stake_history_sysvar,
                    stake_config_sysvar,
                    system_program,
                    stake_program,
                    remaining,
                };
                return Ok(Instruction::Redelegate { accounts, args });
            }
            22u8 => {
                let mut rdr: &[u8] = rest;
                let args = DepositStakeWithSlippageArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list = keys.next().unwrap_or(&"".to_string()).clone();
                let deposit_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let reserve_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let fee_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let referral_fee_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_mint_account = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_clock_account = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_stake_history = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositStakeWithSlippageAccounts {
                    stake_pool,
                    validator_list,
                    deposit_authority,
                    withdraw_authority,
                    stake_account,
                    validator_stake_account,
                    reserve_stake_account,
                    pool_token_account,
                    fee_token_account,
                    referral_fee_account,
                    pool_token_mint_account,
                    sysvar_clock_account,
                    sysvar_stake_history,
                    pool_token_program,
                    stake_program,
                    remaining,
                };
                return Ok(Instruction::DepositStakeWithSlippage { accounts, args });
            }
            23u8 => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawStakeWithSlippageArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let validator_list = keys.next().unwrap_or(&"".to_string()).clone();
                let withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let withdrawal_account = keys.next().unwrap_or(&"".to_string()).clone();
                let new_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let user_transfer_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let burn_pool_tokens = keys.next().unwrap_or(&"".to_string()).clone();
                let fee_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_clock_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawStakeWithSlippageAccounts {
                    stake_pool,
                    validator_list,
                    withdraw_authority,
                    stake_account,
                    withdrawal_account,
                    new_withdraw_authority,
                    user_transfer_authority,
                    burn_pool_tokens,
                    fee_token_account,
                    pool_token_mint,
                    sysvar_clock_account,
                    pool_token_program,
                    stake_program,
                    remaining,
                };
                return Ok(Instruction::WithdrawStakeWithSlippage { accounts, args });
            }
            24u8 => {
                let mut rdr: &[u8] = rest;
                let args = DepositSolWithSlippageArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let reserve_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let deposit_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let fee_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let referral_fee_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let sol_deposit_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositSolWithSlippageAccounts {
                    stake_pool,
                    withdraw_authority,
                    reserve_stake_account,
                    deposit_account,
                    pool_token_account,
                    fee_token_account,
                    referral_fee_account,
                    pool_token_mint,
                    system_program,
                    token_program,
                    sol_deposit_authority,
                    remaining,
                };
                return Ok(Instruction::DepositSolWithSlippage { accounts, args });
            }
            25u8 => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawSolWithSlippageArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let stake_pool = keys.next().unwrap_or(&"".to_string()).clone();
                let withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let transfer_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_burn = keys.next().unwrap_or(&"".to_string()).clone();
                let reserve_stake_account = keys.next().unwrap_or(&"".to_string()).clone();
                let withdraw_account = keys.next().unwrap_or(&"".to_string()).clone();
                let fee_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_token_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_clock = keys.next().unwrap_or(&"".to_string()).clone();
                let sysvar_stake_history = keys.next().unwrap_or(&"".to_string()).clone();
                let stake_program = keys.next().unwrap_or(&"".to_string()).clone();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let sol_withdraw_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawSolWithSlippageAccounts {
                    stake_pool,
                    withdraw_authority,
                    transfer_authority,
                    pool_token_burn,
                    reserve_stake_account,
                    withdraw_account,
                    fee_token_account,
                    pool_token_mint,
                    sysvar_clock,
                    sysvar_stake_history,
                    stake_program,
                    token_program,
                    sol_withdraw_authority,
                    remaining,
                };
                return Ok(Instruction::WithdrawSolWithSlippage { accounts, args });
            }
            _ => anyhow::bail!("Unknown instruction tag: {}", tag),
        }
    }
}
pub mod events {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {}
    pub const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [0; 8];
    impl Event {
        pub fn decode(_data: &[u8]) -> anyhow::Result<Self> {
            anyhow::bail!("Event decoding not implemented for SPL Stake Pool")
        }
    }
}
