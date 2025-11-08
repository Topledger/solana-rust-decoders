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
fn serialize_u64_as_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}
pub use accounts_data::*;
pub use typedefs::*;
pub mod typedefs {
    use super::serialize_u64_as_string;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use anchor_lang::prelude::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde::Serialize;
    use serde::Serializer;
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct AddResolverEvent {
        #[serde(with = "pubkey_serde")]
        pub resolver: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct CancelOrderEvent {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
        #[serde(with = "pubkey_serde")]
        pub payer: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub maker: Pubkey,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub update_ts: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
    pub struct CommissionSwapArgs {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub expect_amount_out: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub min_return: u64,
        pub amounts: Vec<u64>,
        pub routes: Vec<Vec<Route>>,
        pub commission_rate: u16,
        pub commission_direction: bool,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct CommissionWrapUnwrapArgs {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount_in: u64,
        pub wrap_direction: bool,
        pub commission_rate: u16,
        pub commission_direction: bool,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum Dex {
        SplTokenSwap,
        StableSwap,
        Whirlpool,
        MeteoraDynamicpool,
        RaydiumSwap,
        RaydiumStableSwap,
        RaydiumClmmSwap,
        AldrinExchangeV1,
        AldrinExchangeV2,
        LifinityV1,
        LifinityV2,
        RaydiumClmmSwapV2,
        FluxBeam,
        MeteoraDlmm,
        RaydiumCpmmSwap,
        OpenBookV2,
        WhirlpoolV2,
        Phoenix,
        ObricV2,
        SanctumAddLiq,
        SanctumRemoveLiq,
        SanctumNonWsolSwap,
        SanctumWsolSwap,
        PumpfunBuy,
        PumpfunSell,
        StabbleSwap,
        SanctumRouter,
        MeteoraVaultDeposit,
        MeteoraVaultWithdraw,
        Saros,
        MeteoraLst,
        Solfi,
        QualiaSwap,
        Zerofi,
        PumpfunammBuy,
        PumpfunammSell,
        Virtuals,
        VertigoBuy,
        VertigoSell,
        PerpetualsAddLiq,
        PerpetualsRemoveLiq,
        PerpetualsSwap,
        RaydiumLaunchpad,
        LetsBonkFun,
        Woofi,
        MeteoraDbc,
        MeteoraDlmmSwap2,
        MeteoraDAMMV2,
        Gavel,
        BoopfunBuy,
        BoopfunSell,
        MeteoraDbc2,
        GooseFX,
        Dooar,
        Numeraire,
        SaberDecimalWrapperDeposit,
        SaberDecimalWrapperWithdraw,
        SarosDlmm,
        OneDexSwap,
        Manifest,
        ByrealClmm,
        PancakeSwapV3Swap,
        PancakeSwapV3SwapV2,
        Tessera,
        SolRfq {
            rfq_id: u64,
            expected_maker_amount: u64,
            expected_taker_amount: u64,
            maker_send_amount: u64,
            taker_send_amount: u64,
            expiry: u64,
            maker_use_native_sol: bool,
            taker_use_native_sol: bool,
        },
        PumpfunBuy2,
        PumpfunammBuy2,
        Humidifi,
        HeavenBuy,
        HeavenSell,
        SolfiV2,
        PumpfunBuy3,
        PumpfunSell3,
        PumpfunammBuy3,
        PumpfunammSell3,
        Goonfi,
        MoonitBuy,
        MoonitSell,
        RaydiumSwapV2,
        Swaap,
        SugarMoneyBuy {
            bonding_curve_bump: u8,
            bonding_curve_sol_associated_account_bump: u8,
        },
        SugarMoneySell {
            bonding_curve_bump: u8,
            bonding_curve_sol_associated_account_bump: u8,
        },
        MeteoraDAMMV2Swap2,
        AlphaQ,
    }
    impl Default for Dex {
        fn default() -> Self {
            Self::SplTokenSwap
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct FillOrderEvent {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
        #[serde(with = "pubkey_serde")]
        pub payer: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub maker: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub input_token_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub output_token_mint: Pubkey,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub making_amount: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub taking_amount: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub update_ts: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct InitGlobalConfigEvent {
        #[serde(with = "pubkey_serde")]
        pub admin: Pubkey,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub trade_fee: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PauseTradingEvent {
        pub paused: bool,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PlaceOrderEvent {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
        #[serde(with = "pubkey_serde")]
        pub maker: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub input_token_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub output_token_mint: Pubkey,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub making_amount: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub expect_taking_amount: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub min_return_amount: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub create_ts: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub deadline: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub trade_fee: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PlatformFeeWrapUnwrapArgs {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount_in: u64,
        pub commission_info: u32,
        pub platform_fee_rate: u16,
        pub tob: bool,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PlatformFeeWrapUnwrapArgsV2 {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount_in: u64,
        pub commission_info: u32,
        pub platform_fee_rate: u32,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct RefundEvent {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
        #[serde(with = "pubkey_serde")]
        pub maker: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub input_token_mint: Pubkey,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct RemoveResolverEvent {
        #[serde(with = "pubkey_serde")]
        pub resolver: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct Route {
        pub dexes: Vec<Dex>,
        pub weights: Vec<u8>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SetAdminEvent {
        #[serde(with = "pubkey_serde")]
        pub admin: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SetFeeMultiplierEvent {
        pub fee_multiplier: u8,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SetTradeFeeEvent {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub trade_fee: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
    pub struct SwapArgs {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub expect_amount_out: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub min_return: u64,
        pub amounts: Vec<u64>,
        pub routes: Vec<Vec<Route>>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SwapEvent {
        pub dex: Dex,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount_out: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UpdateOrderEvent {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
        #[serde(with = "pubkey_serde")]
        pub maker: Pubkey,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub expect_taking_amount: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub min_return_amount: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub deadline: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub update_ts: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub increase_fee: u64,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct ClaimAccounts {
        pub signer: String,
        pub receiver: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub sa_authority: String,
        pub token_mint: String,
        pub token_program: String,
        pub system_program: String,
        pub associated_token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CommissionSolProxySwapAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CommissionSolSwapAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_account: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CommissionSplProxySwapAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_token_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CommissionSplSwapAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_token_account: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CommissionWrapUnwrapAccounts {
        pub payer: String,
        pub payer_wsol_account: String,
        pub wsol_mint: String,
        pub temp_wsol_account: String,
        pub commission_sol_account: String,
        pub commission_wsol_account: String,
        pub system_program: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateTokenAccountAccounts {
        pub payer: String,
        pub owner: String,
        pub token_account: String,
        pub token_mint: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateTokenAccountWithSeedAccounts {
        pub payer: String,
        pub owner: String,
        pub token_account: String,
        pub token_mint: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlatformFeeSolProxySwapV2Accounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlatformFeeSolWrapUnwrapV2Accounts {
        pub payer: String,
        pub payer_wsol_account: String,
        pub wsol_mint: String,
        pub temp_wsol_account: String,
        pub commission_sol_account: String,
        pub commission_wsol_account: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub system_program: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PlatformFeeSplProxySwapV2Accounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_token_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ProxySwapAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapTobV3Accounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_account: String,
        pub platform_fee_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapTobV3EnhancedAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_account: String,
        pub platform_fee_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapTobV3WithReceiverAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_account: String,
        pub platform_fee_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub sol_receiver: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapV3Accounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_account: String,
        pub platform_fee_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WrapUnwrapV3Accounts {
        pub payer: String,
        pub payer_wsol_account: String,
        pub wsol_mint: String,
        pub temp_wsol_account: String,
        pub commission_account: String,
        pub platform_fee_account: String,
        pub authority_pda: String,
        pub wsol_sa: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::serialize_u64_as_string;
    use super::typedefs::*;
    use serde::Serialize;
    use serde::Serializer;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize, Default)]
    pub struct ClaimInstructionArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CommissionSolProxySwapInstructionArgs {
        pub data: SwapArgs,
        pub commission_rate: u16,
        pub commission_direction: bool,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CommissionSolSwapInstructionArgs {
        pub data: CommissionSwapArgs,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CommissionSplProxySwapInstructionArgs {
        pub data: SwapArgs,
        pub commission_rate: u16,
        pub commission_direction: bool,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CommissionSplSwapInstructionArgs {
        pub data: CommissionSwapArgs,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CommissionWrapUnwrapInstructionArgs {
        pub data: CommissionWrapUnwrapArgs,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateTokenAccountInstructionArgs {
        pub bump: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateTokenAccountWithSeedInstructionArgs {
        pub bump: u8,
        pub seed: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PlatformFeeSolProxySwapV2InstructionArgs {
        pub args: SwapArgs,
        pub commission_info: u32,
        pub platform_fee_rate: u32,
        pub trim_rate: u8,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PlatformFeeSolWrapUnwrapV2InstructionArgs {
        pub args: PlatformFeeWrapUnwrapArgsV2,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PlatformFeeSplProxySwapV2InstructionArgs {
        pub args: SwapArgs,
        pub commission_info: u32,
        pub platform_fee_rate: u32,
        pub trim_rate: u8,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ProxySwapInstructionArgs {
        pub data: SwapArgs,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapInstructionArgs {
        pub data: SwapArgs,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapTobV3InstructionArgs {
        pub args: SwapArgs,
        pub commission_info: u32,
        pub trim_rate: u8,
        pub platform_fee_rate: u16,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapTobV3EnhancedInstructionArgs {
        pub args: SwapArgs,
        pub commission_info: u32,
        pub trim_rate: u8,
        pub charge_rate: u16,
        pub platform_fee_rate: u16,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapTobV3WithReceiverInstructionArgs {
        pub args: SwapArgs,
        pub commission_info: u32,
        pub trim_rate: u8,
        pub platform_fee_rate: u16,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapV3InstructionArgs {
        pub args: SwapArgs,
        pub commission_info: u32,
        pub platform_fee_rate: u16,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WrapUnwrapV3InstructionArgs {
        pub args: PlatformFeeWrapUnwrapArgs,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Claim {
        accounts: ClaimAccounts,
        args: ix_data::ClaimInstructionArgs,
    },
    CommissionSolProxySwap {
        accounts: CommissionSolProxySwapAccounts,
        args: ix_data::CommissionSolProxySwapInstructionArgs,
    },
    CommissionSolSwap {
        accounts: CommissionSolSwapAccounts,
        args: ix_data::CommissionSolSwapInstructionArgs,
    },
    CommissionSplProxySwap {
        accounts: CommissionSplProxySwapAccounts,
        args: ix_data::CommissionSplProxySwapInstructionArgs,
    },
    CommissionSplSwap {
        accounts: CommissionSplSwapAccounts,
        args: ix_data::CommissionSplSwapInstructionArgs,
    },
    CommissionWrapUnwrap {
        accounts: CommissionWrapUnwrapAccounts,
        args: ix_data::CommissionWrapUnwrapInstructionArgs,
    },
    CreateTokenAccount {
        accounts: CreateTokenAccountAccounts,
        args: ix_data::CreateTokenAccountInstructionArgs,
    },
    CreateTokenAccountWithSeed {
        accounts: CreateTokenAccountWithSeedAccounts,
        args: ix_data::CreateTokenAccountWithSeedInstructionArgs,
    },
    PlatformFeeSolProxySwapV2 {
        accounts: PlatformFeeSolProxySwapV2Accounts,
        args: ix_data::PlatformFeeSolProxySwapV2InstructionArgs,
    },
    PlatformFeeSolWrapUnwrapV2 {
        accounts: PlatformFeeSolWrapUnwrapV2Accounts,
        args: ix_data::PlatformFeeSolWrapUnwrapV2InstructionArgs,
    },
    PlatformFeeSplProxySwapV2 {
        accounts: PlatformFeeSplProxySwapV2Accounts,
        args: ix_data::PlatformFeeSplProxySwapV2InstructionArgs,
    },
    ProxySwap {
        accounts: ProxySwapAccounts,
        args: ix_data::ProxySwapInstructionArgs,
    },
    Swap {
        accounts: SwapAccounts,
        args: ix_data::SwapInstructionArgs,
    },
    SwapTobV3 {
        accounts: SwapTobV3Accounts,
        args: ix_data::SwapTobV3InstructionArgs,
    },
    SwapTobV3Enhanced {
        accounts: SwapTobV3EnhancedAccounts,
        args: ix_data::SwapTobV3EnhancedInstructionArgs,
    },
    SwapTobV3WithReceiver {
        accounts: SwapTobV3WithReceiverAccounts,
        args: ix_data::SwapTobV3WithReceiverInstructionArgs,
    },
    SwapV3 {
        accounts: SwapV3Accounts,
        args: ix_data::SwapV3InstructionArgs,
    },
    WrapUnwrapV3 {
        accounts: WrapUnwrapV3Accounts,
        args: ix_data::WrapUnwrapV3InstructionArgs,
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
                let args = ix_data::ClaimInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let signer = keys.next().unwrap_or(&String::new()).clone();
                let receiver = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let token_mint = keys.next().unwrap_or(&String::new()).clone();
                let token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimAccounts {
                    signer,
                    receiver,
                    source_token_account,
                    destination_token_account,
                    sa_authority,
                    token_mint,
                    token_program,
                    system_program,
                    associated_token_program,
                    remaining,
                };
                return Ok(Instruction::Claim { accounts, args });
            }
            [30u8, 33u8, 208u8, 91u8, 31u8, 157u8, 37u8, 18u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::CommissionSolProxySwapInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CommissionSolProxySwapAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CommissionSolProxySwap { accounts, args });
            }
            [81u8, 128u8, 134u8, 73u8, 114u8, 73u8, 45u8, 94u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::CommissionSolSwapInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CommissionSolSwapAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_account,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CommissionSolSwap { accounts, args });
            }
            [96u8, 67u8, 12u8, 151u8, 129u8, 164u8, 18u8, 71u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::CommissionSplProxySwapInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_token_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CommissionSplProxySwapAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_token_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CommissionSplProxySwap { accounts, args });
            }
            [235u8, 71u8, 211u8, 196u8, 114u8, 199u8, 143u8, 92u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::CommissionSplSwapInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_token_account = keys.next().unwrap_or(&String::new()).clone();
                let token_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CommissionSplSwapAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_token_account,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::CommissionSplSwap { accounts, args });
            }
            [12u8, 73u8, 156u8, 71u8, 233u8, 172u8, 189u8, 197u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::CommissionWrapUnwrapInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let payer_wsol_account = keys.next().unwrap_or(&String::new()).clone();
                let wsol_mint = keys.next().unwrap_or(&String::new()).clone();
                let temp_wsol_account = keys.next().unwrap_or(&String::new()).clone();
                let commission_sol_account = keys.next().unwrap_or(&String::new()).clone();
                let commission_wsol_account = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let token_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CommissionWrapUnwrapAccounts {
                    payer,
                    payer_wsol_account,
                    wsol_mint,
                    temp_wsol_account,
                    commission_sol_account,
                    commission_wsol_account,
                    system_program,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::CommissionWrapUnwrap { accounts, args });
            }
            [147u8, 241u8, 123u8, 100u8, 244u8, 132u8, 174u8, 118u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::CreateTokenAccountInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let owner = keys.next().unwrap_or(&String::new()).clone();
                let token_account = keys.next().unwrap_or(&String::new()).clone();
                let token_mint = keys.next().unwrap_or(&String::new()).clone();
                let token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateTokenAccountAccounts {
                    payer,
                    owner,
                    token_account,
                    token_mint,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateTokenAccount { accounts, args });
            }
            [125u8, 191u8, 239u8, 140u8, 66u8, 8u8, 9u8, 228u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    ix_data::CreateTokenAccountWithSeedInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let owner = keys.next().unwrap_or(&String::new()).clone();
                let token_account = keys.next().unwrap_or(&String::new()).clone();
                let token_mint = keys.next().unwrap_or(&String::new()).clone();
                let token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateTokenAccountWithSeedAccounts {
                    payer,
                    owner,
                    token_account,
                    token_mint,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateTokenAccountWithSeed { accounts, args });
            }
            [69u8, 200u8, 254u8, 247u8, 40u8, 52u8, 118u8, 202u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    ix_data::PlatformFeeSolProxySwapV2InstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = PlatformFeeSolProxySwapV2Accounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::PlatformFeeSolProxySwapV2 { accounts, args });
            }
            [196u8, 172u8, 152u8, 92u8, 60u8, 186u8, 64u8, 227u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    ix_data::PlatformFeeSolWrapUnwrapV2InstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let payer_wsol_account = keys.next().unwrap_or(&String::new()).clone();
                let wsol_mint = keys.next().unwrap_or(&String::new()).clone();
                let temp_wsol_account = keys.next().unwrap_or(&String::new()).clone();
                let commission_sol_account = keys.next().unwrap_or(&String::new()).clone();
                let commission_wsol_account = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let token_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = PlatformFeeSolWrapUnwrapV2Accounts {
                    payer,
                    payer_wsol_account,
                    wsol_mint,
                    temp_wsol_account,
                    commission_sol_account,
                    commission_wsol_account,
                    source_token_sa,
                    destination_token_sa,
                    system_program,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::PlatformFeeSolWrapUnwrapV2 { accounts, args });
            }
            [69u8, 164u8, 210u8, 89u8, 146u8, 214u8, 173u8, 67u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    ix_data::PlatformFeeSplProxySwapV2InstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_token_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = PlatformFeeSplProxySwapV2Accounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_token_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::PlatformFeeSplProxySwapV2 { accounts, args });
            }
            [19u8, 44u8, 130u8, 148u8, 72u8, 56u8, 44u8, 238u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::ProxySwapInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = ProxySwapAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::ProxySwap { accounts, args });
            }
            [248u8, 198u8, 158u8, 145u8, 225u8, 117u8, 135u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::SwapInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    remaining,
                };
                return Ok(Instruction::Swap { accounts, args });
            }
            [14u8, 191u8, 44u8, 246u8, 142u8, 225u8, 224u8, 157u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::SwapTobV3InstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let platform_fee_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapTobV3Accounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_account,
                    platform_fee_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::SwapTobV3 { accounts, args });
            }
            [236u8, 71u8, 155u8, 68u8, 198u8, 98u8, 14u8, 118u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::SwapTobV3EnhancedInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let platform_fee_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapTobV3EnhancedAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_account,
                    platform_fee_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::SwapTobV3Enhanced { accounts, args });
            }
            [63u8, 114u8, 246u8, 131u8, 51u8, 2u8, 247u8, 29u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::SwapTobV3WithReceiverInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let platform_fee_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let sol_receiver = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapTobV3WithReceiverAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_account,
                    platform_fee_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    sol_receiver,
                    remaining,
                };
                return Ok(Instruction::SwapTobV3WithReceiver { accounts, args });
            }
            [240u8, 224u8, 38u8, 33u8, 176u8, 31u8, 241u8, 175u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::SwapV3InstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let platform_fee_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapV3Accounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_account,
                    platform_fee_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::SwapV3 { accounts, args });
            }
            [180u8, 178u8, 191u8, 54u8, 70u8, 8u8, 13u8, 224u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::WrapUnwrapV3InstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let payer_wsol_account = keys.next().unwrap_or(&String::new()).clone();
                let wsol_mint = keys.next().unwrap_or(&String::new()).clone();
                let temp_wsol_account = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let platform_fee_account = keys.next().unwrap_or(&String::new()).clone();
                let authority_pda = keys.next().unwrap_or(&String::new()).clone();
                let wsol_sa = keys.next().unwrap_or(&String::new()).clone();
                let token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = WrapUnwrapV3Accounts {
                    payer,
                    payer_wsol_account,
                    wsol_mint,
                    temp_wsol_account,
                    commission_account,
                    platform_fee_account,
                    authority_pda,
                    wsol_sa,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::WrapUnwrapV3 { accounts, args });
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
        AddResolverEvent { args: AddResolverEvent },
        CancelOrderEvent { args: CancelOrderEvent },
        FillOrderEvent { args: FillOrderEvent },
        InitGlobalConfigEvent { args: InitGlobalConfigEvent },
        PauseTradingEvent { args: PauseTradingEvent },
        PlaceOrderEvent { args: PlaceOrderEvent },
        RefundEvent { args: RefundEvent },
        RemoveResolverEvent { args: RemoveResolverEvent },
        SetAdminEvent { args: SetAdminEvent },
        SetFeeMultiplierEvent { args: SetFeeMultiplierEvent },
        SetTradeFeeEvent { args: SetTradeFeeEvent },
        SwapEvent { args: SwapEvent },
        UpdateOrderEvent { args: UpdateOrderEvent },
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
            match disc {
                [173u8, 137u8, 29u8, 251u8, 195u8, 58u8, 115u8, 71u8] => {
                    let mut rdr = &data[16..];
                    let args = AddResolverEvent::deserialize(&mut rdr)?;
                    return Ok(Event::AddResolverEvent { args });
                }
                [174u8, 66u8, 141u8, 17u8, 4u8, 224u8, 162u8, 77u8] => {
                    let mut rdr = &data[16..];
                    let args = CancelOrderEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CancelOrderEvent { args });
                }
                [37u8, 51u8, 197u8, 130u8, 53u8, 15u8, 99u8, 18u8] => {
                    let mut rdr = &data[16..];
                    let args = FillOrderEvent::deserialize(&mut rdr)?;
                    return Ok(Event::FillOrderEvent { args });
                }
                [195u8, 252u8, 133u8, 149u8, 47u8, 126u8, 107u8, 231u8] => {
                    let mut rdr = &data[16..];
                    let args = InitGlobalConfigEvent::deserialize(&mut rdr)?;
                    return Ok(Event::InitGlobalConfigEvent { args });
                }
                [85u8, 23u8, 87u8, 137u8, 206u8, 65u8, 208u8, 58u8] => {
                    let mut rdr = &data[16..];
                    let args = PauseTradingEvent::deserialize(&mut rdr)?;
                    return Ok(Event::PauseTradingEvent { args });
                }
                [65u8, 191u8, 25u8, 91u8, 27u8, 252u8, 192u8, 40u8] => {
                    let mut rdr = &data[16..];
                    let args = PlaceOrderEvent::deserialize(&mut rdr)?;
                    return Ok(Event::PlaceOrderEvent { args });
                }
                [176u8, 159u8, 218u8, 59u8, 94u8, 213u8, 129u8, 218u8] => {
                    let mut rdr = &data[16..];
                    let args = RefundEvent::deserialize(&mut rdr)?;
                    return Ok(Event::RefundEvent { args });
                }
                [57u8, 138u8, 125u8, 122u8, 100u8, 83u8, 156u8, 37u8] => {
                    let mut rdr = &data[16..];
                    let args = RemoveResolverEvent::deserialize(&mut rdr)?;
                    return Ok(Event::RemoveResolverEvent { args });
                }
                [240u8, 117u8, 204u8, 254u8, 89u8, 150u8, 132u8, 94u8] => {
                    let mut rdr = &data[16..];
                    let args = SetAdminEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SetAdminEvent { args });
                }
                [197u8, 91u8, 90u8, 165u8, 244u8, 201u8, 13u8, 154u8] => {
                    let mut rdr = &data[16..];
                    let args = SetFeeMultiplierEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SetFeeMultiplierEvent { args });
                }
                [8u8, 97u8, 163u8, 68u8, 79u8, 99u8, 134u8, 229u8] => {
                    let mut rdr = &data[16..];
                    let args = SetTradeFeeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SetTradeFeeEvent { args });
                }
                [64u8, 198u8, 205u8, 232u8, 38u8, 8u8, 113u8, 226u8] => {
                    let mut rdr = &data[16..];
                    let args = SwapEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SwapEvent { args });
                }
                [55u8, 24u8, 47u8, 240u8, 105u8, 245u8, 30u8, 135u8] => {
                    let mut rdr = &data[16..];
                    let args = UpdateOrderEvent::deserialize(&mut rdr)?;
                    return Ok(Event::UpdateOrderEvent { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
