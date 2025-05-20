use borsh::{BorshDeserialize, BorshSerialize};
use anyhow::{Error, anyhow};
use serde::Serialize;

// -----------------------------------------------------------------------------
// Inlined SPL token-swap layouts
// -----------------------------------------------------------------------------

/// All swap fees (64 bytes)
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq, Serialize)]
pub struct Fees {
    pub trade_fee_numerator:            u64,
    pub trade_fee_denominator:          u64,
    pub owner_trade_fee_numerator:      u64,
    pub owner_trade_fee_denominator:    u64,
    pub owner_withdraw_fee_numerator:   u64,
    pub owner_withdraw_fee_denominator: u64,
    pub host_fee_numerator:             u64,
    pub host_fee_denominator:           u64,
}

/// Curve type tag (1 byte)
#[repr(u8)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, PartialEq, Serialize)]
pub enum CurveType {
    ConstantProduct = 0,
    ConstantPrice   = 1,
    Offset          = 2,
}

/// SwapCurve wrapper: 1-byte tag + 32-byte calculator data
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
pub struct SwapCurve {
    pub curve_type: CurveType,
    pub calculator: [u8; 32],
}

// -----------------------------------------------------------------------------
// Argument structs (typedefs)
// -----------------------------------------------------------------------------

pub mod typedefs {
    use super::*;

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitializeArgs {
        pub fees: Fees,
        pub swap_curve: SwapCurve,
    }

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SwapArgs {
        pub amount_in: u64,
        pub minimum_amount_out: u64,
    }

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DepositAllTokenTypesArgs {
        pub pool_token_amount: u64,
        pub maximum_token_a_amount: u64,
        pub maximum_token_b_amount: u64,
    }

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WithdrawAllTokenTypesArgs {
        pub pool_token_amount: u64,
        pub minimum_token_a_amount: u64,
        pub minimum_token_b_amount: u64,
    }

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DepositSingleTokenTypeExactAmountInArgs {
        pub source_token_amount: u64,
        pub minimum_pool_token_amount: u64,
    }

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WithdrawSingleTokenTypeExactAmountOutArgs {
        pub destination_token_amount: u64,
        pub maximum_pool_token_amount: u64,
    }
}

// -----------------------------------------------------------------------------
// Accounts for each instruction (now String-based)
// -----------------------------------------------------------------------------

pub mod accounts_data {
    use super::*;

    #[derive(Debug, Serialize)]
    pub struct InitializeAccounts {
        pub swap: String,
        pub authority: String,
        pub token_a: String,
        pub token_b: String,
        pub pool: String,
        pub fee_account: String,
        pub destination: String,
        pub token_program: String,
    }

    #[derive(Debug, Serialize)]
    pub struct SwapAccounts {
        pub swap: String,
        pub authority: String,
        pub user_transfer_authority: String,
        pub source: String,
        pub swap_source: String,
        pub swap_destination: String,
        pub destination: String,
        pub pool_mint: String,
        pub pool_fee: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub source_program: String,
        pub destination_program: String,
        pub pool_token_program: String,
        pub host_fee_account: Option<String>,
    }

    #[derive(Debug, Serialize)]
    pub struct DepositAllTokenTypesAccounts {
        pub swap: String,
        pub authority: String,
        pub user_transfer_authority: String,
        pub deposit_token_a: String,
        pub deposit_token_b: String,
        pub swap_token_a: String,
        pub swap_token_b: String,
        pub pool_mint: String,
        pub destination: String,
        pub token_a_mint: String,
        pub token_b_mint: String,
        pub token_a_program: String,
        pub token_b_program: String,
        pub pool_token_program: String,
    }

    #[derive(Debug, Serialize)]
    pub struct WithdrawAllTokenTypesAccounts {
        pub swap: String,
        pub authority: String,
        pub user_transfer_authority: String,
        pub pool_mint: String,
        pub source_pool: String,
        pub swap_token_a: String,
        pub swap_token_b: String,
        pub destination_token_a: String,
        pub destination_token_b: String,
        pub fee_account: String,
        pub token_a_mint: String,
        pub token_b_mint: String,
        pub pool_token_program: String,
    }

    #[derive(Debug, Serialize)]
    pub struct DepositSingleTokenTypeExactAmountInAccounts {
        pub swap: String,
        pub authority: String,
        pub user_transfer_authority: String,
        pub source: String,
        pub swap_token_a: String,
        pub swap_token_b: String,
        pub pool_mint: String,
        pub destination: String,
        pub source_mint: String,
        pub source_program: String,
        pub pool_token_program: String,
    }

