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
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct AccessControl {
        pub bump: u8,
        pub merkle_root: [u8; 32],
        pub is_whitelist_enabled: bool,
        #[serde(with = "pubkey_serde")]
        pub vault_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub window_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub deposit_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub pair_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub unseal_authority: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub access_authority: Pubkey,
        pub sol_usdc_feed_id: [u8; 32],
        pub guardians: [Option<Pubkey>; 5],
        pub is_sealed: bool,
        #[serde(with = "pubkey_serde_option")]
        pub pending_vault_authority: Option<Pubkey>,
        #[serde(with = "pubkey_serde_option")]
        pub pending_window_authority: Option<Pubkey>,
        #[serde(with = "pubkey_serde_option")]
        pub pending_deposit_authority: Option<Pubkey>,
        #[serde(with = "pubkey_serde_option")]
        pub pending_pair_authority: Option<Pubkey>,
        #[serde(with = "pubkey_serde_option")]
        pub pending_unseal_authority: Option<Pubkey>,
        #[serde(with = "pubkey_serde_option")]
        pub pending_access_authority: Option<Pubkey>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum AuthorityType {
        Vault,
        Window,
        Deposit,
        Pair,
        Unseal,
        Access,
    }
    impl Default for AuthorityType {
        fn default() -> Self {
            Self::Vault
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct CreatePairParams {
        pub interval_apr_rate: u64,
        pub seconds_per_interval: i32,
        pub initial_exchange_rate: u64,
        pub deposit_cap: u64,
        pub stake_fee_bps: u16,
        pub swap_fee_bps: u16,
        pub withdraw_fee_bps: u16,
        pub minimum_deposit: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
    pub struct CreateTokenMetadataParams {
        pub name: String,
        pub uri: String,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct CreateWithdrawalWindowParams {
        pub start_time: i64,
        pub end_time: i64,
        pub earliest_withdrawal_time: i64,
        pub expiration_time: i64,
        pub max_withdrawal_amount: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum GuardianOperation {
        Add,
        Remove,
    }
    impl Default for GuardianOperation {
        fn default() -> Self {
            Self::Add
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
    pub struct Pair {
        pub pair_bump: u8,
        pub lst_mint_bump: u8,
        #[serde(with = "pubkey_serde")]
        pub base_token_mint: Pubkey,
        pub base_mint_decimals: u8,
        #[serde(with = "pubkey_serde")]
        pub lst_mint: Pubkey,
        pub lst_mint_decimals: u8,
        pub lst_symbol: String,
        pub interval_apr_rate: u64,
        pub seconds_per_interval: i32,
        pub initial_exchange_rate: u64,
        pub last_yield_change_exchange_rate: u64,
        pub last_yield_change_timestamp: i64,
        pub deposit_cap: u64,
        pub minimum_deposit: u64,
        pub stake_fee_bps: u16,
        pub swap_fee_bps: u16,
        pub withdraw_fee_bps: u16,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PriceFeedMessage {
        pub feed_id: [u8; 32],
        pub price: i64,
        pub conf: u64,
        pub exponent: i32,
        pub publish_time: i64,
        pub prev_publish_time: i64,
        pub ema_price: i64,
        pub ema_conf: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PriceUpdateV2 {
        #[serde(with = "pubkey_serde")]
        pub write_authority: Pubkey,
        pub verification_level: VerificationLevel,
        pub price_message: PriceFeedMessage,
        pub posted_slot: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct RequestWithdrawalEvent {
        pub withdrawal_fee: u64,
        pub input_amount: u64,
        pub output_amount: u64,
        pub sol_usdc_price: i64,
        pub price_exponent: i32,
        pub price_publish_time: i64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct StakeEvent {
        pub staking_fee: u64,
        pub input_amount: u64,
        pub output_amount: u64,
        pub sol_usdc_price: i64,
        pub price_exponent: i32,
        pub price_publish_time: i64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SwapEvent {
        pub source_fee_amount: u64,
        pub destination_fee_amount: u64,
        pub input_amount: u64,
        pub output_amount: u64,
        pub sol_usdc_price: i64,
        pub price_exponent: i32,
        pub price_publish_time: i64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UpdatePairLimitsParams {
        pub deposit_cap: Option<u64>,
        pub minimum_deposit: Option<u64>,
        pub stake_fee_bps: Option<u16>,
        pub swap_fee_bps: Option<u16>,
        pub withdraw_fee_bps: Option<u16>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct UpdatePairYieldParams {
        pub interval_apr_rate: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum VerificationLevel {
        Full,
        Partial { num_signatures: u8 },
    }
    impl Default for VerificationLevel {
        fn default() -> Self {
            Self::Full
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct WithdrawalRequest {
        #[serde(with = "pubkey_serde")]
        pub staker: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub window: Pubkey,
        pub lst_amount: u64,
        pub lst_fee_amount: u64,
        pub base_amount: u64,
        pub bump: u8,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct WithdrawalWindow {
        #[serde(with = "pubkey_serde")]
        pub pair: Pubkey,
        pub start_time: i64,
        pub end_time: i64,
        pub requested_withdrawal_amount: u64,
        pub total_lst_burned: u64,
        pub max_withdrawal_amount: u64,
        pub earliest_withdrawal_time: i64,
        pub expiration_time: i64,
        #[serde(with = "pubkey_serde")]
        pub base_token_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub base_token_account: Pubkey,
        pub is_funded: bool,
        pub bump: u8,
        pub withdrawn_amount: u64,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct AcceptAuthorityTransferAccounts {
        pub access_control: String,
        pub new_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CancelAuthorityTransferAccounts {
        pub access_control: String,
        pub current_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CancelWithdrawalRequestAccounts {
        pub pair: String,
        pub withdrawal_window: String,
        pub withdrawal_request: String,
        pub staker: String,
        pub staker_lst_account: String,
        pub window_lst_account: String,
        pub lst_mint: String,
        pub access_control: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseWithdrawalWindowAccounts {
        pub access_control: String,
        pub pair: String,
        pub pair_base_token_account: String,
        pub pair_lst_account: String,
        pub base_token_mint: String,
        pub lst_mint: String,
        pub withdrawal_window: String,
        pub window_base_token_account: String,
        pub window_lst_account: String,
        pub authority: String,
        pub associated_token_program: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateAccessControlAccounts {
        pub payer: String,
        pub access_control: String,
        pub admin: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreatePairAccounts {
        pub pair_authority: String,
        pub access_control: String,
        pub base_token_mint: String,
        pub pair: String,
        pub pair_base_token_account: String,
        pub lst_mint: String,
        pub lst_fee_account: String,
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateTokenMetadataAccounts {
        pub access_control: String,
        pub pair_authority: String,
        pub pair: String,
        pub lst_mint: String,
        pub metadata: String,
        pub token_metadata_program: String,
        pub token_program: String,
        pub rent: String,
        pub sysvar_instructions: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateWithdrawalWindowAccounts {
        pub pair: String,
        pub authority: String,
        pub access_control: String,
        pub withdrawal_window: String,
        pub base_token_mint: String,
        pub lst_mint: String,
        pub window_base_token_account: String,
        pub window_lst_account: String,
        pub token_program: String,
        pub associated_token_program: String,
        pub rent: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ExecuteWithdrawAccounts {
        pub access_control: String,
        pub pair: String,
        pub base_token_mint: String,
        pub lst_mint: String,
        pub withdrawal_window: String,
        pub window_base_token_account: String,
        pub window_lst_account: String,
        pub withdrawal_request: String,
        pub staker: String,
        pub staker_base_token_account: String,
        pub associated_token_program: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FundWithdrawalWindowAccounts {
        pub access_control: String,
        pub pair: String,
        pub withdrawal_window: String,
        pub deposit_authority: String,
        pub deposit_authority_base_token_account: String,
        pub window_base_token_account: String,
        pub base_token_mint: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitiateAuthorityTransferAccounts {
        pub access_control: String,
        pub current_authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ManageGuardianAccounts {
        pub unseal_authority: String,
        pub access_control: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RequestWithdrawAccounts {
        pub pair: String,
        pub withdrawal_window: String,
        pub withdrawal_request: String,
        pub staker: String,
        pub staker_lst_account: String,
        pub window_lst_account: String,
        pub lst_mint: String,
        pub access_control: String,
        pub price_feed: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RestakeExpiredWithdrawAccounts {
        pub access_control: String,
        pub pair: String,
        pub withdrawal_window: String,
        pub withdrawal_request: String,
        pub rent_receiver: String,
        pub window_authority: String,
        pub staker_lst_token_account: String,
        pub pair_base_token_account: String,
        pub window_base_token_account: String,
        pub window_lst_account: String,
        pub lst_mint: String,
        pub token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SealProgramAccounts {
        pub guardian: String,
        pub access_control: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct StakeAccounts {
        pub access_control: String,
        pub pair: String,
        pub base_token_mint: String,
        pub staker_base_token_account: String,
        pub pair_base_token_account: String,
        pub lst_fee_account: String,
        pub lst_mint: String,
        pub staker_lst_account: String,
        pub price_feed: String,
        pub staker: String,
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapLstAccounts {
        pub source_pair: String,
        pub source_lst_mint: String,
        pub user_source_lst_account: String,
        pub destination_pair: String,
        pub destination_lst_mint: String,
        pub user_destination_lst_account: String,
        pub source_lst_fee_account: String,
        pub destination_lst_fee_account: String,
        pub user: String,
        pub access_control: String,
        pub price_feed: String,
        pub token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UnsealProgramAccounts {
        pub unseal_authority: String,
        pub access_control: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePairLimitsAccounts {
        pub pair: String,
        pub authority: String,
        pub access_control: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePairYieldAccounts {
        pub pair: String,
        pub authority: String,
        pub access_control: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateSolUsdcFeedAccounts {
        pub access_control: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateWhitelistAccounts {
        pub access_control: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct VaultWithdrawAccounts {
        pub pair: String,
        pub authority: String,
        pub access_control: String,
        pub pair_base_token_account: String,
        pub authority_token_account: String,
        pub base_token_mint: String,
        pub associated_token_program: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawFeesAccounts {
        pub access_control: String,
        pub pair: String,
        pub lst_mint: String,
        pub fee_account: String,
        pub destination: String,
        pub destination_owner: String,
        pub vault_authority: String,
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
    pub struct AcceptAuthorityTransferArgs {
        pub authority_type: AuthorityType,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CancelAuthorityTransferArgs {
        pub authority_type: AuthorityType,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CancelWithdrawalRequestArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseWithdrawalWindowArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateAccessControlArgs {
        pub vault_authority: [u8; 32usize],
        pub window_authority: [u8; 32usize],
        pub deposit_authority: [u8; 32usize],
        pub pair_authority: [u8; 32usize],
        pub unseal_authority: [u8; 32usize],
        pub access_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreatePairArgs {
        pub symbol: String,
        pub params: CreatePairParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateTokenMetadataArgs {
        pub params: CreateTokenMetadataParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateWithdrawalWindowArgs {
        pub params: CreateWithdrawalWindowParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ExecuteWithdrawArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FundWithdrawalWindowArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitiateAuthorityTransferArgs {
        pub authority_type: AuthorityType,
        pub new_authority: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ManageGuardianArgs {
        pub guardian: [u8; 32usize],
        pub operation: GuardianOperation,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RequestWithdrawArgs {
        pub amount: u64,
        pub merkle_proof: Option<Vec<[u8; 32usize]>>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RestakeExpiredWithdrawArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SealProgramArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct StakeArgs {
        pub quantity: u64,
        pub merkle_proof: Option<Vec<[u8; 32usize]>>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapLstArgs {
        pub quantity: u64,
        pub merkle_proof: Option<Vec<[u8; 32usize]>>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnsealProgramArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePairLimitsArgs {
        pub params: UpdatePairLimitsParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePairYieldArgs {
        pub params: UpdatePairYieldParams,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateSolUsdcFeedArgs {
        pub new_feed_id: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateWhitelistArgs {
        pub merkle_root: Option<[u8; 32usize]>,
        pub enable_whitelist: Option<bool>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct VaultWithdrawArgs {
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawFeesArgs {
        pub amount: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    AcceptAuthorityTransfer {
        accounts: AcceptAuthorityTransferAccounts,
        args: AcceptAuthorityTransferArgs,
    },
    CancelAuthorityTransfer {
        accounts: CancelAuthorityTransferAccounts,
        args: CancelAuthorityTransferArgs,
    },
    CancelWithdrawalRequest {
        accounts: CancelWithdrawalRequestAccounts,
        args: CancelWithdrawalRequestArgs,
    },
    CloseWithdrawalWindow {
        accounts: CloseWithdrawalWindowAccounts,
        args: CloseWithdrawalWindowArgs,
    },
    CreateAccessControl {
        accounts: CreateAccessControlAccounts,
        args: CreateAccessControlArgs,
    },
    CreatePair {
        accounts: CreatePairAccounts,
        args: CreatePairArgs,
    },
    CreateTokenMetadata {
        accounts: CreateTokenMetadataAccounts,
        args: CreateTokenMetadataArgs,
    },
    CreateWithdrawalWindow {
        accounts: CreateWithdrawalWindowAccounts,
        args: CreateWithdrawalWindowArgs,
    },
    ExecuteWithdraw {
        accounts: ExecuteWithdrawAccounts,
        args: ExecuteWithdrawArgs,
    },
    FundWithdrawalWindow {
        accounts: FundWithdrawalWindowAccounts,
        args: FundWithdrawalWindowArgs,
    },
    InitiateAuthorityTransfer {
        accounts: InitiateAuthorityTransferAccounts,
        args: InitiateAuthorityTransferArgs,
    },
    ManageGuardian {
        accounts: ManageGuardianAccounts,
        args: ManageGuardianArgs,
    },
    RequestWithdraw {
        accounts: RequestWithdrawAccounts,
        args: RequestWithdrawArgs,
    },
    RestakeExpiredWithdraw {
        accounts: RestakeExpiredWithdrawAccounts,
        args: RestakeExpiredWithdrawArgs,
    },
    SealProgram {
        accounts: SealProgramAccounts,
        args: SealProgramArgs,
    },
    Stake {
        accounts: StakeAccounts,
        args: StakeArgs,
    },
    SwapLst {
        accounts: SwapLstAccounts,
        args: SwapLstArgs,
    },
    UnsealProgram {
        accounts: UnsealProgramAccounts,
        args: UnsealProgramArgs,
    },
    UpdatePairLimits {
        accounts: UpdatePairLimitsAccounts,
        args: UpdatePairLimitsArgs,
    },
    UpdatePairYield {
        accounts: UpdatePairYieldAccounts,
        args: UpdatePairYieldArgs,
    },
    UpdateSolUsdcFeed {
        accounts: UpdateSolUsdcFeedAccounts,
        args: UpdateSolUsdcFeedArgs,
    },
    UpdateWhitelist {
        accounts: UpdateWhitelistAccounts,
        args: UpdateWhitelistArgs,
    },
    VaultWithdraw {
        accounts: VaultWithdrawAccounts,
        args: VaultWithdrawArgs,
    },
    WithdrawFees {
        accounts: WithdrawFeesAccounts,
        args: WithdrawFeesArgs,
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
            [239u8, 248u8, 177u8, 2u8, 206u8, 97u8, 46u8, 255u8] => {
                let mut rdr: &[u8] = rest;
                let args = AcceptAuthorityTransferArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let new_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AcceptAuthorityTransferAccounts {
                    access_control,
                    new_authority,
                    remaining,
                };
                return Ok(Instruction::AcceptAuthorityTransfer { accounts, args });
            }
            [94u8, 131u8, 125u8, 184u8, 183u8, 24u8, 125u8, 229u8] => {
                let mut rdr: &[u8] = rest;
                let args = CancelAuthorityTransferArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let current_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CancelAuthorityTransferAccounts {
                    access_control,
                    current_authority,
                    remaining,
                };
                return Ok(Instruction::CancelAuthorityTransfer { accounts, args });
            }
            [82u8, 183u8, 63u8, 72u8, 51u8, 40u8, 167u8, 212u8] => {
                let mut rdr: &[u8] = rest;
                let args = CancelWithdrawalRequestArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pair = keys.next().unwrap().clone();
                let withdrawal_window = keys.next().unwrap().clone();
                let withdrawal_request = keys.next().unwrap().clone();
                let staker = keys.next().unwrap().clone();
                let staker_lst_account = keys.next().unwrap().clone();
                let window_lst_account = keys.next().unwrap().clone();
                let lst_mint = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CancelWithdrawalRequestAccounts {
                    pair,
                    withdrawal_window,
                    withdrawal_request,
                    staker,
                    staker_lst_account,
                    window_lst_account,
                    lst_mint,
                    access_control,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::CancelWithdrawalRequest { accounts, args });
            }
            [189u8, 128u8, 61u8, 161u8, 182u8, 218u8, 119u8, 53u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseWithdrawalWindowArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let pair = keys.next().unwrap().clone();
                let pair_base_token_account = keys.next().unwrap().clone();
                let pair_lst_account = keys.next().unwrap().clone();
                let base_token_mint = keys.next().unwrap().clone();
                let lst_mint = keys.next().unwrap().clone();
                let withdrawal_window = keys.next().unwrap().clone();
                let window_base_token_account = keys.next().unwrap().clone();
                let window_lst_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseWithdrawalWindowAccounts {
                    access_control,
                    pair,
                    pair_base_token_account,
                    pair_lst_account,
                    base_token_mint,
                    lst_mint,
                    withdrawal_window,
                    window_base_token_account,
                    window_lst_account,
                    authority,
                    associated_token_program,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CloseWithdrawalWindow { accounts, args });
            }
            [83u8, 239u8, 186u8, 78u8, 109u8, 109u8, 153u8, 216u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateAccessControlArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let admin = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateAccessControlAccounts {
                    payer,
                    access_control,
                    admin,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateAccessControl { accounts, args });
            }
            [156u8, 190u8, 126u8, 151u8, 163u8, 62u8, 192u8, 220u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreatePairArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pair_authority = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let base_token_mint = keys.next().unwrap().clone();
                let pair = keys.next().unwrap().clone();
                let pair_base_token_account = keys.next().unwrap().clone();
                let lst_mint = keys.next().unwrap().clone();
                let lst_fee_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreatePairAccounts {
                    pair_authority,
                    access_control,
                    base_token_mint,
                    pair,
                    pair_base_token_account,
                    lst_mint,
                    lst_fee_account,
                    token_program,
                    associated_token_program,
                    system_program,
                    rent,
                    remaining,
                };
                return Ok(Instruction::CreatePair { accounts, args });
            }
            [221u8, 80u8, 176u8, 37u8, 153u8, 188u8, 160u8, 68u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateTokenMetadataArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let pair_authority = keys.next().unwrap().clone();
                let pair = keys.next().unwrap().clone();
                let lst_mint = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let token_metadata_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let sysvar_instructions = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateTokenMetadataAccounts {
                    access_control,
                    pair_authority,
                    pair,
                    lst_mint,
                    metadata,
                    token_metadata_program,
                    token_program,
                    rent,
                    sysvar_instructions,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateTokenMetadata { accounts, args });
            }
            [207u8, 170u8, 78u8, 81u8, 85u8, 139u8, 221u8, 153u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateWithdrawalWindowArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pair = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let withdrawal_window = keys.next().unwrap().clone();
                let base_token_mint = keys.next().unwrap().clone();
                let lst_mint = keys.next().unwrap().clone();
                let window_base_token_account = keys.next().unwrap().clone();
                let window_lst_account = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateWithdrawalWindowAccounts {
                    pair,
                    authority,
                    access_control,
                    withdrawal_window,
                    base_token_mint,
                    lst_mint,
                    window_base_token_account,
                    window_lst_account,
                    token_program,
                    associated_token_program,
                    rent,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateWithdrawalWindow { accounts, args });
            }
            [255u8, 93u8, 15u8, 141u8, 187u8, 94u8, 246u8, 162u8] => {
                let mut rdr: &[u8] = rest;
                let args = ExecuteWithdrawArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let pair = keys.next().unwrap().clone();
                let base_token_mint = keys.next().unwrap().clone();
                let lst_mint = keys.next().unwrap().clone();
                let withdrawal_window = keys.next().unwrap().clone();
                let window_base_token_account = keys.next().unwrap().clone();
                let window_lst_account = keys.next().unwrap().clone();
                let withdrawal_request = keys.next().unwrap().clone();
                let staker = keys.next().unwrap().clone();
                let staker_base_token_account = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ExecuteWithdrawAccounts {
                    access_control,
                    pair,
                    base_token_mint,
                    lst_mint,
                    withdrawal_window,
                    window_base_token_account,
                    window_lst_account,
                    withdrawal_request,
                    staker,
                    staker_base_token_account,
                    associated_token_program,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::ExecuteWithdraw { accounts, args });
            }
            [231u8, 141u8, 11u8, 139u8, 32u8, 242u8, 97u8, 13u8] => {
                let mut rdr: &[u8] = rest;
                let args = FundWithdrawalWindowArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let pair = keys.next().unwrap().clone();
                let withdrawal_window = keys.next().unwrap().clone();
                let deposit_authority = keys.next().unwrap().clone();
                let deposit_authority_base_token_account = keys.next().unwrap().clone();
                let window_base_token_account = keys.next().unwrap().clone();
                let base_token_mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FundWithdrawalWindowAccounts {
                    access_control,
                    pair,
                    withdrawal_window,
                    deposit_authority,
                    deposit_authority_base_token_account,
                    window_base_token_account,
                    base_token_mint,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::FundWithdrawalWindow { accounts, args });
            }
            [210u8, 43u8, 101u8, 215u8, 119u8, 140u8, 106u8, 218u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitiateAuthorityTransferArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let current_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitiateAuthorityTransferAccounts {
                    access_control,
                    current_authority,
                    remaining,
                };
                return Ok(Instruction::InitiateAuthorityTransfer { accounts, args });
            }
            [229u8, 240u8, 143u8, 54u8, 137u8, 49u8, 201u8, 209u8] => {
                let mut rdr: &[u8] = rest;
                let args = ManageGuardianArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let unseal_authority = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ManageGuardianAccounts {
                    unseal_authority,
                    access_control,
                    remaining,
                };
                return Ok(Instruction::ManageGuardian { accounts, args });
            }
            [137u8, 95u8, 187u8, 96u8, 250u8, 138u8, 31u8, 182u8] => {
                let mut rdr: &[u8] = rest;
                let args = RequestWithdrawArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pair = keys.next().unwrap().clone();
                let withdrawal_window = keys.next().unwrap().clone();
                let withdrawal_request = keys.next().unwrap().clone();
                let staker = keys.next().unwrap().clone();
                let staker_lst_account = keys.next().unwrap().clone();
                let window_lst_account = keys.next().unwrap().clone();
                let lst_mint = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let price_feed = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RequestWithdrawAccounts {
                    pair,
                    withdrawal_window,
                    withdrawal_request,
                    staker,
                    staker_lst_account,
                    window_lst_account,
                    lst_mint,
                    access_control,
                    price_feed,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::RequestWithdraw { accounts, args });
            }
            [40u8, 200u8, 38u8, 89u8, 138u8, 33u8, 77u8, 91u8] => {
                let mut rdr: &[u8] = rest;
                let args = RestakeExpiredWithdrawArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let pair = keys.next().unwrap().clone();
                let withdrawal_window = keys.next().unwrap().clone();
                let withdrawal_request = keys.next().unwrap().clone();
                let rent_receiver = keys.next().unwrap().clone();
                let window_authority = keys.next().unwrap().clone();
                let staker_lst_token_account = keys.next().unwrap().clone();
                let pair_base_token_account = keys.next().unwrap().clone();
                let window_base_token_account = keys.next().unwrap().clone();
                let window_lst_account = keys.next().unwrap().clone();
                let lst_mint = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RestakeExpiredWithdrawAccounts {
                    access_control,
                    pair,
                    withdrawal_window,
                    withdrawal_request,
                    rent_receiver,
                    window_authority,
                    staker_lst_token_account,
                    pair_base_token_account,
                    window_base_token_account,
                    window_lst_account,
                    lst_mint,
                    token_program,
                    remaining,
                };
                return Ok(Instruction::RestakeExpiredWithdraw { accounts, args });
            }
            [3u8, 61u8, 188u8, 97u8, 28u8, 118u8, 223u8, 207u8] => {
                let mut rdr: &[u8] = rest;
                let args = SealProgramArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let guardian = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SealProgramAccounts {
                    guardian,
                    access_control,
                    remaining,
                };
                return Ok(Instruction::SealProgram { accounts, args });
            }
            [206u8, 176u8, 202u8, 18u8, 200u8, 209u8, 179u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let args = StakeArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let pair = keys.next().unwrap().clone();
                let base_token_mint = keys.next().unwrap().clone();
                let staker_base_token_account = keys.next().unwrap().clone();
                let pair_base_token_account = keys.next().unwrap().clone();
                let lst_fee_account = keys.next().unwrap().clone();
                let lst_mint = keys.next().unwrap().clone();
                let staker_lst_account = keys.next().unwrap().clone();
                let price_feed = keys.next().unwrap().clone();
                let staker = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = StakeAccounts {
                    access_control,
                    pair,
                    base_token_mint,
                    staker_base_token_account,
                    pair_base_token_account,
                    lst_fee_account,
                    lst_mint,
                    staker_lst_account,
                    price_feed,
                    staker,
                    token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::Stake { accounts, args });
            }
            [37u8, 144u8, 201u8, 101u8, 137u8, 182u8, 241u8, 42u8] => {
                let mut rdr: &[u8] = rest;
                let args = SwapLstArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let source_pair = keys.next().unwrap().clone();
                let source_lst_mint = keys.next().unwrap().clone();
                let user_source_lst_account = keys.next().unwrap().clone();
                let destination_pair = keys.next().unwrap().clone();
                let destination_lst_mint = keys.next().unwrap().clone();
                let user_destination_lst_account = keys.next().unwrap().clone();
                let source_lst_fee_account = keys.next().unwrap().clone();
                let destination_lst_fee_account = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let price_feed = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapLstAccounts {
                    source_pair,
                    source_lst_mint,
                    user_source_lst_account,
                    destination_pair,
                    destination_lst_mint,
                    user_destination_lst_account,
                    source_lst_fee_account,
                    destination_lst_fee_account,
                    user,
                    access_control,
                    price_feed,
                    token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::SwapLst { accounts, args });
            }
            [183u8, 82u8, 151u8, 155u8, 126u8, 84u8, 192u8, 135u8] => {
                let mut rdr: &[u8] = rest;
                let args = UnsealProgramArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let unseal_authority = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UnsealProgramAccounts {
                    unseal_authority,
                    access_control,
                    remaining,
                };
                return Ok(Instruction::UnsealProgram { accounts, args });
            }
            [229u8, 193u8, 200u8, 69u8, 199u8, 52u8, 189u8, 87u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePairLimitsArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pair = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePairLimitsAccounts {
                    pair,
                    authority,
                    access_control,
                    remaining,
                };
                return Ok(Instruction::UpdatePairLimits { accounts, args });
            }
            [106u8, 132u8, 147u8, 158u8, 54u8, 40u8, 218u8, 16u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePairYieldArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pair = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePairYieldAccounts {
                    pair,
                    authority,
                    access_control,
                    remaining,
                };
                return Ok(Instruction::UpdatePairYield { accounts, args });
            }
            [163u8, 230u8, 215u8, 217u8, 74u8, 137u8, 74u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateSolUsdcFeedArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateSolUsdcFeedAccounts {
                    access_control,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdateSolUsdcFeed { accounts, args });
            }
            [94u8, 198u8, 33u8, 20u8, 192u8, 97u8, 44u8, 59u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateWhitelistArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateWhitelistAccounts {
                    access_control,
                    authority,
                    remaining,
                };
                return Ok(Instruction::UpdateWhitelist { accounts, args });
            }
            [98u8, 28u8, 187u8, 98u8, 87u8, 69u8, 46u8, 64u8] => {
                let mut rdr: &[u8] = rest;
                let args = VaultWithdrawArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let pair = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let access_control = keys.next().unwrap().clone();
                let pair_base_token_account = keys.next().unwrap().clone();
                let authority_token_account = keys.next().unwrap().clone();
                let base_token_mint = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = VaultWithdrawAccounts {
                    pair,
                    authority,
                    access_control,
                    pair_base_token_account,
                    authority_token_account,
                    base_token_mint,
                    associated_token_program,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::VaultWithdraw { accounts, args });
            }
            [198u8, 212u8, 171u8, 109u8, 144u8, 215u8, 174u8, 89u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawFeesArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let access_control = keys.next().unwrap().clone();
                let pair = keys.next().unwrap().clone();
                let lst_mint = keys.next().unwrap().clone();
                let fee_account = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let destination_owner = keys.next().unwrap().clone();
                let vault_authority = keys.next().unwrap().clone();
                let token_program = keys.next().unwrap().clone();
                let associated_token_program = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawFeesAccounts {
                    access_control,
                    pair,
                    lst_mint,
                    fee_account,
                    destination,
                    destination_owner,
                    vault_authority,
                    token_program,
                    associated_token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::WithdrawFees { accounts, args });
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
        RequestWithdrawalEvent { args: RequestWithdrawalEvent },
        StakeEvent { args: StakeEvent },
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
                [206u8, 170u8, 10u8, 183u8, 182u8, 188u8, 115u8, 200u8] => {
                    let mut rdr = &data[16..];
                    let args = RequestWithdrawalEvent::deserialize(&mut rdr)?;
                    return Ok(Event::RequestWithdrawalEvent { args });
                }
                [226u8, 134u8, 188u8, 173u8, 19u8, 33u8, 75u8, 175u8] => {
                    let mut rdr = &data[16..];
                    let args = StakeEvent::deserialize(&mut rdr)?;
                    return Ok(Event::StakeEvent { args });
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
