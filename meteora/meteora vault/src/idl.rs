pub use accounts_data::*;
pub use ix_data::*;
#[allow(dead_code)]
use std::convert::TryInto;
pub use typedefs::*;
pub mod typedefs {
    use anchor_lang::prelude::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde::Serialize;
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct VaultBumps {
        pub vault_bump: u8,
        pub token_vault_bump: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct StrategyBumps {
        pub strategy_index: u8,
        pub other_bumps: [u8; 10usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LockedProfitTracker {
        pub last_updated_locked_profit: u64,
        pub last_report: u64,
        pub locked_profit_degradation: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum StrategyType {
        PortFinanceWithoutLm,
        PortFinanceWithLm,
        SolendWithoutLm,
        Mango,
        SolendWithLm,
        ApricotWithoutLm,
        Francium,
        Tulip,
        Vault,
        Drift,
        Frakt,
        Marginfi,
    }
    impl Default for StrategyType {
        fn default() -> Self {
            Self::PortFinanceWithoutLm
        }
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitializeAccounts {
        pub vault: String,
        pub payer: String,
        pub tokenVault: String,
        pub tokenMint: String,
        pub lpMint: String,
        pub rent: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct EnableVaultAccounts {
        pub vault: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetOperatorAccounts {
        pub vault: String,
        pub operator: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeStrategyAccounts {
        pub vault: String,
        pub strategyProgram: String,
        pub strategy: String,
        pub reserve: String,
        pub collateralVault: String,
        pub collateralMint: String,
        pub admin: String,
        pub systemProgram: String,
        pub rent: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveStrategyAccounts {
        pub vault: String,
        pub strategy: String,
        pub strategyProgram: String,
        pub collateralVault: String,
        pub reserve: String,
        pub tokenVault: String,
        pub feeVault: String,
        pub lpMint: String,
        pub tokenProgram: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveStrategy2Accounts {
        pub vault: String,
        pub strategy: String,
        pub strategyProgram: String,
        pub collateralVault: String,
        pub reserve: String,
        pub tokenVault: String,
        pub tokenAdminAdvancePayment: String,
        pub tokenVaultAdvancePayment: String,
        pub feeVault: String,
        pub lpMint: String,
        pub tokenProgram: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectDustAccounts {
        pub vault: String,
        pub tokenVault: String,
        pub tokenAdmin: String,
        pub admin: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddStrategyAccounts {
        pub vault: String,
        pub strategy: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositStrategyAccounts {
        pub vault: String,
        pub strategy: String,
        pub tokenVault: String,
        pub feeVault: String,
        pub lpMint: String,
        pub strategyProgram: String,
        pub collateralVault: String,
        pub reserve: String,
        pub tokenProgram: String,
        pub operator: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawStrategyAccounts {
        pub vault: String,
        pub strategy: String,
        pub tokenVault: String,
        pub feeVault: String,
        pub lpMint: String,
        pub strategyProgram: String,
        pub collateralVault: String,
        pub reserve: String,
        pub tokenProgram: String,
        pub operator: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct Withdraw2Accounts {
        pub vault: String,
        pub tokenVault: String,
        pub lpMint: String,
        pub userToken: String,
        pub userLp: String,
        pub user: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositAccounts {
        pub vault: String,
        pub tokenVault: String,
        pub lpMint: String,
        pub userToken: String,
        pub userLp: String,
        pub user: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawAccounts {
        pub vault: String,
        pub tokenVault: String,
        pub lpMint: String,
        pub userToken: String,
        pub userLp: String,
        pub user: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawDirectlyFromStrategyAccounts {
        pub vault: String,
        pub strategy: String,
        pub reserve: String,
        pub strategyProgram: String,
        pub collateralVault: String,
        pub tokenVault: String,
        pub lpMint: String,
        pub feeVault: String,
        pub userToken: String,
        pub userLp: String,
        pub user: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct EnableVaultArgs {
        pub enabled: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetOperatorArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeStrategyArgs {
        pub bumps: StrategyBumps,
        pub strategy_type: StrategyType,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveStrategyArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveStrategy2Args {
        pub max_admin_pay_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectDustArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddStrategyArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositStrategyArgs {
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawStrategyArgs {
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct Withdraw2Args {
        pub unmint_amount: u64,
        pub min_out_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositArgs {
        pub token_amount: u64,
        pub minimum_lp_token_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawArgs {
        pub unmint_amount: u64,
        pub min_out_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawDirectlyFromStrategyArgs {
        pub unmint_amount: u64,
        pub min_out_amount: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArgs,
    },
    EnableVault {
        accounts: EnableVaultAccounts,
        args: EnableVaultArgs,
    },
    SetOperator {
        accounts: SetOperatorAccounts,
        args: SetOperatorArgs,
    },
    InitializeStrategy {
        accounts: InitializeStrategyAccounts,
        args: InitializeStrategyArgs,
    },
    RemoveStrategy {
        accounts: RemoveStrategyAccounts,
        args: RemoveStrategyArgs,
    },
    RemoveStrategy2 {
        accounts: RemoveStrategy2Accounts,
        args: RemoveStrategy2Args,
    },
    CollectDust {
        accounts: CollectDustAccounts,
        args: CollectDustArgs,
    },
    AddStrategy {
        accounts: AddStrategyAccounts,
        args: AddStrategyArgs,
    },
    DepositStrategy {
        accounts: DepositStrategyAccounts,
        args: DepositStrategyArgs,
    },
    WithdrawStrategy {
        accounts: WithdrawStrategyAccounts,
        args: WithdrawStrategyArgs,
    },
    Withdraw2 {
        accounts: Withdraw2Accounts,
        args: Withdraw2Args,
    },
    Deposit {
        accounts: DepositAccounts,
        args: DepositArgs,
    },
    Withdraw {
        accounts: WithdrawAccounts,
        args: WithdrawArgs,
    },
    WithdrawDirectlyFromStrategy {
        accounts: WithdrawDirectlyFromStrategyAccounts,
        args: WithdrawDirectlyFromStrategyArgs,
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
            [175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let tokenVault = keys.next().unwrap().clone();
                let tokenMint = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    vault,
                    payer,
                    tokenVault,
                    tokenMint,
                    lpMint,
                    rent,
                    tokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            [145u8, 82u8, 241u8, 156u8, 26u8, 154u8, 233u8, 211u8] => {
                let mut rdr: &[u8] = rest;
                let args = EnableVaultArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EnableVaultAccounts {
                    vault,
                    admin,
                    remaining,
                };
                return Ok(Instruction::EnableVault { accounts, args });
            }
            [238u8, 153u8, 101u8, 169u8, 243u8, 131u8, 36u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetOperatorArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let operator = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetOperatorAccounts {
                    vault,
                    operator,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetOperator { accounts, args });
            }
            [208u8, 119u8, 144u8, 145u8, 178u8, 57u8, 105u8, 252u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeStrategyArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let strategyProgram = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let collateralVault = keys.next().unwrap().clone();
                let collateralMint = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeStrategyAccounts {
                    vault,
                    strategyProgram,
                    strategy,
                    reserve,
                    collateralVault,
                    collateralMint,
                    admin,
                    systemProgram,
                    rent,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeStrategy { accounts, args });
            }
            [185u8, 238u8, 33u8, 91u8, 134u8, 210u8, 97u8, 26u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveStrategyArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let strategyProgram = keys.next().unwrap().clone();
                let collateralVault = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let tokenVault = keys.next().unwrap().clone();
                let feeVault = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveStrategyAccounts {
                    vault,
                    strategy,
                    strategyProgram,
                    collateralVault,
                    reserve,
                    tokenVault,
                    feeVault,
                    lpMint,
                    tokenProgram,
                    admin,
                    remaining,
                };
                return Ok(Instruction::RemoveStrategy { accounts, args });
            }
            [138u8, 104u8, 208u8, 148u8, 126u8, 35u8, 195u8, 14u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveStrategy2Args::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let strategyProgram = keys.next().unwrap().clone();
                let collateralVault = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let tokenVault = keys.next().unwrap().clone();
                let tokenAdminAdvancePayment = keys.next().unwrap().clone();
                let tokenVaultAdvancePayment = keys.next().unwrap().clone();
                let feeVault = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveStrategy2Accounts {
                    vault,
                    strategy,
                    strategyProgram,
                    collateralVault,
                    reserve,
                    tokenVault,
                    tokenAdminAdvancePayment,
                    tokenVaultAdvancePayment,
                    feeVault,
                    lpMint,
                    tokenProgram,
                    admin,
                    remaining,
                };
                return Ok(Instruction::RemoveStrategy2 { accounts, args });
            }
            [246u8, 149u8, 21u8, 82u8, 160u8, 74u8, 254u8, 240u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectDustArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let tokenVault = keys.next().unwrap().clone();
                let tokenAdmin = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectDustAccounts {
                    vault,
                    tokenVault,
                    tokenAdmin,
                    admin,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::CollectDust { accounts, args });
            }
            [64u8, 123u8, 127u8, 227u8, 192u8, 234u8, 198u8, 20u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddStrategyArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddStrategyAccounts {
                    vault,
                    strategy,
                    admin,
                    remaining,
                };
                return Ok(Instruction::AddStrategy { accounts, args });
            }
            [246u8, 82u8, 57u8, 226u8, 131u8, 222u8, 253u8, 249u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositStrategyArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let tokenVault = keys.next().unwrap().clone();
                let feeVault = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let strategyProgram = keys.next().unwrap().clone();
                let collateralVault = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let operator = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositStrategyAccounts {
                    vault,
                    strategy,
                    tokenVault,
                    feeVault,
                    lpMint,
                    strategyProgram,
                    collateralVault,
                    reserve,
                    tokenProgram,
                    operator,
                    remaining,
                };
                return Ok(Instruction::DepositStrategy { accounts, args });
            }
            [31u8, 45u8, 162u8, 5u8, 193u8, 217u8, 134u8, 188u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawStrategyArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let tokenVault = keys.next().unwrap().clone();
                let feeVault = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let strategyProgram = keys.next().unwrap().clone();
                let collateralVault = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let operator = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawStrategyAccounts {
                    vault,
                    strategy,
                    tokenVault,
                    feeVault,
                    lpMint,
                    strategyProgram,
                    collateralVault,
                    reserve,
                    tokenProgram,
                    operator,
                    remaining,
                };
                return Ok(Instruction::WithdrawStrategy { accounts, args });
            }
            [80u8, 6u8, 111u8, 73u8, 174u8, 211u8, 66u8, 132u8] => {
                let mut rdr: &[u8] = rest;
                let args = Withdraw2Args::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let tokenVault = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let userToken = keys.next().unwrap().clone();
                let userLp = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = Withdraw2Accounts {
                    vault,
                    tokenVault,
                    lpMint,
                    userToken,
                    userLp,
                    user,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Withdraw2 { accounts, args });
            }
            [242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let tokenVault = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let userToken = keys.next().unwrap().clone();
                let userLp = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositAccounts {
                    vault,
                    tokenVault,
                    lpMint,
                    userToken,
                    userLp,
                    user,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Deposit { accounts, args });
            }
            [183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let tokenVault = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let userToken = keys.next().unwrap().clone();
                let userLp = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawAccounts {
                    vault,
                    tokenVault,
                    lpMint,
                    userToken,
                    userLp,
                    user,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Withdraw { accounts, args });
            }
            [201u8, 141u8, 146u8, 46u8, 173u8, 116u8, 198u8, 22u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawDirectlyFromStrategyArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let vault = keys.next().unwrap().clone();
                let strategy = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let strategyProgram = keys.next().unwrap().clone();
                let collateralVault = keys.next().unwrap().clone();
                let tokenVault = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let feeVault = keys.next().unwrap().clone();
                let userToken = keys.next().unwrap().clone();
                let userLp = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawDirectlyFromStrategyAccounts {
                    vault,
                    strategy,
                    reserve,
                    strategyProgram,
                    collateralVault,
                    tokenVault,
                    lpMint,
                    feeVault,
                    userToken,
                    userLp,
                    user,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawDirectlyFromStrategy { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
