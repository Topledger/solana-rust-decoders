extern crate serde;
pub use accounts_data::*;
#[allow(dead_code)]
use borsh::BorshDeserialize;
pub use ix_data::*;
use serde::Serialize;
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitializeAccounts {
        pub token_program: String,
        pub system_program: String,
        pub rent: String,
        pub amm: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub lp_mint_address: String,
        pub coin_mint_address: String,
        pub pc_mint_address: String,
        pub pool_coin_token_account: String,
        pub pool_pc_token_account: String,
        pub pool_withdraw_queue: String,
        pub pool_target_orders_account: String,
        pub user_lp_token_account: String,
        pub pool_temp_lp_token_account: String,
        pub serum_program: String,
        pub serum_market: String,
        pub user_wallet: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct Initialize2Accounts {
        pub token_program: String,
        pub spl_associated_token_account: String,
        pub system_program: String,
        pub rent: String,
        pub amm: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub lp_mint: String,
        pub coin_mint: String,
        pub pc_mint: String,
        pub pool_coin_token_account: String,
        pub pool_pc_token_account: String,
        pub pool_withdraw_queue: String,
        pub amm_target_orders: String,
        pub pool_temp_lp: String,
        pub serum_program: String,
        pub serum_market: String,
        pub user_wallet: String,
        pub user_token_coin: String,
        pub user_token_pc: String,
        pub user_lp_token_account: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MonitorStepAccounts {
        pub token_program: String,
        pub rent: String,
        pub clock: String,
        pub amm: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub amm_target_orders: String,
        pub pool_coin_token_account: String,
        pub pool_pc_token_account: String,
        pub pool_withdraw_queue: String,
        pub serum_program: String,
        pub serum_market: String,
        pub serum_coin_vault_account: String,
        pub serum_pc_vault_account: String,
        pub serum_vault_signer: String,
        pub serum_req_q: String,
        pub serum_event_q: String,
        pub serum_bids: String,
        pub serum_asks: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositAccounts {
        pub token_program: String,
        pub amm: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub amm_target_orders: String,
        pub lp_mint_address: String,
        pub pool_coin_token_account: String,
        pub pool_pc_token_account: String,
        pub serum_market: String,
        pub user_coin_token_account: String,
        pub user_pc_token_account: String,
        pub user_lp_token_account: String,
        pub user_owner: String,
        pub serum_event_queue: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawAccounts {
        pub token_program: String,
        pub amm: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub amm_target_orders: String,
        pub lp_mint_address: String,
        pub pool_coin_token_account: String,
        pub pool_pc_token_account: String,
        pub pool_withdraw_queue: String,
        pub pool_temp_lp_token_account: String,
        pub serum_program: String,
        pub serum_market: String,
        pub serum_coin_vault_account: String,
        pub serum_pc_vault_account: String,
        pub serum_vault_signer: String,
        pub user_lp_token_account: String,
        pub uer_coin_token_account: String,
        pub uer_pc_token_account: String,
        pub user_owner: String,
        pub serum_event_q: String,
        pub serum_bids: String,
        pub serum_asks: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MigrateToOpenBookAccounts {
        pub token_program: String,
        pub system_program: String,
        pub rent: String,
        pub amm: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub amm_token_coin: String,
        pub amm_token_pc: String,
        pub amm_target_orders: String,
        pub serum_program: String,
        pub serum_market: String,
        pub serum_bids: String,
        pub serum_asks: String,
        pub serum_event_queue: String,
        pub serum_coin_vault: String,
        pub serum_pc_vault: String,
        pub serum_vault_signer: String,
        pub new_amm_open_orders: String,
        pub new_serum_program: String,
        pub new_serum_market: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetParamsAccounts {
        pub token_program: String,
        pub amm: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub amm_target_orders: String,
        pub amm_coin_vault: String,
        pub amm_pc_vault: String,
        pub serum_program: String,
        pub serum_market: String,
        pub serum_coin_vault: String,
        pub serum_pc_vault: String,
        pub serum_vault_signer: String,
        pub serum_event_queue: String,
        pub serum_bids: String,
        pub serum_asks: String,
        pub amm_admin_account: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawPnlAccounts {
        pub token_program: String,
        pub amm: String,
        pub amm_config: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub pool_coin_token_account: String,
        pub pool_pc_token_account: String,
        pub coin_pnl_token_account: String,
        pub pc_pnl_token_account: String,
        pub pnl_owner_account: String,
        pub amm_target_orders: String,
        pub serum_program: String,
        pub serum_market: String,
        pub serum_event_queue: String,
        pub serum_coin_vault_account: String,
        pub serum_pc_vault_account: String,
        pub serum_vault_signer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawSrmAccounts {
        pub token_program: String,
        pub amm: String,
        pub amm_owner_account: String,
        pub amm_authority: String,
        pub srm_token: String,
        pub dest_srm_token: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapBaseInAccounts {
        pub token_program: String,
        pub amm: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub amm_target_orders: String,
        pub pool_coin_token_account: String,
        pub pool_pc_token_account: String,
        pub serum_program: String,
        pub serum_market: String,
        pub serum_bids: String,
        pub serum_asks: String,
        pub serum_event_queue: String,
        pub serum_coin_vault_account: String,
        pub serum_pc_vault_account: String,
        pub serum_vault_signer: String,
        pub uer_source_token_account: String,
        pub uer_destination_token_account: String,
        pub user_source_owner: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PreInitializeAccounts {
        pub token_program: String,
        pub system_program: String,
        pub rent: String,
        pub amm_target_orders: String,
        pub pool_withdraw_queue: String,
        pub amm_authority: String,
        pub lp_mint_address: String,
        pub coin_mint_address: String,
        pub pc_mint_address: String,
        pub pool_coin_token_account: String,
        pub pool_pc_token_account: String,
        pub pool_temp_lp_token_account: String,
        pub serum_market: String,
        pub user_wallet: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapBaseOutAccounts {
        pub token_program: String,
        pub amm: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub amm_target_orders: String,
        pub pool_coin_token_account: String,
        pub pool_pc_token_account: String,
        pub serum_program: String,
        pub serum_market: String,
        pub serum_bids: String,
        pub serum_asks: String,
        pub serum_event_queue: String,
        pub serum_coin_vault_account: String,
        pub serum_pc_vault_account: String,
        pub serum_vault_signer: String,
        pub uer_source_token_account: String,
        pub uer_destination_token_account: String,
        pub user_source_owner: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SimulateInfoAccounts {
        pub amm: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub pool_coin_token_account: String,
        pub pool_pc_token_account: String,
        pub lp_mint_address: String,
        pub serum_market: String,
        pub serum_event_queue: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AdminCancelOrdersAccounts {
        pub token_program: String,
        pub amm: String,
        pub amm_authority: String,
        pub amm_open_orders: String,
        pub amm_target_orders: String,
        pub pool_coin_token_account: String,
        pub pool_pc_token_account: String,
        pub amm_owner_account: String,
        pub amm_config: String,
        pub serum_program: String,
        pub serum_market: String,
        pub serum_coin_vault_account: String,
        pub serum_pc_vault_account: String,
        pub serum_vault_signer: String,
        pub serum_event_q: String,
        pub serum_bids: String,
        pub serum_asks: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateConfigAccountAccounts {
        pub admin: String,
        pub amm_config: String,
        pub owner: String,
        pub system_program: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateConfigAccountAccounts {
        pub admin: String,
        pub amm_config: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct InitializeArguments {
        pub nonce: u8,
        pub open_time: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct Initialize2Arguments {
        pub nonce: u8,
        pub open_time: u64,
        pub init_pc_amount: u64,
        pub init_coin_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct MonitorStepArguments {
        pub plan_order_limit: u16,
        pub place_order_limit: u16,
        pub cancel_order_limit: u16,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct DepositArguments {
        pub max_coin_amount: u64,
        pub max_pc_amount: u64,
        pub base_side: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct WithdrawArguments {
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct MigrateToOpenBookArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct SetParamsArguments {
        pub param: u8,
        pub value: Option<Vec<u8>>,
        pub new_pubkey: Option<Vec<u8>>,
        pub fees: Option<Vec<u8>>,
        pub last_order_distance: Option<Vec<u8>>,
        pub need_take_amounts: Option<Vec<u8>>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct WithdrawPnlArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct WithdrawSrmArguments {
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct SwapBaseInArguments {
        pub amount_in: u64,
        pub minimum_amount_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct PreInitializeArguments {
        pub nonce: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct SwapBaseOutArguments {
        pub max_amount_in: u64,
        pub amount_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct SimulateInfoArguments {
        pub param: u8,
        pub swap_base_in_value: Option<Vec<u8>>,
        pub swap_base_out_value: Option<Vec<u8>>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct AdminCancelOrdersArguments {
        pub limit: u16,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct CreateConfigAccountArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug)]
    pub struct UpdateConfigAccountArguments {
        pub param: u8,
        pub owner: [u8; 32],
    }
}
#[derive(Debug)]
pub enum Instruction {
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArguments,
    },
    Initialize2 {
        accounts: Initialize2Accounts,
        args: Initialize2Arguments,
    },
    MonitorStep {
        accounts: MonitorStepAccounts,
        args: MonitorStepArguments,
    },
    Deposit {
        accounts: DepositAccounts,
        args: DepositArguments,
    },
    Withdraw {
        accounts: WithdrawAccounts,
        args: WithdrawArguments,
    },
    MigrateToOpenBook {
        accounts: MigrateToOpenBookAccounts,
        args: MigrateToOpenBookArguments,
    },
    SetParams {
        accounts: SetParamsAccounts,
        args: SetParamsArguments,
    },
    WithdrawPnl {
        accounts: WithdrawPnlAccounts,
        args: WithdrawPnlArguments,
    },
    WithdrawSrm {
        accounts: WithdrawSrmAccounts,
        args: WithdrawSrmArguments,
    },
    SwapBaseIn {
        accounts: SwapBaseInAccounts,
        args: SwapBaseInArguments,
    },
    PreInitialize {
        accounts: PreInitializeAccounts,
        args: PreInitializeArguments,
    },
    SwapBaseOut {
        accounts: SwapBaseOutAccounts,
        args: SwapBaseOutArguments,
    },
    SimulateInfo {
        accounts: SimulateInfoAccounts,
        args: SimulateInfoArguments,
    },
    AdminCancelOrders {
        accounts: AdminCancelOrdersAccounts,
        args: AdminCancelOrdersArguments,
    },
    CreateConfigAccount {
        accounts: CreateConfigAccountAccounts,
        args: CreateConfigAccountArguments,
    },
    UpdateConfigAccount {
        accounts: UpdateConfigAccountAccounts,
        args: UpdateConfigAccountArguments,
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
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let rent = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let lp_mint_address = keys.next().unwrap_or(&"".to_string()).clone();
                let coin_mint_address = keys.next().unwrap_or(&"".to_string()).clone();
                let pc_mint_address = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_withdraw_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_target_orders_account = keys.next().unwrap_or(&"".to_string()).clone();
                let user_lp_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_temp_lp_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_program = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let user_wallet = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    token_program,
                    system_program,
                    rent,
                    amm,
                    amm_authority,
                    amm_open_orders,
                    lp_mint_address,
                    coin_mint_address,
                    pc_mint_address,
                    pool_coin_token_account,
                    pool_pc_token_account,
                    pool_withdraw_queue,
                    pool_target_orders_account,
                    user_lp_token_account,
                    pool_temp_lp_token_account,
                    serum_program,
                    serum_market,
                    user_wallet,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            1u8 => {
                let mut rdr: &[u8] = rest;
                let args = Initialize2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let spl_associated_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let rent = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let lp_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let coin_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let pc_mint = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_withdraw_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_temp_lp = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_program = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let user_wallet = keys.next().unwrap_or(&"".to_string()).clone();
                let user_token_coin = keys.next().unwrap_or(&"".to_string()).clone();
                let user_token_pc = keys.next().unwrap_or(&"".to_string()).clone();
                let user_lp_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = Initialize2Accounts {
                    token_program,
                    spl_associated_token_account,
                    system_program,
                    rent,
                    amm,
                    amm_authority,
                    amm_open_orders,
                    lp_mint,
                    coin_mint,
                    pc_mint,
                    pool_coin_token_account,
                    pool_pc_token_account,
                    pool_withdraw_queue,
                    amm_target_orders,
                    pool_temp_lp,
                    serum_program,
                    serum_market,
                    user_wallet,
                    user_token_coin,
                    user_token_pc,
                    user_lp_token_account,
                    remaining,
                };
                return Ok(Instruction::Initialize2 { accounts, args });
            }
            2u8 => {
                let mut rdr: &[u8] = rest;
                let args = MonitorStepArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let rent = keys.next().unwrap_or(&"".to_string()).clone();
                let clock = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_withdraw_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_program = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_coin_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_pc_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_vault_signer = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_req_q = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_event_q = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_bids = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_asks = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = MonitorStepAccounts {
                    token_program,
                    rent,
                    clock,
                    amm,
                    amm_authority,
                    amm_open_orders,
                    amm_target_orders,
                    pool_coin_token_account,
                    pool_pc_token_account,
                    pool_withdraw_queue,
                    serum_program,
                    serum_market,
                    serum_coin_vault_account,
                    serum_pc_vault_account,
                    serum_vault_signer,
                    serum_req_q,
                    serum_event_q,
                    serum_bids,
                    serum_asks,
                    remaining,
                };
                return Ok(Instruction::MonitorStep { accounts, args });
            }
            3u8 => {
                let mut rdr: &[u8] = rest;
                let args = DepositArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let lp_mint_address = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let user_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let user_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let user_lp_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let user_owner = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_event_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositAccounts {
                    token_program,
                    amm,
                    amm_authority,
                    amm_open_orders,
                    amm_target_orders,
                    lp_mint_address,
                    pool_coin_token_account,
                    pool_pc_token_account,
                    serum_market,
                    user_coin_token_account,
                    user_pc_token_account,
                    user_lp_token_account,
                    user_owner,
                    serum_event_queue,
                    remaining,
                };
                return Ok(Instruction::Deposit { accounts, args });
            }
            4u8 => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let lp_mint_address = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_withdraw_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_temp_lp_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_program = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_coin_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_pc_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_vault_signer = keys.next().unwrap_or(&"".to_string()).clone();
                let user_lp_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let uer_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let uer_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let user_owner = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_event_q = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_bids = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_asks = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawAccounts {
                    token_program,
                    amm,
                    amm_authority,
                    amm_open_orders,
                    amm_target_orders,
                    lp_mint_address,
                    pool_coin_token_account,
                    pool_pc_token_account,
                    pool_withdraw_queue,
                    pool_temp_lp_token_account,
                    serum_program,
                    serum_market,
                    serum_coin_vault_account,
                    serum_pc_vault_account,
                    serum_vault_signer,
                    user_lp_token_account,
                    uer_coin_token_account,
                    uer_pc_token_account,
                    user_owner,
                    serum_event_q,
                    serum_bids,
                    serum_asks,
                    remaining,
                };
                return Ok(Instruction::Withdraw { accounts, args });
            }
            5u8 => {
                let mut rdr: &[u8] = rest;
                let args = MigrateToOpenBookArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let rent = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_token_coin = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_token_pc = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_program = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_bids = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_asks = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_event_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_coin_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_pc_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_vault_signer = keys.next().unwrap_or(&"".to_string()).clone();
                let new_amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let new_serum_program = keys.next().unwrap_or(&"".to_string()).clone();
                let new_serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let admin = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = MigrateToOpenBookAccounts {
                    token_program,
                    system_program,
                    rent,
                    amm,
                    amm_authority,
                    amm_open_orders,
                    amm_token_coin,
                    amm_token_pc,
                    amm_target_orders,
                    serum_program,
                    serum_market,
                    serum_bids,
                    serum_asks,
                    serum_event_queue,
                    serum_coin_vault,
                    serum_pc_vault,
                    serum_vault_signer,
                    new_amm_open_orders,
                    new_serum_program,
                    new_serum_market,
                    admin,
                    remaining,
                };
                return Ok(Instruction::MigrateToOpenBook { accounts, args });
            }
            6u8 => {
                let mut rdr: &[u8] = rest;
                let args = SetParamsArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_coin_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_pc_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_program = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_coin_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_pc_vault = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_vault_signer = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_event_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_bids = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_asks = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_admin_account = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SetParamsAccounts {
                    token_program,
                    amm,
                    amm_authority,
                    amm_open_orders,
                    amm_target_orders,
                    amm_coin_vault,
                    amm_pc_vault,
                    serum_program,
                    serum_market,
                    serum_coin_vault,
                    serum_pc_vault,
                    serum_vault_signer,
                    serum_event_queue,
                    serum_bids,
                    serum_asks,
                    amm_admin_account,
                    remaining,
                };
                return Ok(Instruction::SetParams { accounts, args });
            }
            7u8 => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawPnlArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_config = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let coin_pnl_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pc_pnl_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pnl_owner_account = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_program = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_event_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_coin_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_pc_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_vault_signer = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawPnlAccounts {
                    token_program,
                    amm,
                    amm_config,
                    amm_authority,
                    amm_open_orders,
                    pool_coin_token_account,
                    pool_pc_token_account,
                    coin_pnl_token_account,
                    pc_pnl_token_account,
                    pnl_owner_account,
                    amm_target_orders,
                    serum_program,
                    serum_market,
                    serum_event_queue,
                    serum_coin_vault_account,
                    serum_pc_vault_account,
                    serum_vault_signer,
                    remaining,
                };
                return Ok(Instruction::WithdrawPnl { accounts, args });
            }
            8u8 => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawSrmArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_owner_account = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let srm_token = keys.next().unwrap_or(&"".to_string()).clone();
                let dest_srm_token = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawSrmAccounts {
                    token_program,
                    amm,
                    amm_owner_account,
                    amm_authority,
                    srm_token,
                    dest_srm_token,
                    remaining,
                };
                return Ok(Instruction::WithdrawSrm { accounts, args });
            }
            9u8 => {
                let mut rdr: &[u8] = rest;
                let args = SwapBaseInArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_program = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_bids = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_asks = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_event_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_coin_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_pc_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_vault_signer = keys.next().unwrap_or(&"".to_string()).clone();
                let uer_source_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let uer_destination_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let user_source_owner = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapBaseInAccounts {
                    token_program,
                    amm,
                    amm_authority,
                    amm_open_orders,
                    amm_target_orders,
                    pool_coin_token_account,
                    pool_pc_token_account,
                    serum_program,
                    serum_market,
                    serum_bids,
                    serum_asks,
                    serum_event_queue,
                    serum_coin_vault_account,
                    serum_pc_vault_account,
                    serum_vault_signer,
                    uer_source_token_account,
                    uer_destination_token_account,
                    user_source_owner,
                    remaining,
                };
                return Ok(Instruction::SwapBaseIn { accounts, args });
            }
            10u8 => {
                let mut rdr: &[u8] = rest;
                let args = PreInitializeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let rent = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_withdraw_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let lp_mint_address = keys.next().unwrap_or(&"".to_string()).clone();
                let coin_mint_address = keys.next().unwrap_or(&"".to_string()).clone();
                let pc_mint_address = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_temp_lp_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let user_wallet = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = PreInitializeAccounts {
                    token_program,
                    system_program,
                    rent,
                    amm_target_orders,
                    pool_withdraw_queue,
                    amm_authority,
                    lp_mint_address,
                    coin_mint_address,
                    pc_mint_address,
                    pool_coin_token_account,
                    pool_pc_token_account,
                    pool_temp_lp_token_account,
                    serum_market,
                    user_wallet,
                    remaining,
                };
                return Ok(Instruction::PreInitialize { accounts, args });
            }
            11u8 => {
                let mut rdr: &[u8] = rest;
                let args = SwapBaseOutArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_program = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_bids = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_asks = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_event_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_coin_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_pc_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_vault_signer = keys.next().unwrap_or(&"".to_string()).clone();
                let uer_source_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let uer_destination_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let user_source_owner = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapBaseOutAccounts {
                    token_program,
                    amm,
                    amm_authority,
                    amm_open_orders,
                    amm_target_orders,
                    pool_coin_token_account,
                    pool_pc_token_account,
                    serum_program,
                    serum_market,
                    serum_bids,
                    serum_asks,
                    serum_event_queue,
                    serum_coin_vault_account,
                    serum_pc_vault_account,
                    serum_vault_signer,
                    uer_source_token_account,
                    uer_destination_token_account,
                    user_source_owner,
                    remaining,
                };
                return Ok(Instruction::SwapBaseOut { accounts, args });
            }
            12u8 => {
                let mut rdr: &[u8] = rest;
                let args = SimulateInfoArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let lp_mint_address = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_event_queue = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SimulateInfoAccounts {
                    amm,
                    amm_authority,
                    amm_open_orders,
                    pool_coin_token_account,
                    pool_pc_token_account,
                    lp_mint_address,
                    serum_market,
                    serum_event_queue,
                    remaining,
                };
                return Ok(Instruction::SimulateInfo { accounts, args });
            }
            13u8 => {
                let mut rdr: &[u8] = rest;
                let args = AdminCancelOrdersArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap_or(&"".to_string()).clone();
                let amm = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_authority = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_open_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_target_orders = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_coin_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let pool_pc_token_account = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_owner_account = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_config = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_program = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_market = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_coin_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_pc_vault_account = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_vault_signer = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_event_q = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_bids = keys.next().unwrap_or(&"".to_string()).clone();
                let serum_asks = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = AdminCancelOrdersAccounts {
                    token_program,
                    amm,
                    amm_authority,
                    amm_open_orders,
                    amm_target_orders,
                    pool_coin_token_account,
                    pool_pc_token_account,
                    amm_owner_account,
                    amm_config,
                    serum_program,
                    serum_market,
                    serum_coin_vault_account,
                    serum_pc_vault_account,
                    serum_vault_signer,
                    serum_event_q,
                    serum_bids,
                    serum_asks,
                    remaining,
                };
                return Ok(Instruction::AdminCancelOrders { accounts, args });
            }
            14u8 => {
                let mut rdr: &[u8] = rest;
                let args = CreateConfigAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_config = keys.next().unwrap_or(&"".to_string()).clone();
                let owner = keys.next().unwrap_or(&"".to_string()).clone();
                let system_program = keys.next().unwrap_or(&"".to_string()).clone();
                let rent = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateConfigAccountAccounts {
                    admin,
                    amm_config,
                    owner,
                    system_program,
                    rent,
                    remaining,
                };
                return Ok(Instruction::CreateConfigAccount { accounts, args });
            }
            15u8 => {
                let mut rdr: &[u8] = rest;
                let args = UpdateConfigAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap_or(&"".to_string()).clone();
                let amm_config = keys.next().unwrap_or(&"".to_string()).clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateConfigAccountAccounts {
                    admin,
                    amm_config,
                    remaining,
                };
                return Ok(Instruction::UpdateConfigAccount { accounts, args });
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
            anyhow::bail!("Event decoding not implemented for AMM v4")
        }
    }
}
