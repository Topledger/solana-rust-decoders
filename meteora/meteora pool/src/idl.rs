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
    pub struct TokenMultiplier {
        pub token_a_multiplier: u64,
        pub token_b_multiplier: u64,
        pub precision_factor: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PoolFees {
        pub trade_fee_numerator: u64,
        pub trade_fee_denominator: u64,
        pub protocol_trade_fee_numerator: u64,
        pub protocol_trade_fee_denominator: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Depeg {
        pub base_virtual_price: u64,
        pub base_cache_updated: u64,
        pub depeg_type: DepegType,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ConfigParameters {
        pub trade_fee_numerator: u64,
        pub protocol_trade_fee_numerator: u64,
        pub activation_duration: u64,
        pub vault_config_key: String,
        pub pool_creator_authority: String,
        pub activation_type: u8,
        pub index: u64,
        pub partner_fee_numerator: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CustomizableParams {
        pub trade_fee_numerator: u32,
        pub activation_point: Option<u64>,
        pub has_alpha_vault: bool,
        pub activation_type: u8,
        pub padding: Vec<u8>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Padding {
        pub padding0: Vec<u8>,
        pub padding1: Vec<u64>,
        pub padding2: Vec<u64>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PartnerInfo {
        pub fee_numerator: u64,
        pub partner_authority: String,
        pub pending_fee_a: u64,
        pub pending_fee_b: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Bootstrapping {
        pub activation_point: u64,
        pub whitelisted_vault: String,
        pub pool_creator: String,
        pub activation_type: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ActivationType {
        Slot,
        Timestamp,
    }
    impl Default for ActivationType {
        fn default() -> Self {
            Self::Slot
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RoundDirection {
        Floor,
        Ceiling,
    }
    impl Default for RoundDirection {
        fn default() -> Self {
            Self::Floor
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum TradeDirection {
        AtoB,
        BtoA,
    }
    impl Default for TradeDirection {
        fn default() -> Self {
            Self::AtoB
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum NewCurveType {
        ConstantProduct,
        Stable {
            amp: u64,
            token_multiplier: TokenMultiplier,
            depeg: Depeg,
            last_amp_updated_timestamp: u64,
        },
        NewCurve {
            field_one: u64,
            field_two: u64,
        },
    }
    impl Default for NewCurveType {
        fn default() -> Self {
            Self::ConstantProduct
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum CurveType {
        ConstantProduct,
        Stable {
            amp: u64,
            token_multiplier: TokenMultiplier,
            depeg: Depeg,
            last_amp_updated_timestamp: u64,
        },
    }
    impl Default for CurveType {
        fn default() -> Self {
            Self::ConstantProduct
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum DepegType {
        None,
        Marinade,
        Lido,
        SplStake,
    }
    impl Default for DepegType {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Rounding {
        Up,
        Down,
    }
    impl Default for Rounding {
        fn default() -> Self {
            Self::Up
        }
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PoolType {
        Permissioned,
        Permissionless,
    }
    impl Default for PoolType {
        fn default() -> Self {
            Self::Permissioned
        }
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitializePermissionedPoolAccounts {
        pub pool: String,
        pub lpMint: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub aVault: String,
        pub bVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub adminTokenA: String,
        pub adminTokenB: String,
        pub adminPoolLp: String,
        pub protocolTokenAFee: String,
        pub protocolTokenBFee: String,
        pub admin: String,
        pub feeOwner: String,
        pub rent: String,
        pub mintMetadata: String,
        pub metadataProgram: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MigrateFeeAccountAccounts {
        pub pool: String,
        pub aVaultLp: String,
        pub adminTokenAFee: String,
        pub adminTokenBFee: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub newAdminTokenAFee: String,
        pub newAdminTokenBFee: String,
        pub admin: String,
        pub treasuryTokenAFee: String,
        pub treasuryTokenBFee: String,
        pub Treasury: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePermissionlessPoolAccounts {
        pub pool: String,
        pub lpMint: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub aVault: String,
        pub bVault: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub payerTokenA: String,
        pub payerTokenB: String,
        pub payerPoolLp: String,
        pub protocolTokenAFee: String,
        pub protocolTokenBFee: String,
        pub payer: String,
        pub feeOwner: String,
        pub rent: String,
        pub mintMetadata: String,
        pub metadataProgram: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePermissionlessPoolWithFeeTierAccounts {
        pub pool: String,
        pub lpMint: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub aVault: String,
        pub bVault: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub payerTokenA: String,
        pub payerTokenB: String,
        pub payerPoolLp: String,
        pub protocolTokenAFee: String,
        pub protocolTokenBFee: String,
        pub payer: String,
        pub feeOwner: String,
        pub rent: String,
        pub mintMetadata: String,
        pub metadataProgram: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct EnableOrDisablePoolAccounts {
        pub pool: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapAccounts {
        pub pool: String,
        pub userSourceToken: String,
        pub userDestinationToken: String,
        pub aVault: String,
        pub bVault: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub protocolTokenFee: String,
        pub user: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveLiquiditySingleSideAccounts {
        pub pool: String,
        pub lpMint: String,
        pub userPoolLp: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub aVault: String,
        pub bVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub userDestinationToken: String,
        pub user: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddImbalanceLiquidityAccounts {
        pub pool: String,
        pub lpMint: String,
        pub userPoolLp: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub aVault: String,
        pub bVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub userAToken: String,
        pub userBToken: String,
        pub user: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveBalanceLiquidityAccounts {
        pub pool: String,
        pub lpMint: String,
        pub userPoolLp: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub aVault: String,
        pub bVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub userAToken: String,
        pub userBToken: String,
        pub user: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddBalanceLiquidityAccounts {
        pub pool: String,
        pub lpMint: String,
        pub userPoolLp: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub aVault: String,
        pub bVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub userAToken: String,
        pub userBToken: String,
        pub user: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetPoolFeesAccounts {
        pub pool: String,
        pub feeOperator: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OverrideCurveParamAccounts {
        pub pool: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct GetPoolInfoAccounts {
        pub pool: String,
        pub lpMint: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub aVault: String,
        pub bVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BootstrapLiquidityAccounts {
        pub pool: String,
        pub lpMint: String,
        pub userPoolLp: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub aVault: String,
        pub bVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub userAToken: String,
        pub userBToken: String,
        pub user: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateActivationSlotAccounts {
        pub pool: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateMintMetadataAccounts {
        pub pool: String,
        pub lpMint: String,
        pub aVaultLp: String,
        pub mintMetadata: String,
        pub metadataProgram: String,
        pub systemProgram: String,
        pub payer: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateLockEscrowAccounts {
        pub pool: String,
        pub lockEscrow: String,
        pub owner: String,
        pub lpMint: String,
        pub payer: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LockAccounts {
        pub pool: String,
        pub lpMint: String,
        pub lockEscrow: String,
        pub owner: String,
        pub sourceTokens: String,
        pub escrowVault: String,
        pub tokenProgram: String,
        pub aVault: String,
        pub bVault: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimFeeAccounts {
        pub pool: String,
        pub lpMint: String,
        pub lockEscrow: String,
        pub owner: String,
        pub sourceTokens: String,
        pub escrowVault: String,
        pub tokenProgram: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub aVault: String,
        pub bVault: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub userAToken: String,
        pub userBToken: String,
        pub vaultProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateConfigAccounts {
        pub config: String,
        pub admin: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseConfigAccounts {
        pub config: String,
        pub admin: String,
        pub rentReceiver: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePermissionlessConstantProductPoolWithConfigAccounts {
        pub pool: String,
        pub config: String,
        pub lpMint: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub aVault: String,
        pub bVault: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub payerTokenA: String,
        pub payerTokenB: String,
        pub payerPoolLp: String,
        pub protocolTokenAFee: String,
        pub protocolTokenBFee: String,
        pub payer: String,
        pub rent: String,
        pub mintMetadata: String,
        pub metadataProgram: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializePermissionlessConstantProductPoolWithConfig2Accounts {
        pub pool: String,
        pub config: String,
        pub lpMint: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub aVault: String,
        pub bVault: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub payerTokenA: String,
        pub payerTokenB: String,
        pub payerPoolLp: String,
        pub protocolTokenAFee: String,
        pub protocolTokenBFee: String,
        pub payer: String,
        pub rent: String,
        pub mintMetadata: String,
        pub metadataProgram: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeCustomizablePermissionlessConstantProductPoolAccounts {
        pub pool: String,
        pub lpMint: String,
        pub tokenAMint: String,
        pub tokenBMint: String,
        pub aVault: String,
        pub bVault: String,
        pub aTokenVault: String,
        pub bTokenVault: String,
        pub aVaultLpMint: String,
        pub bVaultLpMint: String,
        pub aVaultLp: String,
        pub bVaultLp: String,
        pub payerTokenA: String,
        pub payerTokenB: String,
        pub payerPoolLp: String,
        pub protocolTokenAFee: String,
        pub protocolTokenBFee: String,
        pub payer: String,
        pub rent: String,
        pub mintMetadata: String,
        pub metadataProgram: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateActivationPointAccounts {
        pub pool: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawProtocolFeesAccounts {
        pub pool: String,
        pub aVaultLp: String,
        pub protocolTokenAFee: String,
        pub protocolTokenBFee: String,
        pub treasuryTokenA: String,
        pub treasuryTokenB: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetWhitelistedVaultAccounts {
        pub pool: String,
        pub admin: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PartnerClaimFeeAccounts {
        pub pool: String,
        pub aVaultLp: String,
        pub protocolTokenAFee: String,
        pub protocolTokenBFee: String,
        pub partnerTokenA: String,
        pub partnerTokenB: String,
        pub tokenProgram: String,
        pub partnerAuthority: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializePermissionedPoolArgs {
        pub curve_type: CurveType,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MigrateFeeAccountArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializePermissionlessPoolArgs {
        pub curve_type: CurveType,
        pub token_a_amount: u64,
        pub token_b_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializePermissionlessPoolWithFeeTierArgs {
        pub curve_type: CurveType,
        pub trade_fee_bps: u64,
        pub token_a_amount: u64,
        pub token_b_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct EnableOrDisablePoolArgs {
        pub enable: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapArgs {
        pub in_amount: u64,
        pub minimum_out_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveLiquiditySingleSideArgs {
        pub pool_token_amount: u64,
        pub minimum_out_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddImbalanceLiquidityArgs {
        pub minimum_pool_token_amount: u64,
        pub token_a_amount: u64,
        pub token_b_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveBalanceLiquidityArgs {
        pub pool_token_amount: u64,
        pub minimum_a_token_out: u64,
        pub minimum_b_token_out: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddBalanceLiquidityArgs {
        pub pool_token_amount: u64,
        pub maximum_token_a_amount: u64,
        pub maximum_token_b_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetPoolFeesArgs {
        pub fees: PoolFees,
        pub new_partner_fee_numerator: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OverrideCurveParamArgs {
        pub curve_type: CurveType,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct GetPoolInfoArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BootstrapLiquidityArgs {
        pub token_a_amount: u64,
        pub token_b_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateActivationSlotArgs {
        pub new_activation_slot: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateMintMetadataArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateLockEscrowArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LockArgs {
        pub max_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimFeeArgs {
        pub max_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateConfigArgs {
        pub config_parameters: ConfigParameters,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseConfigArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializePermissionlessConstantProductPoolWithConfigArgs {
        pub token_a_amount: u64,
        pub token_b_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializePermissionlessConstantProductPoolWithConfig2Args {
        pub token_a_amount: u64,
        pub token_b_amount: u64,
        pub activation_point: Option<u64>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeCustomizablePermissionlessConstantProductPoolArgs {
        pub token_a_amount: u64,
        pub token_b_amount: u64,
        pub params: CustomizableParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateActivationPointArgs {
        pub new_activation_point: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawProtocolFeesArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetWhitelistedVaultArgs {
        pub whitelisted_vault: String,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PartnerClaimFeeArgs {
        pub max_amount_a: u64,
        pub max_amount_b: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitializePermissionedPool {
        accounts: InitializePermissionedPoolAccounts,
        args: InitializePermissionedPoolArgs,
    },
    MigrateFeeAccount {
        accounts: MigrateFeeAccountAccounts,
        args: MigrateFeeAccountArgs,
    },
    InitializePermissionlessPool {
        accounts: InitializePermissionlessPoolAccounts,
        args: InitializePermissionlessPoolArgs,
    },
    InitializePermissionlessPoolWithFeeTier {
        accounts: InitializePermissionlessPoolWithFeeTierAccounts,
        args: InitializePermissionlessPoolWithFeeTierArgs,
    },
    EnableOrDisablePool {
        accounts: EnableOrDisablePoolAccounts,
        args: EnableOrDisablePoolArgs,
    },
    Swap {
        accounts: SwapAccounts,
        args: SwapArgs,
    },
    RemoveLiquiditySingleSide {
        accounts: RemoveLiquiditySingleSideAccounts,
        args: RemoveLiquiditySingleSideArgs,
    },
    AddImbalanceLiquidity {
        accounts: AddImbalanceLiquidityAccounts,
        args: AddImbalanceLiquidityArgs,
    },
    RemoveBalanceLiquidity {
        accounts: RemoveBalanceLiquidityAccounts,
        args: RemoveBalanceLiquidityArgs,
    },
    AddBalanceLiquidity {
        accounts: AddBalanceLiquidityAccounts,
        args: AddBalanceLiquidityArgs,
    },
    SetPoolFees {
        accounts: SetPoolFeesAccounts,
        args: SetPoolFeesArgs,
    },
    OverrideCurveParam {
        accounts: OverrideCurveParamAccounts,
        args: OverrideCurveParamArgs,
    },
    GetPoolInfo {
        accounts: GetPoolInfoAccounts,
        args: GetPoolInfoArgs,
    },
    BootstrapLiquidity {
        accounts: BootstrapLiquidityAccounts,
        args: BootstrapLiquidityArgs,
    },
    UpdateActivationSlot {
        accounts: UpdateActivationSlotAccounts,
        args: UpdateActivationSlotArgs,
    },
    CreateMintMetadata {
        accounts: CreateMintMetadataAccounts,
        args: CreateMintMetadataArgs,
    },
    CreateLockEscrow {
        accounts: CreateLockEscrowAccounts,
        args: CreateLockEscrowArgs,
    },
    Lock {
        accounts: LockAccounts,
        args: LockArgs,
    },
    ClaimFee {
        accounts: ClaimFeeAccounts,
        args: ClaimFeeArgs,
    },
    CreateConfig {
        accounts: CreateConfigAccounts,
        args: CreateConfigArgs,
    },
    CloseConfig {
        accounts: CloseConfigAccounts,
        args: CloseConfigArgs,
    },
    InitializePermissionlessConstantProductPoolWithConfig {
        accounts: InitializePermissionlessConstantProductPoolWithConfigAccounts,
        args: InitializePermissionlessConstantProductPoolWithConfigArgs,
    },
    InitializePermissionlessConstantProductPoolWithConfig2 {
        accounts: InitializePermissionlessConstantProductPoolWithConfig2Accounts,
        args: InitializePermissionlessConstantProductPoolWithConfig2Args,
    },
    InitializeCustomizablePermissionlessConstantProductPool {
        accounts: InitializeCustomizablePermissionlessConstantProductPoolAccounts,
        args: InitializeCustomizablePermissionlessConstantProductPoolArgs,
    },
    UpdateActivationPoint {
        accounts: UpdateActivationPointAccounts,
        args: UpdateActivationPointArgs,
    },
    WithdrawProtocolFees {
        accounts: WithdrawProtocolFeesAccounts,
        args: WithdrawProtocolFeesArgs,
    },
    SetWhitelistedVault {
        accounts: SetWhitelistedVaultAccounts,
        args: SetWhitelistedVaultArgs,
    },
    PartnerClaimFee {
        accounts: PartnerClaimFeeAccounts,
        args: PartnerClaimFeeArgs,
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
            [77u8, 85u8, 178u8, 157u8, 50u8, 48u8, 212u8, 126u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePermissionedPoolArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let adminTokenA = keys.next().unwrap().clone();
                let adminTokenB = keys.next().unwrap().clone();
                let adminPoolLp = keys.next().unwrap().clone();
                let protocolTokenAFee = keys.next().unwrap().clone();
                let protocolTokenBFee = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let feeOwner = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let mintMetadata = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePermissionedPoolAccounts {
                    pool,
                    lpMint,
                    tokenAMint,
                    tokenBMint,
                    aVault,
                    bVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aVaultLp,
                    bVaultLp,
                    adminTokenA,
                    adminTokenB,
                    adminPoolLp,
                    protocolTokenAFee,
                    protocolTokenBFee,
                    admin,
                    feeOwner,
                    rent,
                    mintMetadata,
                    metadataProgram,
                    vaultProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializePermissionedPool { accounts, args });
            }
            [223u8, 60u8, 126u8, 177u8, 109u8, 146u8, 65u8, 81u8] => {
                let mut rdr: &[u8] = rest;
                let args = MigrateFeeAccountArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let adminTokenAFee = keys.next().unwrap().clone();
                let adminTokenBFee = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let newAdminTokenAFee = keys.next().unwrap().clone();
                let newAdminTokenBFee = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let treasuryTokenAFee = keys.next().unwrap().clone();
                let treasuryTokenBFee = keys.next().unwrap().clone();
                let Treasury = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MigrateFeeAccountAccounts {
                    pool,
                    aVaultLp,
                    adminTokenAFee,
                    adminTokenBFee,
                    tokenAMint,
                    tokenBMint,
                    newAdminTokenAFee,
                    newAdminTokenBFee,
                    admin,
                    treasuryTokenAFee,
                    treasuryTokenBFee,
                    Treasury,
                    tokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::MigrateFeeAccount { accounts, args });
            }
            [118u8, 173u8, 41u8, 157u8, 173u8, 72u8, 97u8, 103u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePermissionlessPoolArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let payerTokenA = keys.next().unwrap().clone();
                let payerTokenB = keys.next().unwrap().clone();
                let payerPoolLp = keys.next().unwrap().clone();
                let protocolTokenAFee = keys.next().unwrap().clone();
                let protocolTokenBFee = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let feeOwner = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let mintMetadata = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePermissionlessPoolAccounts {
                    pool,
                    lpMint,
                    tokenAMint,
                    tokenBMint,
                    aVault,
                    bVault,
                    aTokenVault,
                    bTokenVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aVaultLp,
                    bVaultLp,
                    payerTokenA,
                    payerTokenB,
                    payerPoolLp,
                    protocolTokenAFee,
                    protocolTokenBFee,
                    payer,
                    feeOwner,
                    rent,
                    mintMetadata,
                    metadataProgram,
                    vaultProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializePermissionlessPool { accounts, args });
            }
            [6u8, 135u8, 68u8, 147u8, 229u8, 82u8, 169u8, 113u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePermissionlessPoolWithFeeTierArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let payerTokenA = keys.next().unwrap().clone();
                let payerTokenB = keys.next().unwrap().clone();
                let payerPoolLp = keys.next().unwrap().clone();
                let protocolTokenAFee = keys.next().unwrap().clone();
                let protocolTokenBFee = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let feeOwner = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let mintMetadata = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePermissionlessPoolWithFeeTierAccounts {
                    pool,
                    lpMint,
                    tokenAMint,
                    tokenBMint,
                    aVault,
                    bVault,
                    aTokenVault,
                    bTokenVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aVaultLp,
                    bVaultLp,
                    payerTokenA,
                    payerTokenB,
                    payerPoolLp,
                    protocolTokenAFee,
                    protocolTokenBFee,
                    payer,
                    feeOwner,
                    rent,
                    mintMetadata,
                    metadataProgram,
                    vaultProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::InitializePermissionlessPoolWithFeeTier { accounts, args });
            }
            [128u8, 6u8, 228u8, 131u8, 55u8, 161u8, 52u8, 169u8] => {
                let mut rdr: &[u8] = rest;
                let args = EnableOrDisablePoolArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EnableOrDisablePoolAccounts {
                    pool,
                    admin,
                    remaining,
                };
                return Ok(Instruction::EnableOrDisablePool { accounts, args });
            }
            [248u8, 198u8, 158u8, 145u8, 225u8, 117u8, 135u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let userSourceToken = keys.next().unwrap().clone();
                let userDestinationToken = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let protocolTokenFee = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapAccounts {
                    pool,
                    userSourceToken,
                    userDestinationToken,
                    aVault,
                    bVault,
                    aTokenVault,
                    bTokenVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aVaultLp,
                    bVaultLp,
                    protocolTokenFee,
                    user,
                    vaultProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Swap { accounts, args });
            }
            [84u8, 84u8, 177u8, 66u8, 254u8, 185u8, 10u8, 251u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveLiquiditySingleSideArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let userPoolLp = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let userDestinationToken = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveLiquiditySingleSideAccounts {
                    pool,
                    lpMint,
                    userPoolLp,
                    aVaultLp,
                    bVaultLp,
                    aVault,
                    bVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aTokenVault,
                    bTokenVault,
                    userDestinationToken,
                    user,
                    vaultProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::RemoveLiquiditySingleSide { accounts, args });
            }
            [79u8, 35u8, 122u8, 84u8, 173u8, 15u8, 93u8, 191u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddImbalanceLiquidityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let userPoolLp = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let userAToken = keys.next().unwrap().clone();
                let userBToken = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddImbalanceLiquidityAccounts {
                    pool,
                    lpMint,
                    userPoolLp,
                    aVaultLp,
                    bVaultLp,
                    aVault,
                    bVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aTokenVault,
                    bTokenVault,
                    userAToken,
                    userBToken,
                    user,
                    vaultProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::AddImbalanceLiquidity { accounts, args });
            }
            [133u8, 109u8, 44u8, 179u8, 56u8, 238u8, 114u8, 33u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveBalanceLiquidityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let userPoolLp = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let userAToken = keys.next().unwrap().clone();
                let userBToken = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveBalanceLiquidityAccounts {
                    pool,
                    lpMint,
                    userPoolLp,
                    aVaultLp,
                    bVaultLp,
                    aVault,
                    bVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aTokenVault,
                    bTokenVault,
                    userAToken,
                    userBToken,
                    user,
                    vaultProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::RemoveBalanceLiquidity { accounts, args });
            }
            [168u8, 227u8, 50u8, 62u8, 189u8, 171u8, 84u8, 176u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddBalanceLiquidityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let userPoolLp = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let userAToken = keys.next().unwrap().clone();
                let userBToken = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddBalanceLiquidityAccounts {
                    pool,
                    lpMint,
                    userPoolLp,
                    aVaultLp,
                    bVaultLp,
                    aVault,
                    bVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aTokenVault,
                    bTokenVault,
                    userAToken,
                    userBToken,
                    user,
                    vaultProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::AddBalanceLiquidity { accounts, args });
            }
            [102u8, 44u8, 158u8, 54u8, 205u8, 37u8, 126u8, 78u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetPoolFeesArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let feeOperator = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetPoolFeesAccounts {
                    pool,
                    feeOperator,
                    remaining,
                };
                return Ok(Instruction::SetPoolFees { accounts, args });
            }
            [98u8, 86u8, 204u8, 51u8, 94u8, 71u8, 69u8, 187u8] => {
                let mut rdr: &[u8] = rest;
                let args = OverrideCurveParamArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = OverrideCurveParamAccounts {
                    pool,
                    admin,
                    remaining,
                };
                return Ok(Instruction::OverrideCurveParam { accounts, args });
            }
            [9u8, 48u8, 220u8, 101u8, 22u8, 240u8, 78u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = GetPoolInfoArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = GetPoolInfoAccounts {
                    pool,
                    lpMint,
                    aVaultLp,
                    bVaultLp,
                    aVault,
                    bVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    remaining,
                };
                return Ok(Instruction::GetPoolInfo { accounts, args });
            }
            [4u8, 228u8, 215u8, 71u8, 225u8, 253u8, 119u8, 206u8] => {
                let mut rdr: &[u8] = rest;
                let args = BootstrapLiquidityArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let userPoolLp = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let userAToken = keys.next().unwrap().clone();
                let userBToken = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BootstrapLiquidityAccounts {
                    pool,
                    lpMint,
                    userPoolLp,
                    aVaultLp,
                    bVaultLp,
                    aVault,
                    bVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aTokenVault,
                    bTokenVault,
                    userAToken,
                    userBToken,
                    user,
                    vaultProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::BootstrapLiquidity { accounts, args });
            }
            [2u8, 148u8, 176u8, 53u8, 24u8, 52u8, 75u8, 193u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateActivationSlotArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateActivationSlotAccounts {
                    pool,
                    admin,
                    remaining,
                };
                return Ok(Instruction::UpdateActivationSlot { accounts, args });
            }
            [13u8, 70u8, 168u8, 41u8, 250u8, 100u8, 148u8, 90u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateMintMetadataArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let mintMetadata = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateMintMetadataAccounts {
                    pool,
                    lpMint,
                    aVaultLp,
                    mintMetadata,
                    metadataProgram,
                    systemProgram,
                    payer,
                    remaining,
                };
                return Ok(Instruction::CreateMintMetadata { accounts, args });
            }
            [54u8, 87u8, 165u8, 19u8, 69u8, 227u8, 218u8, 224u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateLockEscrowArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lockEscrow = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateLockEscrowAccounts {
                    pool,
                    lockEscrow,
                    owner,
                    lpMint,
                    payer,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::CreateLockEscrow { accounts, args });
            }
            [21u8, 19u8, 208u8, 43u8, 237u8, 62u8, 255u8, 87u8] => {
                let mut rdr: &[u8] = rest;
                let args = LockArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let lockEscrow = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let sourceTokens = keys.next().unwrap().clone();
                let escrowVault = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LockAccounts {
                    pool,
                    lpMint,
                    lockEscrow,
                    owner,
                    sourceTokens,
                    escrowVault,
                    tokenProgram,
                    aVault,
                    bVault,
                    aVaultLp,
                    bVaultLp,
                    aVaultLpMint,
                    bVaultLpMint,
                    remaining,
                };
                return Ok(Instruction::Lock { accounts, args });
            }
            [169u8, 32u8, 79u8, 137u8, 136u8, 232u8, 70u8, 137u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimFeeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let lockEscrow = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let sourceTokens = keys.next().unwrap().clone();
                let escrowVault = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let userAToken = keys.next().unwrap().clone();
                let userBToken = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimFeeAccounts {
                    pool,
                    lpMint,
                    lockEscrow,
                    owner,
                    sourceTokens,
                    escrowVault,
                    tokenProgram,
                    aTokenVault,
                    bTokenVault,
                    aVault,
                    bVault,
                    aVaultLp,
                    bVaultLp,
                    aVaultLpMint,
                    bVaultLpMint,
                    userAToken,
                    userBToken,
                    vaultProgram,
                    remaining,
                };
                return Ok(Instruction::ClaimFee { accounts, args });
            }
            [201u8, 207u8, 243u8, 114u8, 75u8, 111u8, 47u8, 189u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateConfigArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateConfigAccounts {
                    config,
                    admin,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::CreateConfig { accounts, args });
            }
            [145u8, 9u8, 72u8, 157u8, 95u8, 125u8, 61u8, 85u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseConfigArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let config = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let rentReceiver = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseConfigAccounts {
                    config,
                    admin,
                    rentReceiver,
                    remaining,
                };
                return Ok(Instruction::CloseConfig { accounts, args });
            }
            [7u8, 166u8, 138u8, 171u8, 206u8, 171u8, 236u8, 244u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePermissionlessConstantProductPoolWithConfigArgs::deserialize(
                    &mut rdr,
                )?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let payerTokenA = keys.next().unwrap().clone();
                let payerTokenB = keys.next().unwrap().clone();
                let payerPoolLp = keys.next().unwrap().clone();
                let protocolTokenAFee = keys.next().unwrap().clone();
                let protocolTokenBFee = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let mintMetadata = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePermissionlessConstantProductPoolWithConfigAccounts {
                    pool,
                    config,
                    lpMint,
                    tokenAMint,
                    tokenBMint,
                    aVault,
                    bVault,
                    aTokenVault,
                    bTokenVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aVaultLp,
                    bVaultLp,
                    payerTokenA,
                    payerTokenB,
                    payerPoolLp,
                    protocolTokenAFee,
                    protocolTokenBFee,
                    payer,
                    rent,
                    mintMetadata,
                    metadataProgram,
                    vaultProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(
                    Instruction::InitializePermissionlessConstantProductPoolWithConfig {
                        accounts,
                        args,
                    },
                );
            }
            [48u8, 149u8, 220u8, 130u8, 61u8, 11u8, 9u8, 178u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializePermissionlessConstantProductPoolWithConfig2Args::deserialize(
                    &mut rdr,
                )?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let config = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let payerTokenA = keys.next().unwrap().clone();
                let payerTokenB = keys.next().unwrap().clone();
                let payerPoolLp = keys.next().unwrap().clone();
                let protocolTokenAFee = keys.next().unwrap().clone();
                let protocolTokenBFee = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let mintMetadata = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePermissionlessConstantProductPoolWithConfig2Accounts {
                    pool,
                    config,
                    lpMint,
                    tokenAMint,
                    tokenBMint,
                    aVault,
                    bVault,
                    aTokenVault,
                    bTokenVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aVaultLp,
                    bVaultLp,
                    payerTokenA,
                    payerTokenB,
                    payerPoolLp,
                    protocolTokenAFee,
                    protocolTokenBFee,
                    payer,
                    rent,
                    mintMetadata,
                    metadataProgram,
                    vaultProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(
                    Instruction::InitializePermissionlessConstantProductPoolWithConfig2 {
                        accounts,
                        args,
                    },
                );
            }
            [145u8, 24u8, 172u8, 194u8, 219u8, 125u8, 3u8, 190u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    InitializeCustomizablePermissionlessConstantProductPoolArgs::deserialize(
                        &mut rdr,
                    )?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let lpMint = keys.next().unwrap().clone();
                let tokenAMint = keys.next().unwrap().clone();
                let tokenBMint = keys.next().unwrap().clone();
                let aVault = keys.next().unwrap().clone();
                let bVault = keys.next().unwrap().clone();
                let aTokenVault = keys.next().unwrap().clone();
                let bTokenVault = keys.next().unwrap().clone();
                let aVaultLpMint = keys.next().unwrap().clone();
                let bVaultLpMint = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let bVaultLp = keys.next().unwrap().clone();
                let payerTokenA = keys.next().unwrap().clone();
                let payerTokenB = keys.next().unwrap().clone();
                let payerPoolLp = keys.next().unwrap().clone();
                let protocolTokenAFee = keys.next().unwrap().clone();
                let protocolTokenBFee = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let mintMetadata = keys.next().unwrap().clone();
                let metadataProgram = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeCustomizablePermissionlessConstantProductPoolAccounts {
                    pool,
                    lpMint,
                    tokenAMint,
                    tokenBMint,
                    aVault,
                    bVault,
                    aTokenVault,
                    bTokenVault,
                    aVaultLpMint,
                    bVaultLpMint,
                    aVaultLp,
                    bVaultLp,
                    payerTokenA,
                    payerTokenB,
                    payerPoolLp,
                    protocolTokenAFee,
                    protocolTokenBFee,
                    payer,
                    rent,
                    mintMetadata,
                    metadataProgram,
                    vaultProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(
                    Instruction::InitializeCustomizablePermissionlessConstantProductPool {
                        accounts,
                        args,
                    },
                );
            }
            [150u8, 62u8, 125u8, 219u8, 171u8, 220u8, 26u8, 237u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateActivationPointArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateActivationPointAccounts {
                    pool,
                    admin,
                    remaining,
                };
                return Ok(Instruction::UpdateActivationPoint { accounts, args });
            }
            [11u8, 68u8, 165u8, 98u8, 18u8, 208u8, 134u8, 73u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawProtocolFeesArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let protocolTokenAFee = keys.next().unwrap().clone();
                let protocolTokenBFee = keys.next().unwrap().clone();
                let treasuryTokenA = keys.next().unwrap().clone();
                let treasuryTokenB = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawProtocolFeesAccounts {
                    pool,
                    aVaultLp,
                    protocolTokenAFee,
                    protocolTokenBFee,
                    treasuryTokenA,
                    treasuryTokenB,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawProtocolFees { accounts, args });
            }
            [12u8, 148u8, 94u8, 42u8, 55u8, 57u8, 83u8, 247u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetWhitelistedVaultArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetWhitelistedVaultAccounts {
                    pool,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetWhitelistedVault { accounts, args });
            }
            [57u8, 53u8, 176u8, 30u8, 123u8, 70u8, 52u8, 64u8] => {
                let mut rdr: &[u8] = rest;
                let args = PartnerClaimFeeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pool = keys.next().unwrap().clone();
                let aVaultLp = keys.next().unwrap().clone();
                let protocolTokenAFee = keys.next().unwrap().clone();
                let protocolTokenBFee = keys.next().unwrap().clone();
                let partnerTokenA = keys.next().unwrap().clone();
                let partnerTokenB = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let partnerAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PartnerClaimFeeAccounts {
                    pool,
                    aVaultLp,
                    protocolTokenAFee,
                    protocolTokenBFee,
                    partnerTokenA,
                    partnerTokenB,
                    tokenProgram,
                    partnerAuthority,
                    remaining,
                };
                return Ok(Instruction::PartnerClaimFee { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
