use crate::pubkey_serializer::pubkey_serde;
use crate::pubkey_serializer::pubkey_serde_option;
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
    use anchor_lang::prelude::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde::Serialize;
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AmountWithSlippage {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
        pub slippage_bps: u16,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SplitLegDeeper {
        pub percent: u8,
        pub swap_leg: SwapLegSwap,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SplitLeg {
        pub percent: u8,
        pub swap_leg: SwapLegDeeper,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Side {
        Bid,
        Ask,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SwapLegSwap {
        PlaceholderOne,
        PlaceholderTwo,
        Swap { swap: Swap },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SwapLegDeeper {
        Chain { swap_legs: Vec<SwapLegSwap> },
        Split { split_legs: Vec<SplitLegDeeper> },
        Swap { swap: Swap },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum SwapLeg {
        Chain { swap_legs: Vec<SwapLegDeeper> },
        Split { split_legs: Vec<SplitLeg> },
        Swap { swap: Swap },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Swap {
        Saber,
        SaberAddDecimalsDeposit,
        SaberAddDecimalsWithdraw,
        TokenSwap,
        Sencha,
        Step,
        Cropper,
        Raydium,
        Crema { a_to_b: bool },
        Lifinity,
        Mercurial,
        Cykura,
        Serum { side: Side },
        MarinadeDeposit,
        MarinadeUnstake,
        Aldrin { side: Side },
        AldrinV2 { side: Side },
        Whirlpool { a_to_b: bool },
        Invariant { x_to_y: bool },
        Meteora,
        GooseFx,
        DeltaFi { stable: bool },
        Balansol,
        MarcoPolo { x_to_y: bool },
        Dradex { side: Side },
        LifinityV2,
        RaydiumClmm,
        Openbook { side: Side },
        Phoenix { side: Side },
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct RouteAccounts {
        pub tokenProgram: String,
        pub userTransferAuthority: String,
        pub destinationTokenAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WhirlpoolSwapExactOutputAccounts {
        pub swapProgram: String,
        pub tokenProgram: String,
        pub tokenAuthority: String,
        pub whirlpool: String,
        pub tokenOwnerAccountA: String,
        pub tokenVaultA: String,
        pub tokenOwnerAccountB: String,
        pub tokenVaultB: String,
        pub tickArray0: String,
        pub tickArray1: String,
        pub tickArray2: String,
        pub oracle: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RaydiumSwapExactOutputAccounts {
        pub swapProgram: String,
        pub tokenProgram: String,
        pub ammId: String,
        pub ammAuthority: String,
        pub ammOpenOrders: String,
        pub poolCoinTokenAccount: String,
        pub poolPcTokenAccount: String,
        pub serumProgramId: String,
        pub serumMarket: String,
        pub serumBids: String,
        pub serumAsks: String,
        pub serumEventQueue: String,
        pub serumCoinVaultAccount: String,
        pub serumPcVaultAccount: String,
        pub serumVaultSigner: String,
        pub userSourceTokenAccount: String,
        pub userDestinationTokenAccount: String,
        pub userSourceOwner: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RaydiumClmmSwapExactOutputAccounts {
        pub swapProgram: String,
        pub payer: String,
        pub ammConfig: String,
        pub poolState: String,
        pub inputTokenAccount: String,
        pub outputTokenAccount: String,
        pub inputVault: String,
        pub outputVault: String,
        pub observationState: String,
        pub tokenProgram: String,
        pub tickArray: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateOpenOrdersAccounts {
        pub openOrders: String,
        pub payer: String,
        pub dexProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub market: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateOpenOrderV2Accounts {
        pub openOrders: String,
        pub payer: String,
        pub dexProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub market: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MercurialSwapAccounts {
        pub swapProgram: String,
        pub swapState: String,
        pub tokenProgram: String,
        pub poolAuthority: String,
        pub userTransferAuthority: String,
        pub sourceTokenAccount: String,
        pub destinationTokenAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CykuraSwapAccounts {
        pub swapProgram: String,
        pub signer: String,
        pub factoryState: String,
        pub poolState: String,
        pub inputTokenAccount: String,
        pub outputTokenAccount: String,
        pub inputVault: String,
        pub outputVault: String,
        pub lastObservationState: String,
        pub coreProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SerumSwapAccounts {
        pub market: String,
        pub openOrders: String,
        pub requestQueue: String,
        pub eventQueue: String,
        pub bids: String,
        pub asks: String,
        pub coinVault: String,
        pub pcVault: String,
        pub vaultSigner: String,
        pub authority: String,
        pub orderPayerTokenAccount: String,
        pub coinWallet: String,
        pub pcWallet: String,
        pub dexProgram: String,
        pub tokenProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SaberSwapAccounts {
        pub swapProgram: String,
        pub tokenProgram: String,
        pub swap: String,
        pub swapAuthority: String,
        pub userAuthority: String,
        pub inputUserAccount: String,
        pub inputTokenAccount: String,
        pub outputUserAccount: String,
        pub outputTokenAccount: String,
        pub feesTokenAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SaberAddDecimalsAccounts {
        pub addDecimalsProgram: String,
        pub wrapper: String,
        pub wrapperMint: String,
        pub wrapperUnderlyingTokens: String,
        pub owner: String,
        pub userUnderlyingTokens: String,
        pub userWrappedTokens: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TokenSwapAccounts {
        pub tokenSwapProgram: String,
        pub tokenProgram: String,
        pub swap: String,
        pub authority: String,
        pub userTransferAuthority: String,
        pub source: String,
        pub swapSource: String,
        pub swapDestination: String,
        pub destination: String,
        pub poolMint: String,
        pub poolFee: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SenchaSwapAccounts {
        pub swapProgram: String,
        pub tokenProgram: String,
        pub swap: String,
        pub userAuthority: String,
        pub inputUserAccount: String,
        pub inputTokenAccount: String,
        pub inputFeesAccount: String,
        pub outputUserAccount: String,
        pub outputTokenAccount: String,
        pub outputFeesAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct StepSwapAccounts {
        pub tokenSwapProgram: String,
        pub tokenProgram: String,
        pub swap: String,
        pub authority: String,
        pub userTransferAuthority: String,
        pub source: String,
        pub swapSource: String,
        pub swapDestination: String,
        pub destination: String,
        pub poolMint: String,
        pub poolFee: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CropperSwapAccounts {
        pub tokenSwapProgram: String,
        pub tokenProgram: String,
        pub swap: String,
        pub swapState: String,
        pub authority: String,
        pub userTransferAuthority: String,
        pub source: String,
        pub swapSource: String,
        pub swapDestination: String,
        pub destination: String,
        pub poolMint: String,
        pub poolFee: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RaydiumSwapAccounts {
        pub swapProgram: String,
        pub tokenProgram: String,
        pub ammId: String,
        pub ammAuthority: String,
        pub ammOpenOrders: String,
        pub poolCoinTokenAccount: String,
        pub poolPcTokenAccount: String,
        pub serumProgramId: String,
        pub serumMarket: String,
        pub serumBids: String,
        pub serumAsks: String,
        pub serumEventQueue: String,
        pub serumCoinVaultAccount: String,
        pub serumPcVaultAccount: String,
        pub serumVaultSigner: String,
        pub userSourceTokenAccount: String,
        pub userDestinationTokenAccount: String,
        pub userSourceOwner: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CremaSwapAccounts {
        pub swapProgram: String,
        pub clmmConfig: String,
        pub clmmpool: String,
        pub tokenA: String,
        pub tokenB: String,
        pub accountA: String,
        pub accountB: String,
        pub tokenAVault: String,
        pub tokenBVault: String,
        pub tickArrayMap: String,
        pub owner: String,
        pub partner: String,
        pub partnerAtaA: String,
        pub partnerAtaB: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LifinitySwapAccounts {
        pub swapProgram: String,
        pub authority: String,
        pub amm: String,
        pub userTransferAuthority: String,
        pub sourceInfo: String,
        pub destinationInfo: String,
        pub swapSource: String,
        pub swapDestination: String,
        pub poolMint: String,
        pub feeAccount: String,
        pub tokenProgram: String,
        pub pythAccount: String,
        pub pythPcAccount: String,
        pub configAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MarinadeDepositAccounts {
        pub marinadeFinanceProgram: String,
        pub state: String,
        pub msolMint: String,
        pub liqPoolSolLegPda: String,
        pub liqPoolMsolLeg: String,
        pub liqPoolMsolLegAuthority: String,
        pub reservePda: String,
        pub transferFrom: String,
        pub mintTo: String,
        pub msolMintAuthority: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub userWsolTokenAccount: String,
        pub tempWsolTokenAccount: String,
        pub userTransferAuthority: String,
        pub wsolMint: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MarinadeUnstakeAccounts {
        pub marinadeFinanceProgram: String,
        pub state: String,
        pub msolMint: String,
        pub liqPoolSolLegPda: String,
        pub liqPoolMsolLeg: String,
        pub treasuryMsolAccount: String,
        pub getMsolFrom: String,
        pub getMsolFromAuthority: String,
        pub transferSolTo: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub userWsolTokenAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AldrinSwapAccounts {
        pub swapProgram: String,
        pub pool: String,
        pub poolSigner: String,
        pub poolMint: String,
        pub baseTokenVault: String,
        pub quoteTokenVault: String,
        pub feePoolTokenAccount: String,
        pub walletAuthority: String,
        pub userBaseTokenAccount: String,
        pub userQuoteTokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AldrinV2SwapAccounts {
        pub swapProgram: String,
        pub pool: String,
        pub poolSigner: String,
        pub poolMint: String,
        pub baseTokenVault: String,
        pub quoteTokenVault: String,
        pub feePoolTokenAccount: String,
        pub walletAuthority: String,
        pub userBaseTokenAccount: String,
        pub userQuoteTokenAccount: String,
        pub curve: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WhirlpoolSwapAccounts {
        pub swapProgram: String,
        pub tokenProgram: String,
        pub tokenAuthority: String,
        pub whirlpool: String,
        pub tokenOwnerAccountA: String,
        pub tokenVaultA: String,
        pub tokenOwnerAccountB: String,
        pub tokenVaultB: String,
        pub tickArray0: String,
        pub tickArray1: String,
        pub tickArray2: String,
        pub oracle: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InvariantSwapAccounts {
        pub swapProgram: String,
        pub state: String,
        pub pool: String,
        pub tickmap: String,
        pub accountX: String,
        pub accountY: String,
        pub reserveX: String,
        pub reserveY: String,
        pub owner: String,
        pub programAuthority: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MeteoraSwapAccounts {
        pub swapProgram: String,
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
        pub adminTokenFee: String,
        pub user: String,
        pub vaultProgram: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct GoosefxSwapAccounts {
        pub swapProgram: String,
        pub controller: String,
        pub pair: String,
        pub sslIn: String,
        pub sslOut: String,
        pub liabilityVaultIn: String,
        pub swappedLiabilityVaultIn: String,
        pub liabilityVaultOut: String,
        pub swappedLiabilityVaultOut: String,
        pub userInAta: String,
        pub userOutAta: String,
        pub feeCollectorAta: String,
        pub userWallet: String,
        pub feeCollector: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeltafiSwapAccounts {
        pub swapProgram: String,
        pub marketConfig: String,
        pub swapInfo: String,
        pub userSourceToken: String,
        pub userDestinationToken: String,
        pub swapSourceToken: String,
        pub swapDestinationToken: String,
        pub deltafiUser: String,
        pub adminDestinationToken: String,
        pub pythPriceBase: String,
        pub pythPriceQuote: String,
        pub userAuthority: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BalansolSwapAccounts {
        pub swapProgram: String,
        pub authority: String,
        pub pool: String,
        pub taxMan: String,
        pub bidMint: String,
        pub treasurer: String,
        pub srcTreasury: String,
        pub srcAssociatedTokenAccount: String,
        pub askMint: String,
        pub dstTreasury: String,
        pub dstAssociatedTokenAccount: String,
        pub dstTokenAccountTaxman: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MarcoPoloSwapAccounts {
        pub swapProgram: String,
        pub state: String,
        pub pool: String,
        pub tokenX: String,
        pub tokenY: String,
        pub poolXAccount: String,
        pub poolYAccount: String,
        pub swapperXAccount: String,
        pub swapperYAccount: String,
        pub swapper: String,
        pub referrerXAccount: String,
        pub referrerYAccount: String,
        pub referrer: String,
        pub programAuthority: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub associatedTokenProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DradexSwapAccounts {
        pub swapProgram: String,
        pub pair: String,
        pub market: String,
        pub eventQueue: String,
        pub dexUser: String,
        pub marketUser: String,
        pub bids: String,
        pub asks: String,
        pub t0Vault: String,
        pub t1Vault: String,
        pub t0User: String,
        pub t1User: String,
        pub master: String,
        pub signer: String,
        pub systemProgram: String,
        pub tokenProgram: String,
        pub logger: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LifinityV2SwapAccounts {
        pub swapProgram: String,
        pub authority: String,
        pub amm: String,
        pub userTransferAuthority: String,
        pub sourceInfo: String,
        pub destinationInfo: String,
        pub swapSource: String,
        pub swapDestination: String,
        pub poolMint: String,
        pub feeAccount: String,
        pub tokenProgram: String,
        pub oracleMainAccount: String,
        pub oracleSubAccount: String,
        pub oraclePcAccount: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RaydiumClmmSwapAccounts {
        pub swapProgram: String,
        pub payer: String,
        pub ammConfig: String,
        pub poolState: String,
        pub inputTokenAccount: String,
        pub outputTokenAccount: String,
        pub inputVault: String,
        pub outputVault: String,
        pub observationState: String,
        pub tokenProgram: String,
        pub tickArray: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PhoenixSwapAccounts {
        pub swapProgram: String,
        pub logAuthority: String,
        pub market: String,
        pub trader: String,
        pub baseAccount: String,
        pub quoteAccount: String,
        pub baseVault: String,
        pub quoteVault: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ClaimBonkAccounts {
        pub openOrders: String,
        pub bonkMint: String,
        pub openOrdersBonkTokenAccount: String,
        pub market: String,
        pub openOrdersOwner: String,
        pub claimerTokenAccount: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RouteArguments {
        pub swap_leg: SwapLeg,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub in_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WhirlpoolSwapExactOutputArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub out_amount: u64,
        pub in_amount_with_slippage: AmountWithSlippage,
        pub a_to_b: bool,
        pub platform_fee_bps: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RaydiumSwapExactOutputArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub out_amount: u64,
        pub in_amount_with_slippage: AmountWithSlippage,
        pub platform_fee_bps: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RaydiumClmmSwapExactOutputArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub out_amount: u64,
        pub in_amount_with_slippage: AmountWithSlippage,
        pub platform_fee_bps: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateOpenOrdersArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateOpenOrderV2Arguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MercurialSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CykuraSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SerumSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SaberSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SaberAddDecimalsArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TokenSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SenchaSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct StepSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CropperSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RaydiumSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CremaSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LifinitySwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MarinadeDepositArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MarinadeUnstakeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AldrinSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AldrinV2SwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WhirlpoolSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InvariantSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MeteoraSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct GoosefxSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeltafiSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BalansolSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MarcoPoloSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DradexSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LifinityV2SwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RaydiumClmmSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PhoenixSwapArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ClaimBonkArguments {}
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Route {
        accounts: RouteAccounts,
        args: RouteArguments,
    },
    WhirlpoolSwapExactOutput {
        accounts: WhirlpoolSwapExactOutputAccounts,
        args: WhirlpoolSwapExactOutputArguments,
    },
    RaydiumSwapExactOutput {
        accounts: RaydiumSwapExactOutputAccounts,
        args: RaydiumSwapExactOutputArguments,
    },
    RaydiumClmmSwapExactOutput {
        accounts: RaydiumClmmSwapExactOutputAccounts,
        args: RaydiumClmmSwapExactOutputArguments,
    },
    CreateOpenOrders {
        accounts: CreateOpenOrdersAccounts,
        args: CreateOpenOrdersArguments,
    },
    CreateOpenOrderV2 {
        accounts: CreateOpenOrderV2Accounts,
        args: CreateOpenOrderV2Arguments,
    },
    MercurialSwap {
        accounts: MercurialSwapAccounts,
        args: MercurialSwapArguments,
    },
    CykuraSwap {
        accounts: CykuraSwapAccounts,
        args: CykuraSwapArguments,
    },
    SerumSwap {
        accounts: SerumSwapAccounts,
        args: SerumSwapArguments,
    },
    SaberSwap {
        accounts: SaberSwapAccounts,
        args: SaberSwapArguments,
    },
    SaberAddDecimals {
        accounts: SaberAddDecimalsAccounts,
        args: SaberAddDecimalsArguments,
    },
    TokenSwap {
        accounts: TokenSwapAccounts,
        args: TokenSwapArguments,
    },
    SenchaSwap {
        accounts: SenchaSwapAccounts,
        args: SenchaSwapArguments,
    },
    StepSwap {
        accounts: StepSwapAccounts,
        args: StepSwapArguments,
    },
    CropperSwap {
        accounts: CropperSwapAccounts,
        args: CropperSwapArguments,
    },
    RaydiumSwap {
        accounts: RaydiumSwapAccounts,
        args: RaydiumSwapArguments,
    },
    CremaSwap {
        accounts: CremaSwapAccounts,
        args: CremaSwapArguments,
    },
    LifinitySwap {
        accounts: LifinitySwapAccounts,
        args: LifinitySwapArguments,
    },
    MarinadeDeposit {
        accounts: MarinadeDepositAccounts,
        args: MarinadeDepositArguments,
    },
    MarinadeUnstake {
        accounts: MarinadeUnstakeAccounts,
        args: MarinadeUnstakeArguments,
    },
    AldrinSwap {
        accounts: AldrinSwapAccounts,
        args: AldrinSwapArguments,
    },
    AldrinV2Swap {
        accounts: AldrinV2SwapAccounts,
        args: AldrinV2SwapArguments,
    },
    WhirlpoolSwap {
        accounts: WhirlpoolSwapAccounts,
        args: WhirlpoolSwapArguments,
    },
    InvariantSwap {
        accounts: InvariantSwapAccounts,
        args: InvariantSwapArguments,
    },
    MeteoraSwap {
        accounts: MeteoraSwapAccounts,
        args: MeteoraSwapArguments,
    },
    GoosefxSwap {
        accounts: GoosefxSwapAccounts,
        args: GoosefxSwapArguments,
    },
    DeltafiSwap {
        accounts: DeltafiSwapAccounts,
        args: DeltafiSwapArguments,
    },
    BalansolSwap {
        accounts: BalansolSwapAccounts,
        args: BalansolSwapArguments,
    },
    MarcoPoloSwap {
        accounts: MarcoPoloSwapAccounts,
        args: MarcoPoloSwapArguments,
    },
    DradexSwap {
        accounts: DradexSwapAccounts,
        args: DradexSwapArguments,
    },
    LifinityV2Swap {
        accounts: LifinityV2SwapAccounts,
        args: LifinityV2SwapArguments,
    },
    RaydiumClmmSwap {
        accounts: RaydiumClmmSwapAccounts,
        args: RaydiumClmmSwapArguments,
    },
    PhoenixSwap {
        accounts: PhoenixSwapAccounts,
        args: PhoenixSwapArguments,
    },
    ClaimBonk {
        accounts: ClaimBonkAccounts,
        args: ClaimBonkArguments,
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
            [229u8, 23u8, 203u8, 151u8, 122u8, 227u8, 173u8, 42u8] => {
                let mut rdr: &[u8] = rest;
                let args = RouteArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let tokenProgram = keys.next().unwrap().clone();
                let userTransferAuthority = keys.next().unwrap().clone();
                let destinationTokenAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RouteAccounts {
                    tokenProgram,
                    userTransferAuthority,
                    destinationTokenAccount,
                    remaining,
                };
                return Ok(Instruction::Route { accounts, args });
            }
            [39u8, 58u8, 38u8, 128u8, 100u8, 62u8, 191u8, 249u8] => {
                let mut rdr: &[u8] = rest;
                let args = WhirlpoolSwapExactOutputArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenAuthority = keys.next().unwrap().clone();
                let whirlpool = keys.next().unwrap().clone();
                let tokenOwnerAccountA = keys.next().unwrap().clone();
                let tokenVaultA = keys.next().unwrap().clone();
                let tokenOwnerAccountB = keys.next().unwrap().clone();
                let tokenVaultB = keys.next().unwrap().clone();
                let tickArray0 = keys.next().unwrap().clone();
                let tickArray1 = keys.next().unwrap().clone();
                let tickArray2 = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WhirlpoolSwapExactOutputAccounts {
                    swapProgram,
                    tokenProgram,
                    tokenAuthority,
                    whirlpool,
                    tokenOwnerAccountA,
                    tokenVaultA,
                    tokenOwnerAccountB,
                    tokenVaultB,
                    tickArray0,
                    tickArray1,
                    tickArray2,
                    oracle,
                    remaining,
                };
                return Ok(Instruction::WhirlpoolSwapExactOutput { accounts, args });
            }
            [249u8, 201u8, 126u8, 103u8, 127u8, 120u8, 177u8, 29u8] => {
                let mut rdr: &[u8] = rest;
                let args = RaydiumSwapExactOutputArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let ammId = keys.next().unwrap().clone();
                let ammAuthority = keys.next().unwrap().clone();
                let ammOpenOrders = keys.next().unwrap().clone();
                let poolCoinTokenAccount = keys.next().unwrap().clone();
                let poolPcTokenAccount = keys.next().unwrap().clone();
                let serumProgramId = keys.next().unwrap().clone();
                let serumMarket = keys.next().unwrap().clone();
                let serumBids = keys.next().unwrap().clone();
                let serumAsks = keys.next().unwrap().clone();
                let serumEventQueue = keys.next().unwrap().clone();
                let serumCoinVaultAccount = keys.next().unwrap().clone();
                let serumPcVaultAccount = keys.next().unwrap().clone();
                let serumVaultSigner = keys.next().unwrap().clone();
                let userSourceTokenAccount = keys.next().unwrap().clone();
                let userDestinationTokenAccount = keys.next().unwrap().clone();
                let userSourceOwner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RaydiumSwapExactOutputAccounts {
                    swapProgram,
                    tokenProgram,
                    ammId,
                    ammAuthority,
                    ammOpenOrders,
                    poolCoinTokenAccount,
                    poolPcTokenAccount,
                    serumProgramId,
                    serumMarket,
                    serumBids,
                    serumAsks,
                    serumEventQueue,
                    serumCoinVaultAccount,
                    serumPcVaultAccount,
                    serumVaultSigner,
                    userSourceTokenAccount,
                    userDestinationTokenAccount,
                    userSourceOwner,
                    remaining,
                };
                return Ok(Instruction::RaydiumSwapExactOutput { accounts, args });
            }
            [37u8, 178u8, 89u8, 146u8, 91u8, 241u8, 236u8, 97u8] => {
                let mut rdr: &[u8] = rest;
                let args = RaydiumClmmSwapExactOutputArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let inputTokenAccount = keys.next().unwrap().clone();
                let outputTokenAccount = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let outputVault = keys.next().unwrap().clone();
                let observationState = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tickArray = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RaydiumClmmSwapExactOutputAccounts {
                    swapProgram,
                    payer,
                    ammConfig,
                    poolState,
                    inputTokenAccount,
                    outputTokenAccount,
                    inputVault,
                    outputVault,
                    observationState,
                    tokenProgram,
                    tickArray,
                    remaining,
                };
                return Ok(Instruction::RaydiumClmmSwapExactOutput { accounts, args });
            }
            [229u8, 194u8, 212u8, 172u8, 8u8, 10u8, 134u8, 147u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateOpenOrdersArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let openOrders = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let dexProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let market = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateOpenOrdersAccounts {
                    openOrders,
                    payer,
                    dexProgram,
                    systemProgram,
                    rent,
                    market,
                    remaining,
                };
                return Ok(Instruction::CreateOpenOrders { accounts, args });
            }
            [237u8, 33u8, 92u8, 222u8, 220u8, 6u8, 42u8, 203u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateOpenOrderV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let openOrders = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let dexProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let market = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateOpenOrderV2Accounts {
                    openOrders,
                    payer,
                    dexProgram,
                    systemProgram,
                    rent,
                    market,
                    remaining,
                };
                return Ok(Instruction::CreateOpenOrderV2 { accounts, args });
            }
            [2u8, 5u8, 77u8, 173u8, 197u8, 0u8, 7u8, 157u8] => {
                let mut rdr: &[u8] = rest;
                let args = MercurialSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let swapState = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let poolAuthority = keys.next().unwrap().clone();
                let userTransferAuthority = keys.next().unwrap().clone();
                let sourceTokenAccount = keys.next().unwrap().clone();
                let destinationTokenAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MercurialSwapAccounts {
                    swapProgram,
                    swapState,
                    tokenProgram,
                    poolAuthority,
                    userTransferAuthority,
                    sourceTokenAccount,
                    destinationTokenAccount,
                    remaining,
                };
                return Ok(Instruction::MercurialSwap { accounts, args });
            }
            [38u8, 241u8, 21u8, 107u8, 120u8, 59u8, 184u8, 249u8] => {
                let mut rdr: &[u8] = rest;
                let args = CykuraSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let factoryState = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let inputTokenAccount = keys.next().unwrap().clone();
                let outputTokenAccount = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let outputVault = keys.next().unwrap().clone();
                let lastObservationState = keys.next().unwrap().clone();
                let coreProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CykuraSwapAccounts {
                    swapProgram,
                    signer,
                    factoryState,
                    poolState,
                    inputTokenAccount,
                    outputTokenAccount,
                    inputVault,
                    outputVault,
                    lastObservationState,
                    coreProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::CykuraSwap { accounts, args });
            }
            [88u8, 183u8, 70u8, 249u8, 214u8, 118u8, 82u8, 210u8] => {
                let mut rdr: &[u8] = rest;
                let args = SerumSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let market = keys.next().unwrap().clone();
                let openOrders = keys.next().unwrap().clone();
                let requestQueue = keys.next().unwrap().clone();
                let eventQueue = keys.next().unwrap().clone();
                let bids = keys.next().unwrap().clone();
                let asks = keys.next().unwrap().clone();
                let coinVault = keys.next().unwrap().clone();
                let pcVault = keys.next().unwrap().clone();
                let vaultSigner = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let orderPayerTokenAccount = keys.next().unwrap().clone();
                let coinWallet = keys.next().unwrap().clone();
                let pcWallet = keys.next().unwrap().clone();
                let dexProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SerumSwapAccounts {
                    market,
                    openOrders,
                    requestQueue,
                    eventQueue,
                    bids,
                    asks,
                    coinVault,
                    pcVault,
                    vaultSigner,
                    authority,
                    orderPayerTokenAccount,
                    coinWallet,
                    pcWallet,
                    dexProgram,
                    tokenProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::SerumSwap { accounts, args });
            }
            [64u8, 62u8, 98u8, 226u8, 52u8, 74u8, 37u8, 178u8] => {
                let mut rdr: &[u8] = rest;
                let args = SaberSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let swap = keys.next().unwrap().clone();
                let swapAuthority = keys.next().unwrap().clone();
                let userAuthority = keys.next().unwrap().clone();
                let inputUserAccount = keys.next().unwrap().clone();
                let inputTokenAccount = keys.next().unwrap().clone();
                let outputUserAccount = keys.next().unwrap().clone();
                let outputTokenAccount = keys.next().unwrap().clone();
                let feesTokenAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SaberSwapAccounts {
                    swapProgram,
                    tokenProgram,
                    swap,
                    swapAuthority,
                    userAuthority,
                    inputUserAccount,
                    inputTokenAccount,
                    outputUserAccount,
                    outputTokenAccount,
                    feesTokenAccount,
                    remaining,
                };
                return Ok(Instruction::SaberSwap { accounts, args });
            }
            [36u8, 53u8, 231u8, 184u8, 7u8, 181u8, 5u8, 238u8] => {
                let mut rdr: &[u8] = rest;
                let args = SaberAddDecimalsArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let addDecimalsProgram = keys.next().unwrap().clone();
                let wrapper = keys.next().unwrap().clone();
                let wrapperMint = keys.next().unwrap().clone();
                let wrapperUnderlyingTokens = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let userUnderlyingTokens = keys.next().unwrap().clone();
                let userWrappedTokens = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SaberAddDecimalsAccounts {
                    addDecimalsProgram,
                    wrapper,
                    wrapperMint,
                    wrapperUnderlyingTokens,
                    owner,
                    userUnderlyingTokens,
                    userWrappedTokens,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::SaberAddDecimals { accounts, args });
            }
            [187u8, 192u8, 118u8, 212u8, 62u8, 109u8, 28u8, 213u8] => {
                let mut rdr: &[u8] = rest;
                let args = TokenSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let tokenSwapProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let swap = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let userTransferAuthority = keys.next().unwrap().clone();
                let source = keys.next().unwrap().clone();
                let swapSource = keys.next().unwrap().clone();
                let swapDestination = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let poolMint = keys.next().unwrap().clone();
                let poolFee = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TokenSwapAccounts {
                    tokenSwapProgram,
                    tokenProgram,
                    swap,
                    authority,
                    userTransferAuthority,
                    source,
                    swapSource,
                    swapDestination,
                    destination,
                    poolMint,
                    poolFee,
                    remaining,
                };
                return Ok(Instruction::TokenSwap { accounts, args });
            }
            [25u8, 50u8, 7u8, 21u8, 207u8, 248u8, 230u8, 194u8] => {
                let mut rdr: &[u8] = rest;
                let args = SenchaSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let swap = keys.next().unwrap().clone();
                let userAuthority = keys.next().unwrap().clone();
                let inputUserAccount = keys.next().unwrap().clone();
                let inputTokenAccount = keys.next().unwrap().clone();
                let inputFeesAccount = keys.next().unwrap().clone();
                let outputUserAccount = keys.next().unwrap().clone();
                let outputTokenAccount = keys.next().unwrap().clone();
                let outputFeesAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SenchaSwapAccounts {
                    swapProgram,
                    tokenProgram,
                    swap,
                    userAuthority,
                    inputUserAccount,
                    inputTokenAccount,
                    inputFeesAccount,
                    outputUserAccount,
                    outputTokenAccount,
                    outputFeesAccount,
                    remaining,
                };
                return Ok(Instruction::SenchaSwap { accounts, args });
            }
            [155u8, 56u8, 208u8, 198u8, 27u8, 61u8, 149u8, 233u8] => {
                let mut rdr: &[u8] = rest;
                let args = StepSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let tokenSwapProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let swap = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let userTransferAuthority = keys.next().unwrap().clone();
                let source = keys.next().unwrap().clone();
                let swapSource = keys.next().unwrap().clone();
                let swapDestination = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let poolMint = keys.next().unwrap().clone();
                let poolFee = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = StepSwapAccounts {
                    tokenSwapProgram,
                    tokenProgram,
                    swap,
                    authority,
                    userTransferAuthority,
                    source,
                    swapSource,
                    swapDestination,
                    destination,
                    poolMint,
                    poolFee,
                    remaining,
                };
                return Ok(Instruction::StepSwap { accounts, args });
            }
            [230u8, 216u8, 47u8, 182u8, 165u8, 117u8, 210u8, 103u8] => {
                let mut rdr: &[u8] = rest;
                let args = CropperSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let tokenSwapProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let swap = keys.next().unwrap().clone();
                let swapState = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let userTransferAuthority = keys.next().unwrap().clone();
                let source = keys.next().unwrap().clone();
                let swapSource = keys.next().unwrap().clone();
                let swapDestination = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let poolMint = keys.next().unwrap().clone();
                let poolFee = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CropperSwapAccounts {
                    tokenSwapProgram,
                    tokenProgram,
                    swap,
                    swapState,
                    authority,
                    userTransferAuthority,
                    source,
                    swapSource,
                    swapDestination,
                    destination,
                    poolMint,
                    poolFee,
                    remaining,
                };
                return Ok(Instruction::CropperSwap { accounts, args });
            }
            [177u8, 173u8, 42u8, 240u8, 184u8, 4u8, 124u8, 81u8] => {
                let mut rdr: &[u8] = rest;
                let args = RaydiumSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let ammId = keys.next().unwrap().clone();
                let ammAuthority = keys.next().unwrap().clone();
                let ammOpenOrders = keys.next().unwrap().clone();
                let poolCoinTokenAccount = keys.next().unwrap().clone();
                let poolPcTokenAccount = keys.next().unwrap().clone();
                let serumProgramId = keys.next().unwrap().clone();
                let serumMarket = keys.next().unwrap().clone();
                let serumBids = keys.next().unwrap().clone();
                let serumAsks = keys.next().unwrap().clone();
                let serumEventQueue = keys.next().unwrap().clone();
                let serumCoinVaultAccount = keys.next().unwrap().clone();
                let serumPcVaultAccount = keys.next().unwrap().clone();
                let serumVaultSigner = keys.next().unwrap().clone();
                let userSourceTokenAccount = keys.next().unwrap().clone();
                let userDestinationTokenAccount = keys.next().unwrap().clone();
                let userSourceOwner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RaydiumSwapAccounts {
                    swapProgram,
                    tokenProgram,
                    ammId,
                    ammAuthority,
                    ammOpenOrders,
                    poolCoinTokenAccount,
                    poolPcTokenAccount,
                    serumProgramId,
                    serumMarket,
                    serumBids,
                    serumAsks,
                    serumEventQueue,
                    serumCoinVaultAccount,
                    serumPcVaultAccount,
                    serumVaultSigner,
                    userSourceTokenAccount,
                    userDestinationTokenAccount,
                    userSourceOwner,
                    remaining,
                };
                return Ok(Instruction::RaydiumSwap { accounts, args });
            }
            [169u8, 220u8, 41u8, 250u8, 35u8, 190u8, 133u8, 198u8] => {
                let mut rdr: &[u8] = rest;
                let args = CremaSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let clmmConfig = keys.next().unwrap().clone();
                let clmmpool = keys.next().unwrap().clone();
                let tokenA = keys.next().unwrap().clone();
                let tokenB = keys.next().unwrap().clone();
                let accountA = keys.next().unwrap().clone();
                let accountB = keys.next().unwrap().clone();
                let tokenAVault = keys.next().unwrap().clone();
                let tokenBVault = keys.next().unwrap().clone();
                let tickArrayMap = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let partner = keys.next().unwrap().clone();
                let partnerAtaA = keys.next().unwrap().clone();
                let partnerAtaB = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CremaSwapAccounts {
                    swapProgram,
                    clmmConfig,
                    clmmpool,
                    tokenA,
                    tokenB,
                    accountA,
                    accountB,
                    tokenAVault,
                    tokenBVault,
                    tickArrayMap,
                    owner,
                    partner,
                    partnerAtaA,
                    partnerAtaB,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::CremaSwap { accounts, args });
            }
            [23u8, 96u8, 165u8, 33u8, 90u8, 214u8, 96u8, 153u8] => {
                let mut rdr: &[u8] = rest;
                let args = LifinitySwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let amm = keys.next().unwrap().clone();
                let userTransferAuthority = keys.next().unwrap().clone();
                let sourceInfo = keys.next().unwrap().clone();
                let destinationInfo = keys.next().unwrap().clone();
                let swapSource = keys.next().unwrap().clone();
                let swapDestination = keys.next().unwrap().clone();
                let poolMint = keys.next().unwrap().clone();
                let feeAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let pythAccount = keys.next().unwrap().clone();
                let pythPcAccount = keys.next().unwrap().clone();
                let configAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LifinitySwapAccounts {
                    swapProgram,
                    authority,
                    amm,
                    userTransferAuthority,
                    sourceInfo,
                    destinationInfo,
                    swapSource,
                    swapDestination,
                    poolMint,
                    feeAccount,
                    tokenProgram,
                    pythAccount,
                    pythPcAccount,
                    configAccount,
                    remaining,
                };
                return Ok(Instruction::LifinitySwap { accounts, args });
            }
            [62u8, 236u8, 248u8, 28u8, 222u8, 232u8, 182u8, 73u8] => {
                let mut rdr: &[u8] = rest;
                let args = MarinadeDepositArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marinadeFinanceProgram = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let msolMint = keys.next().unwrap().clone();
                let liqPoolSolLegPda = keys.next().unwrap().clone();
                let liqPoolMsolLeg = keys.next().unwrap().clone();
                let liqPoolMsolLegAuthority = keys.next().unwrap().clone();
                let reservePda = keys.next().unwrap().clone();
                let transferFrom = keys.next().unwrap().clone();
                let mintTo = keys.next().unwrap().clone();
                let msolMintAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let userWsolTokenAccount = keys.next().unwrap().clone();
                let tempWsolTokenAccount = keys.next().unwrap().clone();
                let userTransferAuthority = keys.next().unwrap().clone();
                let wsolMint = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MarinadeDepositAccounts {
                    marinadeFinanceProgram,
                    state,
                    msolMint,
                    liqPoolSolLegPda,
                    liqPoolMsolLeg,
                    liqPoolMsolLegAuthority,
                    reservePda,
                    transferFrom,
                    mintTo,
                    msolMintAuthority,
                    systemProgram,
                    tokenProgram,
                    userWsolTokenAccount,
                    tempWsolTokenAccount,
                    userTransferAuthority,
                    wsolMint,
                    rent,
                    remaining,
                };
                return Ok(Instruction::MarinadeDeposit { accounts, args });
            }
            [41u8, 120u8, 15u8, 0u8, 113u8, 219u8, 42u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let args = MarinadeUnstakeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let marinadeFinanceProgram = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let msolMint = keys.next().unwrap().clone();
                let liqPoolSolLegPda = keys.next().unwrap().clone();
                let liqPoolMsolLeg = keys.next().unwrap().clone();
                let treasuryMsolAccount = keys.next().unwrap().clone();
                let getMsolFrom = keys.next().unwrap().clone();
                let getMsolFromAuthority = keys.next().unwrap().clone();
                let transferSolTo = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let userWsolTokenAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MarinadeUnstakeAccounts {
                    marinadeFinanceProgram,
                    state,
                    msolMint,
                    liqPoolSolLegPda,
                    liqPoolMsolLeg,
                    treasuryMsolAccount,
                    getMsolFrom,
                    getMsolFromAuthority,
                    transferSolTo,
                    systemProgram,
                    tokenProgram,
                    userWsolTokenAccount,
                    remaining,
                };
                return Ok(Instruction::MarinadeUnstake { accounts, args });
            }
            [251u8, 232u8, 119u8, 166u8, 225u8, 185u8, 169u8, 161u8] => {
                let mut rdr: &[u8] = rest;
                let args = AldrinSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let poolSigner = keys.next().unwrap().clone();
                let poolMint = keys.next().unwrap().clone();
                let baseTokenVault = keys.next().unwrap().clone();
                let quoteTokenVault = keys.next().unwrap().clone();
                let feePoolTokenAccount = keys.next().unwrap().clone();
                let walletAuthority = keys.next().unwrap().clone();
                let userBaseTokenAccount = keys.next().unwrap().clone();
                let userQuoteTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AldrinSwapAccounts {
                    swapProgram,
                    pool,
                    poolSigner,
                    poolMint,
                    baseTokenVault,
                    quoteTokenVault,
                    feePoolTokenAccount,
                    walletAuthority,
                    userBaseTokenAccount,
                    userQuoteTokenAccount,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::AldrinSwap { accounts, args });
            }
            [190u8, 166u8, 89u8, 139u8, 33u8, 152u8, 16u8, 10u8] => {
                let mut rdr: &[u8] = rest;
                let args = AldrinV2SwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let poolSigner = keys.next().unwrap().clone();
                let poolMint = keys.next().unwrap().clone();
                let baseTokenVault = keys.next().unwrap().clone();
                let quoteTokenVault = keys.next().unwrap().clone();
                let feePoolTokenAccount = keys.next().unwrap().clone();
                let walletAuthority = keys.next().unwrap().clone();
                let userBaseTokenAccount = keys.next().unwrap().clone();
                let userQuoteTokenAccount = keys.next().unwrap().clone();
                let curve = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AldrinV2SwapAccounts {
                    swapProgram,
                    pool,
                    poolSigner,
                    poolMint,
                    baseTokenVault,
                    quoteTokenVault,
                    feePoolTokenAccount,
                    walletAuthority,
                    userBaseTokenAccount,
                    userQuoteTokenAccount,
                    curve,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::AldrinV2Swap { accounts, args });
            }
            [123u8, 229u8, 184u8, 63u8, 12u8, 0u8, 92u8, 145u8] => {
                let mut rdr: &[u8] = rest;
                let args = WhirlpoolSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenAuthority = keys.next().unwrap().clone();
                let whirlpool = keys.next().unwrap().clone();
                let tokenOwnerAccountA = keys.next().unwrap().clone();
                let tokenVaultA = keys.next().unwrap().clone();
                let tokenOwnerAccountB = keys.next().unwrap().clone();
                let tokenVaultB = keys.next().unwrap().clone();
                let tickArray0 = keys.next().unwrap().clone();
                let tickArray1 = keys.next().unwrap().clone();
                let tickArray2 = keys.next().unwrap().clone();
                let oracle = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WhirlpoolSwapAccounts {
                    swapProgram,
                    tokenProgram,
                    tokenAuthority,
                    whirlpool,
                    tokenOwnerAccountA,
                    tokenVaultA,
                    tokenOwnerAccountB,
                    tokenVaultB,
                    tickArray0,
                    tickArray1,
                    tickArray2,
                    oracle,
                    remaining,
                };
                return Ok(Instruction::WhirlpoolSwap { accounts, args });
            }
            [187u8, 193u8, 40u8, 121u8, 47u8, 73u8, 144u8, 177u8] => {
                let mut rdr: &[u8] = rest;
                let args = InvariantSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let tickmap = keys.next().unwrap().clone();
                let accountX = keys.next().unwrap().clone();
                let accountY = keys.next().unwrap().clone();
                let reserveX = keys.next().unwrap().clone();
                let reserveY = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let programAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InvariantSwapAccounts {
                    swapProgram,
                    state,
                    pool,
                    tickmap,
                    accountX,
                    accountY,
                    reserveX,
                    reserveY,
                    owner,
                    programAuthority,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::InvariantSwap { accounts, args });
            }
            [127u8, 125u8, 226u8, 12u8, 81u8, 24u8, 204u8, 35u8] => {
                let mut rdr: &[u8] = rest;
                let args = MeteoraSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
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
                let adminTokenFee = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let vaultProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MeteoraSwapAccounts {
                    swapProgram,
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
                    adminTokenFee,
                    user,
                    vaultProgram,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::MeteoraSwap { accounts, args });
            }
            [222u8, 136u8, 46u8, 123u8, 189u8, 125u8, 124u8, 122u8] => {
                let mut rdr: &[u8] = rest;
                let args = GoosefxSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let controller = keys.next().unwrap().clone();
                let pair = keys.next().unwrap().clone();
                let sslIn = keys.next().unwrap().clone();
                let sslOut = keys.next().unwrap().clone();
                let liabilityVaultIn = keys.next().unwrap().clone();
                let swappedLiabilityVaultIn = keys.next().unwrap().clone();
                let liabilityVaultOut = keys.next().unwrap().clone();
                let swappedLiabilityVaultOut = keys.next().unwrap().clone();
                let userInAta = keys.next().unwrap().clone();
                let userOutAta = keys.next().unwrap().clone();
                let feeCollectorAta = keys.next().unwrap().clone();
                let userWallet = keys.next().unwrap().clone();
                let feeCollector = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = GoosefxSwapAccounts {
                    swapProgram,
                    controller,
                    pair,
                    sslIn,
                    sslOut,
                    liabilityVaultIn,
                    swappedLiabilityVaultIn,
                    liabilityVaultOut,
                    swappedLiabilityVaultOut,
                    userInAta,
                    userOutAta,
                    feeCollectorAta,
                    userWallet,
                    feeCollector,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::GoosefxSwap { accounts, args });
            }
            [132u8, 230u8, 102u8, 120u8, 205u8, 9u8, 237u8, 190u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeltafiSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let marketConfig = keys.next().unwrap().clone();
                let swapInfo = keys.next().unwrap().clone();
                let userSourceToken = keys.next().unwrap().clone();
                let userDestinationToken = keys.next().unwrap().clone();
                let swapSourceToken = keys.next().unwrap().clone();
                let swapDestinationToken = keys.next().unwrap().clone();
                let deltafiUser = keys.next().unwrap().clone();
                let adminDestinationToken = keys.next().unwrap().clone();
                let pythPriceBase = keys.next().unwrap().clone();
                let pythPriceQuote = keys.next().unwrap().clone();
                let userAuthority = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeltafiSwapAccounts {
                    swapProgram,
                    marketConfig,
                    swapInfo,
                    userSourceToken,
                    userDestinationToken,
                    swapSourceToken,
                    swapDestinationToken,
                    deltafiUser,
                    adminDestinationToken,
                    pythPriceBase,
                    pythPriceQuote,
                    userAuthority,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::DeltafiSwap { accounts, args });
            }
            [137u8, 109u8, 253u8, 253u8, 70u8, 109u8, 11u8, 100u8] => {
                let mut rdr: &[u8] = rest;
                let args = BalansolSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let taxMan = keys.next().unwrap().clone();
                let bidMint = keys.next().unwrap().clone();
                let treasurer = keys.next().unwrap().clone();
                let srcTreasury = keys.next().unwrap().clone();
                let srcAssociatedTokenAccount = keys.next().unwrap().clone();
                let askMint = keys.next().unwrap().clone();
                let dstTreasury = keys.next().unwrap().clone();
                let dstAssociatedTokenAccount = keys.next().unwrap().clone();
                let dstTokenAccountTaxman = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BalansolSwapAccounts {
                    swapProgram,
                    authority,
                    pool,
                    taxMan,
                    bidMint,
                    treasurer,
                    srcTreasury,
                    srcAssociatedTokenAccount,
                    askMint,
                    dstTreasury,
                    dstAssociatedTokenAccount,
                    dstTokenAccountTaxman,
                    systemProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::BalansolSwap { accounts, args });
            }
            [241u8, 147u8, 94u8, 15u8, 58u8, 108u8, 179u8, 68u8] => {
                let mut rdr: &[u8] = rest;
                let args = MarcoPoloSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let state = keys.next().unwrap().clone();
                let pool = keys.next().unwrap().clone();
                let tokenX = keys.next().unwrap().clone();
                let tokenY = keys.next().unwrap().clone();
                let poolXAccount = keys.next().unwrap().clone();
                let poolYAccount = keys.next().unwrap().clone();
                let swapperXAccount = keys.next().unwrap().clone();
                let swapperYAccount = keys.next().unwrap().clone();
                let swapper = keys.next().unwrap().clone();
                let referrerXAccount = keys.next().unwrap().clone();
                let referrerYAccount = keys.next().unwrap().clone();
                let referrer = keys.next().unwrap().clone();
                let programAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let associatedTokenProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MarcoPoloSwapAccounts {
                    swapProgram,
                    state,
                    pool,
                    tokenX,
                    tokenY,
                    poolXAccount,
                    poolYAccount,
                    swapperXAccount,
                    swapperYAccount,
                    swapper,
                    referrerXAccount,
                    referrerYAccount,
                    referrer,
                    programAuthority,
                    systemProgram,
                    tokenProgram,
                    associatedTokenProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::MarcoPoloSwap { accounts, args });
            }
            [34u8, 146u8, 160u8, 38u8, 51u8, 85u8, 58u8, 151u8] => {
                let mut rdr: &[u8] = rest;
                let args = DradexSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let pair = keys.next().unwrap().clone();
                let market = keys.next().unwrap().clone();
                let eventQueue = keys.next().unwrap().clone();
                let dexUser = keys.next().unwrap().clone();
                let marketUser = keys.next().unwrap().clone();
                let bids = keys.next().unwrap().clone();
                let asks = keys.next().unwrap().clone();
                let t0Vault = keys.next().unwrap().clone();
                let t1Vault = keys.next().unwrap().clone();
                let t0User = keys.next().unwrap().clone();
                let t1User = keys.next().unwrap().clone();
                let master = keys.next().unwrap().clone();
                let signer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let logger = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DradexSwapAccounts {
                    swapProgram,
                    pair,
                    market,
                    eventQueue,
                    dexUser,
                    marketUser,
                    bids,
                    asks,
                    t0Vault,
                    t1Vault,
                    t0User,
                    t1User,
                    master,
                    signer,
                    systemProgram,
                    tokenProgram,
                    logger,
                    remaining,
                };
                return Ok(Instruction::DradexSwap { accounts, args });
            }
            [19u8, 152u8, 195u8, 245u8, 187u8, 144u8, 74u8, 227u8] => {
                let mut rdr: &[u8] = rest;
                let args = LifinityV2SwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let amm = keys.next().unwrap().clone();
                let userTransferAuthority = keys.next().unwrap().clone();
                let sourceInfo = keys.next().unwrap().clone();
                let destinationInfo = keys.next().unwrap().clone();
                let swapSource = keys.next().unwrap().clone();
                let swapDestination = keys.next().unwrap().clone();
                let poolMint = keys.next().unwrap().clone();
                let feeAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let oracleMainAccount = keys.next().unwrap().clone();
                let oracleSubAccount = keys.next().unwrap().clone();
                let oraclePcAccount = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LifinityV2SwapAccounts {
                    swapProgram,
                    authority,
                    amm,
                    userTransferAuthority,
                    sourceInfo,
                    destinationInfo,
                    swapSource,
                    swapDestination,
                    poolMint,
                    feeAccount,
                    tokenProgram,
                    oracleMainAccount,
                    oracleSubAccount,
                    oraclePcAccount,
                    remaining,
                };
                return Ok(Instruction::LifinityV2Swap { accounts, args });
            }
            [47u8, 184u8, 213u8, 193u8, 35u8, 210u8, 87u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let args = RaydiumClmmSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let ammConfig = keys.next().unwrap().clone();
                let poolState = keys.next().unwrap().clone();
                let inputTokenAccount = keys.next().unwrap().clone();
                let outputTokenAccount = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let outputVault = keys.next().unwrap().clone();
                let observationState = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tickArray = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RaydiumClmmSwapAccounts {
                    swapProgram,
                    payer,
                    ammConfig,
                    poolState,
                    inputTokenAccount,
                    outputTokenAccount,
                    inputVault,
                    outputVault,
                    observationState,
                    tokenProgram,
                    tickArray,
                    remaining,
                };
                return Ok(Instruction::RaydiumClmmSwap { accounts, args });
            }
            [99u8, 66u8, 223u8, 95u8, 236u8, 131u8, 26u8, 140u8] => {
                let mut rdr: &[u8] = rest;
                let args = PhoenixSwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let swapProgram = keys.next().unwrap().clone();
                let logAuthority = keys.next().unwrap().clone();
                let market = keys.next().unwrap().clone();
                let trader = keys.next().unwrap().clone();
                let baseAccount = keys.next().unwrap().clone();
                let quoteAccount = keys.next().unwrap().clone();
                let baseVault = keys.next().unwrap().clone();
                let quoteVault = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PhoenixSwapAccounts {
                    swapProgram,
                    logAuthority,
                    market,
                    trader,
                    baseAccount,
                    quoteAccount,
                    baseVault,
                    quoteVault,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::PhoenixSwap { accounts, args });
            }
            [223u8, 175u8, 186u8, 212u8, 1u8, 206u8, 82u8, 219u8] => {
                let mut rdr: &[u8] = rest;
                let args = ClaimBonkArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let openOrders = keys.next().unwrap().clone();
                let bonkMint = keys.next().unwrap().clone();
                let openOrdersBonkTokenAccount = keys.next().unwrap().clone();
                let market = keys.next().unwrap().clone();
                let openOrdersOwner = keys.next().unwrap().clone();
                let claimerTokenAccount = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimBonkAccounts {
                    openOrders,
                    bonkMint,
                    openOrdersBonkTokenAccount,
                    market,
                    openOrdersOwner,
                    claimerTokenAccount,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::ClaimBonk { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    use borsh::BorshDeserialize;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct Swap {
        #[serde(with = "pubkey_serde")]
        pub amm: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub input_mint: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_amount: u64,
        #[serde(with = "pubkey_serde")]
        pub output_mint: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub output_amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct Fee {
        #[serde(with = "pubkey_serde")]
        pub account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub mint: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        Swap { args: Swap },
        Fee { args: Fee },
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
                [81u8, 108u8, 227u8, 190u8, 205u8, 208u8, 10u8, 196u8] => {
                    let mut rdr = &payload[..];
                    let args = Swap::deserialize(&mut rdr)?;
                    return Ok(Event::Swap { args });
                }
                [6u8, 220u8, 131u8, 59u8, 240u8, 71u8, 51u8, 96u8] => {
                    let mut rdr = &payload[..];
                    let args = Fee::deserialize(&mut rdr)?;
                    return Ok(Event::Fee { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
