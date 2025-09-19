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
    use serde::Serialize;
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Action {
        WhirlpoolsSwap(WhirlpoolsSwapOptions),
        ClearpoolsSwap(ClearpoolsSwapOptions),
        RaydiumAmmSwap(RaydiumAmmSwapOptions),
        LifinityV2Swap(LifinityV2SwapOptions),
        MeteoraDlmmSwap(MeteoraDlmmSwapOptions),
        RaydiumClmmSwap(RaydiumClmmSwapOptions),
        RaydiumClmmSwapV2(RaydiumClmmSwapV2Options),
        PhoenixSwap(PhoenixSwapOptions),
        PumpFunBuy(PumpFunBuyOptions),
        PumpFunSell(PumpFunSellOptions),
        GammaSwap(GammaSwapOptions),
        ObricV2Swap(ObricV2SwapOptions),
        PumpFunAmmBuy(PumpFunAmmBuyOptions),
        PumpFunAmmSell(PumpFunAmmSellOptions),
        SolFiSwap(SolFiSwapOptions),
        RubiconSwap(RubiconSwapOptions),
        MeteoraDammV1Swap(MeteoraDammV1SwapOptions),
        RaydiumCpSwap(RaydiumCpSwapOptions),
        StabbleStableSwap(StabbleStableSwapOptions),
        TesseraVSwap(TesseraVSwapOptions),
        MeteoraDammV2Swap(MeteoraDammV2SwapOptions),
        RaydiumLaunchlabSwap(RaydiumLaunchlabSwapOptions),
        MeteoraDbcSwap(MeteoraDbcSwapOptions),
        HumidiFiSwap(HumidiFiSwapOptions),
        WhirlpoolsSwapV2(WhirlpoolsSwapV2Options),
        MeteoraDlmmSwapV2(MeteoraDlmmSwapV2Options),
        ZeroFiSwap(ZeroFiSwapOptions),
        AlphaQSwap(AlphaQSwapOptions),
        TokenSwap(TokenSwapOptions),
        SolFiV2Swap(SolFiV2SwapOptions),
        MozartSwap(MozartSwapOptions),
        DFlowDynamicRouteV1(DFlowDynamicRouteV1Options),
        HeavenSwap(HeavenSwapOptions),
        NexusSwap(NexusSwapOptions),
        SarosDlmmSwap(SarosDlmmSwapOptions),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AlphaQSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ClearpoolsSwapOptions {
        pub amount: u64,
        pub a_to_b: bool,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DFlowDynamicRouteV1Options {
        pub candidate_actions: Vec<DynamicRouteV1CandidateAction>,
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum DynamicRouteV1CandidateAction {
        SolFi(SolFiDynamicRouteV1Options),
        Rubicon(RubiconDynamicRouteV1Options),
        TesseraV(TesseraVDynamicRouteV1Options),
        HumidiFi(HumidiFiDynamicRouteV1Options),
        SolFiV2(SolFiV2DynamicRouteV1Options),
        Mozart(MozartDynamicRouteV1Options),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FeeEvent {
        pub account: [u8; 32usize],
        pub mint: [u8; 32usize],
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FillOrderParams {
        pub swap_actions: Vec<Action>,
        pub platform_fee_pips: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct GammaSwapOptions {
        pub amount: u64,
        pub endorsed: bool,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct HeavenSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct HumidiFiDynamicRouteV1Options {
        pub swap_id: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct HumidiFiSwapOptions {
        pub amount: u64,
        pub swap_id: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LifinityV2SwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MeteoraDammV1SwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MeteoraDammV2SwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MeteoraDbcSwapOptions {
        pub amount: u64,
        pub is_rate_limiter_applied: bool,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MeteoraDlmmSwapOptions {
        pub amount: u64,
        pub num_bin_arrays: u8,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MeteoraDlmmSwapV2Options {
        pub amount: u64,
        pub num_bin_arrays: u8,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MozartDynamicRouteV1Options();
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MozartSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct NexusSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ObricV2SwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OpenOrderParams {
        pub input_amount: u64,
        pub quoted_out_amount: u64,
        pub fee_budget: u64,
        pub order_account_id: u64,
        pub fillable_for_slots: u32,
        pub slippage_bps: u16,
        pub closer: [u8; 32usize],
        pub flags: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OrchestratorFlags {
        pub flags: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Order {
        pub closer: [u8; 32usize],
        pub output_token_account: [u8; 32usize],
        pub return_input_token_account: [u8; 32usize],
        pub return_rent_to: [u8; 32usize],
        pub id: u64,
        pub quoted_out_amount: u64,
        pub last_fillable_slot: u64,
        pub slippage_bps: u16,
        pub bump: u8,
        pub vault_bump: u8,
        pub flags: u8,
        pub padding1: u8,
        pub padding2: u8,
        pub padding3: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PhoenixSwapOptions {
        pub amount: u64,
        pub side: Side,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PumpFunAmmBuyOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PumpFunAmmSellOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PumpFunBuyOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PumpFunSellOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RaydiumAmmSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RaydiumClmmSwapOptions {
        pub amount: u64,
        pub num_remaining_accounts: u8,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RaydiumClmmSwapV2Options {
        pub amount: u64,
        pub num_remaining_accounts: u8,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RaydiumCpSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RaydiumLaunchlabSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RubiconDynamicRouteV1Options();
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RubiconSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SarosDlmmSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Side {
        Bid,
        Ask,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SolFiDynamicRouteV1Options();
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SolFiSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SolFiV2DynamicRouteV1Options();
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SolFiV2SwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct StabbleStableSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Swap2Params {
        pub actions: Vec<Action>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u16,
        pub positive_slippage_fee_limit_pct: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SwapEvent {
        pub amm: [u8; 32usize],
        pub input_mint: [u8; 32usize],
        pub input_amount: u64,
        pub output_mint: [u8; 32usize],
        pub output_amount: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SwapParams {
        pub actions: Vec<Action>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u16,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TesseraVDynamicRouteV1Options();
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TesseraVSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TokenSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WhirlpoolsSwapOptions {
        pub amount: u64,
        pub a_to_b: bool,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WhirlpoolsSwapV2Options {
        pub amount: u64,
        pub a_to_b: bool,
        pub orchestrator_flags: OrchestratorFlags,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ZeroFiSwapOptions {
        pub amount: u64,
        pub orchestrator_flags: OrchestratorFlags,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct CloseOrderAccounts {
        pub order: String,
        pub order_vault: String,
        pub return_input_token_account: String,
        pub return_rent_to: String,
        pub closer: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateReferralTokenAccountIdempotentAccounts {
        pub payer: String,
        pub project: String,
        pub referral_account: String,
        pub referral_token_account: String,
        pub mint: String,
        pub system_program: String,
        pub token_program: String,
        pub referral_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FillOrderAccounts {
        pub order: String,
        pub order_vault: String,
        pub output_token_account: String,
        pub return_input_token_account: String,
        pub return_rent_to: String,
        pub filler_input_token_account: String,
        pub input_mint: String,
        pub filler_output_token_account: String,
        pub output_mint: String,
        pub filler: String,
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct OpenOrderAccounts {
        pub order: String,
        pub order_vault: String,
        pub input_token_account: String,
        pub output_token_account: String,
        pub return_input_token_account: String,
        pub input_mint: String,
        pub user_token_authority: String,
        pub fee_payer: String,
        pub fee_receiver: String,
        pub rent_depositor: String,
        pub token_program: String,
        pub system_program: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapAccounts {
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub user_token_authority: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct Swap2Accounts {
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub user_token_authority: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct Swap2WithDestinationAccounts {
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub user_token_authority: String,
        pub destination_token_account: String,
        pub destination_token_authority: String,
        pub destination_mint: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct Swap2WithDestinationNativeAccounts {
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub user_token_authority: String,
        pub destination_account: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapWithDestinationAccounts {
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub user_token_authority: String,
        pub destination_token_account: String,
        pub destination_token_authority: String,
        pub destination_mint: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapWithDestinationNativeAccounts {
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub user_token_authority: String,
        pub destination_account: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferFeeAccounts {
        pub from: String,
        pub to: String,
        pub authority: String,
        pub token_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferSolAccounts {
        pub from: String,
        pub to: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferToSponsorAccounts {
        pub user_token_authority: String,
        pub user_token_account: String,
        pub sponsor: String,
        pub sponsor_token_account: String,
        pub mint: String,
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UnwrapSolAccounts {
        pub owner: String,
        pub wrapped_sol_associated_token_account: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WrapSolAccounts {
        pub from: String,
        pub wrapped_sol_associated_token_account: String,
        pub native_mint: String,
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseOrderArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateReferralTokenAccountIdempotentArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FillOrderArguments {
        pub params: FillOrderParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OpenOrderArguments {
        pub params: OpenOrderParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapArguments {
        pub params: SwapParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct Swap2Arguments {
        pub params: Swap2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct Swap2WithDestinationArguments {
        pub params: Swap2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct Swap2WithDestinationNativeArguments {
        pub params: Swap2Params,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapWithDestinationArguments {
        pub params: SwapParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapWithDestinationNativeArguments {
        pub params: SwapParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferFeeArguments {
        #[serde(serialize_with = "super::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferSolArguments {
        #[serde(serialize_with = "super::serialize_to_string")]
        pub lamports: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferToSponsorArguments {
        #[serde(serialize_with = "super::serialize_to_string")]
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnwrapSolArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WrapSolArguments {
        #[serde(serialize_with = "super::serialize_to_string")]
        pub lamports: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    CloseOrder {
        accounts: CloseOrderAccounts,
        args: CloseOrderArguments,
    },
    CreateReferralTokenAccountIdempotent {
        accounts: CreateReferralTokenAccountIdempotentAccounts,
        args: CreateReferralTokenAccountIdempotentArguments,
    },
    FillOrder {
        accounts: FillOrderAccounts,
        args: FillOrderArguments,
    },
    OpenOrder {
        accounts: OpenOrderAccounts,
        args: OpenOrderArguments,
    },
    Swap {
        accounts: SwapAccounts,
        args: SwapArguments,
    },
    Swap2 {
        accounts: Swap2Accounts,
        args: Swap2Arguments,
    },
    Swap2WithDestination {
        accounts: Swap2WithDestinationAccounts,
        args: Swap2WithDestinationArguments,
    },
    Swap2WithDestinationNative {
        accounts: Swap2WithDestinationNativeAccounts,
        args: Swap2WithDestinationNativeArguments,
    },
    SwapWithDestination {
        accounts: SwapWithDestinationAccounts,
        args: SwapWithDestinationArguments,
    },
    SwapWithDestinationNative {
        accounts: SwapWithDestinationNativeAccounts,
        args: SwapWithDestinationNativeArguments,
    },
    TransferFee {
        accounts: TransferFeeAccounts,
        args: TransferFeeArguments,
    },
    TransferSol {
        accounts: TransferSolAccounts,
        args: TransferSolArguments,
    },
    TransferToSponsor {
        accounts: TransferToSponsorAccounts,
        args: TransferToSponsorArguments,
    },
    UnwrapSol {
        accounts: UnwrapSolAccounts,
        args: UnwrapSolArguments,
    },
    WrapSol {
        accounts: WrapSolAccounts,
        args: WrapSolArguments,
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
            [90u8, 103u8, 209u8, 28u8, 7u8, 63u8, 168u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseOrderArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let order = keys.next().unwrap().clone();
                let order_vault = keys.next().unwrap().clone();
                let return_input_token_account = keys.next().unwrap().clone();
                let return_rent_to = keys.next().unwrap().clone();
                let closer = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseOrderAccounts {
                    order,
                    order_vault,
                    return_input_token_account,
                    return_rent_to,
                    closer,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CloseOrder { accounts, args });
            }
            [46u8, 232u8, 41u8, 144u8, 85u8, 37u8, 170u8, 175u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateReferralTokenAccountIdempotentArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let project = keys.next().unwrap().clone();
                let referral_account = keys.next().unwrap().clone();
                let referral_token_account = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let referral_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateReferralTokenAccountIdempotentAccounts {
                    payer,
                    project,
                    referral_account,
                    referral_token_account,
                    mint,
                    system_program,
                    token_program,
                    referral_program,
                    remaining,
                };
                return Ok(Instruction::CreateReferralTokenAccountIdempotent { accounts, args });
            }
            [232u8, 122u8, 115u8, 25u8, 199u8, 143u8, 136u8, 162u8] => {
                let mut rdr: &[u8] = rest;
                let args = FillOrderArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let order = keys.next().unwrap().clone();
                let order_vault = keys.next().unwrap().clone();
                let output_token_account = keys.next().unwrap().clone();
                let return_input_token_account = keys.next().unwrap().clone();
                let return_rent_to = keys.next().unwrap().clone();
                let filler_input_token_account = keys.next().unwrap().clone();
                let input_mint = keys.next().unwrap().clone();
                let filler_output_token_account = keys.next().unwrap().clone();
                let output_mint = keys.next().unwrap().clone();
                let filler = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FillOrderAccounts {
                    order,
                    order_vault,
                    output_token_account,
                    return_input_token_account,
                    return_rent_to,
                    filler_input_token_account,
                    input_mint,
                    filler_output_token_account,
                    output_mint,
                    filler,
                    token_program,
                    associated_token_program,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::FillOrder { accounts, args });
            }
            [206u8, 88u8, 88u8, 143u8, 38u8, 136u8, 50u8, 224u8] => {
                let mut rdr: &[u8] = rest;
                let args = OpenOrderArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let order = keys.next().unwrap().clone();
                let order_vault = keys.next().unwrap().clone();
                let input_token_account = keys.next().unwrap().clone();
                let output_token_account = keys.next().unwrap().clone();
                let return_input_token_account = keys.next().unwrap().clone();
                let input_mint = keys.next().unwrap().clone();
                let user_token_authority = keys.next().unwrap().clone();
                let fee_payer = keys.next().unwrap().clone();
                let fee_receiver = keys.next().unwrap().clone();
                let rent_depositor = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = OpenOrderAccounts {
                    order,
                    order_vault,
                    input_token_account,
                    output_token_account,
                    return_input_token_account,
                    input_mint,
                    user_token_authority,
                    fee_payer,
                    fee_receiver,
                    rent_depositor,
                    token_program,
                    system_program,
                    rent,
                    remaining,
                };
                return Ok(Instruction::OpenOrder { accounts, args });
            }
            [248u8, 198u8, 158u8, 145u8, 225u8, 117u8, 135u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let user_token_authority = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapAccounts {
                    token_program,
                    associated_token_program,
                    system_program,
                    user_token_authority,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Swap { accounts, args });
            }
            [65u8, 75u8, 63u8, 76u8, 235u8, 91u8, 91u8, 136u8] => {
                let mut rdr: &[u8] = rest;
                let args = Swap2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let user_token_authority = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = Swap2Accounts {
                    token_program,
                    associated_token_program,
                    system_program,
                    user_token_authority,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Swap2 { accounts, args });
            }
            [95u8, 123u8, 213u8, 246u8, 122u8, 1u8, 86u8, 231u8] => {
                let mut rdr: &[u8] = rest;
                let args = Swap2WithDestinationArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let user_token_authority = keys.next().unwrap().clone();
                let destination_token_account = keys.next().unwrap().clone();
                let destination_token_authority = keys.next().unwrap().clone();
                let destination_mint = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = Swap2WithDestinationAccounts {
                    token_program,
                    associated_token_program,
                    system_program,
                    user_token_authority,
                    destination_token_account,
                    destination_token_authority,
                    destination_mint,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Swap2WithDestination { accounts, args });
            }
            [222u8, 100u8, 184u8, 146u8, 186u8, 196u8, 105u8, 165u8] => {
                let mut rdr: &[u8] = rest;
                let args = Swap2WithDestinationNativeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let user_token_authority = keys.next().unwrap().clone();
                let destination_account = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = Swap2WithDestinationNativeAccounts {
                    token_program,
                    associated_token_program,
                    system_program,
                    user_token_authority,
                    destination_account,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Swap2WithDestinationNative { accounts, args });
            }
            [168u8, 172u8, 24u8, 77u8, 197u8, 156u8, 135u8, 101u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapWithDestinationArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let user_token_authority = keys.next().unwrap().clone();
                let destination_token_account = keys.next().unwrap().clone();
                let destination_token_authority = keys.next().unwrap().clone();
                let destination_mint = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapWithDestinationAccounts {
                    token_program,
                    associated_token_program,
                    system_program,
                    user_token_authority,
                    destination_token_account,
                    destination_token_authority,
                    destination_mint,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SwapWithDestination { accounts, args });
            }
            [205u8, 77u8, 127u8, 108u8, 241u8, 32u8, 196u8, 195u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapWithDestinationNativeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let user_token_authority = keys.next().unwrap().clone();
                let destination_account = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapWithDestinationNativeAccounts {
                    token_program,
                    associated_token_program,
                    system_program,
                    user_token_authority,
                    destination_account,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SwapWithDestinationNative { accounts, args });
            }
            [129u8, 164u8, 196u8, 21u8, 177u8, 48u8, 180u8, 162u8] => {
                let mut rdr: &[u8] = rest;
                let args = TransferFeeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let from = keys.next().unwrap().clone();
                let to = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let event_authority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferFeeAccounts {
                    from,
                    to,
                    authority,
                    token_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::TransferFee { accounts, args });
            }
            [78u8, 10u8, 236u8, 247u8, 109u8, 117u8, 21u8, 76u8] => {
                let mut rdr: &[u8] = rest;
                let args = TransferSolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let from = keys.next().unwrap().clone();
                let to = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferSolAccounts {
                    from,
                    to,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::TransferSol { accounts, args });
            }
            [155u8, 179u8, 130u8, 151u8, 196u8, 139u8, 253u8, 163u8] => {
                let mut rdr: &[u8] = rest;
                let args = TransferToSponsorArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let user_token_authority = keys.next().unwrap().clone();
                let user_token_account = keys.next().unwrap().clone();
                let sponsor = keys.next().unwrap().clone();
                let sponsor_token_account = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferToSponsorAccounts {
                    user_token_authority,
                    user_token_account,
                    sponsor,
                    sponsor_token_account,
                    mint,
                    token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::TransferToSponsor { accounts, args });
            }
            [99u8, 40u8, 14u8, 105u8, 45u8, 107u8, 172u8, 201u8] => {
                let mut rdr: &[u8] = rest;
                let args = UnwrapSolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let owner = keys.next().unwrap().clone();
                let wrapped_sol_associated_token_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UnwrapSolAccounts {
                    owner,
                    wrapped_sol_associated_token_account,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::UnwrapSol { accounts, args });
            }
            [47u8, 62u8, 155u8, 172u8, 131u8, 205u8, 37u8, 201u8] => {
                let mut rdr: &[u8] = rest;
                let args = WrapSolArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let from = keys.next().unwrap().clone();
                let wrapped_sol_associated_token_account = keys.next().unwrap().clone();
                let native_mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WrapSolAccounts {
                    from,
                    wrapped_sol_associated_token_account,
                    native_mint,
                    token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::WrapSol { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FeeEvent {
        #[serde(with = "pubkey_serde")]
        pub account: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub mint: [u8; 32usize],
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapEvent {
        #[serde(with = "pubkey_serde")]
        pub amm: [u8; 32usize],
        #[serde(with = "pubkey_serde")]
        pub input_mint: [u8; 32usize],
        pub input_amount: u64,
        #[serde(with = "pubkey_serde")]
        pub output_mint: [u8; 32usize],
        pub output_amount: u64,
    }
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
                anyhow::bail!("Event log too short: {}", data.len());
            }
            let (wrapper, rest) = data.split_at(8);
            if wrapper != EVENT_LOG_DISCRIMINATOR {
                anyhow::bail!("Missing event log prefix");
            }
            let (disc_slice, payload) = rest.split_at(8);
            let disc: [u8; 8] = disc_slice.try_into().unwrap();
            match disc {
                [73u8, 79u8, 78u8, 127u8, 184u8, 213u8, 13u8, 220u8] => {
                    let mut rdr = &payload[..];
                    let args = FeeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::FeeEvent { args });
                }
                [64u8, 198u8, 205u8, 232u8, 38u8, 8u8, 113u8, 226u8] => {
                    let mut rdr = &payload[..];
                    let args = SwapEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SwapEvent { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
