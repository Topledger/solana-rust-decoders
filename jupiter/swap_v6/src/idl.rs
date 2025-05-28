use serde::Serializer;
#[allow(dead_code)]
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
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use anchor_lang::prelude::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde::Serialize;
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum AccountsType {
        TransferHookA,
        TransferHookB,
        TransferHookReward,
        TransferHookInput,
        TransferHookIntermediate,
        TransferHookOutput,
        SupplementalTickArrays,
        SupplementalTickArraysOne,
        SupplementalTickArraysTwo,
    }
    impl Default for AccountsType {
        fn default() -> Self {
            Self::TransferHookA
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct FeeEvent {
        #[serde(with = "pubkey_serde")]
        pub account: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub mint: Pubkey,
        pub amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
    pub struct RemainingAccountsInfo {
        pub slices: Vec<RemainingAccountsSlice>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct RemainingAccountsSlice {
        pub accounts_type: AccountsType,
        pub length: u8,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
    pub struct RoutePlanStep {
        pub swap: Swap,
        pub percent: u8,
        pub input_index: u8,
        pub output_index: u8,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum Side {
        Bid,
        Ask,
    }
    impl Default for Side {
        fn default() -> Self {
            Self::Bid
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub enum Swap {
        Saber,
        SaberAddDecimalsDeposit,
        SaberAddDecimalsWithdraw,
        TokenSwap,
        Sencha,
        Step,
        Cropper,
        Raydium,
        Crema {
            a_to_b: bool,
        },
        Lifinity,
        Mercurial,
        Cykura,
        Serum {
            side: Side,
        },
        MarinadeDeposit,
        MarinadeUnstake,
        Aldrin {
            side: Side,
        },
        AldrinV2 {
            side: Side,
        },
        Whirlpool {
            a_to_b: bool,
        },
        Invariant {
            x_to_y: bool,
        },
        Meteora,
        GooseFX,
        DeltaFi {
            stable: bool,
        },
        Balansol,
        MarcoPolo {
            x_to_y: bool,
        },
        Dradex {
            side: Side,
        },
        LifinityV2,
        RaydiumClmm,
        Openbook {
            side: Side,
        },
        Phoenix {
            side: Side,
        },
        Symmetry {
            from_token_id: u64,
            to_token_id: u64,
        },
        TokenSwapV2,
        HeliumTreasuryManagementRedeemV0,
        StakeDexStakeWrappedSol,
        StakeDexSwapViaStake {
            bridge_stake_seed: u32,
        },
        GooseFXV2,
        Perps,
        PerpsAddLiquidity,
        PerpsRemoveLiquidity,
        MeteoraDlmm,
        OpenBookV2 {
            side: Side,
        },
        RaydiumClmmV2,
        StakeDexPrefundWithdrawStakeAndDepositStake {
            bridge_stake_seed: u32,
        },
        Clone {
            pool_index: u8,
            quantity_is_input: bool,
            quantity_is_collateral: bool,
        },
        SanctumS {
            src_lst_value_calc_accs: u8,
            dst_lst_value_calc_accs: u8,
            src_lst_index: u32,
            dst_lst_index: u32,
        },
        SanctumSAddLiquidity {
            lst_value_calc_accs: u8,
            lst_index: u32,
        },
        SanctumSRemoveLiquidity {
            lst_value_calc_accs: u8,
            lst_index: u32,
        },
        RaydiumCP,
        WhirlpoolSwapV2 {
            a_to_b: bool,
            remaining_accounts_info: Option<RemainingAccountsInfo>,
        },
        OneIntro,
        PumpdotfunWrappedBuy,
        PumpdotfunWrappedSell,
        PerpsV2,
        PerpsV2AddLiquidity,
        PerpsV2RemoveLiquidity,
        MoonshotWrappedBuy,
        MoonshotWrappedSell,
        StabbleStableSwap,
        StabbleWeightedSwap,
        Obric {
            x_to_y: bool,
        },
        FoxBuyFromEstimatedCost,
        FoxClaimPartial {
            is_y: bool,
        },
        SolFi {
            is_quote_to_base: bool,
        },
        SolayerDelegateNoInit,
        SolayerUndelegateNoInit,
        TokenMill {
            side: Side,
        },
        DaosFunBuy,
        DaosFunSell,
        ZeroFi,
        StakeDexWithdrawWrappedSol,
        VirtualsBuy,
        VirtualsSell,
        Perena {
            in_index: u8,
            out_index: u8,
        },
        PumpdotfunAmmBuy,
        PumpdotfunAmmSell,
        Gamma,
        MeteoraDlmmSwapV2 {
            remaining_accounts_info: RemainingAccountsInfo,
        },
        Woofi,
        MeteoraDammV2,
        MeteoraDynamicBondingCurveSwap,
        StabbleStableSwapV2,
        StabbleWeightedSwapV2,
        RaydiumLaunchlabBuy {
            share_fee_rate: u64,
        },
        RaydiumLaunchlabSell {
            share_fee_rate: u64,
        },
        BoopdotfunWrappedBuy,
        BoopdotfunWrappedSell,
    }
    impl Default for Swap {
        fn default() -> Self {
            Self::Saber
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SwapEvent {
        #[serde(with = "pubkey_serde")]
        pub amm: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub input_mint: Pubkey,
        pub input_amount: u64,
        #[serde(with = "pubkey_serde")]
        pub output_mint: Pubkey,
        pub output_amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct TokenLedger {
        #[serde(with = "pubkey_serde")]
        pub token_account: Pubkey,
        pub amount: u64,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct ClaimAccounts {
        pub wallet: String,
        pub program_authority: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimTokenAccounts {
        pub payer: String,
        pub wallet: String,
        pub program_authority: String,
        pub program_token_account: String,
        pub destination_token_account: String,
        pub mint: String,
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseTokenAccounts {
        pub operator: String,
        pub wallet: String,
        pub program_authority: String,
        pub program_token_account: String,
        pub mint: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateOpenOrdersAccounts {
        pub open_orders: String,
        pub payer: String,
        pub dex_program: String,
        pub system_program: String,
        pub rent: String,
        pub market: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateProgramOpenOrdersAccounts {
        pub open_orders: String,
        pub payer: String,
        pub program_authority: String,
        pub dex_program: String,
        pub system_program: String,
        pub rent: String,
        pub market: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateTokenLedgerAccounts {
        pub token_ledger: String,
        pub payer: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateTokenAccountAccounts {
        pub token_account: String,
        pub user: String,
        pub mint: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ExactOutRouteAccounts {
        pub token_program: String,
        pub user_transfer_authority: String,
        pub user_source_token_account: String,
        pub user_destination_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub platform_fee_account: String,
        pub token_2022_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RouteAccounts {
        pub token_program: String,
        pub user_transfer_authority: String,
        pub user_source_token_account: String,
        pub user_destination_token_account: String,
        pub destination_token_account: String,
        pub destination_mint: String,
        pub platform_fee_account: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RouteWithTokenLedgerAccounts {
        pub token_program: String,
        pub user_transfer_authority: String,
        pub user_source_token_account: String,
        pub user_destination_token_account: String,
        pub destination_token_account: String,
        pub destination_mint: String,
        pub platform_fee_account: String,
        pub token_ledger: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetTokenLedgerAccounts {
        pub token_ledger: String,
        pub token_account: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SharedAccountsExactOutRouteAccounts {
        pub token_program: String,
        pub program_authority: String,
        pub user_transfer_authority: String,
        pub source_token_account: String,
        pub program_source_token_account: String,
        pub program_destination_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub platform_fee_account: String,
        pub token_2022_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SharedAccountsRouteAccounts {
        pub token_program: String,
        pub program_authority: String,
        pub user_transfer_authority: String,
        pub source_token_account: String,
        pub program_source_token_account: String,
        pub program_destination_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub platform_fee_account: String,
        pub token_2022_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SharedAccountsRouteWithTokenLedgerAccounts {
        pub token_program: String,
        pub program_authority: String,
        pub user_transfer_authority: String,
        pub source_token_account: String,
        pub program_source_token_account: String,
        pub program_destination_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub platform_fee_account: String,
        pub token_2022_program: String,
        pub token_ledger: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimArgs {
        pub id: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimTokenArgs {
        pub id: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseTokenArgs {
        pub id: u8,
        pub burn_all: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateOpenOrdersArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateProgramOpenOrdersArgs {
        pub id: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateTokenLedgerArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateTokenAccountArgs {
        pub bump: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ExactOutRouteArgs {
        pub route_plan: Vec<RoutePlanStep>,
        pub out_amount: u64,
        pub quoted_in_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RouteArgs {
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RouteWithTokenLedgerArgs {
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetTokenLedgerArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SharedAccountsExactOutRouteArgs {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub out_amount: u64,
        pub quoted_in_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SharedAccountsRouteArgs {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SharedAccountsRouteWithTokenLedgerArgs {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Claim {
        accounts: ClaimAccounts,
        args: ClaimArgs,
    },
    ClaimToken {
        accounts: ClaimTokenAccounts,
        args: ClaimTokenArgs,
    },
    CloseToken {
        accounts: CloseTokenAccounts,
        args: CloseTokenArgs,
    },
    CreateOpenOrders {
        accounts: CreateOpenOrdersAccounts,
        args: CreateOpenOrdersArgs,
    },
    CreateProgramOpenOrders {
        accounts: CreateProgramOpenOrdersAccounts,
        args: CreateProgramOpenOrdersArgs,
    },
    CreateTokenLedger {
        accounts: CreateTokenLedgerAccounts,
        args: CreateTokenLedgerArgs,
    },
    CreateTokenAccount {
        accounts: CreateTokenAccountAccounts,
        args: CreateTokenAccountArgs,
    },
    ExactOutRoute {
        accounts: ExactOutRouteAccounts,
        args: ExactOutRouteArgs,
    },
    Route {
        accounts: RouteAccounts,
        args: RouteArgs,
    },
    RouteWithTokenLedger {
        accounts: RouteWithTokenLedgerAccounts,
        args: RouteWithTokenLedgerArgs,
    },
    SetTokenLedger {
        accounts: SetTokenLedgerAccounts,
        args: SetTokenLedgerArgs,
    },
    SharedAccountsExactOutRoute {
        accounts: SharedAccountsExactOutRouteAccounts,
        args: SharedAccountsExactOutRouteArgs,
    },
    SharedAccountsRoute {
        accounts: SharedAccountsRouteAccounts,
        args: SharedAccountsRouteArgs,
    },
    SharedAccountsRouteWithTokenLedger {
        accounts: SharedAccountsRouteWithTokenLedgerAccounts,
        args: SharedAccountsRouteWithTokenLedgerArgs,
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
                let args = ClaimArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let wallet = keys.next().unwrap().clone();
                let program_authority = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimAccounts {
                    wallet,
                    program_authority,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::Claim { accounts, args });
            }
            [116u8, 206u8, 27u8, 191u8, 166u8, 19u8, 0u8, 73u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimTokenArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let wallet = keys.next().unwrap().clone();
                let program_authority = keys.next().unwrap().clone();
                let program_token_account = keys.next().unwrap().clone();
                let destination_token_account = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimTokenAccounts {
                    payer,
                    wallet,
                    program_authority,
                    program_token_account,
                    destination_token_account,
                    mint,
                    token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::ClaimToken { accounts, args });
            }
            [26u8, 74u8, 236u8, 151u8, 104u8, 64u8, 183u8, 249u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseTokenArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let operator = keys.next().unwrap().clone();
                let wallet = keys.next().unwrap().clone();
                let program_authority = keys.next().unwrap().clone();
                let program_token_account = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseTokenAccounts {
                    operator,
                    wallet,
                    program_authority,
                    program_token_account,
                    mint,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::CloseToken { accounts, args });
            }
            [229u8, 194u8, 212u8, 172u8, 8u8, 10u8, 134u8, 147u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateOpenOrdersArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let open_orders = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let dex_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let market = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateOpenOrdersAccounts {
                    open_orders,
                    payer,
                    dex_program,
                    system_program,
                    rent,
                    market,
                    remaining,
                };
                return Ok(Instruction::CreateOpenOrders { accounts, args });
            }
            [28u8, 226u8, 32u8, 148u8, 188u8, 136u8, 113u8, 171u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateProgramOpenOrdersArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let open_orders = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let program_authority = keys.next().unwrap().clone();
                let dex_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let market = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateProgramOpenOrdersAccounts {
                    open_orders,
                    payer,
                    program_authority,
                    dex_program,
                    system_program,
                    rent,
                    market,
                    remaining,
                };
                return Ok(Instruction::CreateProgramOpenOrders { accounts, args });
            }
            [232u8, 242u8, 197u8, 253u8, 240u8, 143u8, 129u8, 52u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateTokenLedgerArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_ledger = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateTokenLedgerAccounts {
                    token_ledger,
                    payer,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateTokenLedger { accounts, args });
            }
            [147u8, 241u8, 123u8, 100u8, 244u8, 132u8, 174u8, 118u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateTokenAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_account = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateTokenAccountAccounts {
                    token_account,
                    user,
                    mint,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateTokenAccount { accounts, args });
            }
            [208u8, 51u8, 239u8, 151u8, 123u8, 43u8, 237u8, 92u8] => {
                let mut rdr: &[u8] = rest;
                let args = ExactOutRouteArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let user_transfer_authority = keys.next().unwrap().clone();
                let user_source_token_account = keys.next().unwrap().clone();
                let user_destination_token_account = keys.next().unwrap().clone();
                let destination_token_account = keys.next().unwrap().clone();
                let source_mint = keys.next().unwrap().clone();
                let destination_mint = keys.next().unwrap().clone();
                let platform_fee_account = keys.next().unwrap().clone();
                let token_2022_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ExactOutRouteAccounts {
                    token_program,
                    user_transfer_authority,
                    user_source_token_account,
                    user_destination_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    platform_fee_account,
                    token_2022_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ExactOutRoute { accounts, args });
            }
            [229u8, 23u8, 203u8, 151u8, 122u8, 227u8, 173u8, 42u8] => {
                let mut rdr: &[u8] = rest;
                let args = RouteArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let user_transfer_authority = keys.next().unwrap().clone();
                let user_source_token_account = keys.next().unwrap().clone();
                let user_destination_token_account = keys.next().unwrap().clone();
                let destination_token_account = keys.next().unwrap().clone();
                let destination_mint = keys.next().unwrap().clone();
                let platform_fee_account = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RouteAccounts {
                    token_program,
                    user_transfer_authority,
                    user_source_token_account,
                    user_destination_token_account,
                    destination_token_account,
                    destination_mint,
                    platform_fee_account,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Route { accounts, args });
            }
            [150u8, 86u8, 71u8, 116u8, 167u8, 93u8, 14u8, 104u8] => {
                let mut rdr: &[u8] = rest;
                let args = RouteWithTokenLedgerArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let user_transfer_authority = keys.next().unwrap().clone();
                let user_source_token_account = keys.next().unwrap().clone();
                let user_destination_token_account = keys.next().unwrap().clone();
                let destination_token_account = keys.next().unwrap().clone();
                let destination_mint = keys.next().unwrap().clone();
                let platform_fee_account = keys.next().unwrap().clone();
                let token_ledger = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RouteWithTokenLedgerAccounts {
                    token_program,
                    user_transfer_authority,
                    user_source_token_account,
                    user_destination_token_account,
                    destination_token_account,
                    destination_mint,
                    platform_fee_account,
                    token_ledger,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::RouteWithTokenLedger { accounts, args });
            }
            [228u8, 85u8, 185u8, 112u8, 78u8, 79u8, 77u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetTokenLedgerArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_ledger = keys.next().unwrap().clone();
                let token_account = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetTokenLedgerAccounts {
                    token_ledger,
                    token_account,
                    remaining,
                };
                return Ok(Instruction::SetTokenLedger { accounts, args });
            }
            [176u8, 209u8, 105u8, 168u8, 154u8, 125u8, 69u8, 62u8] => {
                let mut rdr: &[u8] = rest;
                let args = SharedAccountsExactOutRouteArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let program_authority = keys.next().unwrap().clone();
                let user_transfer_authority = keys.next().unwrap().clone();
                let source_token_account = keys.next().unwrap().clone();
                let program_source_token_account = keys.next().unwrap().clone();
                let program_destination_token_account = keys.next().unwrap().clone();
                let destination_token_account = keys.next().unwrap().clone();
                let source_mint = keys.next().unwrap().clone();
                let destination_mint = keys.next().unwrap().clone();
                let platform_fee_account = keys.next().unwrap().clone();
                let token_2022_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SharedAccountsExactOutRouteAccounts {
                    token_program,
                    program_authority,
                    user_transfer_authority,
                    source_token_account,
                    program_source_token_account,
                    program_destination_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    platform_fee_account,
                    token_2022_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SharedAccountsExactOutRoute { accounts, args });
            }
            [193u8, 32u8, 155u8, 51u8, 65u8, 214u8, 156u8, 129u8] => {
                let mut rdr: &[u8] = rest;
                let args = SharedAccountsRouteArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let program_authority = keys.next().unwrap().clone();
                let user_transfer_authority = keys.next().unwrap().clone();
                let source_token_account = keys.next().unwrap().clone();
                let program_source_token_account = keys.next().unwrap().clone();
                let program_destination_token_account = keys.next().unwrap().clone();
                let destination_token_account = keys.next().unwrap().clone();
                let source_mint = keys.next().unwrap().clone();
                let destination_mint = keys.next().unwrap().clone();
                let platform_fee_account = keys.next().unwrap().clone();
                let token_2022_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SharedAccountsRouteAccounts {
                    token_program,
                    program_authority,
                    user_transfer_authority,
                    source_token_account,
                    program_source_token_account,
                    program_destination_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    platform_fee_account,
                    token_2022_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SharedAccountsRoute { accounts, args });
            }
            [230u8, 121u8, 143u8, 80u8, 119u8, 159u8, 106u8, 170u8] => {
                let mut rdr: &[u8] = rest;
                let args = SharedAccountsRouteWithTokenLedgerArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let program_authority = keys.next().unwrap().clone();
                let user_transfer_authority = keys.next().unwrap().clone();
                let source_token_account = keys.next().unwrap().clone();
                let program_source_token_account = keys.next().unwrap().clone();
                let program_destination_token_account = keys.next().unwrap().clone();
                let destination_token_account = keys.next().unwrap().clone();
                let source_mint = keys.next().unwrap().clone();
                let destination_mint = keys.next().unwrap().clone();
                let platform_fee_account = keys.next().unwrap().clone();
                let token_2022_program = keys.next().unwrap().clone();
                let token_ledger = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SharedAccountsRouteWithTokenLedgerAccounts {
                    token_program,
                    program_authority,
                    user_transfer_authority,
                    source_token_account,
                    program_source_token_account,
                    program_destination_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    platform_fee_account,
                    token_2022_program,
                    token_ledger,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SharedAccountsRouteWithTokenLedger { accounts, args });
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
        FeeEvent { args: FeeEvent },
        SwapEvent { args: SwapEvent },
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
            let payload = &data[16..];
            match disc {
                [73u8, 79u8, 78u8, 127u8, 184u8, 213u8, 13u8, 220u8] => {
                    let mut rdr = &data[16..];
                    let args = FeeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::FeeEvent { args });
                }
                [64u8, 198u8, 205u8, 232u8, 38u8, 8u8, 113u8, 226u8] => {
                    let mut rdr = &data[16..];
                    let args = SwapEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SwapEvent { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