    #[derive(Debug, Serialize)]
    pub struct WithdrawSingleTokenTypeExactAmountOutAccounts {
        pub swap: String,
        pub authority: String,
        pub user_transfer_authority: String,
        pub pool_mint: String,
        pub source_pool: String,
        pub swap_token_a: String,
        pub swap_token_b: String,
        pub destination: String,
        pub fee_account: String,
        pub destination_mint: String,
        pub pool_token_program: String,
        pub destination_program: String,
    }
}

// -----------------------------------------------------------------------------
// Wrappers for Borsh deserialization of instruction data
// -----------------------------------------------------------------------------

pub mod ix_data {
    use super::typedefs::*;
    use borsh::BorshDeserialize;
    use serde::Serialize;

    #[derive(BorshDeserialize, Debug, Serialize)] pub struct InitializeIx { pub args: InitializeArgs }
    #[derive(BorshDeserialize, Debug, Serialize)] pub struct SwapIx     { pub args: SwapArgs }
    #[derive(BorshDeserialize, Debug, Serialize)] pub struct DepositAllTokenTypesIx { pub args: DepositAllTokenTypesArgs }
    #[derive(BorshDeserialize, Debug, Serialize)] pub struct WithdrawAllTokenTypesIx { pub args: WithdrawAllTokenTypesArgs }
    #[derive(BorshDeserialize, Debug, Serialize)] pub struct DepositSingleTokenTypeExactAmountInIx { pub args: DepositSingleTokenTypeExactAmountInArgs }
    #[derive(BorshDeserialize, Debug, Serialize)] pub struct WithdrawSingleTokenTypeExactAmountOutIx { pub args: WithdrawSingleTokenTypeExactAmountOutArgs }
}

// -----------------------------------------------------------------------------
// Main Instruction enum and decoder (accounts as Strings)
// -----------------------------------------------------------------------------

#[derive(Serialize, Debug)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Initialize { accounts: accounts_data::InitializeAccounts, args: typedefs::InitializeArgs },
    Swap       { accounts: accounts_data::SwapAccounts,       args: typedefs::SwapArgs },
    DepositAllTokenTypes { accounts: accounts_data::DepositAllTokenTypesAccounts, args: typedefs::DepositAllTokenTypesArgs },
    WithdrawAllTokenTypes { accounts: accounts_data::WithdrawAllTokenTypesAccounts, args: typedefs::WithdrawAllTokenTypesArgs },
    DepositSingleTokenTypeExactAmountIn { accounts: accounts_data::DepositSingleTokenTypeExactAmountInAccounts, args: typedefs::DepositSingleTokenTypeExactAmountInArgs },
    WithdrawSingleTokenTypeExactAmountOut { accounts: accounts_data::WithdrawSingleTokenTypeExactAmountOutAccounts, args: typedefs::WithdrawSingleTokenTypeExactAmountOutArgs },
}

