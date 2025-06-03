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
    serde_big_array::big_array! { BigArray ; 64 , 51 }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateConfigMode {
        UpdateLoanToValuePct,
        UpdateMaxLiquidationBonusBps,
        UpdateLiquidationThresholdPct,
        UpdateProtocolLiquidationFee,
        UpdateProtocolTakeRate,
        UpdateFeesBorrowFee,
        UpdateFeesFlashLoanFee,
        DeprecatedUpdateFeesReferralFeeBps,
        UpdateDepositLimit,
        UpdateBorrowLimit,
        UpdateTokenInfoLowerHeuristic,
        UpdateTokenInfoUpperHeuristic,
        UpdateTokenInfoExpHeuristic,
        UpdateTokenInfoTwapDivergence,
        UpdateTokenInfoScopeTwap,
        UpdateTokenInfoScopeChain,
        UpdateTokenInfoName,
        UpdateTokenInfoPriceMaxAge,
        UpdateTokenInfoTwapMaxAge,
        UpdateScopePriceFeed,
        UpdatePythPrice,
        UpdateSwitchboardFeed,
        UpdateSwitchboardTwapFeed,
        UpdateBorrowRateCurve,
        UpdateEntireReserveConfig,
        UpdateDebtWithdrawalCap,
        UpdateDepositWithdrawalCap,
        DeprecatedUpdateDebtWithdrawalCapCurrentTotal,
        DeprecatedUpdateDepositWithdrawalCapCurrentTotal,
        UpdateBadDebtLiquidationBonusBps,
        UpdateMinLiquidationBonusBps,
        UpdateDeleveragingMarginCallPeriod,
        UpdateBorrowFactor,
        UpdateAssetTier,
        UpdateElevationGroup,
        UpdateDeleveragingThresholdDecreaseBpsPerDay,
        DeprecatedUpdateMultiplierSideBoost,
        DeprecatedUpdateMultiplierTagBoost,
        UpdateReserveStatus,
        UpdateFarmCollateral,
        UpdateFarmDebt,
        UpdateDisableUsageAsCollateralOutsideEmode,
        UpdateBlockBorrowingAboveUtilizationPct,
        UpdateBlockPriceUsage,
        UpdateBorrowLimitOutsideElevationGroup,
        UpdateBorrowLimitsInElevationGroupAgainstThisReserve,
        UpdateHostFixedInterestRateBps,
        UpdateAutodeleverageEnabled,
        UpdateDeleveragingBonusIncreaseBpsPerDay,
        UpdateProtocolOrderExecutionFee,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateLendingMarketConfigValue {
        Bool(bool),
        U8(u8),
        U8Array([u8; 8usize]),
        U16(u16),
        U64(u64),
        U128(u128),
        Pubkey([u8; 32usize]),
        ElevationGroup(ElevationGroup),
        Name([u8; 32usize]),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateLendingMarketMode {
        UpdateOwner,
        UpdateEmergencyMode,
        UpdateLiquidationCloseFactor,
        UpdateLiquidationMaxValue,
        DeprecatedUpdateGlobalUnhealthyBorrow,
        UpdateGlobalAllowedBorrow,
        UpdateRiskCouncil,
        UpdateMinFullLiquidationThreshold,
        UpdateInsolvencyRiskLtv,
        UpdateElevationGroup,
        UpdateReferralFeeBps,
        DeprecatedUpdateMultiplierPoints,
        UpdatePriceRefreshTriggerToMaxAgePct,
        UpdateAutodeleverageEnabled,
        UpdateBorrowingDisabled,
        UpdateMinNetValueObligationPostAction,
        UpdateMinValueLtvSkipPriorityLiqCheck,
        UpdateMinValueBfSkipPriorityLiqCheck,
        UpdatePaddingFields,
        UpdateName,
        UpdateIndividualAutodeleverageMarginCallPeriodSecs,
        UpdateInitialDepositAmount,
        UpdateObligationOrderExecutionEnabled,
        UpdateImmutableFlag,
        UpdateObligationOrderCreationEnabled,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateGlobalConfigMode {
        PendingAdmin,
        FeeCollector,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LastUpdate {
        pub slot: u64,
        pub stale: u8,
        pub price_status: u8,
        pub placeholder: [u8; 6usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ElevationGroup {
        pub max_liquidation_bonus_bps: u16,
        pub id: u8,
        pub ltv_pct: u8,
        pub liquidation_threshold_pct: u8,
        pub allow_new_loans: u8,
        pub max_reserves_as_collateral: u8,
        pub padding0: u8,
        #[serde(with = "pubkey_serde")]
        pub debt_reserve: [u8; 32usize],
        pub padding1: [u64; 4usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct InitObligationArgs {
        pub tag: u8,
        pub id: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ObligationCollateral {
        #[serde(with = "pubkey_serde")]
        pub deposit_reserve: [u8; 32usize],
        pub deposited_amount: u64,
        pub market_value_sf: u128,
        pub borrowed_amount_against_this_collateral_in_elevation_group: u64,
        pub padding: [u64; 9usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ObligationLiquidity {
        #[serde(with = "pubkey_serde")]
        pub borrow_reserve: [u8; 32usize],
        pub cumulative_borrow_rate_bsf: BigFractionBytes,
        pub padding: u64,
        pub borrowed_amount_sf: u128,
        pub market_value_sf: u128,
        pub borrow_factor_adjusted_market_value_sf: u128,
        pub borrowed_amount_outside_elevation_groups: u64,
        pub padding2: [u64; 7usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ObligationOrder {
        pub condition_threshold_sf: u128,
        pub opportunity_parameter_sf: u128,
        pub min_execution_bonus_bps: u16,
        pub max_execution_bonus_bps: u16,
        pub condition_type: u8,
        pub opportunity_type: u8,
        pub padding1: [u8; 10usize],
        pub padding2: [u128; 5usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum AssetTier {
        Regular,
        IsolatedCollateral,
        IsolatedDebt,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BigFractionBytes {
        pub value: [u64; 4usize],
        pub padding: [u64; 2usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum FeeCalculation {
        Exclusive,
        Inclusive,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ReserveCollateral {
        #[serde(with = "pubkey_serde")]
        pub mint_pubkey: [u8; 32usize],
        pub mint_total_supply: u64,
        #[serde(with = "pubkey_serde")]
        pub supply_vault: [u8; 32usize],
        pub padding1: [u128; 32usize],
        pub padding2: [u128; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ReserveConfig {
        pub status: u8,
        pub asset_tier: u8,
        pub host_fixed_interest_rate_bps: u16,
        pub reserved2: [u8; 9usize],
        pub protocol_order_execution_fee_pct: u8,
        pub protocol_take_rate_pct: u8,
        pub protocol_liquidation_fee_pct: u8,
        pub loan_to_value_pct: u8,
        pub liquidation_threshold_pct: u8,
        pub min_liquidation_bonus_bps: u16,
        pub max_liquidation_bonus_bps: u16,
        pub bad_debt_liquidation_bonus_bps: u16,
        pub deleveraging_margin_call_period_secs: u64,
        pub deleveraging_threshold_decrease_bps_per_day: u64,
        pub fees: ReserveFees,
        pub borrow_rate_curve: BorrowRateCurve,
        pub borrow_factor_pct: u64,
        pub deposit_limit: u64,
        pub borrow_limit: u64,
        pub token_info: TokenInfo,
        pub deposit_withdrawal_cap: WithdrawalCaps,
        pub debt_withdrawal_cap: WithdrawalCaps,
        pub elevation_groups: [u8; 20usize],
        pub disable_usage_as_coll_outside_emode: u8,
        pub utilization_limit_block_borrowing_above_pct: u8,
        pub autodeleverage_enabled: u8,
        pub reserved1: [u8; 1usize],
        pub borrow_limit_outside_elevation_group: u64,
        pub borrow_limit_against_this_collateral_in_elevation_group: [u64; 32usize],
        pub deleveraging_bonus_increase_bps_per_day: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ReserveFarmKind {
        Collateral,
        Debt,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ReserveFees {
        pub borrow_fee_sf: u64,
        pub flash_loan_fee_sf: u64,
        pub padding: [u8; 8usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ReserveLiquidity {
        #[serde(with = "pubkey_serde")]
        pub mint_pubkey: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub supply_vault: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub fee_vault: [u8; 32usize],
        pub available_amount: u64,
        pub borrowed_amount_sf: u128,
        pub market_price_sf: u128,
        pub market_price_last_updated_ts: u64,
        pub mint_decimals: u64,
        pub deposit_limit_crossed_timestamp: u64,
        pub borrow_limit_crossed_timestamp: u64,
        pub cumulative_borrow_rate_bsf: BigFractionBytes,
        pub accumulated_protocol_fees_sf: u128,
        pub accumulated_referrer_fees_sf: u128,
        pub pending_referrer_fees_sf: u128,
        pub absolute_referral_rate_sf: u128,
        #[serde(with = "pubkey_serde")]
        pub token_program: [u8; 32usize],
        #[serde(with = "BigArray")]
        pub padding2: [u64; 51usize],
        pub padding3: [u128; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ReserveStatus {
        Active,
        Obsolete,
        Hidden,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WithdrawalCaps {
        pub config_capacity: i64,
        pub current_total: i64,
        pub last_interval_start_timestamp: u64,
        pub config_interval_length_seconds: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PriceHeuristic {
        pub lower: u64,
        pub upper: u64,
        pub exp: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PythConfiguration {
        #[serde(with = "pubkey_serde")]
        pub price: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ScopeConfiguration {
        #[serde(with = "pubkey_serde")]
        pub price_feed: [u8; 32usize],
        pub price_chain: [u16; 4usize],
        pub twap_chain: [u16; 4usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SwitchboardConfiguration {
        #[serde(with = "pubkey_serde")]
        pub price_aggregator: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub twap_aggregator: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TokenInfo {
        pub name: [u8; 32usize],
        pub heuristic: PriceHeuristic,
        pub max_twap_divergence_bps: u64,
        pub max_age_price_seconds: u64,
        pub max_age_twap_seconds: u64,
        pub scope_configuration: ScopeConfiguration,
        pub switchboard_configuration: SwitchboardConfiguration,
        pub pyth_configuration: PythConfiguration,
        pub block_price_usage: u8,
        pub reserved: [u8; 7usize],
        pub padding: [u64; 19usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BorrowRateCurve {
        pub points: [CurvePoint; 11usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CurvePoint {
        pub utilization_rate_bps: u32,
        pub borrow_rate_bps: u32,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitLendingMarketAccounts {
        pub lendingMarketOwner: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateLendingMarketAccounts {
        pub lendingMarketOwner: String,
        pub lendingMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateLendingMarketOwnerAccounts {
        pub lendingMarketOwnerCached: String,
        pub lendingMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitReserveAccounts {
        pub lendingMarketOwner: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub reserve: String,
        pub reserveLiquidityMint: String,
        pub reserveLiquiditySupply: String,
        pub feeReceiver: String,
        pub reserveCollateralMint: String,
        pub reserveCollateralSupply: String,
        pub initialLiquiditySource: String,
        pub rent: String,
        pub liquidityTokenProgram: String,
        pub collateralTokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitFarmsForReserveAccounts {
        pub lendingMarketOwner: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub reserve: String,
        pub farmsProgram: String,
        pub farmsGlobalConfig: String,
        pub farmState: String,
        pub farmsVaultAuthority: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateReserveConfigAccounts {
        pub signer: String,
        pub globalConfig: String,
        pub lendingMarket: String,
        pub reserve: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RedeemFeesAccounts {
        pub reserve: String,
        pub reserveLiquidityMint: String,
        pub reserveLiquidityFeeReceiver: String,
        pub reserveSupplyLiquidity: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawProtocolFeeAccounts {
        pub globalConfig: String,
        pub lendingMarket: String,
        pub reserve: String,
        pub reserveLiquidityMint: String,
        pub lendingMarketAuthority: String,
        pub feeVault: String,
        pub feeCollectorAta: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SocializeLossAccounts {
        pub riskCouncil: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub reserve: String,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SocializeLossV2Accounts {
        pub riskCouncil: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub reserve: String,
        pub instructionSysvarAccount: String,
        pub obligationFarmUserState: String,
        pub reserveFarmState: String,
        pub lendingMarketAuthority: String,
        pub farmsProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MarkObligationForDeleveragingAccounts {
        pub riskCouncil: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RefreshReserveAccounts {
        pub reserve: String,
        pub lendingMarket: String,
        pub pythOracle: String,
        pub switchboardPriceOracle: String,
        pub switchboardTwapOracle: String,
        pub scopePrices: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RefreshReservesBatchAccounts {
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositReserveLiquidityAccounts {
        pub owner: String,
        pub reserve: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub reserveLiquidityMint: String,
        pub reserveLiquiditySupply: String,
        pub reserveCollateralMint: String,
        pub userSourceLiquidity: String,
        pub userDestinationCollateral: String,
        pub collateralTokenProgram: String,
        pub liquidityTokenProgram: String,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RedeemReserveCollateralAccounts {
        pub owner: String,
        pub lendingMarket: String,
        pub reserve: String,
        pub lendingMarketAuthority: String,
        pub reserveLiquidityMint: String,
        pub reserveCollateralMint: String,
        pub reserveLiquiditySupply: String,
        pub userSourceCollateral: String,
        pub userDestinationLiquidity: String,
        pub collateralTokenProgram: String,
        pub liquidityTokenProgram: String,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitObligationAccounts {
        pub obligationOwner: String,
        pub feePayer: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub seed1Account: String,
        pub seed2Account: String,
        pub ownerUserMetadata: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitObligationFarmsForReserveAccounts {
        pub payer: String,
        pub owner: String,
        pub obligation: String,
        pub lendingMarketAuthority: String,
        pub reserve: String,
        pub reserveFarmState: String,
        pub obligationFarm: String,
        pub lendingMarket: String,
        pub farmsProgram: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RefreshObligationFarmsForReserveAccounts {
        pub crank: String,
        pub obligation: String,
        pub lendingMarketAuthority: String,
        pub reserve: String,
        pub reserveFarmState: String,
        pub obligationFarmUserState: String,
        pub lendingMarket: String,
        pub farmsProgram: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RefreshObligationAccounts {
        pub lendingMarket: String,
        pub obligation: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositObligationCollateralAccounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub depositReserve: String,
        pub reserveDestinationCollateral: String,
        pub userSourceCollateral: String,
        pub tokenProgram: String,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositObligationCollateralV2Accounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub depositReserve: String,
        pub reserveDestinationCollateral: String,
        pub userSourceCollateral: String,
        pub tokenProgram: String,
        pub instructionSysvarAccount: String,
        pub lendingMarketAuthority: String,
        pub obligationFarmUserState: String,
        pub reserveFarmState: String,
        pub farmsProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawObligationCollateralAccounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub withdrawReserve: String,
        pub reserveSourceCollateral: String,
        pub userDestinationCollateral: String,
        pub tokenProgram: String,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawObligationCollateralV2Accounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub withdrawReserve: String,
        pub reserveSourceCollateral: String,
        pub userDestinationCollateral: String,
        pub tokenProgram: String,
        pub instructionSysvarAccount: String,
        pub obligationFarmUserState: String,
        pub reserveFarmState: String,
        pub farmsProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BorrowObligationLiquidityAccounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub borrowReserve: String,
        pub borrowReserveLiquidityMint: String,
        pub reserveSourceLiquidity: String,
        pub borrowReserveLiquidityFeeReceiver: String,
        pub userDestinationLiquidity: String,
        pub referrerTokenState: String,
        pub tokenProgram: String,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BorrowObligationLiquidityV2Accounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub borrowReserve: String,
        pub borrowReserveLiquidityMint: String,
        pub reserveSourceLiquidity: String,
        pub borrowReserveLiquidityFeeReceiver: String,
        pub userDestinationLiquidity: String,
        pub referrerTokenState: String,
        pub tokenProgram: String,
        pub instructionSysvarAccount: String,
        pub obligationFarmUserState: String,
        pub reserveFarmState: String,
        pub farmsProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RepayObligationLiquidityAccounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub repayReserve: String,
        pub reserveLiquidityMint: String,
        pub reserveDestinationLiquidity: String,
        pub userSourceLiquidity: String,
        pub tokenProgram: String,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RepayObligationLiquidityV2Accounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub repayReserve: String,
        pub reserveLiquidityMint: String,
        pub reserveDestinationLiquidity: String,
        pub userSourceLiquidity: String,
        pub tokenProgram: String,
        pub instructionSysvarAccount: String,
        pub obligationFarmUserState: String,
        pub reserveFarmState: String,
        pub lendingMarketAuthority: String,
        pub farmsProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RepayAndWithdrawAndRedeemAccounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub repayReserve: String,
        pub reserveLiquidityMint: String,
        pub reserveDestinationLiquidity: String,
        pub userSourceLiquidity: String,
        pub tokenProgram: String,
        pub instructionSysvarAccount: String,
        pub lendingMarketAuthority: String,
        pub withdrawReserve: String,
        pub reserveSourceCollateral: String,
        pub reserveCollateralMint: String,
        pub reserveLiquiditySupply: String,
        pub userDestinationLiquidity: String,
        pub placeholderUserDestinationCollateral: String,
        pub collateralTokenProgram: String,
        pub liquidityTokenProgram: String,
        pub obligationFarmUserState: String,
        pub reserveFarmState: String,
        pub farmsProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositAndWithdrawAccounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub reserve: String,
        pub reserveLiquidityMint: String,
        pub reserveLiquiditySupply: String,
        pub reserveCollateralMint: String,
        pub reserveDestinationDepositCollateral: String,
        pub userSourceLiquidity: String,
        pub placeholderUserDestinationCollateral: String,
        pub collateralTokenProgram: String,
        pub liquidityTokenProgram: String,
        pub instructionSysvarAccount: String,
        pub withdrawReserve: String,
        pub reserveSourceCollateral: String,
        pub userDestinationLiquidity: String,
        pub obligationFarmUserState: String,
        pub reserveFarmState: String,
        pub farmsProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositReserveLiquidityAndObligationCollateralAccounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub reserve: String,
        pub reserveLiquidityMint: String,
        pub reserveLiquiditySupply: String,
        pub reserveCollateralMint: String,
        pub reserveDestinationDepositCollateral: String,
        pub userSourceLiquidity: String,
        pub placeholderUserDestinationCollateral: String,
        pub collateralTokenProgram: String,
        pub liquidityTokenProgram: String,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DepositReserveLiquidityAndObligationCollateralV2Accounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub reserve: String,
        pub reserveLiquidityMint: String,
        pub reserveLiquiditySupply: String,
        pub reserveCollateralMint: String,
        pub reserveDestinationDepositCollateral: String,
        pub userSourceLiquidity: String,
        pub placeholderUserDestinationCollateral: String,
        pub collateralTokenProgram: String,
        pub liquidityTokenProgram: String,
        pub instructionSysvarAccount: String,
        pub obligationFarmUserState: String,
        pub reserveFarmState: String,
        pub farmsProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawObligationCollateralAndRedeemReserveCollateralAccounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub withdrawReserve: String,
        pub reserveLiquidityMint: String,
        pub reserveSourceCollateral: String,
        pub reserveCollateralMint: String,
        pub reserveLiquiditySupply: String,
        pub userDestinationLiquidity: String,
        pub placeholderUserDestinationCollateral: String,
        pub collateralTokenProgram: String,
        pub liquidityTokenProgram: String,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub withdrawReserve: String,
        pub reserveLiquidityMint: String,
        pub reserveSourceCollateral: String,
        pub reserveCollateralMint: String,
        pub reserveLiquiditySupply: String,
        pub userDestinationLiquidity: String,
        pub placeholderUserDestinationCollateral: String,
        pub collateralTokenProgram: String,
        pub liquidityTokenProgram: String,
        pub instructionSysvarAccount: String,
        pub obligationFarmUserState: String,
        pub reserveFarmState: String,
        pub farmsProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LiquidateObligationAndRedeemReserveCollateralAccounts {
        pub liquidator: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub repayReserve: String,
        pub repayReserveLiquidityMint: String,
        pub repayReserveLiquiditySupply: String,
        pub withdrawReserve: String,
        pub withdrawReserveLiquidityMint: String,
        pub withdrawReserveCollateralMint: String,
        pub withdrawReserveCollateralSupply: String,
        pub withdrawReserveLiquiditySupply: String,
        pub withdrawReserveLiquidityFeeReceiver: String,
        pub userSourceLiquidity: String,
        pub userDestinationCollateral: String,
        pub userDestinationLiquidity: String,
        pub collateralTokenProgram: String,
        pub repayLiquidityTokenProgram: String,
        pub withdrawLiquidityTokenProgram: String,
        pub instructionSysvarAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LiquidateObligationAndRedeemReserveCollateralV2Accounts {
        pub liquidator: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub repayReserve: String,
        pub repayReserveLiquidityMint: String,
        pub repayReserveLiquiditySupply: String,
        pub withdrawReserve: String,
        pub withdrawReserveLiquidityMint: String,
        pub withdrawReserveCollateralMint: String,
        pub withdrawReserveCollateralSupply: String,
        pub withdrawReserveLiquiditySupply: String,
        pub withdrawReserveLiquidityFeeReceiver: String,
        pub userSourceLiquidity: String,
        pub userDestinationCollateral: String,
        pub userDestinationLiquidity: String,
        pub collateralTokenProgram: String,
        pub repayLiquidityTokenProgram: String,
        pub withdrawLiquidityTokenProgram: String,
        pub instructionSysvarAccount: String,
        pub obligationFarmUserState: String,
        pub reserveFarmState: String,
        pub farmsProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FlashRepayReserveLiquidityAccounts {
        pub userTransferAuthority: String,
        pub lendingMarketAuthority: String,
        pub lendingMarket: String,
        pub reserve: String,
        pub reserveLiquidityMint: String,
        pub reserveDestinationLiquidity: String,
        pub userSourceLiquidity: String,
        pub reserveLiquidityFeeReceiver: String,
        pub referrerTokenState: String,
        pub referrerAccount: String,
        pub sysvarInfo: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FlashBorrowReserveLiquidityAccounts {
        pub userTransferAuthority: String,
        pub lendingMarketAuthority: String,
        pub lendingMarket: String,
        pub reserve: String,
        pub reserveLiquidityMint: String,
        pub reserveSourceLiquidity: String,
        pub userDestinationLiquidity: String,
        pub reserveLiquidityFeeReceiver: String,
        pub referrerTokenState: String,
        pub referrerAccount: String,
        pub sysvarInfo: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RequestElevationGroupAccounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitReferrerTokenStateAccounts {
        pub payer: String,
        pub lendingMarket: String,
        pub reserve: String,
        pub referrer: String,
        pub referrerTokenState: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitUserMetadataAccounts {
        pub owner: String,
        pub feePayer: String,
        pub userMetadata: String,
        pub referrerUserMetadata: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawReferrerFeesAccounts {
        pub referrer: String,
        pub referrerTokenState: String,
        pub reserve: String,
        pub reserveLiquidityMint: String,
        pub reserveSupplyLiquidity: String,
        pub referrerTokenAccount: String,
        pub lendingMarket: String,
        pub lendingMarketAuthority: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitReferrerStateAndShortUrlAccounts {
        pub referrer: String,
        pub referrerState: String,
        pub referrerShortUrl: String,
        pub referrerUserMetadata: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeleteReferrerStateAndShortUrlAccounts {
        pub referrer: String,
        pub referrerState: String,
        pub shortUrl: String,
        pub rent: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetObligationOrderAccounts {
        pub owner: String,
        pub obligation: String,
        pub lendingMarket: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitGlobalConfigAccounts {
        pub payer: String,
        pub globalConfig: String,
        pub programData: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateGlobalConfigAccounts {
        pub globalAdmin: String,
        pub globalConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateGlobalConfigAdminAccounts {
        pub pendingAdmin: String,
        pub globalConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct IdlMissingTypesAccounts {
        pub signer: String,
        pub globalConfig: String,
        pub lendingMarket: String,
        pub reserve: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    serde_big_array::big_array! { BigArray ; 64 , 51 , 72 }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitLendingMarketArguments {
        pub quote_currency: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateLendingMarketArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub mode: u64,
        #[serde(with = "BigArray")]
        pub value: [u8; 72usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateLendingMarketOwnerArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitReserveArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitFarmsForReserveArguments {
        pub mode: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateReserveConfigArguments {
        pub mode: UpdateConfigMode,
        pub value: Vec<u8>,
        pub skip_config_integrity_validation: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RedeemFeesArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawProtocolFeeArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SocializeLossArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SocializeLossV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MarkObligationForDeleveragingArguments {
        pub autodeleverage_target_ltv_pct: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RefreshReserveArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RefreshReservesBatchArguments {
        pub skip_price_updates: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositReserveLiquidityArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RedeemReserveCollateralArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub collateral_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitObligationArguments {
        pub args: InitObligationArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitObligationFarmsForReserveArguments {
        pub mode: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RefreshObligationFarmsForReserveArguments {
        pub mode: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RefreshObligationArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositObligationCollateralArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub collateral_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositObligationCollateralV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub collateral_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawObligationCollateralArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub collateral_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawObligationCollateralV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub collateral_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BorrowObligationLiquidityArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BorrowObligationLiquidityV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RepayObligationLiquidityArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RepayObligationLiquidityV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RepayAndWithdrawAndRedeemArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub repay_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub withdraw_collateral_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositAndWithdrawArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub withdraw_collateral_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositReserveLiquidityAndObligationCollateralArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DepositReserveLiquidityAndObligationCollateralV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawObligationCollateralAndRedeemReserveCollateralArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub collateral_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawObligationCollateralAndRedeemReserveCollateralV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub collateral_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LiquidateObligationAndRedeemReserveCollateralArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_acceptable_received_liquidity_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_allowed_ltv_override_percent: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LiquidateObligationAndRedeemReserveCollateralV2Arguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_acceptable_received_liquidity_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_allowed_ltv_override_percent: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FlashRepayReserveLiquidityArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
        pub borrow_instruction_index: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FlashBorrowReserveLiquidityArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub liquidity_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RequestElevationGroupArguments {
        pub elevation_group: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitReferrerTokenStateArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitUserMetadataArguments {
        #[serde(with = "pubkey_serde")]
        pub user_lookup_table: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawReferrerFeesArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitReferrerStateAndShortUrlArguments {
        pub short_url: String,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeleteReferrerStateAndShortUrlArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetObligationOrderArguments {
        pub index: u8,
        pub order: ObligationOrder,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitGlobalConfigArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGlobalConfigArguments {
        pub mode: UpdateGlobalConfigMode,
        pub value: Vec<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGlobalConfigAdminArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct IdlMissingTypesArguments {
        pub reserve_farm_kind: ReserveFarmKind,
        pub asset_tier: AssetTier,
        pub fee_calculation: FeeCalculation,
        pub reserve_status: ReserveStatus,
        pub update_config_mode: UpdateConfigMode,
        pub update_lending_market_config_value: UpdateLendingMarketConfigValue,
        pub update_lending_market_config_mode: UpdateLendingMarketMode,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitLendingMarket {
        accounts: InitLendingMarketAccounts,
        args: InitLendingMarketArguments,
    },
    UpdateLendingMarket {
        accounts: UpdateLendingMarketAccounts,
        args: UpdateLendingMarketArguments,
    },
    UpdateLendingMarketOwner {
        accounts: UpdateLendingMarketOwnerAccounts,
        args: UpdateLendingMarketOwnerArguments,
    },
    InitReserve {
        accounts: InitReserveAccounts,
        args: InitReserveArguments,
    },
    InitFarmsForReserve {
        accounts: InitFarmsForReserveAccounts,
        args: InitFarmsForReserveArguments,
    },
    UpdateReserveConfig {
        accounts: UpdateReserveConfigAccounts,
        args: UpdateReserveConfigArguments,
    },
    RedeemFees {
        accounts: RedeemFeesAccounts,
        args: RedeemFeesArguments,
    },
    WithdrawProtocolFee {
        accounts: WithdrawProtocolFeeAccounts,
        args: WithdrawProtocolFeeArguments,
    },
    SocializeLoss {
        accounts: SocializeLossAccounts,
        args: SocializeLossArguments,
    },
    SocializeLossV2 {
        accounts: SocializeLossV2Accounts,
        args: SocializeLossV2Arguments,
    },
    MarkObligationForDeleveraging {
        accounts: MarkObligationForDeleveragingAccounts,
        args: MarkObligationForDeleveragingArguments,
    },
    RefreshReserve {
        accounts: RefreshReserveAccounts,
        args: RefreshReserveArguments,
    },
    RefreshReservesBatch {
        accounts: RefreshReservesBatchAccounts,
        args: RefreshReservesBatchArguments,
    },
    DepositReserveLiquidity {
        accounts: DepositReserveLiquidityAccounts,
        args: DepositReserveLiquidityArguments,
    },
    RedeemReserveCollateral {
        accounts: RedeemReserveCollateralAccounts,
        args: RedeemReserveCollateralArguments,
    },
    InitObligation {
        accounts: InitObligationAccounts,
        args: InitObligationArguments,
    },
    InitObligationFarmsForReserve {
        accounts: InitObligationFarmsForReserveAccounts,
        args: InitObligationFarmsForReserveArguments,
    },
    RefreshObligationFarmsForReserve {
        accounts: RefreshObligationFarmsForReserveAccounts,
        args: RefreshObligationFarmsForReserveArguments,
    },
    RefreshObligation {
        accounts: RefreshObligationAccounts,
        args: RefreshObligationArguments,
    },
    DepositObligationCollateral {
        accounts: DepositObligationCollateralAccounts,
        args: DepositObligationCollateralArguments,
    },
    DepositObligationCollateralV2 {
        accounts: DepositObligationCollateralV2Accounts,
        args: DepositObligationCollateralV2Arguments,
    },
    WithdrawObligationCollateral {
        accounts: WithdrawObligationCollateralAccounts,
        args: WithdrawObligationCollateralArguments,
    },
    WithdrawObligationCollateralV2 {
        accounts: WithdrawObligationCollateralV2Accounts,
        args: WithdrawObligationCollateralV2Arguments,
    },
    BorrowObligationLiquidity {
        accounts: BorrowObligationLiquidityAccounts,
        args: BorrowObligationLiquidityArguments,
    },
    BorrowObligationLiquidityV2 {
        accounts: BorrowObligationLiquidityV2Accounts,
        args: BorrowObligationLiquidityV2Arguments,
    },
    RepayObligationLiquidity {
        accounts: RepayObligationLiquidityAccounts,
        args: RepayObligationLiquidityArguments,
    },
    RepayObligationLiquidityV2 {
        accounts: RepayObligationLiquidityV2Accounts,
        args: RepayObligationLiquidityV2Arguments,
    },
    RepayAndWithdrawAndRedeem {
        accounts: RepayAndWithdrawAndRedeemAccounts,
        args: RepayAndWithdrawAndRedeemArguments,
    },
    DepositAndWithdraw {
        accounts: DepositAndWithdrawAccounts,
        args: DepositAndWithdrawArguments,
    },
    DepositReserveLiquidityAndObligationCollateral {
        accounts: DepositReserveLiquidityAndObligationCollateralAccounts,
        args: DepositReserveLiquidityAndObligationCollateralArguments,
    },
    DepositReserveLiquidityAndObligationCollateralV2 {
        accounts: DepositReserveLiquidityAndObligationCollateralV2Accounts,
        args: DepositReserveLiquidityAndObligationCollateralV2Arguments,
    },
    WithdrawObligationCollateralAndRedeemReserveCollateral {
        accounts: WithdrawObligationCollateralAndRedeemReserveCollateralAccounts,
        args: WithdrawObligationCollateralAndRedeemReserveCollateralArguments,
    },
    WithdrawObligationCollateralAndRedeemReserveCollateralV2 {
        accounts: WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts,
        args: WithdrawObligationCollateralAndRedeemReserveCollateralV2Arguments,
    },
    LiquidateObligationAndRedeemReserveCollateral {
        accounts: LiquidateObligationAndRedeemReserveCollateralAccounts,
        args: LiquidateObligationAndRedeemReserveCollateralArguments,
    },
    LiquidateObligationAndRedeemReserveCollateralV2 {
        accounts: LiquidateObligationAndRedeemReserveCollateralV2Accounts,
        args: LiquidateObligationAndRedeemReserveCollateralV2Arguments,
    },
    FlashRepayReserveLiquidity {
        accounts: FlashRepayReserveLiquidityAccounts,
        args: FlashRepayReserveLiquidityArguments,
    },
    FlashBorrowReserveLiquidity {
        accounts: FlashBorrowReserveLiquidityAccounts,
        args: FlashBorrowReserveLiquidityArguments,
    },
    RequestElevationGroup {
        accounts: RequestElevationGroupAccounts,
        args: RequestElevationGroupArguments,
    },
    InitReferrerTokenState {
        accounts: InitReferrerTokenStateAccounts,
        args: InitReferrerTokenStateArguments,
    },
    InitUserMetadata {
        accounts: InitUserMetadataAccounts,
        args: InitUserMetadataArguments,
    },
    WithdrawReferrerFees {
        accounts: WithdrawReferrerFeesAccounts,
        args: WithdrawReferrerFeesArguments,
    },
    InitReferrerStateAndShortUrl {
        accounts: InitReferrerStateAndShortUrlAccounts,
        args: InitReferrerStateAndShortUrlArguments,
    },
    DeleteReferrerStateAndShortUrl {
        accounts: DeleteReferrerStateAndShortUrlAccounts,
        args: DeleteReferrerStateAndShortUrlArguments,
    },
    SetObligationOrder {
        accounts: SetObligationOrderAccounts,
        args: SetObligationOrderArguments,
    },
    InitGlobalConfig {
        accounts: InitGlobalConfigAccounts,
        args: InitGlobalConfigArguments,
    },
    UpdateGlobalConfig {
        accounts: UpdateGlobalConfigAccounts,
        args: UpdateGlobalConfigArguments,
    },
    UpdateGlobalConfigAdmin {
        accounts: UpdateGlobalConfigAdminAccounts,
        args: UpdateGlobalConfigAdminArguments,
    },
    IdlMissingTypes {
        accounts: IdlMissingTypesAccounts,
        args: IdlMissingTypesArguments,
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
            [34u8, 162u8, 116u8, 14u8, 101u8, 137u8, 94u8, 239u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitLendingMarketArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lendingMarketOwner = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitLendingMarketAccounts {
                    lendingMarketOwner,
                    lendingMarket,
                    lendingMarketAuthority,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::InitLendingMarket { accounts, args });
            }
            [209u8, 157u8, 53u8, 210u8, 97u8, 180u8, 31u8, 45u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateLendingMarketArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lendingMarketOwner = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateLendingMarketAccounts {
                    lendingMarketOwner,
                    lendingMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateLendingMarket { accounts, args });
            }
            [118u8, 224u8, 10u8, 62u8, 196u8, 230u8, 184u8, 89u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateLendingMarketOwnerArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lendingMarketOwnerCached = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateLendingMarketOwnerAccounts {
                    lendingMarketOwnerCached,
                    lendingMarket,
                    remaining,
                };
                return Ok(Instruction::UpdateLendingMarketOwner { accounts, args });
            }
            [138u8, 245u8, 71u8, 225u8, 153u8, 4u8, 3u8, 43u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitReserveArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lendingMarketOwner = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveLiquiditySupply = keys.next().unwrap().clone();
                let feeReceiver = keys.next().unwrap().clone();
                let reserveCollateralMint = keys.next().unwrap().clone();
                let reserveCollateralSupply = keys.next().unwrap().clone();
                let initialLiquiditySource = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let liquidityTokenProgram = keys.next().unwrap().clone();
                let collateralTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitReserveAccounts {
                    lendingMarketOwner,
                    lendingMarket,
                    lendingMarketAuthority,
                    reserve,
                    reserveLiquidityMint,
                    reserveLiquiditySupply,
                    feeReceiver,
                    reserveCollateralMint,
                    reserveCollateralSupply,
                    initialLiquiditySource,
                    rent,
                    liquidityTokenProgram,
                    collateralTokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitReserve { accounts, args });
            }
            [218u8, 6u8, 62u8, 233u8, 1u8, 33u8, 232u8, 82u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitFarmsForReserveArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lendingMarketOwner = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let farmsGlobalConfig = keys.next().unwrap().clone();
                let farmState = keys.next().unwrap().clone();
                let farmsVaultAuthority = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitFarmsForReserveAccounts {
                    lendingMarketOwner,
                    lendingMarket,
                    lendingMarketAuthority,
                    reserve,
                    farmsProgram,
                    farmsGlobalConfig,
                    farmState,
                    farmsVaultAuthority,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitFarmsForReserve { accounts, args });
            }
            [61u8, 148u8, 100u8, 70u8, 143u8, 107u8, 17u8, 13u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateReserveConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let signer = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateReserveConfigAccounts {
                    signer,
                    globalConfig,
                    lendingMarket,
                    reserve,
                    remaining,
                };
                return Ok(Instruction::UpdateReserveConfig { accounts, args });
            }
            [215u8, 39u8, 180u8, 41u8, 173u8, 46u8, 248u8, 220u8] => {
                let mut rdr: &[u8] = rest;
                let args = RedeemFeesArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let reserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveLiquidityFeeReceiver = keys.next().unwrap().clone();
                let reserveSupplyLiquidity = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RedeemFeesAccounts {
                    reserve,
                    reserveLiquidityMint,
                    reserveLiquidityFeeReceiver,
                    reserveSupplyLiquidity,
                    lendingMarket,
                    lendingMarketAuthority,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::RedeemFees { accounts, args });
            }
            [158u8, 201u8, 158u8, 189u8, 33u8, 93u8, 162u8, 103u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawProtocolFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let globalConfig = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let feeVault = keys.next().unwrap().clone();
                let feeCollectorAta = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawProtocolFeeAccounts {
                    globalConfig,
                    lendingMarket,
                    reserve,
                    reserveLiquidityMint,
                    lendingMarketAuthority,
                    feeVault,
                    feeCollectorAta,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawProtocolFee { accounts, args });
            }
            [245u8, 75u8, 91u8, 0u8, 236u8, 97u8, 19u8, 3u8] => {
                let mut rdr: &[u8] = rest;
                let args = SocializeLossArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let riskCouncil = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SocializeLossAccounts {
                    riskCouncil,
                    obligation,
                    lendingMarket,
                    reserve,
                    instructionSysvarAccount,
                    remaining,
                };
                return Ok(Instruction::SocializeLoss { accounts, args });
            }
            [238u8, 95u8, 98u8, 220u8, 187u8, 40u8, 204u8, 154u8] => {
                let mut rdr: &[u8] = rest;
                let args = SocializeLossV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let riskCouncil = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let obligationFarmUserState = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SocializeLossV2Accounts {
                    riskCouncil,
                    obligation,
                    lendingMarket,
                    reserve,
                    instructionSysvarAccount,
                    obligationFarmUserState,
                    reserveFarmState,
                    lendingMarketAuthority,
                    farmsProgram,
                    remaining,
                };
                return Ok(Instruction::SocializeLossV2 { accounts, args });
            }
            [164u8, 35u8, 182u8, 19u8, 0u8, 116u8, 243u8, 127u8] => {
                let mut rdr: &[u8] = rest;
                let args = MarkObligationForDeleveragingArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let riskCouncil = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MarkObligationForDeleveragingAccounts {
                    riskCouncil,
                    obligation,
                    lendingMarket,
                    remaining,
                };
                return Ok(Instruction::MarkObligationForDeleveraging { accounts, args });
            }
            [2u8, 218u8, 138u8, 235u8, 79u8, 201u8, 25u8, 102u8] => {
                let mut rdr: &[u8] = rest;
                let args = RefreshReserveArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let reserve = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let pythOracle = keys.next().unwrap().clone();
                let switchboardPriceOracle = keys.next().unwrap().clone();
                let switchboardTwapOracle = keys.next().unwrap().clone();
                let scopePrices = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RefreshReserveAccounts {
                    reserve,
                    lendingMarket,
                    pythOracle,
                    switchboardPriceOracle,
                    switchboardTwapOracle,
                    scopePrices,
                    remaining,
                };
                return Ok(Instruction::RefreshReserve { accounts, args });
            }
            [144u8, 110u8, 26u8, 103u8, 162u8, 204u8, 252u8, 147u8] => {
                let mut rdr: &[u8] = rest;
                let args = RefreshReservesBatchArguments::deserialize(&mut rdr)?;
                let accounts = RefreshReservesBatchAccounts {
                    remaining: account_keys.iter().cloned().collect(),
                };
                return Ok(Instruction::RefreshReservesBatch { accounts, args });
            }
            [169u8, 201u8, 30u8, 126u8, 6u8, 205u8, 102u8, 68u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositReserveLiquidityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveLiquiditySupply = keys.next().unwrap().clone();
                let reserveCollateralMint = keys.next().unwrap().clone();
                let userSourceLiquidity = keys.next().unwrap().clone();
                let userDestinationCollateral = keys.next().unwrap().clone();
                let collateralTokenProgram = keys.next().unwrap().clone();
                let liquidityTokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositReserveLiquidityAccounts {
                    owner,
                    reserve,
                    lendingMarket,
                    lendingMarketAuthority,
                    reserveLiquidityMint,
                    reserveLiquiditySupply,
                    reserveCollateralMint,
                    userSourceLiquidity,
                    userDestinationCollateral,
                    collateralTokenProgram,
                    liquidityTokenProgram,
                    instructionSysvarAccount,
                    remaining,
                };
                return Ok(Instruction::DepositReserveLiquidity { accounts, args });
            }
            [234u8, 117u8, 181u8, 125u8, 185u8, 142u8, 220u8, 29u8] => {
                let mut rdr: &[u8] = rest;
                let args = RedeemReserveCollateralArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveCollateralMint = keys.next().unwrap().clone();
                let reserveLiquiditySupply = keys.next().unwrap().clone();
                let userSourceCollateral = keys.next().unwrap().clone();
                let userDestinationLiquidity = keys.next().unwrap().clone();
                let collateralTokenProgram = keys.next().unwrap().clone();
                let liquidityTokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RedeemReserveCollateralAccounts {
                    owner,
                    lendingMarket,
                    reserve,
                    lendingMarketAuthority,
                    reserveLiquidityMint,
                    reserveCollateralMint,
                    reserveLiquiditySupply,
                    userSourceCollateral,
                    userDestinationLiquidity,
                    collateralTokenProgram,
                    liquidityTokenProgram,
                    instructionSysvarAccount,
                    remaining,
                };
                return Ok(Instruction::RedeemReserveCollateral { accounts, args });
            }
            [251u8, 10u8, 231u8, 76u8, 27u8, 11u8, 159u8, 96u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitObligationArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let obligationOwner = keys.next().unwrap().clone();
                let feePayer = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let seed1Account = keys.next().unwrap().clone();
                let seed2Account = keys.next().unwrap().clone();
                let ownerUserMetadata = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitObligationAccounts {
                    obligationOwner,
                    feePayer,
                    obligation,
                    lendingMarket,
                    seed1Account,
                    seed2Account,
                    ownerUserMetadata,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitObligation { accounts, args });
            }
            [136u8, 63u8, 15u8, 186u8, 211u8, 152u8, 168u8, 164u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitObligationFarmsForReserveArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let obligationFarm = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitObligationFarmsForReserveAccounts {
                    payer,
                    owner,
                    obligation,
                    lendingMarketAuthority,
                    reserve,
                    reserveFarmState,
                    obligationFarm,
                    lendingMarket,
                    farmsProgram,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitObligationFarmsForReserve { accounts, args });
            }
            [140u8, 144u8, 253u8, 21u8, 10u8, 74u8, 248u8, 3u8] => {
                let mut rdr: &[u8] = rest;
                let args = RefreshObligationFarmsForReserveArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let crank = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let obligationFarmUserState = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RefreshObligationFarmsForReserveAccounts {
                    crank,
                    obligation,
                    lendingMarketAuthority,
                    reserve,
                    reserveFarmState,
                    obligationFarmUserState,
                    lendingMarket,
                    farmsProgram,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::RefreshObligationFarmsForReserve { accounts, args });
            }
            [33u8, 132u8, 147u8, 228u8, 151u8, 192u8, 72u8, 89u8] => {
                let mut rdr: &[u8] = rest;
                let args = RefreshObligationArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let lendingMarket = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RefreshObligationAccounts {
                    lendingMarket,
                    obligation,
                    remaining,
                };
                return Ok(Instruction::RefreshObligation { accounts, args });
            }
            [108u8, 209u8, 4u8, 72u8, 21u8, 22u8, 118u8, 133u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositObligationCollateralArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let depositReserve = keys.next().unwrap().clone();
                let reserveDestinationCollateral = keys.next().unwrap().clone();
                let userSourceCollateral = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositObligationCollateralAccounts {
                    owner,
                    obligation,
                    lendingMarket,
                    depositReserve,
                    reserveDestinationCollateral,
                    userSourceCollateral,
                    tokenProgram,
                    instructionSysvarAccount,
                    remaining,
                };
                return Ok(Instruction::DepositObligationCollateral { accounts, args });
            }
            [137u8, 145u8, 151u8, 94u8, 167u8, 113u8, 4u8, 145u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositObligationCollateralV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let depositReserve = keys.next().unwrap().clone();
                let reserveDestinationCollateral = keys.next().unwrap().clone();
                let userSourceCollateral = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let obligationFarmUserState = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositObligationCollateralV2Accounts {
                    owner,
                    obligation,
                    lendingMarket,
                    depositReserve,
                    reserveDestinationCollateral,
                    userSourceCollateral,
                    tokenProgram,
                    instructionSysvarAccount,
                    lendingMarketAuthority,
                    obligationFarmUserState,
                    reserveFarmState,
                    farmsProgram,
                    remaining,
                };
                return Ok(Instruction::DepositObligationCollateralV2 { accounts, args });
            }
            [37u8, 116u8, 205u8, 103u8, 243u8, 192u8, 92u8, 198u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawObligationCollateralArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let withdrawReserve = keys.next().unwrap().clone();
                let reserveSourceCollateral = keys.next().unwrap().clone();
                let userDestinationCollateral = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawObligationCollateralAccounts {
                    owner,
                    obligation,
                    lendingMarket,
                    lendingMarketAuthority,
                    withdrawReserve,
                    reserveSourceCollateral,
                    userDestinationCollateral,
                    tokenProgram,
                    instructionSysvarAccount,
                    remaining,
                };
                return Ok(Instruction::WithdrawObligationCollateral { accounts, args });
            }
            [202u8, 249u8, 117u8, 114u8, 231u8, 192u8, 47u8, 138u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawObligationCollateralV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let withdrawReserve = keys.next().unwrap().clone();
                let reserveSourceCollateral = keys.next().unwrap().clone();
                let userDestinationCollateral = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let obligationFarmUserState = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawObligationCollateralV2Accounts {
                    owner,
                    obligation,
                    lendingMarket,
                    lendingMarketAuthority,
                    withdrawReserve,
                    reserveSourceCollateral,
                    userDestinationCollateral,
                    tokenProgram,
                    instructionSysvarAccount,
                    obligationFarmUserState,
                    reserveFarmState,
                    farmsProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawObligationCollateralV2 { accounts, args });
            }
            [121u8, 127u8, 18u8, 204u8, 73u8, 245u8, 225u8, 65u8] => {
                let mut rdr: &[u8] = rest;
                let args = BorrowObligationLiquidityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let borrowReserve = keys.next().unwrap().clone();
                let borrowReserveLiquidityMint = keys.next().unwrap().clone();
                let reserveSourceLiquidity = keys.next().unwrap().clone();
                let borrowReserveLiquidityFeeReceiver = keys.next().unwrap().clone();
                let userDestinationLiquidity = keys.next().unwrap().clone();
                let referrerTokenState = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BorrowObligationLiquidityAccounts {
                    owner,
                    obligation,
                    lendingMarket,
                    lendingMarketAuthority,
                    borrowReserve,
                    borrowReserveLiquidityMint,
                    reserveSourceLiquidity,
                    borrowReserveLiquidityFeeReceiver,
                    userDestinationLiquidity,
                    referrerTokenState,
                    tokenProgram,
                    instructionSysvarAccount,
                    remaining,
                };
                return Ok(Instruction::BorrowObligationLiquidity { accounts, args });
            }
            [161u8, 128u8, 143u8, 245u8, 171u8, 199u8, 194u8, 6u8] => {
                let mut rdr: &[u8] = rest;
                let args = BorrowObligationLiquidityV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let borrowReserve = keys.next().unwrap().clone();
                let borrowReserveLiquidityMint = keys.next().unwrap().clone();
                let reserveSourceLiquidity = keys.next().unwrap().clone();
                let borrowReserveLiquidityFeeReceiver = keys.next().unwrap().clone();
                let userDestinationLiquidity = keys.next().unwrap().clone();
                let referrerTokenState = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let obligationFarmUserState = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BorrowObligationLiquidityV2Accounts {
                    owner,
                    obligation,
                    lendingMarket,
                    lendingMarketAuthority,
                    borrowReserve,
                    borrowReserveLiquidityMint,
                    reserveSourceLiquidity,
                    borrowReserveLiquidityFeeReceiver,
                    userDestinationLiquidity,
                    referrerTokenState,
                    tokenProgram,
                    instructionSysvarAccount,
                    obligationFarmUserState,
                    reserveFarmState,
                    farmsProgram,
                    remaining,
                };
                return Ok(Instruction::BorrowObligationLiquidityV2 { accounts, args });
            }
            [145u8, 178u8, 13u8, 225u8, 76u8, 240u8, 147u8, 72u8] => {
                let mut rdr: &[u8] = rest;
                let args = RepayObligationLiquidityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let repayReserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveDestinationLiquidity = keys.next().unwrap().clone();
                let userSourceLiquidity = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RepayObligationLiquidityAccounts {
                    owner,
                    obligation,
                    lendingMarket,
                    repayReserve,
                    reserveLiquidityMint,
                    reserveDestinationLiquidity,
                    userSourceLiquidity,
                    tokenProgram,
                    instructionSysvarAccount,
                    remaining,
                };
                return Ok(Instruction::RepayObligationLiquidity { accounts, args });
            }
            [116u8, 174u8, 213u8, 76u8, 180u8, 53u8, 210u8, 144u8] => {
                let mut rdr: &[u8] = rest;
                let args = RepayObligationLiquidityV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let repayReserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveDestinationLiquidity = keys.next().unwrap().clone();
                let userSourceLiquidity = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let obligationFarmUserState = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RepayObligationLiquidityV2Accounts {
                    owner,
                    obligation,
                    lendingMarket,
                    repayReserve,
                    reserveLiquidityMint,
                    reserveDestinationLiquidity,
                    userSourceLiquidity,
                    tokenProgram,
                    instructionSysvarAccount,
                    obligationFarmUserState,
                    reserveFarmState,
                    lendingMarketAuthority,
                    farmsProgram,
                    remaining,
                };
                return Ok(Instruction::RepayObligationLiquidityV2 { accounts, args });
            }
            [2u8, 54u8, 152u8, 3u8, 148u8, 96u8, 109u8, 218u8] => {
                let mut rdr: &[u8] = rest;
                let args = RepayAndWithdrawAndRedeemArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let repayReserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveDestinationLiquidity = keys.next().unwrap().clone();
                let userSourceLiquidity = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let withdrawReserve = keys.next().unwrap().clone();
                let reserveSourceCollateral = keys.next().unwrap().clone();
                let reserveCollateralMint = keys.next().unwrap().clone();
                let reserveLiquiditySupply = keys.next().unwrap().clone();
                let userDestinationLiquidity = keys.next().unwrap().clone();
                let placeholderUserDestinationCollateral = keys.next().unwrap().clone();
                let collateralTokenProgram = keys.next().unwrap().clone();
                let liquidityTokenProgram = keys.next().unwrap().clone();
                let obligationFarmUserState = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RepayAndWithdrawAndRedeemAccounts {
                    owner,
                    obligation,
                    lendingMarket,
                    repayReserve,
                    reserveLiquidityMint,
                    reserveDestinationLiquidity,
                    userSourceLiquidity,
                    tokenProgram,
                    instructionSysvarAccount,
                    lendingMarketAuthority,
                    withdrawReserve,
                    reserveSourceCollateral,
                    reserveCollateralMint,
                    reserveLiquiditySupply,
                    userDestinationLiquidity,
                    placeholderUserDestinationCollateral,
                    collateralTokenProgram,
                    liquidityTokenProgram,
                    obligationFarmUserState,
                    reserveFarmState,
                    farmsProgram,
                    remaining,
                };
                return Ok(Instruction::RepayAndWithdrawAndRedeem { accounts, args });
            }
            [141u8, 153u8, 39u8, 15u8, 64u8, 61u8, 88u8, 84u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositAndWithdrawArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveLiquiditySupply = keys.next().unwrap().clone();
                let reserveCollateralMint = keys.next().unwrap().clone();
                let reserveDestinationDepositCollateral = keys.next().unwrap().clone();
                let userSourceLiquidity = keys.next().unwrap().clone();
                let placeholderUserDestinationCollateral = keys.next().unwrap().clone();
                let collateralTokenProgram = keys.next().unwrap().clone();
                let liquidityTokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let withdrawReserve = keys.next().unwrap().clone();
                let reserveSourceCollateral = keys.next().unwrap().clone();
                let userDestinationLiquidity = keys.next().unwrap().clone();
                let obligationFarmUserState = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositAndWithdrawAccounts {
                    owner,
                    obligation,
                    lendingMarket,
                    lendingMarketAuthority,
                    reserve,
                    reserveLiquidityMint,
                    reserveLiquiditySupply,
                    reserveCollateralMint,
                    reserveDestinationDepositCollateral,
                    userSourceLiquidity,
                    placeholderUserDestinationCollateral,
                    collateralTokenProgram,
                    liquidityTokenProgram,
                    instructionSysvarAccount,
                    withdrawReserve,
                    reserveSourceCollateral,
                    userDestinationLiquidity,
                    obligationFarmUserState,
                    reserveFarmState,
                    farmsProgram,
                    remaining,
                };
                return Ok(Instruction::DepositAndWithdraw { accounts, args });
            }
            [129u8, 199u8, 4u8, 2u8, 222u8, 39u8, 26u8, 46u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    DepositReserveLiquidityAndObligationCollateralArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveLiquiditySupply = keys.next().unwrap().clone();
                let reserveCollateralMint = keys.next().unwrap().clone();
                let reserveDestinationDepositCollateral = keys.next().unwrap().clone();
                let userSourceLiquidity = keys.next().unwrap().clone();
                let placeholderUserDestinationCollateral = keys.next().unwrap().clone();
                let collateralTokenProgram = keys.next().unwrap().clone();
                let liquidityTokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositReserveLiquidityAndObligationCollateralAccounts {
                    owner,
                    obligation,
                    lendingMarket,
                    lendingMarketAuthority,
                    reserve,
                    reserveLiquidityMint,
                    reserveLiquiditySupply,
                    reserveCollateralMint,
                    reserveDestinationDepositCollateral,
                    userSourceLiquidity,
                    placeholderUserDestinationCollateral,
                    collateralTokenProgram,
                    liquidityTokenProgram,
                    instructionSysvarAccount,
                    remaining,
                };
                return Ok(
                    Instruction::DepositReserveLiquidityAndObligationCollateral { accounts, args },
                );
            }
            [216u8, 224u8, 191u8, 27u8, 204u8, 151u8, 102u8, 175u8] => {
                let mut rdr: &[u8] = rest;
                let args = DepositReserveLiquidityAndObligationCollateralV2Arguments::deserialize(
                    &mut rdr,
                )?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveLiquiditySupply = keys.next().unwrap().clone();
                let reserveCollateralMint = keys.next().unwrap().clone();
                let reserveDestinationDepositCollateral = keys.next().unwrap().clone();
                let userSourceLiquidity = keys.next().unwrap().clone();
                let placeholderUserDestinationCollateral = keys.next().unwrap().clone();
                let collateralTokenProgram = keys.next().unwrap().clone();
                let liquidityTokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let obligationFarmUserState = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DepositReserveLiquidityAndObligationCollateralV2Accounts {
                    owner,
                    obligation,
                    lendingMarket,
                    lendingMarketAuthority,
                    reserve,
                    reserveLiquidityMint,
                    reserveLiquiditySupply,
                    reserveCollateralMint,
                    reserveDestinationDepositCollateral,
                    userSourceLiquidity,
                    placeholderUserDestinationCollateral,
                    collateralTokenProgram,
                    liquidityTokenProgram,
                    instructionSysvarAccount,
                    obligationFarmUserState,
                    reserveFarmState,
                    farmsProgram,
                    remaining,
                };
                return Ok(
                    Instruction::DepositReserveLiquidityAndObligationCollateralV2 {
                        accounts,
                        args,
                    },
                );
            }
            [75u8, 93u8, 93u8, 220u8, 34u8, 150u8, 218u8, 196u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    WithdrawObligationCollateralAndRedeemReserveCollateralArguments::deserialize(
                        &mut rdr,
                    )?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let withdrawReserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveSourceCollateral = keys.next().unwrap().clone();
                let reserveCollateralMint = keys.next().unwrap().clone();
                let reserveLiquiditySupply = keys.next().unwrap().clone();
                let userDestinationLiquidity = keys.next().unwrap().clone();
                let placeholderUserDestinationCollateral = keys.next().unwrap().clone();
                let collateralTokenProgram = keys.next().unwrap().clone();
                let liquidityTokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawObligationCollateralAndRedeemReserveCollateralAccounts {
                    owner,
                    obligation,
                    lendingMarket,
                    lendingMarketAuthority,
                    withdrawReserve,
                    reserveLiquidityMint,
                    reserveSourceCollateral,
                    reserveCollateralMint,
                    reserveLiquiditySupply,
                    userDestinationLiquidity,
                    placeholderUserDestinationCollateral,
                    collateralTokenProgram,
                    liquidityTokenProgram,
                    instructionSysvarAccount,
                    remaining,
                };
                return Ok(
                    Instruction::WithdrawObligationCollateralAndRedeemReserveCollateral {
                        accounts,
                        args,
                    },
                );
            }
            [235u8, 52u8, 119u8, 152u8, 149u8, 197u8, 20u8, 7u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    WithdrawObligationCollateralAndRedeemReserveCollateralV2Arguments::deserialize(
                        &mut rdr,
                    )?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let withdrawReserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveSourceCollateral = keys.next().unwrap().clone();
                let reserveCollateralMint = keys.next().unwrap().clone();
                let reserveLiquiditySupply = keys.next().unwrap().clone();
                let userDestinationLiquidity = keys.next().unwrap().clone();
                let placeholderUserDestinationCollateral = keys.next().unwrap().clone();
                let collateralTokenProgram = keys.next().unwrap().clone();
                let liquidityTokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let obligationFarmUserState = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawObligationCollateralAndRedeemReserveCollateralV2Accounts {
                    owner,
                    obligation,
                    lendingMarket,
                    lendingMarketAuthority,
                    withdrawReserve,
                    reserveLiquidityMint,
                    reserveSourceCollateral,
                    reserveCollateralMint,
                    reserveLiquiditySupply,
                    userDestinationLiquidity,
                    placeholderUserDestinationCollateral,
                    collateralTokenProgram,
                    liquidityTokenProgram,
                    instructionSysvarAccount,
                    obligationFarmUserState,
                    reserveFarmState,
                    farmsProgram,
                    remaining,
                };
                return Ok(
                    Instruction::WithdrawObligationCollateralAndRedeemReserveCollateralV2 {
                        accounts,
                        args,
                    },
                );
            }
            [177u8, 71u8, 154u8, 188u8, 226u8, 133u8, 74u8, 55u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    LiquidateObligationAndRedeemReserveCollateralArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let liquidator = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let repayReserve = keys.next().unwrap().clone();
                let repayReserveLiquidityMint = keys.next().unwrap().clone();
                let repayReserveLiquiditySupply = keys.next().unwrap().clone();
                let withdrawReserve = keys.next().unwrap().clone();
                let withdrawReserveLiquidityMint = keys.next().unwrap().clone();
                let withdrawReserveCollateralMint = keys.next().unwrap().clone();
                let withdrawReserveCollateralSupply = keys.next().unwrap().clone();
                let withdrawReserveLiquiditySupply = keys.next().unwrap().clone();
                let withdrawReserveLiquidityFeeReceiver = keys.next().unwrap().clone();
                let userSourceLiquidity = keys.next().unwrap().clone();
                let userDestinationCollateral = keys.next().unwrap().clone();
                let userDestinationLiquidity = keys.next().unwrap().clone();
                let collateralTokenProgram = keys.next().unwrap().clone();
                let repayLiquidityTokenProgram = keys.next().unwrap().clone();
                let withdrawLiquidityTokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LiquidateObligationAndRedeemReserveCollateralAccounts {
                    liquidator,
                    obligation,
                    lendingMarket,
                    lendingMarketAuthority,
                    repayReserve,
                    repayReserveLiquidityMint,
                    repayReserveLiquiditySupply,
                    withdrawReserve,
                    withdrawReserveLiquidityMint,
                    withdrawReserveCollateralMint,
                    withdrawReserveCollateralSupply,
                    withdrawReserveLiquiditySupply,
                    withdrawReserveLiquidityFeeReceiver,
                    userSourceLiquidity,
                    userDestinationCollateral,
                    userDestinationLiquidity,
                    collateralTokenProgram,
                    repayLiquidityTokenProgram,
                    withdrawLiquidityTokenProgram,
                    instructionSysvarAccount,
                    remaining,
                };
                return Ok(Instruction::LiquidateObligationAndRedeemReserveCollateral {
                    accounts,
                    args,
                });
            }
            [162u8, 161u8, 35u8, 143u8, 30u8, 187u8, 185u8, 103u8] => {
                let mut rdr: &[u8] = rest;
                let args = LiquidateObligationAndRedeemReserveCollateralV2Arguments::deserialize(
                    &mut rdr,
                )?;
                let mut keys = account_keys.iter();
                let liquidator = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let repayReserve = keys.next().unwrap().clone();
                let repayReserveLiquidityMint = keys.next().unwrap().clone();
                let repayReserveLiquiditySupply = keys.next().unwrap().clone();
                let withdrawReserve = keys.next().unwrap().clone();
                let withdrawReserveLiquidityMint = keys.next().unwrap().clone();
                let withdrawReserveCollateralMint = keys.next().unwrap().clone();
                let withdrawReserveCollateralSupply = keys.next().unwrap().clone();
                let withdrawReserveLiquiditySupply = keys.next().unwrap().clone();
                let withdrawReserveLiquidityFeeReceiver = keys.next().unwrap().clone();
                let userSourceLiquidity = keys.next().unwrap().clone();
                let userDestinationCollateral = keys.next().unwrap().clone();
                let userDestinationLiquidity = keys.next().unwrap().clone();
                let collateralTokenProgram = keys.next().unwrap().clone();
                let repayLiquidityTokenProgram = keys.next().unwrap().clone();
                let withdrawLiquidityTokenProgram = keys.next().unwrap().clone();
                let instructionSysvarAccount = keys.next().unwrap().clone();
                let obligationFarmUserState = keys.next().unwrap().clone();
                let reserveFarmState = keys.next().unwrap().clone();
                let farmsProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LiquidateObligationAndRedeemReserveCollateralV2Accounts {
                    liquidator,
                    obligation,
                    lendingMarket,
                    lendingMarketAuthority,
                    repayReserve,
                    repayReserveLiquidityMint,
                    repayReserveLiquiditySupply,
                    withdrawReserve,
                    withdrawReserveLiquidityMint,
                    withdrawReserveCollateralMint,
                    withdrawReserveCollateralSupply,
                    withdrawReserveLiquiditySupply,
                    withdrawReserveLiquidityFeeReceiver,
                    userSourceLiquidity,
                    userDestinationCollateral,
                    userDestinationLiquidity,
                    collateralTokenProgram,
                    repayLiquidityTokenProgram,
                    withdrawLiquidityTokenProgram,
                    instructionSysvarAccount,
                    obligationFarmUserState,
                    reserveFarmState,
                    farmsProgram,
                    remaining,
                };
                return Ok(
                    Instruction::LiquidateObligationAndRedeemReserveCollateralV2 { accounts, args },
                );
            }
            [185u8, 117u8, 0u8, 203u8, 96u8, 245u8, 180u8, 186u8] => {
                let mut rdr: &[u8] = rest;
                let args = FlashRepayReserveLiquidityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let userTransferAuthority = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveDestinationLiquidity = keys.next().unwrap().clone();
                let userSourceLiquidity = keys.next().unwrap().clone();
                let reserveLiquidityFeeReceiver = keys.next().unwrap().clone();
                let referrerTokenState = keys.next().unwrap().clone();
                let referrerAccount = keys.next().unwrap().clone();
                let sysvarInfo = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FlashRepayReserveLiquidityAccounts {
                    userTransferAuthority,
                    lendingMarketAuthority,
                    lendingMarket,
                    reserve,
                    reserveLiquidityMint,
                    reserveDestinationLiquidity,
                    userSourceLiquidity,
                    reserveLiquidityFeeReceiver,
                    referrerTokenState,
                    referrerAccount,
                    sysvarInfo,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::FlashRepayReserveLiquidity { accounts, args });
            }
            [135u8, 231u8, 52u8, 167u8, 7u8, 52u8, 212u8, 193u8] => {
                let mut rdr: &[u8] = rest;
                let args = FlashBorrowReserveLiquidityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let userTransferAuthority = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveSourceLiquidity = keys.next().unwrap().clone();
                let userDestinationLiquidity = keys.next().unwrap().clone();
                let reserveLiquidityFeeReceiver = keys.next().unwrap().clone();
                let referrerTokenState = keys.next().unwrap().clone();
                let referrerAccount = keys.next().unwrap().clone();
                let sysvarInfo = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FlashBorrowReserveLiquidityAccounts {
                    userTransferAuthority,
                    lendingMarketAuthority,
                    lendingMarket,
                    reserve,
                    reserveLiquidityMint,
                    reserveSourceLiquidity,
                    userDestinationLiquidity,
                    reserveLiquidityFeeReceiver,
                    referrerTokenState,
                    referrerAccount,
                    sysvarInfo,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::FlashBorrowReserveLiquidity { accounts, args });
            }
            [36u8, 119u8, 251u8, 129u8, 34u8, 240u8, 7u8, 147u8] => {
                let mut rdr: &[u8] = rest;
                let args = RequestElevationGroupArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RequestElevationGroupAccounts {
                    owner,
                    obligation,
                    lendingMarket,
                    remaining,
                };
                return Ok(Instruction::RequestElevationGroup { accounts, args });
            }
            [116u8, 45u8, 66u8, 148u8, 58u8, 13u8, 218u8, 115u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitReferrerTokenStateArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let referrer = keys.next().unwrap().clone();
                let referrerTokenState = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitReferrerTokenStateAccounts {
                    payer,
                    lendingMarket,
                    reserve,
                    referrer,
                    referrerTokenState,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitReferrerTokenState { accounts, args });
            }
            [117u8, 169u8, 176u8, 69u8, 197u8, 23u8, 15u8, 162u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitUserMetadataArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let feePayer = keys.next().unwrap().clone();
                let userMetadata = keys.next().unwrap().clone();
                let referrerUserMetadata = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitUserMetadataAccounts {
                    owner,
                    feePayer,
                    userMetadata,
                    referrerUserMetadata,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitUserMetadata { accounts, args });
            }
            [171u8, 118u8, 121u8, 201u8, 233u8, 140u8, 23u8, 228u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawReferrerFeesArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let referrer = keys.next().unwrap().clone();
                let referrerTokenState = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let reserveLiquidityMint = keys.next().unwrap().clone();
                let reserveSupplyLiquidity = keys.next().unwrap().clone();
                let referrerTokenAccount = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let lendingMarketAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawReferrerFeesAccounts {
                    referrer,
                    referrerTokenState,
                    reserve,
                    reserveLiquidityMint,
                    reserveSupplyLiquidity,
                    referrerTokenAccount,
                    lendingMarket,
                    lendingMarketAuthority,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawReferrerFees { accounts, args });
            }
            [165u8, 19u8, 25u8, 127u8, 100u8, 55u8, 31u8, 90u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitReferrerStateAndShortUrlArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let referrer = keys.next().unwrap().clone();
                let referrerState = keys.next().unwrap().clone();
                let referrerShortUrl = keys.next().unwrap().clone();
                let referrerUserMetadata = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitReferrerStateAndShortUrlAccounts {
                    referrer,
                    referrerState,
                    referrerShortUrl,
                    referrerUserMetadata,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitReferrerStateAndShortUrl { accounts, args });
            }
            [153u8, 185u8, 99u8, 28u8, 228u8, 179u8, 187u8, 150u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeleteReferrerStateAndShortUrlArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let referrer = keys.next().unwrap().clone();
                let referrerState = keys.next().unwrap().clone();
                let shortUrl = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeleteReferrerStateAndShortUrlAccounts {
                    referrer,
                    referrerState,
                    shortUrl,
                    rent,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::DeleteReferrerStateAndShortUrl { accounts, args });
            }
            [81u8, 1u8, 99u8, 156u8, 211u8, 83u8, 78u8, 46u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetObligationOrderArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let obligation = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetObligationOrderAccounts {
                    owner,
                    obligation,
                    lendingMarket,
                    remaining,
                };
                return Ok(Instruction::SetObligationOrder { accounts, args });
            }
            [140u8, 136u8, 214u8, 48u8, 87u8, 0u8, 120u8, 255u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitGlobalConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let programData = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitGlobalConfigAccounts {
                    payer,
                    globalConfig,
                    programData,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::InitGlobalConfig { accounts, args });
            }
            [164u8, 84u8, 130u8, 189u8, 111u8, 58u8, 250u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateGlobalConfigArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let globalAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGlobalConfigAccounts {
                    globalAdmin,
                    globalConfig,
                    remaining,
                };
                return Ok(Instruction::UpdateGlobalConfig { accounts, args });
            }
            [184u8, 87u8, 23u8, 193u8, 156u8, 238u8, 175u8, 119u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateGlobalConfigAdminArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pendingAdmin = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGlobalConfigAdminAccounts {
                    pendingAdmin,
                    globalConfig,
                    remaining,
                };
                return Ok(Instruction::UpdateGlobalConfigAdmin { accounts, args });
            }
            [130u8, 80u8, 38u8, 153u8, 80u8, 212u8, 182u8, 253u8] => {
                let mut rdr: &[u8] = rest;
                let args = IdlMissingTypesArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let signer = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let lendingMarket = keys.next().unwrap().clone();
                let reserve = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = IdlMissingTypesAccounts {
                    signer,
                    globalConfig,
                    lendingMarket,
                    reserve,
                    remaining,
                };
                return Ok(Instruction::IdlMissingTypes { accounts, args });
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
}
