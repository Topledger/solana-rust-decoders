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
    serde_big_array::big_array! { BigArray ; 60 , 64 , 51 , 128 , 72 , 256 }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AddCustodyParams {
        pub is_stable: bool,
        pub oracle: OracleParams,
        pub pricing: PricingParams,
        pub permissions: Permissions,
        pub hourly_funding_dbps: u64,
        pub target_ratio_bps: u64,
        pub increase_position_bps: u64,
        pub decrease_position_bps: u64,
        #[serde(with = "pubkey_serde")]
        pub doves_oracle: [u8; 32usize],
        pub max_position_size_usd: u64,
        pub jump_rate: JumpRateState,
        pub price_impact_fee_factor: u64,
        pub price_impact_exponent: f32,
        pub delta_imbalance_threshold_decimal: u64,
        pub max_fee_bps: u64,
        #[serde(with = "pubkey_serde")]
        pub doves_ag_oracle: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AddLiquidity2Params {
        pub token_amount_in: u64,
        pub min_lp_amount_out: u64,
        pub token_amount_pre_swap: Option<u64>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AddPoolParams {
        pub name: String,
        pub limit: Limit,
        pub fees: Fees,
        pub max_request_execution_sec: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ClosePositionRequestParams {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreateDecreasePositionMarketRequestParams {
        pub collateral_usd_delta: u64,
        pub size_usd_delta: u64,
        pub price_slippage: u64,
        pub jupiter_minimum_out: Option<u64>,
        pub entire_position: Option<bool>,
        pub counter: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreateDecreasePositionRequest2Params {
        pub collateral_usd_delta: u64,
        pub size_usd_delta: u64,
        pub request_type: RequestType,
        pub price_slippage: Option<u64>,
        pub jupiter_minimum_out: Option<u64>,
        pub trigger_price: Option<u64>,
        pub trigger_above_threshold: Option<bool>,
        pub entire_position: Option<bool>,
        pub counter: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreateIncreasePositionMarketRequestParams {
        pub size_usd_delta: u64,
        pub collateral_token_delta: u64,
        pub side: Side,
        pub price_slippage: u64,
        pub jupiter_minimum_out: Option<u64>,
        pub counter: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreateTokenMetadataParams {
        pub name: String,
        pub symbol: String,
        pub uri: String,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DecreasePosition4Params {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DecreasePositionWithInternalSwapParams {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct GetAddLiquidityAmountAndFee2Params {
        pub token_amount_in: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct GetAssetsUnderManagement2Params {
        pub mode: Option<PriceCalcMode>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct GetRemoveLiquidityAmountAndFee2Params {
        pub lp_amount_in: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct IncreasePosition4Params {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct IncreasePositionPreSwapParams {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct IncreasePositionWithInternalSwapParams {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitParams {
        pub allow_swap: bool,
        pub allow_add_liquidity: bool,
        pub allow_remove_liquidity: bool,
        pub allow_increase_position: bool,
        pub allow_decrease_position: bool,
        pub allow_collateral_withdrawal: bool,
        pub allow_liquidate_position: bool,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InstantCreateLimitOrderParams {
        pub size_usd_delta: u64,
        pub collateral_token_delta: u64,
        pub side: Side,
        pub trigger_price: u64,
        pub trigger_above_threshold: bool,
        pub counter: u64,
        pub request_time: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InstantCreateTpslParams {
        pub collateral_usd_delta: u64,
        pub size_usd_delta: u64,
        pub trigger_price: u64,
        pub trigger_above_threshold: bool,
        pub entire_position: bool,
        pub counter: u64,
        pub request_time: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InstantDecreasePositionParams {
        pub collateral_usd_delta: u64,
        pub size_usd_delta: u64,
        pub price_slippage: u64,
        pub entire_position: Option<bool>,
        pub request_time: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InstantIncreasePositionParams {
        pub size_usd_delta: u64,
        pub collateral_token_delta: Option<u64>,
        pub side: Side,
        pub price_slippage: u64,
        pub request_time: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InstantUpdateLimitOrderParams {
        pub size_usd_delta: u64,
        pub trigger_price: u64,
        pub request_time: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InstantUpdateTpslParams {
        pub size_usd_delta: u64,
        pub trigger_price: u64,
        pub request_time: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiquidateFullPosition4Params {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OperatorSetCustodyConfigParams {
        pub pricing: PricingParams,
        pub hourly_funding_dbps: u64,
        pub target_ratio_bps: u64,
        pub increase_position_bps: u64,
        pub decrease_position_bps: u64,
        pub max_position_size_usd: u64,
        pub jump_rate: JumpRateState,
        pub price_impact_fee_factor: u64,
        pub price_impact_exponent: f32,
        pub delta_imbalance_threshold_decimal: u64,
        pub max_fee_bps: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OperatorSetPoolConfigParams {
        pub fees: Fees,
        pub limit: Limit,
        pub max_request_execution_sec: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RefreshAssetsUnderManagementParams {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RemoveLiquidity2Params {
        pub lp_amount_in: u64,
        pub min_amount_out: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SetCustodyConfigParams {
        pub oracle: OracleParams,
        pub pricing: PricingParams,
        pub permissions: Permissions,
        pub hourly_funding_dbps: u64,
        pub target_ratio_bps: u64,
        pub increase_position_bps: u64,
        pub decrease_position_bps: u64,
        #[serde(with = "pubkey_serde")]
        pub doves_oracle: [u8; 32usize],
        pub max_position_size_usd: u64,
        pub jump_rate: JumpRateState,
        pub price_impact_fee_factor: u64,
        pub price_impact_exponent: f32,
        pub delta_imbalance_threshold_decimal: u64,
        pub max_fee_bps: u64,
        #[serde(with = "pubkey_serde")]
        pub doves_ag_oracle: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SetPerpetualsConfigParams {
        pub permissions: Permissions,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SetPoolConfigParams {
        pub fees: Fees,
        pub limit: Limit,
        pub max_request_execution_sec: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SetTestTimeParams {
        pub time: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Swap2Params {
        pub amount_in: u64,
        pub min_amount_out: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TestInitParams {
        pub allow_swap: bool,
        pub allow_add_liquidity: bool,
        pub allow_remove_liquidity: bool,
        pub allow_increase_position: bool,
        pub allow_decrease_position: bool,
        pub allow_collateral_withdrawal: bool,
        pub allow_liquidate_position: bool,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TransferAdminParams {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateDecreasePositionRequest2Params {
        pub size_usd_delta: u64,
        pub trigger_price: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WithdrawFees2Params {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PriceImpactBuffer {
        #[serde(with = "BigArray")]
        pub open_interest: [i64; 60usize],
        pub last_updated: i64,
        pub fee_factor: u64,
        pub exponent: f32,
        pub delta_imbalance_threshold_decimal: u64,
        pub max_fee_bps: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Assets {
        pub fees_reserves: u64,
        pub owned: u64,
        pub locked: u64,
        pub guaranteed_usd: u64,
        pub global_short_sizes: u64,
        pub global_short_average_prices: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PricingParams {
        pub trade_impact_fee_scalar: u64,
        pub buffer: u64,
        pub swap_spread: u64,
        pub max_leverage: u64,
        pub max_global_long_sizes: u64,
        pub max_global_short_sizes: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FundingRateState {
        pub cumulative_interest_rate: u128,
        pub last_update: i64,
        pub hourly_funding_dbps: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct JumpRateState {
        pub min_rate_bps: u64,
        pub max_rate_bps: u64,
        pub target_rate_bps: u64,
        pub target_utilization_rate: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OraclePrice {
        pub price: u64,
        pub exponent: i32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Price {
        pub price: u64,
        pub expo: i32,
        pub publish_time: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OracleParams {
        #[serde(with = "pubkey_serde")]
        pub oracle_account: [u8; 32usize],
        pub oracle_type: OracleType,
        pub buffer: u64,
        pub max_price_age_sec: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AmountAndFee {
        pub amount: u64,
        pub fee: u64,
        pub fee_bps: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Permissions {
        pub allow_swap: bool,
        pub allow_add_liquidity: bool,
        pub allow_remove_liquidity: bool,
        pub allow_increase_position: bool,
        pub allow_decrease_position: bool,
        pub allow_collateral_withdrawal: bool,
        pub allow_liquidate_position: bool,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Fees {
        pub swap_multiplier: u64,
        pub stable_swap_multiplier: u64,
        pub add_remove_liquidity_bps: u64,
        pub swap_bps: u64,
        pub tax_bps: u64,
        pub stable_swap_bps: u64,
        pub stable_swap_tax_bps: u64,
        pub liquidation_reward_bps: u64,
        pub protocol_share_bps: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PoolApr {
        pub last_updated: i64,
        pub fee_apr_bps: u64,
        pub realized_fee_usd: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Limit {
        pub max_aum_usd: u128,
        pub token_weightage_buffer_bps: u128,
        pub buffer: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PriceImpactMechanism {
        TradeSize,
        DeltaImbalance,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OracleType {
        None,
        Test,
        Pyth,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PriceCalcMode {
        Min,
        Max,
        Ignore,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PriceStaleTolerance {
        Strict,
        Loose,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum TradePoolType {
        Increase,
        Decrease,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RequestType {
        Market,
        Trigger,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RequestChange {
        None,
        Increase,
        Decrease,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Side {
        None,
        Long,
        Short,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitAccounts {
        pub upgradeAuthority: String,
        pub admin: String,
        pub transferAuthority: String,
        pub perpetuals: String,
        pub perpetualsProgram: String,
        pub perpetualsProgramData: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddPoolAccounts {
        pub admin: String,
        pub transferAuthority: String,
        pub perpetuals: String,
        pub pool: String,
        pub lpTokenMint: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddCustodyAccounts {
        pub admin: String,
        pub transferAuthority: String,
        pub perpetuals: String,
        pub pool: String,
        pub custody: String,
        pub custodyTokenAccount: String,
        pub custodyTokenMint: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetCustodyConfigAccounts {
        pub admin: String,
        pub perpetuals: String,
        pub custody: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetPoolConfigAccounts {
        pub admin: String,
        pub perpetuals: String,
        pub pool: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetPerpetualsConfigAccounts {
        pub admin: String,
        pub perpetuals: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferAdminAccounts {
        pub admin: String,
        pub newAdmin: String,
        pub perpetuals: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawFees2Accounts {
        pub keeper: String,
        pub transferAuthority: String,
        pub perpetuals: String,
        pub pool: String,
        pub custody: String,
        pub custodyTokenAccount: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub receivingTokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateTokenMetadataAccounts {
        pub admin: String,
        pub perpetuals: String,
        pub pool: String,
        pub transferAuthority: String,
        pub metadata: String,
        pub lpTokenMint: String,
        pub tokenMetadataProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateTokenLedgerAccounts {
        pub tokenLedger: String,
        pub payer: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ReallocCustodyAccounts {
        pub keeper: String,
        pub custody: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ReallocPoolAccounts {
        pub keeper: String,
        pub pool: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OperatorSetCustodyConfigAccounts {
        pub operator: String,
        pub custody: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OperatorSetPoolConfigAccounts {
        pub operator: String,
        pub pool: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TestInitAccounts {
        pub upgradeAuthority: String,
        pub admin: String,
        pub transferAuthority: String,
        pub perpetuals: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetTestTimeAccounts {
        pub admin: String,
        pub perpetuals: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetTokenLedgerAccounts {
        pub tokenLedger: String,
        pub tokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct Swap2Accounts {
        pub owner: String,
        pub fundingAccount: String,
        pub receivingAccount: String,
        pub transferAuthority: String,
        pub perpetuals: String,
        pub pool: String,
        pub receivingCustody: String,
        pub receivingCustodyDovesPriceAccount: String,
        pub receivingCustodyPythnetPriceAccount: String,
        pub receivingCustodyTokenAccount: String,
        pub dispensingCustody: String,
        pub dispensingCustodyDovesPriceAccount: String,
        pub dispensingCustodyPythnetPriceAccount: String,
        pub dispensingCustodyTokenAccount: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidity2Accounts {
        pub owner: String,
        pub fundingAccount: String,
        pub lpTokenAccount: String,
        pub transferAuthority: String,
        pub perpetuals: String,
        pub pool: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub custodyTokenAccount: String,
        pub lpTokenMint: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveLiquidity2Accounts {
        pub owner: String,
        pub receivingAccount: String,
        pub lpTokenAccount: String,
        pub transferAuthority: String,
        pub perpetuals: String,
        pub pool: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub custodyTokenAccount: String,
        pub lpTokenMint: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateIncreasePositionMarketRequestAccounts {
        pub owner: String,
        pub fundingAccount: String,
        pub perpetuals: String,
        pub pool: String,
        pub position: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub custody: String,
        pub collateralCustody: String,
        pub inputMint: String,
        pub referral: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateDecreasePositionRequest2Accounts {
        pub owner: String,
        pub receivingAccount: String,
        pub perpetuals: String,
        pub pool: String,
        pub position: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub collateralCustody: String,
        pub desiredMint: String,
        pub referral: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateDecreasePositionMarketRequestAccounts {
        pub owner: String,
        pub receivingAccount: String,
        pub perpetuals: String,
        pub pool: String,
        pub position: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub custody: String,
        pub collateralCustody: String,
        pub desiredMint: String,
        pub referral: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateDecreasePositionRequest2Accounts {
        pub owner: String,
        pub perpetuals: String,
        pub pool: String,
        pub position: String,
        pub positionRequest: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClosePositionRequestAccounts {
        pub keeper: String,
        pub owner: String,
        pub ownerAta: String,
        pub pool: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub position: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClosePositionRequest2Accounts {
        pub keeper: String,
        pub owner: String,
        pub ownerAta: String,
        pub pool: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub position: String,
        pub mint: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub associatedTokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct IncreasePosition4Accounts {
        pub keeper: String,
        pub perpetuals: String,
        pub pool: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub position: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub collateralCustody: String,
        pub collateralCustodyDovesPriceAccount: String,
        pub collateralCustodyPythnetPriceAccount: String,
        pub collateralCustodyTokenAccount: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct IncreasePositionPreSwapAccounts {
        pub keeper: String,
        pub keeperAta: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub position: String,
        pub collateralCustody: String,
        pub collateralCustodyTokenAccount: String,
        pub instruction: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct IncreasePositionWithInternalSwapAccounts {
        pub keeper: String,
        pub perpetuals: String,
        pub pool: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub position: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub collateralCustody: String,
        pub collateralCustodyDovesPriceAccount: String,
        pub collateralCustodyPythnetPriceAccount: String,
        pub collateralCustodyTokenAccount: String,
        pub receivingCustody: String,
        pub receivingCustodyDovesPriceAccount: String,
        pub receivingCustodyPythnetPriceAccount: String,
        pub receivingCustodyTokenAccount: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DecreasePosition4Accounts {
        pub keeper: String,
        pub owner: String,
        pub transferAuthority: String,
        pub perpetuals: String,
        pub pool: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub position: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub collateralCustody: String,
        pub collateralCustodyDovesPriceAccount: String,
        pub collateralCustodyPythnetPriceAccount: String,
        pub collateralCustodyTokenAccount: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DecreasePositionWithInternalSwapAccounts {
        pub keeper: String,
        pub owner: String,
        pub transferAuthority: String,
        pub perpetuals: String,
        pub pool: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub position: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub collateralCustody: String,
        pub collateralCustodyDovesPriceAccount: String,
        pub collateralCustodyPythnetPriceAccount: String,
        pub collateralCustodyTokenAccount: String,
        pub dispensingCustody: String,
        pub dispensingCustodyDovesPriceAccount: String,
        pub dispensingCustodyPythnetPriceAccount: String,
        pub dispensingCustodyTokenAccount: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LiquidateFullPosition4Accounts {
        pub signer: String,
        pub perpetuals: String,
        pub pool: String,
        pub position: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub collateralCustody: String,
        pub collateralCustodyDovesPriceAccount: String,
        pub collateralCustodyPythnetPriceAccount: String,
        pub collateralCustodyTokenAccount: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RefreshAssetsUnderManagementAccounts {
        pub keeper: String,
        pub perpetuals: String,
        pub pool: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InstantCreateTpslAccounts {
        pub keeper: String,
        pub apiKeeper: String,
        pub owner: String,
        pub receivingAccount: String,
        pub perpetuals: String,
        pub pool: String,
        pub position: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub collateralCustody: String,
        pub desiredMint: String,
        pub referral: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InstantCreateLimitOrderAccounts {
        pub keeper: String,
        pub apiKeeper: String,
        pub owner: String,
        pub fundingAccount: String,
        pub perpetuals: String,
        pub pool: String,
        pub position: String,
        pub positionRequest: String,
        pub positionRequestAta: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub collateralCustody: String,
        pub inputMint: String,
        pub referral: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InstantIncreasePositionAccounts {
        pub keeper: String,
        pub apiKeeper: String,
        pub owner: String,
        pub fundingAccount: String,
        pub perpetuals: String,
        pub pool: String,
        pub position: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub collateralCustody: String,
        pub collateralCustodyDovesPriceAccount: String,
        pub collateralCustodyPythnetPriceAccount: String,
        pub collateralCustodyTokenAccount: String,
        pub tokenLedger: String,
        pub referral: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InstantDecreasePositionAccounts {
        pub keeper: String,
        pub apiKeeper: String,
        pub owner: String,
        pub receivingAccount: String,
        pub transferAuthority: String,
        pub perpetuals: String,
        pub pool: String,
        pub position: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub collateralCustody: String,
        pub collateralCustodyDovesPriceAccount: String,
        pub collateralCustodyPythnetPriceAccount: String,
        pub collateralCustodyTokenAccount: String,
        pub desiredMint: String,
        pub referral: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InstantUpdateLimitOrderAccounts {
        pub keeper: String,
        pub apiKeeper: String,
        pub owner: String,
        pub perpetuals: String,
        pub pool: String,
        pub position: String,
        pub positionRequest: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InstantUpdateTpslAccounts {
        pub keeper: String,
        pub apiKeeper: String,
        pub owner: String,
        pub perpetuals: String,
        pub pool: String,
        pub position: String,
        pub positionRequest: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct GetAddLiquidityAmountAndFee2Accounts {
        pub perpetuals: String,
        pub pool: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub lpTokenMint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct GetRemoveLiquidityAmountAndFee2Accounts {
        pub perpetuals: String,
        pub pool: String,
        pub custody: String,
        pub custodyDovesPriceAccount: String,
        pub custodyPythnetPriceAccount: String,
        pub lpTokenMint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct GetAssetsUnderManagement2Accounts {
        pub perpetuals: String,
        pub pool: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    serde_big_array::big_array! { BigArray ; 60 , 64 , 51 , 72 , 128 , 256 }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitArguments {
        pub params: InitParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddPoolArguments {
        pub params: AddPoolParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddCustodyArguments {
        pub params: AddCustodyParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetCustodyConfigArguments {
        pub params: SetCustodyConfigParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetPoolConfigArguments {
        pub params: SetPoolConfigParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetPerpetualsConfigArguments {
        pub params: SetPerpetualsConfigParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferAdminArguments {
        pub params: TransferAdminParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawFees2Arguments {
        pub params: WithdrawFees2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateTokenMetadataArguments {
        pub params: CreateTokenMetadataParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateTokenLedgerArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ReallocCustodyArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ReallocPoolArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OperatorSetCustodyConfigArguments {
        pub params: OperatorSetCustodyConfigParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OperatorSetPoolConfigArguments {
        pub params: OperatorSetPoolConfigParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TestInitArguments {
        pub params: TestInitParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetTestTimeArguments {
        pub params: SetTestTimeParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetTokenLedgerArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct Swap2Arguments {
        pub params: Swap2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidity2Arguments {
        pub params: AddLiquidity2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveLiquidity2Arguments {
        pub params: RemoveLiquidity2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateIncreasePositionMarketRequestArguments {
        pub params: CreateIncreasePositionMarketRequestParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateDecreasePositionRequest2Arguments {
        pub params: CreateDecreasePositionRequest2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateDecreasePositionMarketRequestArguments {
        pub params: CreateDecreasePositionMarketRequestParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateDecreasePositionRequest2Arguments {
        pub params: UpdateDecreasePositionRequest2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClosePositionRequestArguments {
        pub params: ClosePositionRequestParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClosePositionRequest2Arguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreasePosition4Arguments {
        pub params: IncreasePosition4Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreasePositionPreSwapArguments {
        pub params: IncreasePositionPreSwapParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreasePositionWithInternalSwapArguments {
        pub params: IncreasePositionWithInternalSwapParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DecreasePosition4Arguments {
        pub params: DecreasePosition4Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DecreasePositionWithInternalSwapArguments {
        pub params: DecreasePositionWithInternalSwapParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LiquidateFullPosition4Arguments {
        pub params: LiquidateFullPosition4Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RefreshAssetsUnderManagementArguments {
        pub params: RefreshAssetsUnderManagementParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InstantCreateTpslArguments {
        pub params: InstantCreateTpslParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InstantCreateLimitOrderArguments {
        pub params: InstantCreateLimitOrderParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InstantIncreasePositionArguments {
        pub params: InstantIncreasePositionParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InstantDecreasePositionArguments {
        pub params: InstantDecreasePositionParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InstantUpdateLimitOrderArguments {
        pub params: InstantUpdateLimitOrderParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InstantUpdateTpslArguments {
        pub params: InstantUpdateTpslParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct GetAddLiquidityAmountAndFee2Arguments {
        pub params: GetAddLiquidityAmountAndFee2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct GetRemoveLiquidityAmountAndFee2Arguments {
        pub params: GetRemoveLiquidityAmountAndFee2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct GetAssetsUnderManagement2Arguments {
        pub params: GetAssetsUnderManagement2Params,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Init {
        accounts: InitAccounts,
        args: InitArguments,
    },
    AddPool {
        accounts: AddPoolAccounts,
        args: AddPoolArguments,
    },
    AddCustody {
        accounts: AddCustodyAccounts,
        args: AddCustodyArguments,
    },
    SetCustodyConfig {
        accounts: SetCustodyConfigAccounts,
        args: SetCustodyConfigArguments,
    },
    SetPoolConfig {
        accounts: SetPoolConfigAccounts,
        args: SetPoolConfigArguments,
    },
    SetPerpetualsConfig {
        accounts: SetPerpetualsConfigAccounts,
        args: SetPerpetualsConfigArguments,
    },
    TransferAdmin {
        accounts: TransferAdminAccounts,
        args: TransferAdminArguments,
    },
    WithdrawFees2 {
        accounts: WithdrawFees2Accounts,
        args: WithdrawFees2Arguments,
    },
    CreateTokenMetadata {
        accounts: CreateTokenMetadataAccounts,
        args: CreateTokenMetadataArguments,
    },
    CreateTokenLedger {
        accounts: CreateTokenLedgerAccounts,
        args: CreateTokenLedgerArguments,
    },
    ReallocCustody {
        accounts: ReallocCustodyAccounts,
        args: ReallocCustodyArguments,
    },
    ReallocPool {
        accounts: ReallocPoolAccounts,
        args: ReallocPoolArguments,
    },
    OperatorSetCustodyConfig {
        accounts: OperatorSetCustodyConfigAccounts,
        args: OperatorSetCustodyConfigArguments,
    },
    OperatorSetPoolConfig {
        accounts: OperatorSetPoolConfigAccounts,
        args: OperatorSetPoolConfigArguments,
    },
    TestInit {
        accounts: TestInitAccounts,
        args: TestInitArguments,
    },
    SetTestTime {
        accounts: SetTestTimeAccounts,
        args: SetTestTimeArguments,
    },
    SetTokenLedger {
        accounts: SetTokenLedgerAccounts,
        args: SetTokenLedgerArguments,
    },
    Swap2 {
        accounts: Swap2Accounts,
        args: Swap2Arguments,
    },
    AddLiquidity2 {
        accounts: AddLiquidity2Accounts,
        args: AddLiquidity2Arguments,
    },
    RemoveLiquidity2 {
        accounts: RemoveLiquidity2Accounts,
        args: RemoveLiquidity2Arguments,
    },
    CreateIncreasePositionMarketRequest {
        accounts: CreateIncreasePositionMarketRequestAccounts,
        args: CreateIncreasePositionMarketRequestArguments,
    },
    CreateDecreasePositionRequest2 {
        accounts: CreateDecreasePositionRequest2Accounts,
        args: CreateDecreasePositionRequest2Arguments,
    },
    CreateDecreasePositionMarketRequest {
        accounts: CreateDecreasePositionMarketRequestAccounts,
        args: CreateDecreasePositionMarketRequestArguments,
    },
    UpdateDecreasePositionRequest2 {
        accounts: UpdateDecreasePositionRequest2Accounts,
        args: UpdateDecreasePositionRequest2Arguments,
    },
    ClosePositionRequest {
        accounts: ClosePositionRequestAccounts,
        args: ClosePositionRequestArguments,
    },
    ClosePositionRequest2 {
        accounts: ClosePositionRequest2Accounts,
        args: ClosePositionRequest2Arguments,
    },
    IncreasePosition4 {
        accounts: IncreasePosition4Accounts,
        args: IncreasePosition4Arguments,
    },
    IncreasePositionPreSwap {
        accounts: IncreasePositionPreSwapAccounts,
        args: IncreasePositionPreSwapArguments,
    },
    IncreasePositionWithInternalSwap {
        accounts: IncreasePositionWithInternalSwapAccounts,
        args: IncreasePositionWithInternalSwapArguments,
    },
    DecreasePosition4 {
        accounts: DecreasePosition4Accounts,
        args: DecreasePosition4Arguments,
    },
    DecreasePositionWithInternalSwap {
        accounts: DecreasePositionWithInternalSwapAccounts,
        args: DecreasePositionWithInternalSwapArguments,
    },
    LiquidateFullPosition4 {
        accounts: LiquidateFullPosition4Accounts,
        args: LiquidateFullPosition4Arguments,
    },
    RefreshAssetsUnderManagement {
        accounts: RefreshAssetsUnderManagementAccounts,
        args: RefreshAssetsUnderManagementArguments,
    },
    InstantCreateTpsl {
        accounts: InstantCreateTpslAccounts,
        args: InstantCreateTpslArguments,
    },
    InstantCreateLimitOrder {
        accounts: InstantCreateLimitOrderAccounts,
        args: InstantCreateLimitOrderArguments,
    },
    InstantIncreasePosition {
        accounts: InstantIncreasePositionAccounts,
        args: InstantIncreasePositionArguments,
    },
    InstantDecreasePosition {
        accounts: InstantDecreasePositionAccounts,
        args: InstantDecreasePositionArguments,
    },
    InstantUpdateLimitOrder {
        accounts: InstantUpdateLimitOrderAccounts,
        args: InstantUpdateLimitOrderArguments,
    },
    InstantUpdateTpsl {
        accounts: InstantUpdateTpslAccounts,
        args: InstantUpdateTpslArguments,
    },
    GetAddLiquidityAmountAndFee2 {
        accounts: GetAddLiquidityAmountAndFee2Accounts,
        args: GetAddLiquidityAmountAndFee2Arguments,
    },
    GetRemoveLiquidityAmountAndFee2 {
        accounts: GetRemoveLiquidityAmountAndFee2Accounts,
        args: GetRemoveLiquidityAmountAndFee2Arguments,
    },
    GetAssetsUnderManagement2 {
        accounts: GetAssetsUnderManagement2Accounts,
        args: GetAssetsUnderManagement2Arguments,
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
            [220u8, 59u8, 207u8, 236u8, 108u8, 250u8, 47u8, 100u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let upgradeAuthority = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let perpetualsProgram = keys.next().unwrap().clone();
                let perpetualsProgramData = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitAccounts {
                    upgradeAuthority,
                    admin,
                    transferAuthority,
                    perpetuals,
                    perpetualsProgram,
                    perpetualsProgramData,
                    systemProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Init { accounts, args });
            }
            [115u8, 230u8, 212u8, 211u8, 175u8, 49u8, 39u8, 169u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddPoolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let lpTokenMint = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddPoolAccounts {
                    admin,
                    transferAuthority,
                    perpetuals,
                    pool,
                    lpTokenMint,
                    systemProgram,
                    tokenProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::AddPool { accounts, args });
            }
            [247u8, 254u8, 126u8, 17u8, 26u8, 6u8, 215u8, 117u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddCustodyArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyTokenAccount = keys.next().unwrap().clone();
                let custodyTokenMint = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddCustodyAccounts {
                    admin,
                    transferAuthority,
                    perpetuals,
                    pool,
                    custody,
                    custodyTokenAccount,
                    custodyTokenMint,
                    systemProgram,
                    tokenProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::AddCustody { accounts, args });
            }
            [133u8, 97u8, 130u8, 143u8, 215u8, 229u8, 36u8, 176u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetCustodyConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetCustodyConfigAccounts {
                    admin,
                    perpetuals,
                    custody,
                    remaining,
                };
                return Ok(Instruction::SetCustodyConfig { accounts, args });
            }
            [216u8, 87u8, 65u8, 125u8, 113u8, 110u8, 185u8, 120u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetPoolConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetPoolConfigAccounts {
                    admin,
                    perpetuals,
                    pool,
                    remaining,
                };
                return Ok(Instruction::SetPoolConfig { accounts, args });
            }
            [80u8, 72u8, 21u8, 191u8, 29u8, 121u8, 45u8, 111u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetPerpetualsConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetPerpetualsConfigAccounts {
                    admin,
                    perpetuals,
                    remaining,
                };
                return Ok(Instruction::SetPerpetualsConfig { accounts, args });
            }
            [42u8, 242u8, 66u8, 106u8, 228u8, 10u8, 111u8, 156u8] => {
                let mut rdr: &[u8] = rest;
                let args = TransferAdminArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let newAdmin = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferAdminAccounts {
                    admin,
                    newAdmin,
                    perpetuals,
                    remaining,
                };
                return Ok(Instruction::TransferAdmin { accounts, args });
            }
            [252u8, 128u8, 143u8, 145u8, 225u8, 221u8, 159u8, 207u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawFees2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyTokenAccount = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let receivingTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawFees2Accounts {
                    keeper,
                    transferAuthority,
                    perpetuals,
                    pool,
                    custody,
                    custodyTokenAccount,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    receivingTokenAccount,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawFees2 { accounts, args });
            }
            [221u8, 80u8, 176u8, 37u8, 153u8, 188u8, 160u8, 68u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateTokenMetadataArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let lpTokenMint = keys.next().unwrap().clone();
                let tokenMetadataProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateTokenMetadataAccounts {
                    admin,
                    perpetuals,
                    pool,
                    transferAuthority,
                    metadata,
                    lpTokenMint,
                    tokenMetadataProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::CreateTokenMetadata { accounts, args });
            }
            [232u8, 242u8, 197u8, 253u8, 240u8, 143u8, 129u8, 52u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateTokenLedgerArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let tokenLedger = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateTokenLedgerAccounts {
                    tokenLedger,
                    payer,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::CreateTokenLedger { accounts, args });
            }
            [123u8, 58u8, 109u8, 139u8, 133u8, 7u8, 225u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = ReallocCustodyArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ReallocCustodyAccounts {
                    keeper,
                    custody,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::ReallocCustody { accounts, args });
            }
            [114u8, 128u8, 37u8, 167u8, 71u8, 227u8, 40u8, 178u8] => {
                let mut rdr: &[u8] = rest;
                let args = ReallocPoolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ReallocPoolAccounts {
                    keeper,
                    pool,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::ReallocPool { accounts, args });
            }
            [166u8, 137u8, 92u8, 204u8, 145u8, 224u8, 24u8, 218u8] => {
                let mut rdr: &[u8] = rest;
                let args = OperatorSetCustodyConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let operator = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = OperatorSetCustodyConfigAccounts {
                    operator,
                    custody,
                    remaining,
                };
                return Ok(Instruction::OperatorSetCustodyConfig { accounts, args });
            }
            [76u8, 201u8, 80u8, 18u8, 199u8, 92u8, 246u8, 105u8] => {
                let mut rdr: &[u8] = rest;
                let args = OperatorSetPoolConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let operator = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = OperatorSetPoolConfigAccounts {
                    operator,
                    pool,
                    remaining,
                };
                return Ok(Instruction::OperatorSetPoolConfig { accounts, args });
            }
            [48u8, 51u8, 92u8, 122u8, 81u8, 19u8, 112u8, 41u8] => {
                let mut rdr: &[u8] = rest;
                let args = TestInitArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let upgradeAuthority = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TestInitAccounts {
                    upgradeAuthority,
                    admin,
                    transferAuthority,
                    perpetuals,
                    systemProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::TestInit { accounts, args });
            }
            [242u8, 231u8, 177u8, 251u8, 126u8, 145u8, 159u8, 104u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetTestTimeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let admin = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetTestTimeAccounts {
                    admin,
                    perpetuals,
                    remaining,
                };
                return Ok(Instruction::SetTestTime { accounts, args });
            }
            [228u8, 85u8, 185u8, 112u8, 78u8, 79u8, 77u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetTokenLedgerArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let tokenLedger = keys.next().unwrap().clone();
                let tokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetTokenLedgerAccounts {
                    tokenLedger,
                    tokenAccount,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::SetTokenLedger { accounts, args });
            }
            [65u8, 75u8, 63u8, 76u8, 235u8, 91u8, 91u8, 136u8] => {
                let mut rdr: &[u8] = rest;
                let args = Swap2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let fundingAccount = keys.next().unwrap().clone();
                let receivingAccount = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let receivingCustody = keys.next().unwrap().clone();
                let receivingCustodyDovesPriceAccount = keys.next().unwrap().clone();
                let receivingCustodyPythnetPriceAccount = keys.next().unwrap().clone();
                let receivingCustodyTokenAccount = keys.next().unwrap().clone();
                let dispensingCustody = keys.next().unwrap().clone();
                let dispensingCustodyDovesPriceAccount = keys.next().unwrap().clone();
                let dispensingCustodyPythnetPriceAccount = keys.next().unwrap().clone();
                let dispensingCustodyTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = Swap2Accounts {
                    owner,
                    fundingAccount,
                    receivingAccount,
                    transferAuthority,
                    perpetuals,
                    pool,
                    receivingCustody,
                    receivingCustodyDovesPriceAccount,
                    receivingCustodyPythnetPriceAccount,
                    receivingCustodyTokenAccount,
                    dispensingCustody,
                    dispensingCustodyDovesPriceAccount,
                    dispensingCustodyPythnetPriceAccount,
                    dispensingCustodyTokenAccount,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Swap2 { accounts, args });
            }
            [228u8, 162u8, 78u8, 28u8, 70u8, 219u8, 116u8, 115u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddLiquidity2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let fundingAccount = keys.next().unwrap().clone();
                let lpTokenAccount = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let custodyTokenAccount = keys.next().unwrap().clone();
                let lpTokenMint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidity2Accounts {
                    owner,
                    fundingAccount,
                    lpTokenAccount,
                    transferAuthority,
                    perpetuals,
                    pool,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    custodyTokenAccount,
                    lpTokenMint,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AddLiquidity2 { accounts, args });
            }
            [230u8, 215u8, 82u8, 127u8, 241u8, 101u8, 227u8, 146u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveLiquidity2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let receivingAccount = keys.next().unwrap().clone();
                let lpTokenAccount = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let custodyTokenAccount = keys.next().unwrap().clone();
                let lpTokenMint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveLiquidity2Accounts {
                    owner,
                    receivingAccount,
                    lpTokenAccount,
                    transferAuthority,
                    perpetuals,
                    pool,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    custodyTokenAccount,
                    lpTokenMint,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::RemoveLiquidity2 { accounts, args });
            }
            [184u8, 85u8, 199u8, 24u8, 105u8, 171u8, 156u8, 56u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateIncreasePositionMarketRequestArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let fundingAccount = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let referral = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateIncreasePositionMarketRequestAccounts {
                    owner,
                    fundingAccount,
                    perpetuals,
                    pool,
                    position,
                    positionRequest,
                    positionRequestAta,
                    custody,
                    collateralCustody,
                    inputMint,
                    referral,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CreateIncreasePositionMarketRequest { accounts, args });
            }
            [105u8, 64u8, 201u8, 82u8, 250u8, 14u8, 109u8, 77u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateDecreasePositionRequest2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let receivingAccount = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let desiredMint = keys.next().unwrap().clone();
                let referral = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateDecreasePositionRequest2Accounts {
                    owner,
                    receivingAccount,
                    perpetuals,
                    pool,
                    position,
                    positionRequest,
                    positionRequestAta,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    collateralCustody,
                    desiredMint,
                    referral,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CreateDecreasePositionRequest2 { accounts, args });
            }
            [74u8, 198u8, 195u8, 86u8, 193u8, 99u8, 1u8, 79u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateDecreasePositionMarketRequestArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let receivingAccount = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let desiredMint = keys.next().unwrap().clone();
                let referral = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateDecreasePositionMarketRequestAccounts {
                    owner,
                    receivingAccount,
                    perpetuals,
                    pool,
                    position,
                    positionRequest,
                    positionRequestAta,
                    custody,
                    collateralCustody,
                    desiredMint,
                    referral,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::CreateDecreasePositionMarketRequest { accounts, args });
            }
            [144u8, 200u8, 249u8, 255u8, 108u8, 217u8, 249u8, 116u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateDecreasePositionRequest2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateDecreasePositionRequest2Accounts {
                    owner,
                    perpetuals,
                    pool,
                    position,
                    positionRequest,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    remaining,
                };
                return Ok(Instruction::UpdateDecreasePositionRequest2 { accounts, args });
            }
            [40u8, 105u8, 217u8, 188u8, 220u8, 45u8, 109u8, 110u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClosePositionRequestArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let ownerAta = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClosePositionRequestAccounts {
                    keeper,
                    owner,
                    ownerAta,
                    pool,
                    positionRequest,
                    positionRequestAta,
                    position,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClosePositionRequest { accounts, args });
            }
            [121u8, 68u8, 162u8, 28u8, 216u8, 47u8, 200u8, 66u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClosePositionRequest2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let ownerAta = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClosePositionRequest2Accounts {
                    keeper,
                    owner,
                    ownerAta,
                    pool,
                    positionRequest,
                    positionRequestAta,
                    position,
                    mint,
                    tokenProgram,
                    systemProgram,
                    associatedTokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClosePositionRequest2 { accounts, args });
            }
            [67u8, 147u8, 53u8, 23u8, 43u8, 57u8, 16u8, 67u8] => {
                let mut rdr: &[u8] = rest;
                let args = IncreasePosition4Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let collateralCustodyDovesPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = IncreasePosition4Accounts {
                    keeper,
                    perpetuals,
                    pool,
                    positionRequest,
                    positionRequestAta,
                    position,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    collateralCustody,
                    collateralCustodyDovesPriceAccount,
                    collateralCustodyPythnetPriceAccount,
                    collateralCustodyTokenAccount,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::IncreasePosition4 { accounts, args });
            }
            [26u8, 136u8, 225u8, 217u8, 22u8, 21u8, 83u8, 20u8] => {
                let mut rdr: &[u8] = rest;
                let args = IncreasePositionPreSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let keeperAta = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let collateralCustodyTokenAccount = keys.next().unwrap().clone();
                let instruction = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = IncreasePositionPreSwapAccounts {
                    keeper,
                    keeperAta,
                    positionRequest,
                    positionRequestAta,
                    position,
                    collateralCustody,
                    collateralCustodyTokenAccount,
                    instruction,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::IncreasePositionPreSwap { accounts, args });
            }
            [114u8, 55u8, 106u8, 140u8, 199u8, 221u8, 32u8, 112u8] => {
                let mut rdr: &[u8] = rest;
                let args = IncreasePositionWithInternalSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let collateralCustodyDovesPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyTokenAccount = keys.next().unwrap().clone();
                let receivingCustody = keys.next().unwrap().clone();
                let receivingCustodyDovesPriceAccount = keys.next().unwrap().clone();
                let receivingCustodyPythnetPriceAccount = keys.next().unwrap().clone();
                let receivingCustodyTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = IncreasePositionWithInternalSwapAccounts {
                    keeper,
                    perpetuals,
                    pool,
                    positionRequest,
                    positionRequestAta,
                    position,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    collateralCustody,
                    collateralCustodyDovesPriceAccount,
                    collateralCustodyPythnetPriceAccount,
                    collateralCustodyTokenAccount,
                    receivingCustody,
                    receivingCustodyDovesPriceAccount,
                    receivingCustodyPythnetPriceAccount,
                    receivingCustodyTokenAccount,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::IncreasePositionWithInternalSwap { accounts, args });
            }
            [185u8, 161u8, 114u8, 175u8, 96u8, 148u8, 3u8, 170u8] => {
                let mut rdr: &[u8] = rest;
                let args = DecreasePosition4Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let collateralCustodyDovesPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DecreasePosition4Accounts {
                    keeper,
                    owner,
                    transferAuthority,
                    perpetuals,
                    pool,
                    positionRequest,
                    positionRequestAta,
                    position,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    collateralCustody,
                    collateralCustodyDovesPriceAccount,
                    collateralCustodyPythnetPriceAccount,
                    collateralCustodyTokenAccount,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::DecreasePosition4 { accounts, args });
            }
            [131u8, 17u8, 153u8, 110u8, 119u8, 100u8, 97u8, 38u8] => {
                let mut rdr: &[u8] = rest;
                let args = DecreasePositionWithInternalSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let collateralCustodyDovesPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyTokenAccount = keys.next().unwrap().clone();
                let dispensingCustody = keys.next().unwrap().clone();
                let dispensingCustodyDovesPriceAccount = keys.next().unwrap().clone();
                let dispensingCustodyPythnetPriceAccount = keys.next().unwrap().clone();
                let dispensingCustodyTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DecreasePositionWithInternalSwapAccounts {
                    keeper,
                    owner,
                    transferAuthority,
                    perpetuals,
                    pool,
                    positionRequest,
                    positionRequestAta,
                    position,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    collateralCustody,
                    collateralCustodyDovesPriceAccount,
                    collateralCustodyPythnetPriceAccount,
                    collateralCustodyTokenAccount,
                    dispensingCustody,
                    dispensingCustodyDovesPriceAccount,
                    dispensingCustodyPythnetPriceAccount,
                    dispensingCustodyTokenAccount,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::DecreasePositionWithInternalSwap { accounts, args });
            }
            [64u8, 176u8, 88u8, 51u8, 168u8, 188u8, 156u8, 175u8] => {
                let mut rdr: &[u8] = rest;
                let args = LiquidateFullPosition4Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let signer = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let collateralCustodyDovesPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyTokenAccount = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LiquidateFullPosition4Accounts {
                    signer,
                    perpetuals,
                    pool,
                    position,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    collateralCustody,
                    collateralCustodyDovesPriceAccount,
                    collateralCustodyPythnetPriceAccount,
                    collateralCustodyTokenAccount,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::LiquidateFullPosition4 { accounts, args });
            }
            [162u8, 0u8, 215u8, 55u8, 225u8, 15u8, 185u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let args = RefreshAssetsUnderManagementArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RefreshAssetsUnderManagementAccounts {
                    keeper,
                    perpetuals,
                    pool,
                    remaining,
                };
                return Ok(Instruction::RefreshAssetsUnderManagement { accounts, args });
            }
            [117u8, 98u8, 66u8, 127u8, 30u8, 50u8, 73u8, 185u8] => {
                let mut rdr: &[u8] = rest;
                let args = InstantCreateTpslArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let apiKeeper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let receivingAccount = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let desiredMint = keys.next().unwrap().clone();
                let referral = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InstantCreateTpslAccounts {
                    keeper,
                    apiKeeper,
                    owner,
                    receivingAccount,
                    perpetuals,
                    pool,
                    position,
                    positionRequest,
                    positionRequestAta,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    collateralCustody,
                    desiredMint,
                    referral,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InstantCreateTpsl { accounts, args });
            }
            [194u8, 37u8, 195u8, 123u8, 40u8, 127u8, 126u8, 156u8] => {
                let mut rdr: &[u8] = rest;
                let args = InstantCreateLimitOrderArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let apiKeeper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let fundingAccount = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let positionRequestAta = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let referral = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InstantCreateLimitOrderAccounts {
                    keeper,
                    apiKeeper,
                    owner,
                    fundingAccount,
                    perpetuals,
                    pool,
                    position,
                    positionRequest,
                    positionRequestAta,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    collateralCustody,
                    inputMint,
                    referral,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InstantCreateLimitOrder { accounts, args });
            }
            [164u8, 126u8, 68u8, 182u8, 223u8, 166u8, 64u8, 183u8] => {
                let mut rdr: &[u8] = rest;
                let args = InstantIncreasePositionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let apiKeeper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let fundingAccount = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let collateralCustodyDovesPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyTokenAccount = keys.next().unwrap().clone();
                let tokenLedger = keys.next().unwrap().clone();
                let referral = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InstantIncreasePositionAccounts {
                    keeper,
                    apiKeeper,
                    owner,
                    fundingAccount,
                    perpetuals,
                    pool,
                    position,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    collateralCustody,
                    collateralCustodyDovesPriceAccount,
                    collateralCustodyPythnetPriceAccount,
                    collateralCustodyTokenAccount,
                    tokenLedger,
                    referral,
                    tokenProgram,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InstantIncreasePosition { accounts, args });
            }
            [46u8, 23u8, 240u8, 44u8, 30u8, 138u8, 94u8, 140u8] => {
                let mut rdr: &[u8] = rest;
                let args = InstantDecreasePositionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let apiKeeper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let receivingAccount = keys.next().unwrap().clone();
                let transferAuthority = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustody = keys.next().unwrap().clone();
                let collateralCustodyDovesPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyPythnetPriceAccount = keys.next().unwrap().clone();
                let collateralCustodyTokenAccount = keys.next().unwrap().clone();
                let desiredMint = keys.next().unwrap().clone();
                let referral = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InstantDecreasePositionAccounts {
                    keeper,
                    apiKeeper,
                    owner,
                    receivingAccount,
                    transferAuthority,
                    perpetuals,
                    pool,
                    position,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    collateralCustody,
                    collateralCustodyDovesPriceAccount,
                    collateralCustodyPythnetPriceAccount,
                    collateralCustodyTokenAccount,
                    desiredMint,
                    referral,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InstantDecreasePosition { accounts, args });
            }
            [136u8, 245u8, 229u8, 58u8, 121u8, 141u8, 12u8, 207u8] => {
                let mut rdr: &[u8] = rest;
                let args = InstantUpdateLimitOrderArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let apiKeeper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InstantUpdateLimitOrderAccounts {
                    keeper,
                    apiKeeper,
                    owner,
                    perpetuals,
                    pool,
                    position,
                    positionRequest,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    remaining,
                };
                return Ok(Instruction::InstantUpdateLimitOrder { accounts, args });
            }
            [144u8, 228u8, 114u8, 37u8, 165u8, 242u8, 111u8, 101u8] => {
                let mut rdr: &[u8] = rest;
                let args = InstantUpdateTpslArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let keeper = keys.next().unwrap().clone();
                let apiKeeper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let positionRequest = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InstantUpdateTpslAccounts {
                    keeper,
                    apiKeeper,
                    owner,
                    perpetuals,
                    pool,
                    position,
                    positionRequest,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InstantUpdateTpsl { accounts, args });
            }
            [109u8, 157u8, 55u8, 169u8, 8u8, 81u8, 4u8, 118u8] => {
                let mut rdr: &[u8] = rest;
                let args = GetAddLiquidityAmountAndFee2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let lpTokenMint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = GetAddLiquidityAmountAndFee2Accounts {
                    perpetuals,
                    pool,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    lpTokenMint,
                    remaining,
                };
                return Ok(Instruction::GetAddLiquidityAmountAndFee2 { accounts, args });
            }
            [183u8, 59u8, 72u8, 110u8, 223u8, 243u8, 150u8, 142u8] => {
                let mut rdr: &[u8] = rest;
                let args = GetRemoveLiquidityAmountAndFee2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let custody = keys.next().unwrap().clone();
                let custodyDovesPriceAccount = keys.next().unwrap().clone();
                let custodyPythnetPriceAccount = keys.next().unwrap().clone();
                let lpTokenMint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = GetRemoveLiquidityAmountAndFee2Accounts {
                    perpetuals,
                    pool,
                    custody,
                    custodyDovesPriceAccount,
                    custodyPythnetPriceAccount,
                    lpTokenMint,
                    remaining,
                };
                return Ok(Instruction::GetRemoveLiquidityAmountAndFee2 { accounts, args });
            }
            [193u8, 210u8, 13u8, 249u8, 113u8, 149u8, 29u8, 84u8] => {
                let mut rdr: &[u8] = rest;
                let args = GetAssetsUnderManagement2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let perpetuals = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = GetAssetsUnderManagement2Accounts {
                    perpetuals,
                    pool,
                    remaining,
                };
                return Ok(Instruction::GetAssetsUnderManagement2 { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use borsh::BorshDeserialize;
    use serde::Serialize;
    pub use typedefs::*;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreatePositionRequestEvent {
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_key: [u8; 32usize],
        pub position_side: u8,
        #[serde(with = "pubkey_serde")]
        pub position_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_mint: [u8; 32usize],
        pub size_usd_delta: u64,
        pub collateral_delta: u64,
        pub price_slippage: Option<u64>,
        pub jupiter_minimum_out: Option<u64>,
        pub pre_swap_amount: Option<u64>,
        pub request_change: u8,
        pub open_time: i64,
        #[serde(with = "pubkey_serde_option")]
        pub referral: Option<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InstantCreateTpslEvent {
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_key: [u8; 32usize],
        pub position_side: u8,
        #[serde(with = "pubkey_serde")]
        pub position_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_mint: [u8; 32usize],
        pub size_usd_delta: u64,
        pub collateral_delta: u64,
        pub entire_position: bool,
        pub open_time: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InstantUpdateTpslEvent {
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_key: [u8; 32usize],
        pub position_side: u8,
        #[serde(with = "pubkey_serde")]
        pub position_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_mint: [u8; 32usize],
        pub size_usd_delta: u64,
        pub collateral_delta: u64,
        pub entire_position: bool,
        pub update_time: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClosePositionRequestEvent {
        pub entire_position: Option<bool>,
        pub executed: bool,
        pub request_change: u8,
        pub request_type: u8,
        pub side: u8,
        #[serde(with = "pubkey_serde")]
        pub position_request_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub mint: [u8; 32usize],
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreasePositionEvent {
        #[serde(with = "pubkey_serde")]
        pub position_key: [u8; 32usize],
        pub position_side: u8,
        #[serde(with = "pubkey_serde")]
        pub position_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_custody: [u8; 32usize],
        pub position_size_usd: u64,
        #[serde(with = "pubkey_serde")]
        pub position_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_mint: [u8; 32usize],
        pub position_request_change: u8,
        pub position_request_type: u8,
        pub position_request_collateral_delta: u64,
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        pub size_usd_delta: u64,
        pub collateral_usd_delta: u64,
        pub collateral_token_delta: u64,
        pub price: u64,
        pub price_slippage: Option<u64>,
        pub fee_token: u64,
        pub fee_usd: u64,
        pub open_time: i64,
        #[serde(with = "pubkey_serde_option")]
        pub referral: Option<[u8; 32usize]>,
        pub position_fee_usd: u64,
        pub funding_fee_usd: u64,
        pub price_impact_fee_usd: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreasePositionPreSwapEvent {
        #[serde(with = "pubkey_serde")]
        pub position_request_key: [u8; 32usize],
        pub transfer_amount: u64,
        pub collateral_custody_pre_swap_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DecreasePositionEvent {
        #[serde(with = "pubkey_serde")]
        pub position_key: [u8; 32usize],
        pub position_side: u8,
        #[serde(with = "pubkey_serde")]
        pub position_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_custody: [u8; 32usize],
        pub position_size_usd: u64,
        #[serde(with = "pubkey_serde")]
        pub position_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_mint: [u8; 32usize],
        pub position_request_change: u8,
        pub position_request_type: u8,
        pub has_profit: bool,
        pub pnl_delta: u64,
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        pub size_usd_delta: u64,
        pub transfer_amount_usd: u64,
        pub transfer_token: Option<u64>,
        pub price: u64,
        pub price_slippage: Option<u64>,
        pub fee_usd: u64,
        pub open_time: i64,
        #[serde(with = "pubkey_serde_option")]
        pub referral: Option<[u8; 32usize]>,
        pub position_fee_usd: u64,
        pub funding_fee_usd: u64,
        pub price_impact_fee_usd: u64,
        pub original_position_collateral_usd: u64,
        pub position_collateral_usd: u64,
        pub position_open_time: i64,
        pub position_price: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DecreasePositionPostSwapEvent {
        #[serde(with = "pubkey_serde")]
        pub position_request_key: [u8; 32usize],
        pub swap_amount: u64,
        pub jupiter_minimum_out: Option<u64>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LiquidateFullPositionEvent {
        #[serde(with = "pubkey_serde")]
        pub position_key: [u8; 32usize],
        pub position_side: u8,
        #[serde(with = "pubkey_serde")]
        pub position_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_mint: [u8; 32usize],
        pub position_size_usd: u64,
        pub has_profit: bool,
        pub pnl_delta: u64,
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        pub transfer_amount_usd: u64,
        pub transfer_token: u64,
        pub price: u64,
        pub fee_usd: u64,
        pub liquidation_fee_usd: u64,
        pub open_time: i64,
        pub position_fee_usd: u64,
        pub funding_fee_usd: u64,
        pub price_impact_fee_usd: u64,
        pub original_position_collateral_usd: u64,
        pub position_collateral_usd: u64,
        pub position_open_time: i64,
        pub position_price: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PoolSwapEvent {
        #[serde(with = "pubkey_serde")]
        pub receiving_custody_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub dispensing_custody_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool_key: [u8; 32usize],
        pub amount_in: u64,
        pub amount_out: u64,
        pub swap_usd_amount: u64,
        pub amount_out_after_fees: u64,
        pub fee_bps: u64,
        #[serde(with = "pubkey_serde")]
        pub owner_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub receiving_account_key: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PoolSwapExactOutEvent {
        #[serde(with = "pubkey_serde")]
        pub receiving_custody_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub dispensing_custody_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool_key: [u8; 32usize],
        pub amount_in: u64,
        pub amount_in_after_fees: u64,
        pub amount_out: u64,
        pub swap_usd_amount: u64,
        pub fee_bps: u64,
        #[serde(with = "pubkey_serde")]
        pub owner_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub receiving_account_key: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidityEvent {
        #[serde(with = "pubkey_serde")]
        pub custody_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool_key: [u8; 32usize],
        pub token_amount_in: u64,
        pub pre_pool_amount_usd: u128,
        pub token_amount_usd: u64,
        pub fee_bps: u64,
        pub token_amount_after_fee: u64,
        pub mint_amount_usd: u64,
        pub lp_amount: u64,
        pub post_pool_amount_usd: u128,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveLiquidityEvent {
        #[serde(with = "pubkey_serde")]
        pub custody_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool_key: [u8; 32usize],
        pub lp_amount_in: u64,
        pub remove_amount_usd: u64,
        pub fee_bps: u64,
        pub remove_token_amount: u64,
        pub token_amount_after_fee: u64,
        pub post_pool_amount_usd: u128,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InstantCreateLimitOrderEvent {
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_key: [u8; 32usize],
        pub position_side: u8,
        #[serde(with = "pubkey_serde")]
        pub position_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_key: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_request_mint: [u8; 32usize],
        pub size_usd_delta: u64,
        pub collateral_delta: u64,
        pub open_time: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InstantIncreasePositionEvent {
        #[serde(with = "pubkey_serde")]
        pub position_key: [u8; 32usize],
        pub position_side: u8,
        #[serde(with = "pubkey_serde")]
        pub position_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_custody: [u8; 32usize],
        pub position_size_usd: u64,
        #[serde(with = "pubkey_serde")]
        pub position_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        pub size_usd_delta: u64,
        pub collateral_usd_delta: u64,
        pub collateral_token_delta: u64,
        pub price: u64,
        pub price_slippage: u64,
        pub fee_token: u64,
        pub fee_usd: u64,
        pub open_time: i64,
        #[serde(with = "pubkey_serde_option")]
        pub referral: Option<[u8; 32usize]>,
        pub position_fee_usd: u64,
        pub funding_fee_usd: u64,
        pub price_impact_fee_usd: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InstantDecreasePositionEvent {
        #[serde(with = "pubkey_serde")]
        pub position_key: [u8; 32usize],
        pub position_side: u8,
        #[serde(with = "pubkey_serde")]
        pub position_custody: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position_collateral_custody: [u8; 32usize],
        pub position_size_usd: u64,
        #[serde(with = "pubkey_serde")]
        pub position_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub desired_mint: [u8; 32usize],
        pub has_profit: bool,
        pub pnl_delta: u64,
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub pool: [u8; 32usize],
        pub size_usd_delta: u64,
        pub transfer_amount_usd: u64,
        pub transfer_token: u64,
        pub price: u64,
        pub price_slippage: u64,
        pub fee_usd: u64,
        pub open_time: i64,
        #[serde(with = "pubkey_serde_option")]
        pub referral: Option<[u8; 32usize]>,
        pub position_fee_usd: u64,
        pub funding_fee_usd: u64,
        pub original_position_collateral_usd: u64,
        pub position_collateral_usd: u64,
        pub price_impact_fee_usd: u64,
        pub position_open_time: i64,
        pub position_price: u64,
    }
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        CreatePositionRequestEvent { args: CreatePositionRequestEvent },
        InstantCreateTpslEvent { args: InstantCreateTpslEvent },
        InstantUpdateTpslEvent { args: InstantUpdateTpslEvent },
        ClosePositionRequestEvent { args: ClosePositionRequestEvent },
        IncreasePositionEvent { args: IncreasePositionEvent },
        IncreasePositionPreSwapEvent { args: IncreasePositionPreSwapEvent },
        DecreasePositionEvent { args: DecreasePositionEvent },
        DecreasePositionPostSwapEvent { args: DecreasePositionPostSwapEvent },
        LiquidateFullPositionEvent { args: LiquidateFullPositionEvent },
        PoolSwapEvent { args: PoolSwapEvent },
        PoolSwapExactOutEvent { args: PoolSwapExactOutEvent },
        AddLiquidityEvent { args: AddLiquidityEvent },
        RemoveLiquidityEvent { args: RemoveLiquidityEvent },
        InstantCreateLimitOrderEvent { args: InstantCreateLimitOrderEvent },
        InstantIncreasePositionEvent { args: InstantIncreasePositionEvent },
        InstantDecreasePositionEvent { args: InstantDecreasePositionEvent },
    }
    pub const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
    impl Event {
        #[doc = r" Decode a raw Anchor‐logged event:"]
        #[doc = r"  [ EVENT_LOG_DISCRIMINATOR (8) ]"]
        #[doc = r"  [ REAL_EVENT_DISCRIMINATOR (8) ]"]
        #[doc = r"  [ Borsh payload …           ]"]
        pub fn decode(data: &[u8]) -> anyhow::Result<Self> {
            if data.len() < 16 {
                anyhow::bail!("Event log too short: {}", data.len());
            }
            let (wrapper, rest) = data.split_at(8);
            if wrapper != EVENT_LOG_DISCRIMINATOR {
                anyhow::bail!(
                    "Missing event log prefix: expected {:x?}, got {:x?}",
                    EVENT_LOG_DISCRIMINATOR,
                    wrapper
                );
            }
            let (disc_slice, payload) = rest.split_at(8);
            let disc: [u8; 8] = disc_slice.try_into().unwrap();
            match disc {
                [2u8, 238u8, 94u8, 53u8, 105u8, 211u8, 46u8, 186u8] => {
                    let mut rdr = &payload[..];
                    let args = CreatePositionRequestEvent::deserialize(&mut rdr)?;
                    return Ok(Event::CreatePositionRequestEvent { args });
                }
                [242u8, 54u8, 6u8, 95u8, 24u8, 141u8, 103u8, 198u8] => {
                    let mut rdr = &payload[..];
                    let args = InstantCreateTpslEvent::deserialize(&mut rdr)?;
                    return Ok(Event::InstantCreateTpslEvent { args });
                }
                [177u8, 22u8, 47u8, 37u8, 120u8, 246u8, 17u8, 101u8] => {
                    let mut rdr = &payload[..];
                    let args = InstantUpdateTpslEvent::deserialize(&mut rdr)?;
                    return Ok(Event::InstantUpdateTpslEvent { args });
                }
                [21u8, 34u8, 92u8, 158u8, 224u8, 29u8, 180u8, 243u8] => {
                    let mut rdr = &payload[..];
                    let args = ClosePositionRequestEvent::deserialize(&mut rdr)?;
                    return Ok(Event::ClosePositionRequestEvent { args });
                }
                [245u8, 113u8, 85u8, 52u8, 214u8, 187u8, 153u8, 132u8] => {
                    let mut rdr = &payload[..];
                    let args = IncreasePositionEvent::deserialize(&mut rdr)?;
                    return Ok(Event::IncreasePositionEvent { args });
                }
                [237u8, 107u8, 9u8, 139u8, 22u8, 75u8, 4u8, 213u8] => {
                    let mut rdr = &payload[..];
                    let args = IncreasePositionPreSwapEvent::deserialize(&mut rdr)?;
                    return Ok(Event::IncreasePositionPreSwapEvent { args });
                }
                [64u8, 156u8, 43u8, 74u8, 109u8, 131u8, 16u8, 127u8] => {
                    let mut rdr = &payload[..];
                    let args = DecreasePositionEvent::deserialize(&mut rdr)?;
                    return Ok(Event::DecreasePositionEvent { args });
                }
                [23u8, 210u8, 16u8, 233u8, 98u8, 245u8, 89u8, 82u8] => {
                    let mut rdr = &payload[..];
                    let args = DecreasePositionPostSwapEvent::deserialize(&mut rdr)?;
                    return Ok(Event::DecreasePositionPostSwapEvent { args });
                }
                [128u8, 101u8, 71u8, 168u8, 128u8, 72u8, 86u8, 84u8] => {
                    let mut rdr = &payload[..];
                    let args = LiquidateFullPositionEvent::deserialize(&mut rdr)?;
                    return Ok(Event::LiquidateFullPositionEvent { args });
                }
                [40u8, 107u8, 212u8, 26u8, 223u8, 136u8, 39u8, 220u8] => {
                    let mut rdr = &payload[..];
                    let args = PoolSwapEvent::deserialize(&mut rdr)?;
                    return Ok(Event::PoolSwapEvent { args });
                }
                [121u8, 118u8, 11u8, 11u8, 198u8, 66u8, 142u8, 115u8] => {
                    let mut rdr = &payload[..];
                    let args = PoolSwapExactOutEvent::deserialize(&mut rdr)?;
                    return Ok(Event::PoolSwapExactOutEvent { args });
                }
                [27u8, 178u8, 153u8, 186u8, 47u8, 196u8, 140u8, 45u8] => {
                    let mut rdr = &payload[..];
                    let args = AddLiquidityEvent::deserialize(&mut rdr)?;
                    return Ok(Event::AddLiquidityEvent { args });
                }
                [141u8, 199u8, 182u8, 123u8, 159u8, 94u8, 215u8, 102u8] => {
                    let mut rdr = &payload[..];
                    let args = RemoveLiquidityEvent::deserialize(&mut rdr)?;
                    return Ok(Event::RemoveLiquidityEvent { args });
                }
                [10u8, 163u8, 85u8, 115u8, 129u8, 224u8, 80u8, 192u8] => {
                    let mut rdr = &payload[..];
                    let args = InstantCreateLimitOrderEvent::deserialize(&mut rdr)?;
                    return Ok(Event::InstantCreateLimitOrderEvent { args });
                }
                [205u8, 236u8, 57u8, 4u8, 209u8, 106u8, 87u8, 69u8] => {
                    let mut rdr = &payload[..];
                    let args = InstantIncreasePositionEvent::deserialize(&mut rdr)?;
                    return Ok(Event::InstantIncreasePositionEvent { args });
                }
                [171u8, 173u8, 106u8, 25u8, 239u8, 190u8, 58u8, 59u8] => {
                    let mut rdr = &payload[..];
                    let args = InstantDecreasePositionEvent::deserialize(&mut rdr)?;
                    return Ok(Event::InstantDecreasePositionEvent { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
