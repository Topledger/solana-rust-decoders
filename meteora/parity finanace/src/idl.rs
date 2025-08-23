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
    serde_big_array::big_array! { BigArray ; 60 , 62 , 64 , 51 , 72 , 96 , 128 , 256 }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitPresetParameters2Ix {
        pub index: u16,
        pub bin_step: u16,
        pub base_factor: u16,
        pub filter_period: u16,
        pub decay_period: u16,
        pub reduction_factor: u16,
        pub variable_fee_control: u32,
        pub max_volatility_accumulator: u32,
        pub protocol_share: u16,
        pub base_fee_power_factor: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitPresetParametersIx {
        pub bin_step: u16,
        pub base_factor: u16,
        pub filter_period: u16,
        pub decay_period: u16,
        pub reduction_factor: u16,
        pub variable_fee_control: u32,
        pub max_volatility_accumulator: u32,
        pub protocol_share: u16,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BaseFeeParameter {
        pub protocol_share: u16,
        pub base_factor: u16,
        pub base_fee_power_factor: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DynamicFeeParameter {
        pub filter_period: u16,
        pub decay_period: u16,
        pub reduction_factor: u16,
        pub variable_fee_control: u32,
        pub max_volatility_accumulator: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiquidityParameterByStrategyOneSide {
        pub amount: u64,
        pub active_id: i32,
        pub max_active_bin_slippage: i32,
        pub strategy_parameters: StrategyParameters,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiquidityParameterByStrategy {
        pub amount_x: u64,
        pub amount_y: u64,
        pub active_id: i32,
        pub max_active_bin_slippage: i32,
        pub strategy_parameters: StrategyParameters,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct StrategyParameters {
        pub min_bin_id: i32,
        pub max_bin_id: i32,
        pub strategy_type: StrategyType,
        #[serde(with = "BigArray")]
        pub parameteres: [u8; 64usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiquidityOneSideParameter {
        pub amount: u64,
        pub active_id: i32,
        pub max_active_bin_slippage: i32,
        pub bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BinLiquidityDistributionByWeight {
        pub bin_id: i32,
        pub weight: u16,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiquidityParameterByWeight {
        pub amount_x: u64,
        pub amount_y: u64,
        pub active_id: i32,
        pub max_active_bin_slippage: i32,
        pub bin_liquidity_dist: Vec<BinLiquidityDistributionByWeight>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AddLiquiditySingleSidePreciseParameter {
        pub bins: Vec<CompressedBinDepositAmount>,
        pub decompress_multiplier: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CompressedBinDepositAmount {
        pub bin_id: i32,
        pub amount: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BinLiquidityDistribution {
        pub bin_id: i32,
        pub distribution_x: u16,
        pub distribution_y: u16,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LiquidityParameter {
        pub amount_x: u64,
        pub amount_y: u64,
        pub bin_liquidity_dist: Vec<BinLiquidityDistribution>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CustomizableParams {
        pub active_id: i32,
        pub bin_step: u16,
        pub base_factor: u16,
        pub activation_type: u8,
        pub has_alpha_vault: bool,
        pub activation_point: Option<u64>,
        pub creator_pool_on_off_control: bool,
        pub base_fee_power_factor: u8,
        #[serde(with = "BigArray")]
        pub padding: [u8; 62usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitPermissionPairIx {
        pub active_id: i32,
        pub bin_step: u16,
        pub base_factor: u16,
        pub base_fee_power_factor: u8,
        pub activation_type: u8,
        pub protocol_share: u16,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AddLiquiditySingleSidePreciseParameter2 {
        pub bins: Vec<CompressedBinDepositAmount>,
        pub decompress_multiplier: u64,
        pub max_amount: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CompressedBinDepositAmount2 {
        pub bin_id: i32,
        pub amount: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitializeLbPair2Params {
        pub active_id: i32,
        #[serde(with = "BigArray")]
        pub padding: [u8; 96usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BinLiquidityReduction {
        pub bin_id: i32,
        pub bps_to_remove: u16,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Bin {
        pub amount_x: u64,
        pub amount_y: u64,
        pub price: u128,
        pub liquidity_supply: u128,
        pub reward_per_token_stored: [u128; 2usize],
        pub fee_amount_x_per_token_stored: u128,
        pub fee_amount_y_per_token_stored: u128,
        pub amount_x_in: u128,
        pub amount_y_in: u128,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ProtocolFee {
        pub amount_x: u64,
        pub amount_y: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RewardInfo {
        #[serde(with = "pubkey_serde")]
        pub mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub vault: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub funder: [u8; 32usize],
        pub reward_duration: u64,
        pub reward_duration_end: u64,
        pub reward_rate: u128,
        pub last_update_time: u64,
        pub cumulative_seconds_with_empty_liquidity_reward: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Observation {
        pub cumulative_active_bin_id: i128,
        pub created_at: i64,
        pub last_updated_at: i64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct StaticParameters {
        pub base_factor: u16,
        pub filter_period: u16,
        pub decay_period: u16,
        pub reduction_factor: u16,
        pub variable_fee_control: u32,
        pub max_volatility_accumulator: u32,
        pub min_bin_id: i32,
        pub max_bin_id: i32,
        pub protocol_share: u16,
        pub base_fee_power_factor: u8,
        pub padding: [u8; 5usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct VariableParameters {
        pub volatility_accumulator: u32,
        pub volatility_reference: u32,
        pub index_reference: i32,
        pub padding: [u8; 4usize],
        pub last_update_timestamp: i64,
        pub padding1: [u8; 8usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FeeInfo {
        pub fee_x_per_token_complete: u128,
        pub fee_y_per_token_complete: u128,
        pub fee_x_pending: u64,
        pub fee_y_pending: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UserRewardInfo {
        pub reward_per_token_completes: [u128; 2usize],
        pub reward_pendings: [u64; 2usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RemainingAccountsSlice {
        pub accounts_type: AccountsType,
        pub length: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RemainingAccountsInfo {
        pub slices: Vec<RemainingAccountsSlice>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum StrategyType {
        SpotOneSide,
        CurveOneSide,
        BidAskOneSide,
        SpotBalanced,
        CurveBalanced,
        BidAskBalanced,
        SpotImBalanced,
        CurveImBalanced,
        BidAskImBalanced,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Rounding {
        Up,
        Down,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ActivationType {
        Slot,
        Timestamp,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum LayoutVersion {
        V0,
        V1,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PairType {
        Permissionless,
        Permission,
        CustomizablePermissionless,
        PermissionlessV2,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PairStatus {
        Enabled,
        Disabled,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum TokenProgramFlags {
        TokenProgram,
        TokenProgram2022,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum AccountsType {
        TransferHookX,
        TransferHookY,
        TransferHookReward,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitializeLbPairAccounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub tokenMintX: String,
        pub tokenMintY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub oracle: String,
        pub presetParameter: String,
        pub funder: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePermissionLbPairAccounts {
        pub base: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub tokenMintX: String,
        pub tokenMintY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub oracle: String,
        pub admin: String,
        pub tokenBadgeX: String,
        pub tokenBadgeY: String,
        pub tokenProgramX: String,
        pub tokenProgramY: String,
        pub systemProgram: String,
        pub rent: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeCustomizablePermissionlessLbPairAccounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub tokenMintX: String,
        pub tokenMintY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub oracle: String,
        pub userTokenX: String,
        pub funder: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub userTokenY: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeBinArrayBitmapExtensionAccounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub funder: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeBinArrayAccounts {
        pub lbPair: String,
        pub binArray: String,
        pub funder: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidityAccounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidityByWeightAccounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidityByStrategyAccounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidityByStrategyOneSideAccounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userToken: String,
        pub reserve: String,
        pub tokenMint: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidityOneSideAccounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userToken: String,
        pub reserve: String,
        pub tokenMint: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveLiquidityAccounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePositionAccounts {
        pub payer: String,
        pub position: String,
        pub lbPair: String,
        pub owner: String,
        pub systemProgram: String,
        pub rent: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePositionPdaAccounts {
        pub payer: String,
        pub base: String,
        pub position: String,
        pub lbPair: String,
        pub owner: String,
        pub systemProgram: String,
        pub rent: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePositionByOperatorAccounts {
        pub payer: String,
        pub base: String,
        pub position: String,
        pub lbPair: String,
        pub owner: String,
        pub operator: String,
        pub operatorTokenX: String,
        pub ownerTokenX: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePositionOperatorAccounts {
        pub position: String,
        pub owner: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapAccounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub reserveX: String,
        pub reserveY: String,
        pub userTokenIn: String,
        pub userTokenOut: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub oracle: String,
        pub hostFeeIn: String,
        pub user: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapExactOutAccounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub reserveX: String,
        pub reserveY: String,
        pub userTokenIn: String,
        pub userTokenOut: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub oracle: String,
        pub hostFeeIn: String,
        pub user: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapWithPriceImpactAccounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub reserveX: String,
        pub reserveY: String,
        pub userTokenIn: String,
        pub userTokenOut: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub oracle: String,
        pub hostFeeIn: String,
        pub user: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawProtocolFeeAccounts {
        pub lbPair: String,
        pub reserveX: String,
        pub reserveY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub receiverTokenX: String,
        pub receiverTokenY: String,
        pub claimFeeOperator: String,
        pub operator: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub memoProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeRewardAccounts {
        pub lbPair: String,
        pub rewardVault: String,
        pub rewardMint: String,
        pub tokenBadge: String,
        pub admin: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FundRewardAccounts {
        pub lbPair: String,
        pub rewardVault: String,
        pub rewardMint: String,
        pub funderTokenAccount: String,
        pub funder: String,
        pub binArray: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateRewardFunderAccounts {
        pub lbPair: String,
        pub admin: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateRewardDurationAccounts {
        pub lbPair: String,
        pub admin: String,
        pub binArray: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimRewardAccounts {
        pub lbPair: String,
        pub position: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub rewardVault: String,
        pub rewardMint: String,
        pub userTokenAccount: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimFeeAccounts {
        pub lbPair: String,
        pub position: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub reserveX: String,
        pub reserveY: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClosePositionAccounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub rentReceiver: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateBaseFeeParametersAccounts {
        pub lbPair: String,
        pub admin: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateDynamicFeeParametersAccounts {
        pub lbPair: String,
        pub admin: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct IncreaseOracleLengthAccounts {
        pub oracle: String,
        pub funder: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePresetParameterAccounts {
        pub presetParameter: String,
        pub admin: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClosePresetParameterAccounts {
        pub presetParameter: String,
        pub admin: String,
        pub rentReceiver: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClosePresetParameter2Accounts {
        pub presetParameter: String,
        pub admin: String,
        pub rentReceiver: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveAllLiquidityAccounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetPairStatusAccounts {
        pub lbPair: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MigratePositionAccounts {
        pub positionV2: String,
        pub positionV1: String,
        pub lbPair: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub owner: String,
        pub systemProgram: String,
        pub rentReceiver: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MigrateBinArrayAccounts {
        pub lbPair: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateFeesAndRewardsAccounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub owner: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawIneligibleRewardAccounts {
        pub lbPair: String,
        pub rewardVault: String,
        pub rewardMint: String,
        pub funderTokenAccount: String,
        pub funder: String,
        pub binArray: String,
        pub tokenProgram: String,
        pub memoProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetActivationPointAccounts {
        pub lbPair: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveLiquidityByRangeAccounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidityOneSidePreciseAccounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userToken: String,
        pub reserve: String,
        pub tokenMint: String,
        pub binArrayLower: String,
        pub binArrayUpper: String,
        pub sender: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct GoToABinAccounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub fromBinArray: String,
        pub toBinArray: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetPreActivationDurationAccounts {
        pub lbPair: String,
        pub creator: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetPreActivationSwapAddressAccounts {
        pub lbPair: String,
        pub creator: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetPairStatusPermissionlessAccounts {
        pub lbPair: String,
        pub creator: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeTokenBadgeAccounts {
        pub tokenMint: String,
        pub tokenBadge: String,
        pub admin: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateClaimProtocolFeeOperatorAccounts {
        pub claimFeeOperator: String,
        pub operator: String,
        pub admin: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseClaimProtocolFeeOperatorAccounts {
        pub claimFeeOperator: String,
        pub rentReceiver: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePresetParameter2Accounts {
        pub presetParameter: String,
        pub admin: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeLbPair2Accounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub tokenMintX: String,
        pub tokenMintY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub oracle: String,
        pub presetParameter: String,
        pub funder: String,
        pub tokenBadgeX: String,
        pub tokenBadgeY: String,
        pub tokenProgramX: String,
        pub tokenProgramY: String,
        pub systemProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeCustomizablePermissionlessLbPair2Accounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub tokenMintX: String,
        pub tokenMintY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub oracle: String,
        pub userTokenX: String,
        pub funder: String,
        pub tokenBadgeX: String,
        pub tokenBadgeY: String,
        pub tokenProgramX: String,
        pub tokenProgramY: String,
        pub systemProgram: String,
        pub userTokenY: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimFee2Accounts {
        pub lbPair: String,
        pub position: String,
        pub sender: String,
        pub reserveX: String,
        pub reserveY: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub tokenProgramX: String,
        pub tokenProgramY: String,
        pub memoProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimReward2Accounts {
        pub lbPair: String,
        pub position: String,
        pub sender: String,
        pub rewardVault: String,
        pub rewardMint: String,
        pub userTokenAccount: String,
        pub tokenProgram: String,
        pub memoProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidity2Accounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub sender: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidityByStrategy2Accounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub sender: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddLiquidityOneSidePrecise2Accounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userToken: String,
        pub reserve: String,
        pub tokenMint: String,
        pub sender: String,
        pub tokenProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveLiquidity2Accounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub sender: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub memoProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveLiquidityByRange2Accounts {
        pub position: String,
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub userTokenX: String,
        pub userTokenY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub sender: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub memoProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct Swap2Accounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub reserveX: String,
        pub reserveY: String,
        pub userTokenIn: String,
        pub userTokenOut: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub oracle: String,
        pub hostFeeIn: String,
        pub user: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub memoProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapExactOut2Accounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub reserveX: String,
        pub reserveY: String,
        pub userTokenIn: String,
        pub userTokenOut: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub oracle: String,
        pub hostFeeIn: String,
        pub user: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub memoProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapWithPriceImpact2Accounts {
        pub lbPair: String,
        pub binArrayBitmapExtension: String,
        pub reserveX: String,
        pub reserveY: String,
        pub userTokenIn: String,
        pub userTokenOut: String,
        pub tokenXMint: String,
        pub tokenYMint: String,
        pub oracle: String,
        pub hostFeeIn: String,
        pub user: String,
        pub tokenXProgram: String,
        pub tokenYProgram: String,
        pub memoProgram: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClosePosition2Accounts {
        pub position: String,
        pub sender: String,
        pub rentReceiver: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateFeesAndReward2Accounts {
        pub position: String,
        pub lbPair: String,
        pub owner: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClosePositionIfEmptyAccounts {
        pub position: String,
        pub sender: String,
        pub rentReceiver: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    serde_big_array::big_array! { BigArray ; 60 , 62 , 64 , 51 , 72 , 96 , 128 , 256 }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeLbPairArguments {
        pub active_id: i32,
        pub bin_step: u16,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializePermissionLbPairArguments {
        pub ix_data: InitPermissionPairIx,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeCustomizablePermissionlessLbPairArguments {
        pub params: CustomizableParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeBinArrayBitmapExtensionArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeBinArrayArguments {
        pub index: i64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidityArguments {
        pub liquidity_parameter: LiquidityParameter,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidityByWeightArguments {
        pub liquidity_parameter: LiquidityParameterByWeight,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidityByStrategyArguments {
        pub liquidity_parameter: LiquidityParameterByStrategy,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidityByStrategyOneSideArguments {
        pub liquidity_parameter: LiquidityParameterByStrategyOneSide,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidityOneSideArguments {
        pub liquidity_parameter: LiquidityOneSideParameter,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveLiquidityArguments {
        pub bin_liquidity_removal: Vec<BinLiquidityReduction>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializePositionArguments {
        pub lower_bin_id: i32,
        pub width: i32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializePositionPdaArguments {
        pub lower_bin_id: i32,
        pub width: i32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializePositionByOperatorArguments {
        pub lower_bin_id: i32,
        pub width: i32,
        #[serde(with = "pubkey_serde")]
        pub fee_owner: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub lock_release_point: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePositionOperatorArguments {
        #[serde(with = "pubkey_serde")]
        pub operator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_amount_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapExactOutArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_in_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub out_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapWithPriceImpactArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
        pub active_id: Option<i32>,
        pub max_price_impact_bps: u16,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawProtocolFeeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_x: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_y: u64,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeRewardArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_duration: u64,
        #[serde(with = "pubkey_serde")]
        pub funder: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FundRewardArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        pub carry_forward: bool,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateRewardFunderArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
        #[serde(with = "pubkey_serde")]
        pub new_funder: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateRewardDurationArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub new_duration: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimRewardArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimFeeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClosePositionArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateBaseFeeParametersArguments {
        pub fee_parameter: BaseFeeParameter,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateDynamicFeeParametersArguments {
        pub fee_parameter: DynamicFeeParameter,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreaseOracleLengthArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub length_to_add: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializePresetParameterArguments {
        pub ix: InitPresetParametersIx,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClosePresetParameterArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClosePresetParameter2Arguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveAllLiquidityArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetPairStatusArguments {
        pub status: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MigratePositionArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MigrateBinArrayArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateFeesAndRewardsArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawIneligibleRewardArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetActivationPointArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub activation_point: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveLiquidityByRangeArguments {
        pub from_bin_id: i32,
        pub to_bin_id: i32,
        pub bps_to_remove: u16,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidityOneSidePreciseArguments {
        pub parameter: AddLiquiditySingleSidePreciseParameter,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct GoToABinArguments {
        pub bin_id: i32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetPreActivationDurationArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub pre_activation_duration: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetPreActivationSwapAddressArguments {
        #[serde(with = "pubkey_serde")]
        pub pre_activation_swap_address: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetPairStatusPermissionlessArguments {
        pub status: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeTokenBadgeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateClaimProtocolFeeOperatorArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseClaimProtocolFeeOperatorArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializePresetParameter2Arguments {
        pub ix: InitPresetParameters2Ix,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeLbPair2Arguments {
        pub params: InitializeLbPair2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeCustomizablePermissionlessLbPair2Arguments {
        pub params: CustomizableParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimFee2Arguments {
        pub min_bin_id: i32,
        pub max_bin_id: i32,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimReward2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub reward_index: u64,
        pub min_bin_id: i32,
        pub max_bin_id: i32,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidity2Arguments {
        pub liquidity_parameter: LiquidityParameter,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidityByStrategy2Arguments {
        pub liquidity_parameter: LiquidityParameterByStrategy,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidityOneSidePrecise2Arguments {
        pub liquidity_parameter: AddLiquiditySingleSidePreciseParameter2,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveLiquidity2Arguments {
        pub bin_liquidity_removal: Vec<BinLiquidityReduction>,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveLiquidityByRange2Arguments {
        pub from_bin_id: i32,
        pub to_bin_id: i32,
        pub bps_to_remove: u16,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct Swap2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_amount_out: u64,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapExactOut2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_in_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub out_amount: u64,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapWithPriceImpact2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount_in: u64,
        pub active_id: Option<i32>,
        pub max_price_impact_bps: u16,
        pub remaining_accounts_info: RemainingAccountsInfo,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClosePosition2Arguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateFeesAndReward2Arguments {
        pub min_bin_id: i32,
        pub max_bin_id: i32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClosePositionIfEmptyArguments {}
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitializeLbPair {
        accounts: InitializeLbPairAccounts,
        args: InitializeLbPairArguments,
    },
    InitializePermissionLbPair {
        accounts: InitializePermissionLbPairAccounts,
        args: InitializePermissionLbPairArguments,
    },
    InitializeCustomizablePermissionlessLbPair {
        accounts: InitializeCustomizablePermissionlessLbPairAccounts,
        args: InitializeCustomizablePermissionlessLbPairArguments,
    },
    InitializeBinArrayBitmapExtension {
        accounts: InitializeBinArrayBitmapExtensionAccounts,
        args: InitializeBinArrayBitmapExtensionArguments,
    },
    InitializeBinArray {
        accounts: InitializeBinArrayAccounts,
        args: InitializeBinArrayArguments,
    },
    AddLiquidity {
        accounts: AddLiquidityAccounts,
        args: AddLiquidityArguments,
    },
    AddLiquidityByWeight {
        accounts: AddLiquidityByWeightAccounts,
        args: AddLiquidityByWeightArguments,
    },
    AddLiquidityByStrategy {
        accounts: AddLiquidityByStrategyAccounts,
        args: AddLiquidityByStrategyArguments,
    },
    AddLiquidityByStrategyOneSide {
        accounts: AddLiquidityByStrategyOneSideAccounts,
        args: AddLiquidityByStrategyOneSideArguments,
    },
    AddLiquidityOneSide {
        accounts: AddLiquidityOneSideAccounts,
        args: AddLiquidityOneSideArguments,
    },
    RemoveLiquidity {
        accounts: RemoveLiquidityAccounts,
        args: RemoveLiquidityArguments,
    },
    InitializePosition {
        accounts: InitializePositionAccounts,
        args: InitializePositionArguments,
    },
    InitializePositionPda {
        accounts: InitializePositionPdaAccounts,
        args: InitializePositionPdaArguments,
    },
    InitializePositionByOperator {
        accounts: InitializePositionByOperatorAccounts,
        args: InitializePositionByOperatorArguments,
    },
    UpdatePositionOperator {
        accounts: UpdatePositionOperatorAccounts,
        args: UpdatePositionOperatorArguments,
    },
    Swap {
        accounts: SwapAccounts,
        args: SwapArguments,
    },
    SwapExactOut {
        accounts: SwapExactOutAccounts,
        args: SwapExactOutArguments,
    },
    SwapWithPriceImpact {
        accounts: SwapWithPriceImpactAccounts,
        args: SwapWithPriceImpactArguments,
    },
    WithdrawProtocolFee {
        accounts: WithdrawProtocolFeeAccounts,
        args: WithdrawProtocolFeeArguments,
    },
    InitializeReward {
        accounts: InitializeRewardAccounts,
        args: InitializeRewardArguments,
    },
    FundReward {
        accounts: FundRewardAccounts,
        args: FundRewardArguments,
    },
    UpdateRewardFunder {
        accounts: UpdateRewardFunderAccounts,
        args: UpdateRewardFunderArguments,
    },
    UpdateRewardDuration {
        accounts: UpdateRewardDurationAccounts,
        args: UpdateRewardDurationArguments,
    },
    ClaimReward {
        accounts: ClaimRewardAccounts,
        args: ClaimRewardArguments,
    },
    ClaimFee {
        accounts: ClaimFeeAccounts,
        args: ClaimFeeArguments,
    },
    ClosePosition {
        accounts: ClosePositionAccounts,
        args: ClosePositionArguments,
    },
    UpdateBaseFeeParameters {
        accounts: UpdateBaseFeeParametersAccounts,
        args: UpdateBaseFeeParametersArguments,
    },
    UpdateDynamicFeeParameters {
        accounts: UpdateDynamicFeeParametersAccounts,
        args: UpdateDynamicFeeParametersArguments,
    },
    IncreaseOracleLength {
        accounts: IncreaseOracleLengthAccounts,
        args: IncreaseOracleLengthArguments,
    },
    InitializePresetParameter {
        accounts: InitializePresetParameterAccounts,
        args: InitializePresetParameterArguments,
    },
    ClosePresetParameter {
        accounts: ClosePresetParameterAccounts,
        args: ClosePresetParameterArguments,
    },
    ClosePresetParameter2 {
        accounts: ClosePresetParameter2Accounts,
        args: ClosePresetParameter2Arguments,
    },
    RemoveAllLiquidity {
        accounts: RemoveAllLiquidityAccounts,
        args: RemoveAllLiquidityArguments,
    },
    SetPairStatus {
        accounts: SetPairStatusAccounts,
        args: SetPairStatusArguments,
    },
    MigratePosition {
        accounts: MigratePositionAccounts,
        args: MigratePositionArguments,
    },
    MigrateBinArray {
        accounts: MigrateBinArrayAccounts,
        args: MigrateBinArrayArguments,
    },
    UpdateFeesAndRewards {
        accounts: UpdateFeesAndRewardsAccounts,
        args: UpdateFeesAndRewardsArguments,
    },
    WithdrawIneligibleReward {
        accounts: WithdrawIneligibleRewardAccounts,
        args: WithdrawIneligibleRewardArguments,
    },
    SetActivationPoint {
        accounts: SetActivationPointAccounts,
        args: SetActivationPointArguments,
    },
    RemoveLiquidityByRange {
        accounts: RemoveLiquidityByRangeAccounts,
        args: RemoveLiquidityByRangeArguments,
    },
    AddLiquidityOneSidePrecise {
        accounts: AddLiquidityOneSidePreciseAccounts,
        args: AddLiquidityOneSidePreciseArguments,
    },
    GoToABin {
        accounts: GoToABinAccounts,
        args: GoToABinArguments,
    },
    SetPreActivationDuration {
        accounts: SetPreActivationDurationAccounts,
        args: SetPreActivationDurationArguments,
    },
    SetPreActivationSwapAddress {
        accounts: SetPreActivationSwapAddressAccounts,
        args: SetPreActivationSwapAddressArguments,
    },
    SetPairStatusPermissionless {
        accounts: SetPairStatusPermissionlessAccounts,
        args: SetPairStatusPermissionlessArguments,
    },
    InitializeTokenBadge {
        accounts: InitializeTokenBadgeAccounts,
        args: InitializeTokenBadgeArguments,
    },
    CreateClaimProtocolFeeOperator {
        accounts: CreateClaimProtocolFeeOperatorAccounts,
        args: CreateClaimProtocolFeeOperatorArguments,
    },
    CloseClaimProtocolFeeOperator {
        accounts: CloseClaimProtocolFeeOperatorAccounts,
        args: CloseClaimProtocolFeeOperatorArguments,
    },
    InitializePresetParameter2 {
        accounts: InitializePresetParameter2Accounts,
        args: InitializePresetParameter2Arguments,
    },
    InitializeLbPair2 {
        accounts: InitializeLbPair2Accounts,
        args: InitializeLbPair2Arguments,
    },
    InitializeCustomizablePermissionlessLbPair2 {
        accounts: InitializeCustomizablePermissionlessLbPair2Accounts,
        args: InitializeCustomizablePermissionlessLbPair2Arguments,
    },
    ClaimFee2 {
        accounts: ClaimFee2Accounts,
        args: ClaimFee2Arguments,
    },
    ClaimReward2 {
        accounts: ClaimReward2Accounts,
        args: ClaimReward2Arguments,
    },
    AddLiquidity2 {
        accounts: AddLiquidity2Accounts,
        args: AddLiquidity2Arguments,
    },
    AddLiquidityByStrategy2 {
        accounts: AddLiquidityByStrategy2Accounts,
        args: AddLiquidityByStrategy2Arguments,
    },
    AddLiquidityOneSidePrecise2 {
        accounts: AddLiquidityOneSidePrecise2Accounts,
        args: AddLiquidityOneSidePrecise2Arguments,
    },
    RemoveLiquidity2 {
        accounts: RemoveLiquidity2Accounts,
        args: RemoveLiquidity2Arguments,
    },
    RemoveLiquidityByRange2 {
        accounts: RemoveLiquidityByRange2Accounts,
        args: RemoveLiquidityByRange2Arguments,
    },
    Swap2 {
        accounts: Swap2Accounts,
        args: Swap2Arguments,
    },
    SwapExactOut2 {
        accounts: SwapExactOut2Accounts,
        args: SwapExactOut2Arguments,
    },
    SwapWithPriceImpact2 {
        accounts: SwapWithPriceImpact2Accounts,
        args: SwapWithPriceImpact2Arguments,
    },
    ClosePosition2 {
        accounts: ClosePosition2Accounts,
        args: ClosePosition2Arguments,
    },
    UpdateFeesAndReward2 {
        accounts: UpdateFeesAndReward2Accounts,
        args: UpdateFeesAndReward2Arguments,
    },
    ClosePositionIfEmpty {
        accounts: ClosePositionIfEmptyAccounts,
        args: ClosePositionIfEmptyArguments,
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
            [45u8, 154u8, 237u8, 210u8, 221u8, 15u8, 166u8, 92u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeLbPairArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let tokenMintX = keys.next().unwrap().clone();
                let tokenMintY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let presetParameter = keys.next().unwrap().clone();
                let funder = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeLbPairAccounts {
                    lbPair,
                    binArrayBitmapExtension,
                    tokenMintX,
                    tokenMintY,
                    reserveX,
                    reserveY,
                    oracle,
                    presetParameter,
                    funder,
                    tokenProgram,
                    systemProgram,
                    rent,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitializeLbPair { accounts, args });
            }
            [108u8, 102u8, 213u8, 85u8, 251u8, 3u8, 53u8, 21u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePermissionLbPairArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let base = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let tokenMintX = keys.next().unwrap().clone();
                let tokenMintY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let tokenBadgeX = keys.next().unwrap().clone();
                let tokenBadgeY = keys.next().unwrap().clone();
                let tokenProgramX = keys.next().unwrap().clone();
                let tokenProgramY = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePermissionLbPairAccounts {
                    base,
                    lbPair,
                    binArrayBitmapExtension,
                    tokenMintX,
                    tokenMintY,
                    reserveX,
                    reserveY,
                    oracle,
                    admin,
                    tokenBadgeX,
                    tokenBadgeY,
                    tokenProgramX,
                    tokenProgramY,
                    systemProgram,
                    rent,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitializePermissionLbPair { accounts, args });
            }
            [46u8, 39u8, 41u8, 135u8, 111u8, 183u8, 200u8, 64u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    InitializeCustomizablePermissionlessLbPairArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let tokenMintX = keys.next().unwrap().clone();
                let tokenMintY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let funder = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeCustomizablePermissionlessLbPairAccounts {
                    lbPair,
                    binArrayBitmapExtension,
                    tokenMintX,
                    tokenMintY,
                    reserveX,
                    reserveY,
                    oracle,
                    userTokenX,
                    funder,
                    tokenProgram,
                    systemProgram,
                    userTokenY,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitializeCustomizablePermissionlessLbPair {
                    accounts,
                    args,
                });
            }
            [47u8, 157u8, 226u8, 180u8, 12u8, 240u8, 33u8, 71u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeBinArrayBitmapExtensionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let funder = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeBinArrayBitmapExtensionAccounts {
                    lbPair,
                    binArrayBitmapExtension,
                    funder,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::InitializeBinArrayBitmapExtension { accounts, args });
            }
            [35u8, 86u8, 19u8, 185u8, 78u8, 212u8, 75u8, 211u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeBinArrayArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArray = keys.next().unwrap().clone();
                let funder = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeBinArrayAccounts {
                    lbPair,
                    binArray,
                    funder,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeBinArray { accounts, args });
            }
            [181u8, 157u8, 89u8, 67u8, 143u8, 182u8, 52u8, 72u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddLiquidityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidityAccounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userTokenX,
                    userTokenY,
                    reserveX,
                    reserveY,
                    tokenXMint,
                    tokenYMint,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    tokenXProgram,
                    tokenYProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AddLiquidity { accounts, args });
            }
            [28u8, 140u8, 238u8, 99u8, 231u8, 162u8, 21u8, 149u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddLiquidityByWeightArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidityByWeightAccounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userTokenX,
                    userTokenY,
                    reserveX,
                    reserveY,
                    tokenXMint,
                    tokenYMint,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    tokenXProgram,
                    tokenYProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AddLiquidityByWeight { accounts, args });
            }
            [7u8, 3u8, 150u8, 127u8, 148u8, 40u8, 61u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddLiquidityByStrategyArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidityByStrategyAccounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userTokenX,
                    userTokenY,
                    reserveX,
                    reserveY,
                    tokenXMint,
                    tokenYMint,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    tokenXProgram,
                    tokenYProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AddLiquidityByStrategy { accounts, args });
            }
            [41u8, 5u8, 238u8, 175u8, 100u8, 225u8, 6u8, 205u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddLiquidityByStrategyOneSideArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userToken = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let tokenMint = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidityByStrategyOneSideAccounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userToken,
                    reserve,
                    tokenMint,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AddLiquidityByStrategyOneSide { accounts, args });
            }
            [94u8, 155u8, 103u8, 151u8, 70u8, 95u8, 220u8, 165u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddLiquidityOneSideArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userToken = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let tokenMint = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidityOneSideAccounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userToken,
                    reserve,
                    tokenMint,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AddLiquidityOneSide { accounts, args });
            }
            [80u8, 85u8, 209u8, 72u8, 24u8, 206u8, 177u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveLiquidityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveLiquidityAccounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userTokenX,
                    userTokenY,
                    reserveX,
                    reserveY,
                    tokenXMint,
                    tokenYMint,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    tokenXProgram,
                    tokenYProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::RemoveLiquidity { accounts, args });
            }
            [219u8, 192u8, 234u8, 71u8, 190u8, 191u8, 102u8, 80u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePositionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePositionAccounts {
                    payer,
                    position,
                    lbPair,
                    owner,
                    systemProgram,
                    rent,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitializePosition { accounts, args });
            }
            [46u8, 82u8, 125u8, 146u8, 85u8, 141u8, 228u8, 153u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePositionPdaArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let base = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePositionPdaAccounts {
                    payer,
                    base,
                    position,
                    lbPair,
                    owner,
                    systemProgram,
                    rent,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitializePositionPda { accounts, args });
            }
            [251u8, 189u8, 190u8, 244u8, 117u8, 254u8, 35u8, 148u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePositionByOperatorArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let base = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let operator = keys.next().unwrap().clone();
                let operatorTokenX = keys.next().unwrap().clone();
                let ownerTokenX = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePositionByOperatorAccounts {
                    payer,
                    base,
                    position,
                    lbPair,
                    owner,
                    operator,
                    operatorTokenX,
                    ownerTokenX,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitializePositionByOperator { accounts, args });
            }
            [202u8, 184u8, 103u8, 143u8, 180u8, 191u8, 116u8, 217u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePositionOperatorArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePositionOperatorAccounts {
                    position,
                    owner,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdatePositionOperator { accounts, args });
            }
            [248u8, 198u8, 158u8, 145u8, 225u8, 117u8, 135u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let userTokenIn = keys.next().unwrap().clone();
                let userTokenOut = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let hostFeeIn = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapAccounts {
                    lbPair,
                    binArrayBitmapExtension,
                    reserveX,
                    reserveY,
                    userTokenIn,
                    userTokenOut,
                    tokenXMint,
                    tokenYMint,
                    oracle,
                    hostFeeIn,
                    user,
                    tokenXProgram,
                    tokenYProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Swap { accounts, args });
            }
            [250u8, 73u8, 101u8, 33u8, 38u8, 207u8, 75u8, 184u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapExactOutArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let userTokenIn = keys.next().unwrap().clone();
                let userTokenOut = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let hostFeeIn = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapExactOutAccounts {
                    lbPair,
                    binArrayBitmapExtension,
                    reserveX,
                    reserveY,
                    userTokenIn,
                    userTokenOut,
                    tokenXMint,
                    tokenYMint,
                    oracle,
                    hostFeeIn,
                    user,
                    tokenXProgram,
                    tokenYProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SwapExactOut { accounts, args });
            }
            [56u8, 173u8, 230u8, 208u8, 173u8, 228u8, 156u8, 205u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapWithPriceImpactArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let userTokenIn = keys.next().unwrap().clone();
                let userTokenOut = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let hostFeeIn = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapWithPriceImpactAccounts {
                    lbPair,
                    binArrayBitmapExtension,
                    reserveX,
                    reserveY,
                    userTokenIn,
                    userTokenOut,
                    tokenXMint,
                    tokenYMint,
                    oracle,
                    hostFeeIn,
                    user,
                    tokenXProgram,
                    tokenYProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SwapWithPriceImpact { accounts, args });
            }
            [158u8, 201u8, 158u8, 189u8, 33u8, 93u8, 162u8, 103u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawProtocolFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let receiverTokenX = keys.next().unwrap().clone();
                let receiverTokenY = keys.next().unwrap().clone();
                let claimFeeOperator = keys.next().unwrap().clone();
                let operator = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawProtocolFeeAccounts {
                    lbPair,
                    reserveX,
                    reserveY,
                    tokenXMint,
                    tokenYMint,
                    receiverTokenX,
                    receiverTokenY,
                    claimFeeOperator,
                    operator,
                    tokenXProgram,
                    tokenYProgram,
                    memoProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawProtocolFee { accounts, args });
            }
            [95u8, 135u8, 192u8, 196u8, 242u8, 129u8, 230u8, 68u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeRewardArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let tokenBadge = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeRewardAccounts {
                    lbPair,
                    rewardVault,
                    rewardMint,
                    tokenBadge,
                    admin,
                    tokenProgram,
                    systemProgram,
                    rent,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitializeReward { accounts, args });
            }
            [188u8, 50u8, 249u8, 165u8, 93u8, 151u8, 38u8, 63u8] => {
                let mut rdr: &[u8] = rest;
                let args = FundRewardArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let funderTokenAccount = keys.next().unwrap().clone();
                let funder = keys.next().unwrap().clone();
                let binArray = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FundRewardAccounts {
                    lbPair,
                    rewardVault,
                    rewardMint,
                    funderTokenAccount,
                    funder,
                    binArray,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::FundReward { accounts, args });
            }
            [211u8, 28u8, 48u8, 32u8, 215u8, 160u8, 35u8, 23u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateRewardFunderArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateRewardFunderAccounts {
                    lbPair,
                    admin,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdateRewardFunder { accounts, args });
            }
            [138u8, 174u8, 196u8, 169u8, 213u8, 235u8, 254u8, 107u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateRewardDurationArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let binArray = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateRewardDurationAccounts {
                    lbPair,
                    admin,
                    binArray,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdateRewardDuration { accounts, args });
            }
            [149u8, 95u8, 181u8, 242u8, 94u8, 90u8, 158u8, 162u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimRewardArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let userTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimRewardAccounts {
                    lbPair,
                    position,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    rewardVault,
                    rewardMint,
                    userTokenAccount,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClaimReward { accounts, args });
            }
            [169u8, 32u8, 79u8, 137u8, 136u8, 232u8, 70u8, 137u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimFeeAccounts {
                    lbPair,
                    position,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    reserveX,
                    reserveY,
                    userTokenX,
                    userTokenY,
                    tokenXMint,
                    tokenYMint,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClaimFee { accounts, args });
            }
            [123u8, 134u8, 81u8, 0u8, 49u8, 68u8, 98u8, 98u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClosePositionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let rentReceiver = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClosePositionAccounts {
                    position,
                    lbPair,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    rentReceiver,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClosePosition { accounts, args });
            }
            [75u8, 168u8, 223u8, 161u8, 16u8, 195u8, 3u8, 47u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateBaseFeeParametersArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateBaseFeeParametersAccounts {
                    lbPair,
                    admin,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdateBaseFeeParameters { accounts, args });
            }
            [92u8, 161u8, 46u8, 246u8, 255u8, 189u8, 22u8, 22u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateDynamicFeeParametersArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateDynamicFeeParametersAccounts {
                    lbPair,
                    admin,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::UpdateDynamicFeeParameters { accounts, args });
            }
            [190u8, 61u8, 125u8, 87u8, 103u8, 79u8, 158u8, 173u8] => {
                let mut rdr: &[u8] = rest;
                let args = IncreaseOracleLengthArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let oracle = keys.next().unwrap().clone();
                let funder = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = IncreaseOracleLengthAccounts {
                    oracle,
                    funder,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::IncreaseOracleLength { accounts, args });
            }
            [66u8, 188u8, 71u8, 211u8, 98u8, 109u8, 14u8, 186u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePresetParameterArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let presetParameter = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePresetParameterAccounts {
                    presetParameter,
                    admin,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::InitializePresetParameter { accounts, args });
            }
            [4u8, 148u8, 145u8, 100u8, 134u8, 26u8, 181u8, 61u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClosePresetParameterArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let presetParameter = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let rentReceiver = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClosePresetParameterAccounts {
                    presetParameter,
                    admin,
                    rentReceiver,
                    remaining,
                };
                return Ok(Instruction::ClosePresetParameter { accounts, args });
            }
            [39u8, 25u8, 95u8, 107u8, 116u8, 17u8, 115u8, 28u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClosePresetParameter2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let presetParameter = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let rentReceiver = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClosePresetParameter2Accounts {
                    presetParameter,
                    admin,
                    rentReceiver,
                    remaining,
                };
                return Ok(Instruction::ClosePresetParameter2 { accounts, args });
            }
            [10u8, 51u8, 61u8, 35u8, 112u8, 105u8, 24u8, 85u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveAllLiquidityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveAllLiquidityAccounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userTokenX,
                    userTokenY,
                    reserveX,
                    reserveY,
                    tokenXMint,
                    tokenYMint,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    tokenXProgram,
                    tokenYProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::RemoveAllLiquidity { accounts, args });
            }
            [67u8, 248u8, 231u8, 137u8, 154u8, 149u8, 217u8, 174u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetPairStatusArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetPairStatusAccounts {
                    lbPair,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetPairStatus { accounts, args });
            }
            [15u8, 132u8, 59u8, 50u8, 199u8, 6u8, 251u8, 46u8] => {
                let mut rdr: &[u8] = rest;
                let args = MigratePositionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let positionV2 = keys.next().unwrap().clone();
                let positionV1 = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rentReceiver = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MigratePositionAccounts {
                    positionV2,
                    positionV1,
                    lbPair,
                    binArrayLower,
                    binArrayUpper,
                    owner,
                    systemProgram,
                    rentReceiver,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::MigratePosition { accounts, args });
            }
            [17u8, 23u8, 159u8, 211u8, 101u8, 184u8, 41u8, 241u8] => {
                let mut rdr: &[u8] = rest;
                let args = MigrateBinArrayArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MigrateBinArrayAccounts { lbPair, remaining };
                return Ok(Instruction::MigrateBinArray { accounts, args });
            }
            [154u8, 230u8, 250u8, 13u8, 236u8, 209u8, 75u8, 223u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateFeesAndRewardsArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateFeesAndRewardsAccounts {
                    position,
                    lbPair,
                    binArrayLower,
                    binArrayUpper,
                    owner,
                    remaining,
                };
                return Ok(Instruction::UpdateFeesAndRewards { accounts, args });
            }
            [148u8, 206u8, 42u8, 195u8, 247u8, 49u8, 103u8, 8u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawIneligibleRewardArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let funderTokenAccount = keys.next().unwrap().clone();
                let funder = keys.next().unwrap().clone();
                let binArray = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawIneligibleRewardAccounts {
                    lbPair,
                    rewardVault,
                    rewardMint,
                    funderTokenAccount,
                    funder,
                    binArray,
                    tokenProgram,
                    memoProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::WithdrawIneligibleReward { accounts, args });
            }
            [91u8, 249u8, 15u8, 165u8, 26u8, 129u8, 254u8, 125u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetActivationPointArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetActivationPointAccounts {
                    lbPair,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetActivationPoint { accounts, args });
            }
            [26u8, 82u8, 102u8, 152u8, 240u8, 74u8, 105u8, 26u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveLiquidityByRangeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveLiquidityByRangeAccounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userTokenX,
                    userTokenY,
                    reserveX,
                    reserveY,
                    tokenXMint,
                    tokenYMint,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    tokenXProgram,
                    tokenYProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::RemoveLiquidityByRange { accounts, args });
            }
            [161u8, 194u8, 103u8, 84u8, 171u8, 71u8, 250u8, 154u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddLiquidityOneSidePreciseArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userToken = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let tokenMint = keys.next().unwrap().clone();
                let binArrayLower = keys.next().unwrap().clone();
                let binArrayUpper = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidityOneSidePreciseAccounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userToken,
                    reserve,
                    tokenMint,
                    binArrayLower,
                    binArrayUpper,
                    sender,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AddLiquidityOneSidePrecise { accounts, args });
            }
            [146u8, 72u8, 174u8, 224u8, 40u8, 253u8, 84u8, 174u8] => {
                let mut rdr: &[u8] = rest;
                let args = GoToABinArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let fromBinArray = keys.next().unwrap().clone();
                let toBinArray = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = GoToABinAccounts {
                    lbPair,
                    binArrayBitmapExtension,
                    fromBinArray,
                    toBinArray,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::GoToABin { accounts, args });
            }
            [165u8, 61u8, 201u8, 244u8, 130u8, 159u8, 22u8, 100u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetPreActivationDurationArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let creator = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetPreActivationDurationAccounts {
                    lbPair,
                    creator,
                    remaining,
                };
                return Ok(Instruction::SetPreActivationDuration { accounts, args });
            }
            [57u8, 139u8, 47u8, 123u8, 216u8, 80u8, 223u8, 10u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetPreActivationSwapAddressArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let creator = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetPreActivationSwapAddressAccounts {
                    lbPair,
                    creator,
                    remaining,
                };
                return Ok(Instruction::SetPreActivationSwapAddress { accounts, args });
            }
            [78u8, 59u8, 152u8, 211u8, 70u8, 183u8, 46u8, 208u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetPairStatusPermissionlessArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let creator = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetPairStatusPermissionlessAccounts {
                    lbPair,
                    creator,
                    remaining,
                };
                return Ok(Instruction::SetPairStatusPermissionless { accounts, args });
            }
            [253u8, 77u8, 205u8, 95u8, 27u8, 224u8, 89u8, 223u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeTokenBadgeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let tokenMint = keys.next().unwrap().clone();
                let tokenBadge = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeTokenBadgeAccounts {
                    tokenMint,
                    tokenBadge,
                    admin,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeTokenBadge { accounts, args });
            }
            [51u8, 19u8, 150u8, 252u8, 105u8, 157u8, 48u8, 91u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateClaimProtocolFeeOperatorArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let claimFeeOperator = keys.next().unwrap().clone();
                let operator = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateClaimProtocolFeeOperatorAccounts {
                    claimFeeOperator,
                    operator,
                    admin,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::CreateClaimProtocolFeeOperator { accounts, args });
            }
            [8u8, 41u8, 87u8, 35u8, 80u8, 48u8, 121u8, 26u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseClaimProtocolFeeOperatorArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let claimFeeOperator = keys.next().unwrap().clone();
                let rentReceiver = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseClaimProtocolFeeOperatorAccounts {
                    claimFeeOperator,
                    rentReceiver,
                    admin,
                    remaining,
                };
                return Ok(Instruction::CloseClaimProtocolFeeOperator { accounts, args });
            }
            [184u8, 7u8, 240u8, 171u8, 103u8, 47u8, 183u8, 121u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePresetParameter2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let presetParameter = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePresetParameter2Accounts {
                    presetParameter,
                    admin,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializePresetParameter2 { accounts, args });
            }
            [73u8, 59u8, 36u8, 120u8, 237u8, 83u8, 108u8, 198u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeLbPair2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let tokenMintX = keys.next().unwrap().clone();
                let tokenMintY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let presetParameter = keys.next().unwrap().clone();
                let funder = keys.next().unwrap().clone();
                let tokenBadgeX = keys.next().unwrap().clone();
                let tokenBadgeY = keys.next().unwrap().clone();
                let tokenProgramX = keys.next().unwrap().clone();
                let tokenProgramY = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeLbPair2Accounts {
                    lbPair,
                    binArrayBitmapExtension,
                    tokenMintX,
                    tokenMintY,
                    reserveX,
                    reserveY,
                    oracle,
                    presetParameter,
                    funder,
                    tokenBadgeX,
                    tokenBadgeY,
                    tokenProgramX,
                    tokenProgramY,
                    systemProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitializeLbPair2 { accounts, args });
            }
            [243u8, 73u8, 129u8, 126u8, 51u8, 19u8, 241u8, 107u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    InitializeCustomizablePermissionlessLbPair2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let tokenMintX = keys.next().unwrap().clone();
                let tokenMintY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let funder = keys.next().unwrap().clone();
                let tokenBadgeX = keys.next().unwrap().clone();
                let tokenBadgeY = keys.next().unwrap().clone();
                let tokenProgramX = keys.next().unwrap().clone();
                let tokenProgramY = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeCustomizablePermissionlessLbPair2Accounts {
                    lbPair,
                    binArrayBitmapExtension,
                    tokenMintX,
                    tokenMintY,
                    reserveX,
                    reserveY,
                    oracle,
                    userTokenX,
                    funder,
                    tokenBadgeX,
                    tokenBadgeY,
                    tokenProgramX,
                    tokenProgramY,
                    systemProgram,
                    userTokenY,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::InitializeCustomizablePermissionlessLbPair2 {
                    accounts,
                    args,
                });
            }
            [112u8, 191u8, 101u8, 171u8, 28u8, 144u8, 127u8, 187u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimFee2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let tokenProgramX = keys.next().unwrap().clone();
                let tokenProgramY = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimFee2Accounts {
                    lbPair,
                    position,
                    sender,
                    reserveX,
                    reserveY,
                    userTokenX,
                    userTokenY,
                    tokenXMint,
                    tokenYMint,
                    tokenProgramX,
                    tokenProgramY,
                    memoProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClaimFee2 { accounts, args });
            }
            [190u8, 3u8, 127u8, 119u8, 178u8, 87u8, 157u8, 183u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimReward2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let position = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let rewardVault = keys.next().unwrap().clone();
                let rewardMint = keys.next().unwrap().clone();
                let userTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimReward2Accounts {
                    lbPair,
                    position,
                    sender,
                    rewardVault,
                    rewardMint,
                    userTokenAccount,
                    tokenProgram,
                    memoProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClaimReward2 { accounts, args });
            }
            [228u8, 162u8, 78u8, 28u8, 70u8, 219u8, 116u8, 115u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddLiquidity2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidity2Accounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userTokenX,
                    userTokenY,
                    reserveX,
                    reserveY,
                    tokenXMint,
                    tokenYMint,
                    sender,
                    tokenXProgram,
                    tokenYProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AddLiquidity2 { accounts, args });
            }
            [3u8, 221u8, 149u8, 218u8, 111u8, 141u8, 118u8, 213u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddLiquidityByStrategy2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidityByStrategy2Accounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userTokenX,
                    userTokenY,
                    reserveX,
                    reserveY,
                    tokenXMint,
                    tokenYMint,
                    sender,
                    tokenXProgram,
                    tokenYProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AddLiquidityByStrategy2 { accounts, args });
            }
            [33u8, 51u8, 163u8, 201u8, 117u8, 98u8, 125u8, 231u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddLiquidityOneSidePrecise2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userToken = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let tokenMint = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddLiquidityOneSidePrecise2Accounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userToken,
                    reserve,
                    tokenMint,
                    sender,
                    tokenProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::AddLiquidityOneSidePrecise2 { accounts, args });
            }
            [230u8, 215u8, 82u8, 127u8, 241u8, 101u8, 227u8, 146u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveLiquidity2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveLiquidity2Accounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userTokenX,
                    userTokenY,
                    reserveX,
                    reserveY,
                    tokenXMint,
                    tokenYMint,
                    sender,
                    tokenXProgram,
                    tokenYProgram,
                    memoProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::RemoveLiquidity2 { accounts, args });
            }
            [204u8, 2u8, 195u8, 145u8, 53u8, 145u8, 145u8, 205u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveLiquidityByRange2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let userTokenX = keys.next().unwrap().clone();
                let userTokenY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveLiquidityByRange2Accounts {
                    position,
                    lbPair,
                    binArrayBitmapExtension,
                    userTokenX,
                    userTokenY,
                    reserveX,
                    reserveY,
                    tokenXMint,
                    tokenYMint,
                    sender,
                    tokenXProgram,
                    tokenYProgram,
                    memoProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::RemoveLiquidityByRange2 { accounts, args });
            }
            [65u8, 75u8, 63u8, 76u8, 235u8, 91u8, 91u8, 136u8] => {
                let mut rdr: &[u8] = rest;
                let args = Swap2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let userTokenIn = keys.next().unwrap().clone();
                let userTokenOut = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let hostFeeIn = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = Swap2Accounts {
                    lbPair,
                    binArrayBitmapExtension,
                    reserveX,
                    reserveY,
                    userTokenIn,
                    userTokenOut,
                    tokenXMint,
                    tokenYMint,
                    oracle,
                    hostFeeIn,
                    user,
                    tokenXProgram,
                    tokenYProgram,
                    memoProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Swap2 { accounts, args });
            }
            [43u8, 215u8, 247u8, 132u8, 137u8, 60u8, 243u8, 81u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapExactOut2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let userTokenIn = keys.next().unwrap().clone();
                let userTokenOut = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let hostFeeIn = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapExactOut2Accounts {
                    lbPair,
                    binArrayBitmapExtension,
                    reserveX,
                    reserveY,
                    userTokenIn,
                    userTokenOut,
                    tokenXMint,
                    tokenYMint,
                    oracle,
                    hostFeeIn,
                    user,
                    tokenXProgram,
                    tokenYProgram,
                    memoProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SwapExactOut2 { accounts, args });
            }
            [74u8, 98u8, 192u8, 214u8, 177u8, 51u8, 75u8, 51u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapWithPriceImpact2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lbPair = keys.next().unwrap().clone();
                let binArrayBitmapExtension = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let userTokenIn = keys.next().unwrap().clone();
                let userTokenOut = keys.next().unwrap().clone();
                let tokenXMint = keys.next().unwrap().clone();
                let tokenYMint = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let hostFeeIn = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let tokenXProgram = keys.next().unwrap().clone();
                let tokenYProgram = keys.next().unwrap().clone();
                let memoProgram = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapWithPriceImpact2Accounts {
                    lbPair,
                    binArrayBitmapExtension,
                    reserveX,
                    reserveY,
                    userTokenIn,
                    userTokenOut,
                    tokenXMint,
                    tokenYMint,
                    oracle,
                    hostFeeIn,
                    user,
                    tokenXProgram,
                    tokenYProgram,
                    memoProgram,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SwapWithPriceImpact2 { accounts, args });
            }
            [174u8, 90u8, 35u8, 115u8, 186u8, 40u8, 147u8, 226u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClosePosition2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let rentReceiver = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClosePosition2Accounts {
                    position,
                    sender,
                    rentReceiver,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClosePosition2 { accounts, args });
            }
            [32u8, 142u8, 184u8, 154u8, 103u8, 65u8, 184u8, 88u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateFeesAndReward2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let lbPair = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateFeesAndReward2Accounts {
                    position,
                    lbPair,
                    owner,
                    remaining,
                };
                return Ok(Instruction::UpdateFeesAndReward2 { accounts, args });
            }
            [59u8, 124u8, 212u8, 118u8, 91u8, 152u8, 110u8, 157u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClosePositionIfEmptyArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let position = keys.next().unwrap().clone();
                let sender = keys.next().unwrap().clone();
                let rentReceiver = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClosePositionIfEmptyAccounts {
                    position,
                    sender,
                    rentReceiver,
                    eventAuthority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ClosePositionIfEmpty { accounts, args });
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
    pub struct CompositionFee {
        #[serde(with = "pubkey_serde")]
        pub from: [u8; 32usize],
        pub bin_id: i16,
        pub token_x_fee_amount: u64,
        pub token_y_fee_amount: u64,
        pub protocol_token_x_fee_amount: u64,
        pub protocol_token_y_fee_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddLiquidity {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub from: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position: [u8; 32usize],
        pub amounts: [u64; 2usize],
        pub active_bin_id: i32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveLiquidity {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub from: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position: [u8; 32usize],
        pub amounts: [u64; 2usize],
        pub active_bin_id: i32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct Swap {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub from: [u8; 32usize],
        pub start_bin_id: i32,
        pub end_bin_id: i32,
        pub amount_in: u64,
        pub amount_out: u64,
        pub swap_for_y: bool,
        pub fee: u64,
        pub protocol_fee: u64,
        pub fee_bps: u128,
        pub host_fee: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimReward {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        pub reward_index: u64,
        pub total_reward: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FundReward {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub funder: [u8; 32usize],
        pub reward_index: u64,
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeReward {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub reward_mint: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub funder: [u8; 32usize],
        pub reward_index: u64,
        pub reward_duration: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateRewardDuration {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        pub reward_index: u64,
        pub old_reward_duration: u64,
        pub new_reward_duration: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateRewardFunder {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        pub reward_index: u64,
        #[serde(with = "pubkey_serde")]
        pub old_funder: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub new_funder: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PositionClose {
        #[serde(with = "pubkey_serde")]
        pub position: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimFee {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        pub fee_x: u64,
        pub fee_y: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LbPairCreate {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        pub bin_step: u16,
        #[serde(with = "pubkey_serde")]
        pub token_x: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub token_y: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PositionCreate {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreasePositionLength {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        pub length_to_add: u16,
        pub side: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DecreasePositionLength {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub position: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        pub length_to_remove: u16,
        pub side: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FeeParameterUpdate {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        pub protocol_share: u16,
        pub base_factor: u16,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DynamicFeeParameterUpdate {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        pub filter_period: u16,
        pub decay_period: u16,
        pub reduction_factor: u16,
        pub variable_fee_control: u32,
        pub max_volatility_accumulator: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IncreaseObservation {
        #[serde(with = "pubkey_serde")]
        pub oracle: [u8; 32usize],
        pub new_observation_length: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawIneligibleReward {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub reward_mint: [u8; 32usize],
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePositionOperator {
        #[serde(with = "pubkey_serde")]
        pub position: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub old_operator: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub new_operator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePositionLockReleasePoint {
        #[serde(with = "pubkey_serde")]
        pub position: [u8; 32usize],
        pub current_point: u64,
        pub new_lock_release_point: u64,
        pub old_lock_release_point: u64,
        #[serde(with = "pubkey_serde")]
        pub sender: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct GoToABin {
        #[serde(with = "pubkey_serde")]
        pub lb_pair: [u8; 32usize],
        pub from_bin_id: i32,
        pub to_bin_id: i32,
    }
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        CompositionFee {
            args: CompositionFee,
        },
        AddLiquidity {
            args: AddLiquidity,
        },
        RemoveLiquidity {
            args: RemoveLiquidity,
        },
        Swap {
            args: Swap,
        },
        ClaimReward {
            args: ClaimReward,
        },
        FundReward {
            args: FundReward,
        },
        InitializeReward {
            args: InitializeReward,
        },
        UpdateRewardDuration {
            args: UpdateRewardDuration,
        },
        UpdateRewardFunder {
            args: UpdateRewardFunder,
        },
        PositionClose {
            args: PositionClose,
        },
        ClaimFee {
            args: ClaimFee,
        },
        LbPairCreate {
            args: LbPairCreate,
        },
        PositionCreate {
            args: PositionCreate,
        },
        IncreasePositionLength {
            args: IncreasePositionLength,
        },
        DecreasePositionLength {
            args: DecreasePositionLength,
        },
        FeeParameterUpdate {
            args: FeeParameterUpdate,
        },
        DynamicFeeParameterUpdate {
            args: DynamicFeeParameterUpdate,
        },
        IncreaseObservation {
            args: IncreaseObservation,
        },
        WithdrawIneligibleReward {
            args: WithdrawIneligibleReward,
        },
        UpdatePositionOperator {
            args: UpdatePositionOperator,
        },
        UpdatePositionLockReleasePoint {
            args: UpdatePositionLockReleasePoint,
        },
        GoToABin {
            args: GoToABin,
        },
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
                [128u8, 151u8, 123u8, 106u8, 17u8, 102u8, 113u8, 142u8] => {
                    let mut rdr = &payload[..];
                    let args = CompositionFee::deserialize(&mut rdr)?;
                    return Ok(Event::CompositionFee { args });
                }
                [31u8, 94u8, 125u8, 90u8, 227u8, 52u8, 61u8, 186u8] => {
                    let mut rdr = &payload[..];
                    let args = AddLiquidity::deserialize(&mut rdr)?;
                    return Ok(Event::AddLiquidity { args });
                }
                [116u8, 244u8, 97u8, 232u8, 103u8, 31u8, 152u8, 58u8] => {
                    let mut rdr = &payload[..];
                    let args = RemoveLiquidity::deserialize(&mut rdr)?;
                    return Ok(Event::RemoveLiquidity { args });
                }
                [81u8, 108u8, 227u8, 190u8, 205u8, 208u8, 10u8, 196u8] => {
                    let mut rdr = &payload[..];
                    let args = Swap::deserialize(&mut rdr)?;
                    return Ok(Event::Swap { args });
                }
                [148u8, 116u8, 134u8, 204u8, 22u8, 171u8, 85u8, 95u8] => {
                    let mut rdr = &payload[..];
                    let args = ClaimReward::deserialize(&mut rdr)?;
                    return Ok(Event::ClaimReward { args });
                }
                [246u8, 228u8, 58u8, 130u8, 145u8, 170u8, 79u8, 204u8] => {
                    let mut rdr = &payload[..];
                    let args = FundReward::deserialize(&mut rdr)?;
                    return Ok(Event::FundReward { args });
                }
                [211u8, 153u8, 88u8, 62u8, 149u8, 60u8, 177u8, 70u8] => {
                    let mut rdr = &payload[..];
                    let args = InitializeReward::deserialize(&mut rdr)?;
                    return Ok(Event::InitializeReward { args });
                }
                [223u8, 245u8, 224u8, 153u8, 49u8, 29u8, 163u8, 172u8] => {
                    let mut rdr = &payload[..];
                    let args = UpdateRewardDuration::deserialize(&mut rdr)?;
                    return Ok(Event::UpdateRewardDuration { args });
                }
                [224u8, 178u8, 174u8, 74u8, 252u8, 165u8, 85u8, 180u8] => {
                    let mut rdr = &payload[..];
                    let args = UpdateRewardFunder::deserialize(&mut rdr)?;
                    return Ok(Event::UpdateRewardFunder { args });
                }
                [255u8, 196u8, 16u8, 107u8, 28u8, 202u8, 53u8, 128u8] => {
                    let mut rdr = &payload[..];
                    let args = PositionClose::deserialize(&mut rdr)?;
                    return Ok(Event::PositionClose { args });
                }
                [75u8, 122u8, 154u8, 48u8, 140u8, 74u8, 123u8, 163u8] => {
                    let mut rdr = &payload[..];
                    let args = ClaimFee::deserialize(&mut rdr)?;
                    return Ok(Event::ClaimFee { args });
                }
                [185u8, 74u8, 252u8, 125u8, 27u8, 215u8, 188u8, 111u8] => {
                    let mut rdr = &payload[..];
                    let args = LbPairCreate::deserialize(&mut rdr)?;
                    return Ok(Event::LbPairCreate { args });
                }
                [144u8, 142u8, 252u8, 84u8, 157u8, 53u8, 37u8, 121u8] => {
                    let mut rdr = &payload[..];
                    let args = PositionCreate::deserialize(&mut rdr)?;
                    return Ok(Event::PositionCreate { args });
                }
                [157u8, 239u8, 42u8, 204u8, 30u8, 56u8, 223u8, 46u8] => {
                    let mut rdr = &payload[..];
                    let args = IncreasePositionLength::deserialize(&mut rdr)?;
                    return Ok(Event::IncreasePositionLength { args });
                }
                [52u8, 118u8, 235u8, 85u8, 172u8, 169u8, 15u8, 128u8] => {
                    let mut rdr = &payload[..];
                    let args = DecreasePositionLength::deserialize(&mut rdr)?;
                    return Ok(Event::DecreasePositionLength { args });
                }
                [48u8, 76u8, 241u8, 117u8, 144u8, 215u8, 242u8, 44u8] => {
                    let mut rdr = &payload[..];
                    let args = FeeParameterUpdate::deserialize(&mut rdr)?;
                    return Ok(Event::FeeParameterUpdate { args });
                }
                [88u8, 88u8, 178u8, 135u8, 194u8, 146u8, 91u8, 243u8] => {
                    let mut rdr = &payload[..];
                    let args = DynamicFeeParameterUpdate::deserialize(&mut rdr)?;
                    return Ok(Event::DynamicFeeParameterUpdate { args });
                }
                [99u8, 249u8, 17u8, 121u8, 166u8, 156u8, 207u8, 215u8] => {
                    let mut rdr = &payload[..];
                    let args = IncreaseObservation::deserialize(&mut rdr)?;
                    return Ok(Event::IncreaseObservation { args });
                }
                [231u8, 189u8, 65u8, 149u8, 102u8, 215u8, 154u8, 244u8] => {
                    let mut rdr = &payload[..];
                    let args = WithdrawIneligibleReward::deserialize(&mut rdr)?;
                    return Ok(Event::WithdrawIneligibleReward { args });
                }
                [39u8, 115u8, 48u8, 204u8, 246u8, 47u8, 66u8, 57u8] => {
                    let mut rdr = &payload[..];
                    let args = UpdatePositionOperator::deserialize(&mut rdr)?;
                    return Ok(Event::UpdatePositionOperator { args });
                }
                [133u8, 214u8, 66u8, 224u8, 64u8, 12u8, 7u8, 191u8] => {
                    let mut rdr = &payload[..];
                    let args = UpdatePositionLockReleasePoint::deserialize(&mut rdr)?;
                    return Ok(Event::UpdatePositionLockReleasePoint { args });
                }
                [59u8, 138u8, 76u8, 68u8, 138u8, 131u8, 176u8, 67u8] => {
                    let mut rdr = &payload[..];
                    let args = GoToABin::deserialize(&mut rdr)?;
                    return Ok(Event::GoToABin { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