impl Instruction {
    pub fn decode(account_keys: &[String], data: &[u8]) -> std::result::Result<Self, Error> {
        let (&tag, rest) = data.split_first().ok_or_else(|| anyhow!("Empty instruction data"))?;
        let mut keys = account_keys.iter();
        match tag {
            0 => {
                let args = typedefs::InitializeArgs::try_from_slice(rest)?;
                let accounts = accounts_data::InitializeAccounts {
                    swap: keys.next().unwrap().clone(),
                    authority: keys.next().unwrap().clone(),
                    token_a: keys.next().unwrap().clone(),
                    token_b: keys.next().unwrap().clone(),
                    pool: keys.next().unwrap().clone(),
                    fee_account: keys.next().unwrap().clone(),
                    destination: keys.next().unwrap().clone(),
                    token_program: keys.next().unwrap().clone(),
                };
                Ok(Instruction::Initialize { accounts, args })
            }
            1 => {
                let args = typedefs::SwapArgs::try_from_slice(rest)?;
                let accounts = accounts_data::SwapAccounts {
                    swap: keys.next().unwrap().clone(),
                    authority: keys.next().unwrap().clone(),
                    user_transfer_authority: keys.next().unwrap().clone(),
                    source: keys.next().unwrap().clone(),
                    swap_source: keys.next().unwrap().clone(),
                    swap_destination: keys.next().unwrap().clone(),
                    destination: keys.next().unwrap().clone(),
                    pool_mint: keys.next().unwrap().clone(),
                    pool_fee: keys.next().unwrap().clone(),
                    source_mint: keys.next().unwrap().clone(),
                    destination_mint: keys.next().unwrap().clone(),
                    source_program: keys.next().unwrap().clone(),
                    destination_program: keys.next().unwrap().clone(),
                    pool_token_program: keys.next().unwrap().clone(),
                    host_fee_account: keys.next().cloned(),
                };
                Ok(Instruction::Swap { accounts, args })
            }
            2 => {
                let args = typedefs::DepositAllTokenTypesArgs::try_from_slice(rest)?;
                let accounts = accounts_data::DepositAllTokenTypesAccounts {
                    swap: keys.next().unwrap().clone(), authority: keys.next().unwrap().clone(), user_transfer_authority: keys.next().unwrap().clone(), deposit_token_a: keys.next().unwrap().clone(), deposit_token_b: keys.next().unwrap().clone(), swap_token_a: keys.next().unwrap().clone(), swap_token_b: keys.next().unwrap().clone(), pool_mint: keys.next().unwrap().clone(), destination: keys.next().unwrap().clone(), token_a_mint: keys.next().unwrap().clone(), token_b_mint: keys.next().unwrap().clone(), token_a_program: keys.next().unwrap().clone(), token_b_program: keys.next().unwrap().clone(), pool_token_program: keys.next().unwrap().clone(),
                };
                Ok(Instruction::DepositAllTokenTypes { accounts, args })
            }
            3 => {
                let args = typedefs::WithdrawAllTokenTypesArgs::try_from_slice(rest)?;
                let accounts = accounts_data::WithdrawAllTokenTypesAccounts {
                    swap: keys.next().unwrap().clone(), authority: keys.next().unwrap().clone(), user_transfer_authority: keys.next().unwrap().clone(), pool_mint: keys.next().unwrap().clone(), source_pool: keys.next().unwrap().clone(), swap_token_a: keys.next().unwrap().clone(), swap_token_b: keys.next().unwrap().clone(), destination_token_a: keys.next().unwrap().clone(), destination_token_b: keys.next().unwrap().clone(), fee_account: keys.next().unwrap().clone(), token_a_mint: keys.next().unwrap().clone(), token_b_mint: keys.next().unwrap().clone(), pool_token_program: keys.next().unwrap().clone(),
                };
                Ok(Instruction::WithdrawAllTokenTypes { accounts, args })
            }
            4 => {
                let args = typedefs::DepositSingleTokenTypeExactAmountInArgs::try_from_slice(rest)?;
                let accounts = accounts_data::DepositSingleTokenTypeExactAmountInAccounts {
                    swap: keys.next().unwrap().clone(), authority: keys.next().unwrap().clone(), user_transfer_authority: keys.next().unwrap().clone(), source: keys.next().unwrap().clone(), swap_token_a: keys.next().unwrap().clone(), swap_token_b: keys.next().unwrap().clone(), pool_mint: keys.next().unwrap().clone(), destination: keys.next().unwrap().clone(), source_mint: keys.next().unwrap().clone(), source_program: keys.next().unwrap().clone(), pool_token_program: keys.next().unwrap().clone(),
                };
                Ok(Instruction::DepositSingleTokenTypeExactAmountIn { accounts, args })
            }
            5 => {
                let args = typedefs::WithdrawSingleTokenTypeExactAmountOutArgs::try_from_slice(rest)?;
                let accounts = accounts_data::WithdrawSingleTokenTypeExactAmountOutAccounts {
                    swap: keys.next().unwrap().clone(), authority: keys.next().unwrap().clone(), user_transfer_authority: keys.next().unwrap().clone(), pool_mint: keys.next().unwrap().clone(), source_pool: keys.next().unwrap().clone(), swap_token_a: keys.next().unwrap().clone(), swap_token_b: keys.next().unwrap().clone(), destination: keys.next().unwrap().clone(), fee_account: keys.next().unwrap().clone(), destination_mint: keys.next().unwrap().clone(), pool_token_program: keys.next().unwrap().clone(), destination_program: keys.next().unwrap().clone(),
                };
                Ok(Instruction::WithdrawSingleTokenTypeExactAmountOut { accounts, args })
            }
            _ => Err(anyhow!("Unknown instruction tag: {}", tag)),
        }
    }
}
