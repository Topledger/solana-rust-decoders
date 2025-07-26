extern crate serde;
pub use accounts::*;
use anchor_lang::prelude::*;
#[allow(dead_code)]
use borsh::BorshDeserialize;
pub use ix::*;
pub use typedefs::*;
pub mod typedefs {
    use anchor_lang::prelude::*;
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct AmmConfig {
        pub bump: u8,
        pub disable_create_pool: bool,
        pub index: u16,
        pub trade_fee_rate: u64,
        pub protocol_fee_rate: u64,
        pub fund_fee_rate: u64,
        pub create_pool_fee: u64,
        pub protocol_owner: Pubkey,
        pub fund_owner: Pubkey,
        pub padding: [u64; 16],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LpChangeEvent {
        pub pool_id: Pubkey,
        pub lp_amount_before: u64,
        pub token_0_vault_before: u64,
        pub token_1_vault_before: u64,
        pub token_0_amount: u64,
        pub token_1_amount: u64,
        pub token_0_transfer_fee: u64,
        pub token_1_transfer_fee: u64,
        pub change_type: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Observation {
        pub block_timestamp: u64,
        pub cumulative_token_0_price_x32: u128,
        pub cumulative_token_1_price_x32: u128,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
    pub struct ObservationState {
        pub initialized: bool,
        pub observation_index: u16,
        pub pool_id: Pubkey,
        pub observations: [Observation; 100],
        pub padding: [u64; 4],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PoolState {
        pub amm_config: Pubkey,
        pub pool_creator: Pubkey,
        pub token_0_vault: Pubkey,
        pub token_1_vault: Pubkey,
        pub lp_mint: Pubkey,
        pub token_0_mint: Pubkey,
        pub token_1_mint: Pubkey,
        pub token_0_program: Pubkey,
        pub token_1_program: Pubkey,
        pub observation_key: Pubkey,
        pub auth_bump: u8,
        pub status: u8,
        pub lp_mint_decimals: u8,
        pub mint_0_decimals: u8,
        pub mint_1_decimals: u8,
        pub lp_supply: u64,
        pub protocol_fees_token_0: u64,
        pub protocol_fees_token_1: u64,
        pub fund_fees_token_0: u64,
        pub fund_fees_token_1: u64,
        pub open_time: u64,
        pub recent_epoch: u64,
        pub padding: [u64; 31],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SwapEvent {
        pub pool_id: Pubkey,
        pub input_vault_before: u64,
        pub output_vault_before: u64,
        pub input_amount: u64,
        pub output_amount: u64,
        pub input_transfer_fee: u64,
        pub output_transfer_fee: u64,
        pub base_input: bool,
    }
}
pub mod accounts {
    #[derive(Debug)]
    pub struct CollectFundFeeAccounts {
        pub owner: String,
        pub authority: String,
        pub pool_state: String,
        pub amm_config: String,
        pub token_0_vault: String,
        pub token_1_vault: String,
        pub vault_0_mint: String,
        pub vault_1_mint: String,
        pub recipient_token_0_account: String,
        pub recipient_token_1_account: String,
        pub token_program: String,
        pub token_program_2022: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug)]
    pub struct CollectProtocolFeeAccounts {
        pub owner: String,
        pub authority: String,
        pub pool_state: String,
        pub amm_config: String,
        pub token_0_vault: String,
        pub token_1_vault: String,
        pub vault_0_mint: String,
        pub vault_1_mint: String,
        pub recipient_token_0_account: String,
        pub recipient_token_1_account: String,
        pub token_program: String,
        pub token_program_2022: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug)]
    pub struct CreateAmmConfigAccounts {
        pub owner: String,
        pub amm_config: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug)]
    pub struct DepositAccounts {
        pub owner: String,
        pub authority: String,
        pub pool_state: String,
        pub owner_lp_token: String,
        pub token_0_account: String,
        pub token_1_account: String,
        pub token_0_vault: String,
        pub token_1_vault: String,
        pub token_program: String,
        pub token_program_2022: String,
        pub vault_0_mint: String,
        pub vault_1_mint: String,
        pub lp_mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug)]
    pub struct InitializeAccounts {
        pub creator: String,
        pub amm_config: String,
        pub authority: String,
        pub pool_state: String,
        pub token_0_mint: String,
        pub token_1_mint: String,
        pub lp_mint: String,
        pub creator_token_0: String,
        pub creator_token_1: String,
        pub creator_lp_token: String,
        pub token_0_vault: String,
        pub token_1_vault: String,
        pub create_pool_fee: String,
        pub observation_state: String,
        pub token_program: String,
        pub token_0_program: String,
        pub token_1_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug)]
    pub struct SwapBaseInputAccounts {
        pub payer: String,
        pub authority: String,
        pub amm_config: String,
        pub pool_state: String,
        pub input_token_account: String,
        pub output_token_account: String,
        pub input_vault: String,
        pub output_vault: String,
        pub input_token_program: String,
        pub output_token_program: String,
        pub input_token_mint: String,
        pub output_token_mint: String,
        pub observation_state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug)]
    pub struct SwapBaseOutputAccounts {
        pub payer: String,
        pub authority: String,
        pub amm_config: String,
        pub pool_state: String,
        pub input_token_account: String,
        pub output_token_account: String,
        pub input_vault: String,
        pub output_vault: String,
        pub input_token_program: String,
        pub output_token_program: String,
        pub input_token_mint: String,
        pub output_token_mint: String,
        pub observation_state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug)]
    pub struct UpdateAmmConfigAccounts {
        pub owner: String,
        pub amm_config: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug)]
    pub struct UpdatePoolStatusAccounts {
        pub authority: String,
        pub pool_state: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug)]
    pub struct WithdrawAccounts {
        pub owner: String,
        pub authority: String,
        pub pool_state: String,
        pub owner_lp_token: String,
        pub token_0_account: String,
        pub token_1_account: String,
        pub token_0_vault: String,
        pub token_1_vault: String,
        pub token_program: String,
        pub token_program_2022: String,
        pub vault_0_mint: String,
        pub vault_1_mint: String,
        pub lp_mint: String,
        pub memo_program: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix {
    use super::*;
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CollectFundFeeArgs {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CollectProtocolFeeArgs {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CreateAmmConfigArgs {
        pub index: u16,
        pub trade_fee_rate: u64,
        pub protocol_fee_rate: u64,
        pub fund_fee_rate: u64,
        pub create_pool_fee: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct DepositArgs {
        pub lp_token_amount: u64,
        pub maximum_token_0_amount: u64,
        pub maximum_token_1_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct InitializeArgs {
        pub init_amount_0: u64,
        pub init_amount_1: u64,
        pub open_time: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct SwapBaseInputArgs {
        pub amount_in: u64,
        pub minimum_amount_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct SwapBaseOutputArgs {
        pub max_amount_in: u64,
        pub amount_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct UpdateAmmConfigArgs {
        pub param: u8,
        pub value: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct UpdatePoolStatusArgs {
        pub status: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct WithdrawArgs {
        pub lp_token_amount: u64,
        pub minimum_token_0_amount: u64,
        pub minimum_token_1_amount: u64,
    }
}
#[derive(Debug)]
pub enum Instruction {
    CollectFundFee {
        accounts: CollectFundFeeAccounts,
        args: CollectFundFeeArgs,
    },
    CollectProtocolFee {
        accounts: CollectProtocolFeeAccounts,
        args: CollectProtocolFeeArgs,
    },
    CreateAmmConfig {
        accounts: CreateAmmConfigAccounts,
        args: CreateAmmConfigArgs,
    },
    Deposit {
        accounts: DepositAccounts,
        args: DepositArgs,
    },
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArgs,
    },
    SwapBaseInput {
        accounts: SwapBaseInputAccounts,
        args: SwapBaseInputArgs,
    },
    SwapBaseOutput {
        accounts: SwapBaseOutputAccounts,
        args: SwapBaseOutputArgs,
    },
    UpdateAmmConfig {
        accounts: UpdateAmmConfigAccounts,
        args: UpdateAmmConfigArgs,
    },
    UpdatePoolStatus {
        accounts: UpdatePoolStatusAccounts,
        args: UpdatePoolStatusArgs,
    },
    Withdraw {
        accounts: WithdrawAccounts,
        args: WithdrawArgs,
    },
}
impl Instruction {
    pub fn decode(account_keys: &[String], data: &[u8]) -> anyhow::Result<Self> {
        if data.len() < 8 {
            anyhow::bail!("data too short");
        }
        let (disc, rest) = data.split_at(8);
        let disc: [u8; 8] = disc.try_into().unwrap();
        match disc {
            [167u8, 138u8, 78u8, 149u8, 223u8, 194u8, 6u8, 126u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectFundFeeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let token_0_vault = keys.next().unwrap().clone();
                let token_1_vault = keys.next().unwrap().clone();
                let vault_0_mint = keys.next().unwrap().clone();
                let vault_1_mint = keys.next().unwrap().clone();
                let recipient_token_0_account = keys.next().unwrap().clone();
                let recipient_token_1_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectFundFeeAccounts {
                    owner,
                    authority,
                    pool_state,
                    amm_config,
                    token_0_vault,
                    token_1_vault,
                    vault_0_mint,
                    vault_1_mint,
                    recipient_token_0_account,
                    recipient_token_1_account,
                    token_program,
                    token_program_2022,
                    remaining,
                };
                return Ok(Instruction::CollectFundFee { accounts, args });
            }
            [136u8, 136u8, 252u8, 221u8, 194u8, 66u8, 126u8, 89u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectProtocolFeeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let token_0_vault = keys.next().unwrap().clone();
                let token_1_vault = keys.next().unwrap().clone();
                let vault_0_mint = keys.next().unwrap().clone();
                let vault_1_mint = keys.next().unwrap().clone();
                let recipient_token_0_account = keys.next().unwrap().clone();
                let recipient_token_1_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectProtocolFeeAccounts {
                    owner,
                    authority,
                    pool_state,
                    amm_config,
                    token_0_vault,
                    token_1_vault,
                    vault_0_mint,
                    vault_1_mint,
                    recipient_token_0_account,
                    recipient_token_1_account,
                    token_program,
                    token_program_2022,
                    remaining,
                };
                return Ok(Instruction::CollectProtocolFee { accounts, args });
            }
            [137u8, 52u8, 237u8, 212u8, 215u8, 117u8, 108u8, 104u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateAmmConfigArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateAmmConfigAccounts {
                    owner,
                    amm_config,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateAmmConfig { accounts, args });
            }
            [242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let owner_lp_token = keys.next().unwrap().clone();
                let token_0_account = keys.next().unwrap().clone();
                let token_1_account = keys.next().unwrap().clone();
                let token_0_vault = keys.next().unwrap().clone();
                let token_1_vault = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let vault_0_mint = keys.next().unwrap().clone();
                let vault_1_mint = keys.next().unwrap().clone();
                let lp_mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositAccounts {
                    owner,
                    authority,
                    pool_state,
                    owner_lp_token,
                    token_0_account,
                    token_1_account,
                    token_0_vault,
                    token_1_vault,
                    token_program,
                    token_program_2022,
                    vault_0_mint,
                    vault_1_mint,
                    lp_mint,
                    remaining,
                };
                return Ok(Instruction::Deposit { accounts, args });
            }
            [175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let creator = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let token_0_mint = keys.next().unwrap().clone();
                let token_1_mint = keys.next().unwrap().clone();
                let lp_mint = keys.next().unwrap().clone();
                let creator_token_0 = keys.next().unwrap().clone();
                let creator_token_1 = keys.next().unwrap().clone();
                let creator_lp_token = keys.next().unwrap().clone();
                let token_0_vault = keys.next().unwrap().clone();
                let token_1_vault = keys.next().unwrap().clone();
                let create_pool_fee = keys.next().unwrap().clone();
                let observation_state = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_0_program = keys.next().unwrap().clone();
                let token_1_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    creator,
                    amm_config,
                    authority,
                    pool_state,
                    token_0_mint,
                    token_1_mint,
                    lp_mint,
                    creator_token_0,
                    creator_token_1,
                    creator_lp_token,
                    token_0_vault,
                    token_1_vault,
                    create_pool_fee,
                    observation_state,
                    token_program,
                    token_0_program,
                    token_1_program,
                    associated_token_program,
                    system_program,
                    rent,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            [143u8, 190u8, 90u8, 218u8, 196u8, 30u8, 51u8, 222u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapBaseInputArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let input_token_account = keys.next().unwrap().clone();
                let output_token_account = keys.next().unwrap().clone();
                let input_vault = keys.next().unwrap().clone();
                let output_vault = keys.next().unwrap().clone();
                let input_token_program = keys.next().unwrap().clone();
                let output_token_program = keys.next().unwrap().clone();
                let input_token_mint = keys.next().unwrap().clone();
                let output_token_mint = keys.next().unwrap().clone();
                let observation_state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapBaseInputAccounts {
                    payer,
                    authority,
                    amm_config,
                    pool_state,
                    input_token_account,
                    output_token_account,
                    input_vault,
                    output_vault,
                    input_token_program,
                    output_token_program,
                    input_token_mint,
                    output_token_mint,
                    observation_state,
                    remaining,
                };
                return Ok(Instruction::SwapBaseInput { accounts, args });
            }
            [55u8, 217u8, 98u8, 86u8, 163u8, 74u8, 180u8, 173u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapBaseOutputArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let input_token_account = keys.next().unwrap().clone();
                let output_token_account = keys.next().unwrap().clone();
                let input_vault = keys.next().unwrap().clone();
                let output_vault = keys.next().unwrap().clone();
                let input_token_program = keys.next().unwrap().clone();
                let output_token_program = keys.next().unwrap().clone();
                let input_token_mint = keys.next().unwrap().clone();
                let output_token_mint = keys.next().unwrap().clone();
                let observation_state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapBaseOutputAccounts {
                    payer,
                    authority,
                    amm_config,
                    pool_state,
                    input_token_account,
                    output_token_account,
                    input_vault,
                    output_vault,
                    input_token_program,
                    output_token_program,
                    input_token_mint,
                    output_token_mint,
                    observation_state,
                    remaining,
                };
                return Ok(Instruction::SwapBaseOutput { accounts, args });
            }
            [49u8, 60u8, 174u8, 136u8, 154u8, 28u8, 116u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateAmmConfigArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let amm_config = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateAmmConfigAccounts {
                    owner,
                    amm_config,
                    remaining,
                };
                return Ok(Instruction::UpdateAmmConfig { accounts, args });
            }
            [130u8, 87u8, 108u8, 6u8, 46u8, 224u8, 117u8, 123u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePoolStatusArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePoolStatusAccounts {
                    authority,
                    pool_state,
                    remaining,
                };
                return Ok(Instruction::UpdatePoolStatus { accounts, args });
            }
            [183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let pool_state = keys.next().unwrap().clone();
                let owner_lp_token = keys.next().unwrap().clone();
                let token_0_account = keys.next().unwrap().clone();
                let token_1_account = keys.next().unwrap().clone();
                let token_0_vault = keys.next().unwrap().clone();
                let token_1_vault = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let token_program_2022 = keys.next().unwrap().clone();
                let vault_0_mint = keys.next().unwrap().clone();
                let vault_1_mint = keys.next().unwrap().clone();
                let lp_mint = keys.next().unwrap().clone();
                let memo_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawAccounts {
                    owner,
                    authority,
                    pool_state,
                    owner_lp_token,
                    token_0_account,
                    token_1_account,
                    token_0_vault,
                    token_1_vault,
                    token_program,
                    token_program_2022,
                    vault_0_mint,
                    vault_1_mint,
                    lp_mint,
                    memo_program,
                    remaining,
                };
                return Ok(Instruction::Withdraw { accounts, args });
            }
            _ => anyhow::bail!("unknown discriminator: {:?}", disc),
        }
    }
}
