use anyhow::Result;
use std::io::Read;
mod pubkey_serde {
    use bs58;
    use serde::Serializer;
    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = bs58::encode(bytes).into_string();
        serializer.serialize_str(&s)
    }
}
mod pubkey_serde_option {
    use bs58;
    use serde::Serializer;
    pub fn serialize<S>(bytes: &Option<[u8; 32]>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match bytes {
            Some(bytes) => {
                let s = bs58::encode(bytes).into_string();
                serializer.serialize_some(&s)
            }
            None => serializer.serialize_none(),
        }
    }
}
fn serialize_decryptable_balance<S>(bytes: &[u8; 36], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use base64::{engine::general_purpose, Engine as _};
    let encoded = general_purpose::STANDARD.encode(bytes);
    serializer.serialize_str(&encoded)
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeMintArguments {
    pub decimals: u8,
    #[serde(with = "pubkey_serde")]
    pub mint_authority: [u8; 32usize],
    #[serde(with = "pubkey_serde_option")]
    pub freeze_authority: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeAccountArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeMultisigArguments {
    pub m: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct TransferArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ApproveArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct RevokeArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct SetAuthorityArguments {
    pub authority_type: AuthorityType,
    #[serde(with = "pubkey_serde_option")]
    pub new_authority: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct MintToArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct BurnArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct CloseAccountArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct FreezeAccountArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ThawAccountArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct TransferCheckedArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
    pub decimals: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ApproveCheckedArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
    pub decimals: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct MintToCheckedArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
    pub decimals: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct BurnCheckedArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
    pub decimals: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeAccount2Arguments {
    #[serde(with = "pubkey_serde")]
    pub owner: [u8; 32usize],
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct SyncNativeArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeAccount3Arguments {
    #[serde(with = "pubkey_serde")]
    pub owner: [u8; 32usize],
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeMultisig2Arguments {
    pub m: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeMint2Arguments {
    pub decimals: u8,
    #[serde(with = "pubkey_serde")]
    pub mint_authority: [u8; 32usize],
    #[serde(with = "pubkey_serde_option")]
    pub freeze_authority: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct GetAccountDataSizeArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeImmutableOwnerArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct AmountToUiAmountArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UiAmountToAmountArguments {
    pub ui_amount: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeMintCloseAuthorityArguments {
    #[serde(with = "pubkey_serde_option")]
    pub close_authority: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeTransferFeeConfigArguments {
    #[serde(with = "pubkey_serde_option")]
    pub transfer_fee_config_authority: Option<[u8; 32usize]>,
    #[serde(with = "pubkey_serde_option")]
    pub withdraw_withheld_authority: Option<[u8; 32usize]>,
    pub transfer_fee_basis_points: u16,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub maximum_fee: u64,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct TransferCheckedWithFeeArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
    pub decimals: u8,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub fee: u64,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct WithdrawWithheldTokensFromMintArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct WithdrawWithheldTokensFromAccountsArguments {
    pub num_token_accounts: u8,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct HarvestWithheldTokensToMintArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct SetTransferFeeArguments {
    pub transfer_fee_basis_points: u16,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub maximum_fee: u64,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeConfidentialTransferMintArguments {
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    pub auto_approve_new_accounts: bool,
    #[serde(with = "pubkey_serde_option")]
    pub auditor_elgamal_pubkey: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateConfidentialTransferMintArguments {
    pub auto_approve_new_accounts: bool,
    #[serde(with = "pubkey_serde_option")]
    pub auditor_elgamal_pubkey: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ConfigureConfidentialTransferAccountArguments {
    #[serde(serialize_with = "serialize_decryptable_balance")]
    pub decryptable_zero_balance: [u8; 36],
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub maximum_pending_balance_credit_counter: u64,
    pub proof_instruction_offset: i8,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ApproveConfidentialTransferAccountArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EmptyConfidentialTransferAccountArguments {
    pub proof_instruction_offset: i8,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ConfidentialDepositArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
    pub decimals: u8,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ConfidentialWithdrawArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
    pub decimals: u8,
    #[serde(serialize_with = "serialize_decryptable_balance")]
    pub new_decryptable_available_balance: [u8; 36],
    pub equality_proof_instruction_offset: i8,
    pub range_proof_instruction_offset: i8,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ConfidentialTransferArguments {
    #[serde(serialize_with = "serialize_decryptable_balance")]
    pub new_source_decryptable_available_balance: [u8; 36],
    pub equality_proof_instruction_offset: i8,
    pub ciphertext_validity_proof_instruction_offset: i8,
    pub range_proof_instruction_offset: i8,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ApplyConfidentialPendingBalanceArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub expected_pending_balance_credit_counter: u64,
    #[serde(serialize_with = "serialize_decryptable_balance")]
    pub new_decryptable_available_balance: [u8; 36],
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EnableConfidentialCreditsArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct DisableConfidentialCreditsArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EnableNonConfidentialCreditsArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct DisableNonConfidentialCreditsArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ConfidentialTransferWithFeeArguments {
    #[serde(serialize_with = "serialize_decryptable_balance")]
    pub new_source_decryptable_available_balance: [u8; 36],
    pub equality_proof_instruction_offset: i8,
    pub transfer_amount_ciphertext_validity_proof_instruction_offset: i8,
    pub fee_sigma_proof_instruction_offset: i8,
    pub fee_ciphertext_validity_proof_instruction_offset: i8,
    pub range_proof_instruction_offset: i8,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeDefaultAccountStateArguments {
    pub state: AccountState,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateDefaultAccountStateArguments {
    pub state: AccountState,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ReallocateArguments {
    pub new_extension_types: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EnableMemoTransfersArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct DisableMemoTransfersArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct CreateNativeMintArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeNonTransferableMintArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeInterestBearingMintArguments {
    #[serde(with = "pubkey_serde_option")]
    pub rate_authority: Option<[u8; 32usize]>,
    pub rate: i16,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateRateInterestBearingMintArguments {
    pub rate: i16,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EnableCpiGuardArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct DisableCpiGuardArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializePermanentDelegateArguments {
    #[serde(with = "pubkey_serde")]
    pub delegate: [u8; 32usize],
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeTransferHookArguments {
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    #[serde(with = "pubkey_serde_option")]
    pub program_id: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateTransferHookArguments {
    #[serde(with = "pubkey_serde_option")]
    pub program_id: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeConfidentialTransferFeeArguments {
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    #[serde(with = "pubkey_serde_option")]
    pub withdraw_withheld_authority_el_gamal_pubkey: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct WithdrawWithheldTokensFromMintForConfidentialTransferFeeArguments {
    pub proof_instruction_offset: i8,
    #[serde(serialize_with = "serialize_decryptable_balance")]
    pub new_decryptable_available_balance: [u8; 36],
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeArguments {
    pub num_token_accounts: u8,
    pub proof_instruction_offset: i8,
    #[serde(serialize_with = "serialize_decryptable_balance")]
    pub new_decryptable_available_balance: [u8; 36],
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct HarvestWithheldTokensToMintForConfidentialTransferFeeArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EnableHarvestToMintArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct DisableHarvestToMintArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct WithdrawExcessLamportsArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeMetadataPointerArguments {
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    #[serde(with = "pubkey_serde_option")]
    pub metadata_address: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateMetadataPointerArguments {
    #[serde(with = "pubkey_serde_option")]
    pub metadata_address: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeGroupPointerArguments {
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    #[serde(with = "pubkey_serde_option")]
    pub group_address: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateGroupPointerArguments {
    #[serde(with = "pubkey_serde_option")]
    pub group_address: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeGroupMemberPointerArguments {
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    #[serde(with = "pubkey_serde_option")]
    pub member_address: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateGroupMemberPointerArguments {
    #[serde(with = "pubkey_serde_option")]
    pub member_address: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeScaledUiAmountMintArguments {
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    pub multiplier: u8,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateMultiplierScaledUiMintArguments {
    pub multiplier: u8,
    pub effective_timestamp: i64,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializePausableConfigArguments {
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct PauseArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ResumeArguments {
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeTokenMetadataArguments {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateTokenMetadataFieldArguments {
    pub field: TokenMetadataField,
    pub value: String,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct RemoveTokenMetadataKeyArguments {
    pub idempotent: bool,
    pub key: String,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateTokenMetadataUpdateAuthorityArguments {
    #[serde(with = "pubkey_serde_option")]
    pub new_update_authority: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EmitTokenMetadataArguments {
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeTokenGroupArguments {
    #[serde(with = "pubkey_serde_option")]
    pub update_authority: Option<[u8; 32usize]>,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub max_size: u64,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateTokenGroupMaxSizeArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub max_size: u64,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateTokenGroupUpdateAuthorityArguments {
    #[serde(with = "pubkey_serde_option")]
    pub new_update_authority: Option<[u8; 32usize]>,
    pub extension_type: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeTokenGroupMemberArguments {
    pub extension_type: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub rent: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeAccountAccounts {
    pub remaining: Vec<String>,
    pub account: String,
    pub mint: String,
    pub owner: String,
    pub rent: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeMultisigAccounts {
    pub remaining: Vec<String>,
    pub multisig: String,
    pub rent: String,
}
#[derive(Debug, Serialize)]
pub struct TransferAccounts {
    pub remaining: Vec<String>,
    pub source: String,
    pub destination: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct ApproveAccounts {
    pub remaining: Vec<String>,
    pub source: String,
    pub delegate: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct RevokeAccounts {
    pub remaining: Vec<String>,
    pub source: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct SetAuthorityAccounts {
    pub remaining: Vec<String>,
    pub owned: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct MintToAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub token: String,
    pub mint_authority: String,
}
#[derive(Debug, Serialize)]
pub struct BurnAccounts {
    pub remaining: Vec<String>,
    pub account: String,
    pub mint: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct CloseAccountAccounts {
    pub remaining: Vec<String>,
    pub account: String,
    pub destination: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct FreezeAccountAccounts {
    pub remaining: Vec<String>,
    pub account: String,
    pub mint: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct ThawAccountAccounts {
    pub remaining: Vec<String>,
    pub account: String,
    pub mint: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct TransferCheckedAccounts {
    pub remaining: Vec<String>,
    pub source: String,
    pub mint: String,
    pub destination: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct ApproveCheckedAccounts {
    pub remaining: Vec<String>,
    pub source: String,
    pub mint: String,
    pub delegate: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct MintToCheckedAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub token: String,
    pub mint_authority: String,
}
#[derive(Debug, Serialize)]
pub struct BurnCheckedAccounts {
    pub remaining: Vec<String>,
    pub account: String,
    pub mint: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeAccount2Accounts {
    pub remaining: Vec<String>,
    pub account: String,
    pub mint: String,
    pub rent: String,
}
#[derive(Debug, Serialize)]
pub struct SyncNativeAccounts {
    pub remaining: Vec<String>,
    pub account: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeAccount3Accounts {
    pub remaining: Vec<String>,
    pub account: String,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeMultisig2Accounts {
    pub remaining: Vec<String>,
    pub multisig: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeMint2Accounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct GetAccountDataSizeAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeImmutableOwnerAccounts {
    pub remaining: Vec<String>,
    pub account: String,
}
#[derive(Debug, Serialize)]
pub struct AmountToUiAmountAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct UiAmountToAmountAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeMintCloseAuthorityAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeTransferFeeConfigAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct TransferCheckedWithFeeAccounts {
    pub remaining: Vec<String>,
    pub source: String,
    pub mint: String,
    pub destination: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct WithdrawWithheldTokensFromMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub fee_receiver: String,
    pub withdraw_withheld_authority: String,
}
#[derive(Debug, Serialize)]
pub struct WithdrawWithheldTokensFromAccountsAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub fee_receiver: String,
    pub withdraw_withheld_authority: String,
}
#[derive(Debug, Serialize)]
pub struct HarvestWithheldTokensToMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct SetTransferFeeAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub transfer_fee_config_authority: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeConfidentialTransferMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateConfidentialTransferMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct ConfigureConfidentialTransferAccountAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub mint: String,
    pub instructions_sysvar_or_context_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct ApproveConfidentialTransferAccountAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub mint: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct EmptyConfidentialTransferAccountAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub instructions_sysvar_or_context_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct ConfidentialDepositAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub mint: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct ConfidentialWithdrawAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub mint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_sysvar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equality_record: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_record: Option<String>,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct ConfidentialTransferAccounts {
    pub remaining: Vec<String>,
    pub source_token: String,
    pub mint: String,
    pub destination_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_sysvar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equality_record: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_validity_record: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_record: Option<String>,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct ApplyConfidentialPendingBalanceAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct EnableConfidentialCreditsAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct DisableConfidentialCreditsAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct EnableNonConfidentialCreditsAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct DisableNonConfidentialCreditsAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct ConfidentialTransferWithFeeAccounts {
    pub remaining: Vec<String>,
    pub source_token: String,
    pub mint: String,
    pub destination_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_sysvar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equality_record: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_amount_ciphertext_validity_record: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_sigma_record: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_ciphertext_validity_record: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_record: Option<String>,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeDefaultAccountStateAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateDefaultAccountStateAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub freeze_authority: String,
}
#[derive(Debug, Serialize)]
pub struct ReallocateAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub payer: String,
    pub system_program: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct EnableMemoTransfersAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct DisableMemoTransfersAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct CreateNativeMintAccounts {
    pub remaining: Vec<String>,
    pub payer: String,
    pub native_mint: String,
    pub system_program: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeNonTransferableMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeInterestBearingMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateRateInterestBearingMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub rate_authority: String,
}
#[derive(Debug, Serialize)]
pub struct EnableCpiGuardAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct DisableCpiGuardAccounts {
    pub remaining: Vec<String>,
    pub token: String,
    pub owner: String,
}
#[derive(Debug, Serialize)]
pub struct InitializePermanentDelegateAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeTransferHookAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateTransferHookAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeConfidentialTransferFeeAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct WithdrawWithheldTokensFromMintForConfidentialTransferFeeAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub destination: String,
    pub instructions_sysvar_or_context_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub destination: String,
    pub instructions_sysvar_or_context_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct HarvestWithheldTokensToMintForConfidentialTransferFeeAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct EnableHarvestToMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct DisableHarvestToMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct WithdrawExcessLamportsAccounts {
    pub remaining: Vec<String>,
    pub source_account: String,
    pub destination_account: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeMetadataPointerAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateMetadataPointerAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub metadata_pointer_authority: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeGroupPointerAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateGroupPointerAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub group_pointer_authority: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeGroupMemberPointerAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateGroupMemberPointerAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub group_member_pointer_authority: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeScaledUiAmountMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateMultiplierScaledUiMintAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct InitializePausableConfigAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
}
#[derive(Debug, Serialize)]
pub struct PauseAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct ResumeAccounts {
    pub remaining: Vec<String>,
    pub mint: String,
    pub authority: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeTokenMetadataAccounts {
    pub remaining: Vec<String>,
    pub metadata: String,
    pub update_authority: String,
    pub mint: String,
    pub mint_authority: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateTokenMetadataFieldAccounts {
    pub remaining: Vec<String>,
    pub metadata: String,
    pub update_authority: String,
}
#[derive(Debug, Serialize)]
pub struct RemoveTokenMetadataKeyAccounts {
    pub remaining: Vec<String>,
    pub metadata: String,
    pub update_authority: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateTokenMetadataUpdateAuthorityAccounts {
    pub remaining: Vec<String>,
    pub metadata: String,
    pub update_authority: String,
}
#[derive(Debug, Serialize)]
pub struct EmitTokenMetadataAccounts {
    pub remaining: Vec<String>,
    pub metadata: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeTokenGroupAccounts {
    pub remaining: Vec<String>,
    pub group: String,
    pub mint: String,
    pub mint_authority: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateTokenGroupMaxSizeAccounts {
    pub remaining: Vec<String>,
    pub group: String,
    pub update_authority: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateTokenGroupUpdateAuthorityAccounts {
    pub remaining: Vec<String>,
    pub group: String,
    pub update_authority: String,
}
#[derive(Debug, Serialize)]
pub struct InitializeTokenGroupMemberAccounts {
    pub remaining: Vec<String>,
    pub member: String,
    pub member_mint: String,
    pub member_mint_authority: String,
    pub group: String,
    pub group_update_authority: String,
}
#[derive(Debug, Serialize)]
pub enum TokenMetadataField {
    Name,
    Symbol,
    Uri,
    Key(String),
}
impl ::borsh::BorshDeserialize for TokenMetadataField {
    fn deserialize(buf: &mut &[u8]) -> ::borsh::maybestd::io::Result<Self> {
        let field_discriminator: u8 = ::borsh::BorshDeserialize::deserialize(buf)?;
        match field_discriminator {
            0 => Ok(TokenMetadataField::Name),
            1 => Ok(TokenMetadataField::Symbol),
            2 => Ok(TokenMetadataField::Uri),
            3 => {
                let key_len: u32 = ::borsh::BorshDeserialize::deserialize(buf)?;
                let mut key_bytes = vec![0u8; key_len as usize];
                use std::io::Read;
                buf.read_exact(&mut key_bytes)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                let key_string = String::from_utf8(key_bytes)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                Ok(TokenMetadataField::Key(key_string))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Unknown TokenMetadataField discriminator: {}",
                    field_discriminator
                ),
            )),
        }
    }
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> ::borsh::maybestd::io::Result<Self> {
        let field_discriminator: u8 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;
        match field_discriminator {
            0 => Ok(TokenMetadataField::Name),
            1 => Ok(TokenMetadataField::Symbol),
            2 => Ok(TokenMetadataField::Uri),
            3 => {
                let key_len: u32 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;
                let mut key_bytes = vec![0u8; key_len as usize];
                reader.read_exact(&mut key_bytes)?;
                let key_string = String::from_utf8(key_bytes)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                Ok(TokenMetadataField::Key(key_string))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Unknown TokenMetadataField discriminator: {}",
                    field_discriminator
                ),
            )),
        }
    }
}
#[derive(Debug, Serialize, :: borsh :: BorshDeserialize)]
pub struct AuthorityType(pub u8);
#[derive(Debug, Serialize, :: borsh :: BorshDeserialize)]
pub struct DecryptableBalance(pub Vec<u8>);
#[derive(Debug, Serialize, :: borsh :: BorshDeserialize)]
pub struct AccountState(pub u8);
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitializeMint {
        accounts: InitializeMintAccounts,
        args: InitializeMintArguments,
    },
    InitializeAccount {
        accounts: InitializeAccountAccounts,
        args: InitializeAccountArguments,
    },
    InitializeMultisig {
        accounts: InitializeMultisigAccounts,
        args: InitializeMultisigArguments,
    },
    Transfer {
        accounts: TransferAccounts,
        args: TransferArguments,
    },
    Approve {
        accounts: ApproveAccounts,
        args: ApproveArguments,
    },
    Revoke {
        accounts: RevokeAccounts,
        args: RevokeArguments,
    },
    SetAuthority {
        accounts: SetAuthorityAccounts,
        args: SetAuthorityArguments,
    },
    MintTo {
        accounts: MintToAccounts,
        args: MintToArguments,
    },
    Burn {
        accounts: BurnAccounts,
        args: BurnArguments,
    },
    CloseAccount {
        accounts: CloseAccountAccounts,
        args: CloseAccountArguments,
    },
    FreezeAccount {
        accounts: FreezeAccountAccounts,
        args: FreezeAccountArguments,
    },
    ThawAccount {
        accounts: ThawAccountAccounts,
        args: ThawAccountArguments,
    },
    TransferChecked {
        accounts: TransferCheckedAccounts,
        args: TransferCheckedArguments,
    },
    ApproveChecked {
        accounts: ApproveCheckedAccounts,
        args: ApproveCheckedArguments,
    },
    MintToChecked {
        accounts: MintToCheckedAccounts,
        args: MintToCheckedArguments,
    },
    BurnChecked {
        accounts: BurnCheckedAccounts,
        args: BurnCheckedArguments,
    },
    InitializeAccount2 {
        accounts: InitializeAccount2Accounts,
        args: InitializeAccount2Arguments,
    },
    SyncNative {
        accounts: SyncNativeAccounts,
        args: SyncNativeArguments,
    },
    InitializeAccount3 {
        accounts: InitializeAccount3Accounts,
        args: InitializeAccount3Arguments,
    },
    InitializeMultisig2 {
        accounts: InitializeMultisig2Accounts,
        args: InitializeMultisig2Arguments,
    },
    InitializeMint2 {
        accounts: InitializeMint2Accounts,
        args: InitializeMint2Arguments,
    },
    GetAccountDataSize {
        accounts: GetAccountDataSizeAccounts,
        args: GetAccountDataSizeArguments,
    },
    InitializeImmutableOwner {
        accounts: InitializeImmutableOwnerAccounts,
        args: InitializeImmutableOwnerArguments,
    },
    AmountToUiAmount {
        accounts: AmountToUiAmountAccounts,
        args: AmountToUiAmountArguments,
    },
    UiAmountToAmount {
        accounts: UiAmountToAmountAccounts,
        args: UiAmountToAmountArguments,
    },
    InitializeMintCloseAuthority {
        accounts: InitializeMintCloseAuthorityAccounts,
        args: InitializeMintCloseAuthorityArguments,
    },
    #[serde(rename = "TransferFee")]
    InitializeTransferFeeConfig {
        accounts: InitializeTransferFeeConfigAccounts,
        args: InitializeTransferFeeConfigArguments,
    },
    #[serde(rename = "TransferFee")]
    TransferCheckedWithFee {
        accounts: TransferCheckedWithFeeAccounts,
        args: TransferCheckedWithFeeArguments,
    },
    #[serde(rename = "TransferFee")]
    WithdrawWithheldTokensFromMint {
        accounts: WithdrawWithheldTokensFromMintAccounts,
        args: WithdrawWithheldTokensFromMintArguments,
    },
    #[serde(rename = "TransferFee")]
    WithdrawWithheldTokensFromAccounts {
        accounts: WithdrawWithheldTokensFromAccountsAccounts,
        args: WithdrawWithheldTokensFromAccountsArguments,
    },
    #[serde(rename = "TransferFee")]
    HarvestWithheldTokensToMint {
        accounts: HarvestWithheldTokensToMintAccounts,
        args: HarvestWithheldTokensToMintArguments,
    },
    #[serde(rename = "TransferFee")]
    SetTransferFee {
        accounts: SetTransferFeeAccounts,
        args: SetTransferFeeArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    InitializeConfidentialTransferMint {
        accounts: InitializeConfidentialTransferMintAccounts,
        args: InitializeConfidentialTransferMintArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    UpdateConfidentialTransferMint {
        accounts: UpdateConfidentialTransferMintAccounts,
        args: UpdateConfidentialTransferMintArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    ConfigureConfidentialTransferAccount {
        accounts: ConfigureConfidentialTransferAccountAccounts,
        args: ConfigureConfidentialTransferAccountArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    ApproveConfidentialTransferAccount {
        accounts: ApproveConfidentialTransferAccountAccounts,
        args: ApproveConfidentialTransferAccountArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    EmptyConfidentialTransferAccount {
        accounts: EmptyConfidentialTransferAccountAccounts,
        args: EmptyConfidentialTransferAccountArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    ConfidentialDeposit {
        accounts: ConfidentialDepositAccounts,
        args: ConfidentialDepositArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    ConfidentialWithdraw {
        accounts: ConfidentialWithdrawAccounts,
        args: ConfidentialWithdrawArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    ConfidentialTransfer {
        accounts: ConfidentialTransferAccounts,
        args: ConfidentialTransferArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    ApplyConfidentialPendingBalance {
        accounts: ApplyConfidentialPendingBalanceAccounts,
        args: ApplyConfidentialPendingBalanceArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    EnableConfidentialCredits {
        accounts: EnableConfidentialCreditsAccounts,
        args: EnableConfidentialCreditsArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    DisableConfidentialCredits {
        accounts: DisableConfidentialCreditsAccounts,
        args: DisableConfidentialCreditsArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    EnableNonConfidentialCredits {
        accounts: EnableNonConfidentialCreditsAccounts,
        args: EnableNonConfidentialCreditsArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    DisableNonConfidentialCredits {
        accounts: DisableNonConfidentialCreditsAccounts,
        args: DisableNonConfidentialCreditsArguments,
    },
    #[serde(rename = "ConfidentialTransfer")]
    ConfidentialTransferWithFee {
        accounts: ConfidentialTransferWithFeeAccounts,
        args: ConfidentialTransferWithFeeArguments,
    },
    #[serde(rename = "DefaultAccountState")]
    InitializeDefaultAccountState {
        accounts: InitializeDefaultAccountStateAccounts,
        args: InitializeDefaultAccountStateArguments,
    },
    #[serde(rename = "DefaultAccountState")]
    UpdateDefaultAccountState {
        accounts: UpdateDefaultAccountStateAccounts,
        args: UpdateDefaultAccountStateArguments,
    },
    Reallocate {
        accounts: ReallocateAccounts,
        args: ReallocateArguments,
    },
    #[serde(rename = "MemoTransfers")]
    EnableMemoTransfers {
        accounts: EnableMemoTransfersAccounts,
        args: EnableMemoTransfersArguments,
    },
    #[serde(rename = "MemoTransfers")]
    DisableMemoTransfers {
        accounts: DisableMemoTransfersAccounts,
        args: DisableMemoTransfersArguments,
    },
    CreateNativeMint {
        accounts: CreateNativeMintAccounts,
        args: CreateNativeMintArguments,
    },
    InitializeNonTransferableMint {
        accounts: InitializeNonTransferableMintAccounts,
        args: InitializeNonTransferableMintArguments,
    },
    #[serde(rename = "InterestBearingMint")]
    InitializeInterestBearingMint {
        accounts: InitializeInterestBearingMintAccounts,
        args: InitializeInterestBearingMintArguments,
    },
    #[serde(rename = "InterestBearingMint")]
    UpdateRateInterestBearingMint {
        accounts: UpdateRateInterestBearingMintAccounts,
        args: UpdateRateInterestBearingMintArguments,
    },
    #[serde(rename = "CpiGuard")]
    EnableCpiGuard {
        accounts: EnableCpiGuardAccounts,
        args: EnableCpiGuardArguments,
    },
    #[serde(rename = "CpiGuard")]
    DisableCpiGuard {
        accounts: DisableCpiGuardAccounts,
        args: DisableCpiGuardArguments,
    },
    InitializePermanentDelegate {
        accounts: InitializePermanentDelegateAccounts,
        args: InitializePermanentDelegateArguments,
    },
    #[serde(rename = "TransferHook")]
    InitializeTransferHook {
        accounts: InitializeTransferHookAccounts,
        args: InitializeTransferHookArguments,
    },
    #[serde(rename = "TransferHook")]
    UpdateTransferHook {
        accounts: UpdateTransferHookAccounts,
        args: UpdateTransferHookArguments,
    },
    #[serde(rename = "ConfidentialTransferFee")]
    InitializeConfidentialTransferFee {
        accounts: InitializeConfidentialTransferFeeAccounts,
        args: InitializeConfidentialTransferFeeArguments,
    },
    #[serde(rename = "ConfidentialTransferFee")]
    WithdrawWithheldTokensFromMintForConfidentialTransferFee {
        accounts: WithdrawWithheldTokensFromMintForConfidentialTransferFeeAccounts,
        args: WithdrawWithheldTokensFromMintForConfidentialTransferFeeArguments,
    },
    #[serde(rename = "ConfidentialTransferFee")]
    WithdrawWithheldTokensFromAccountsForConfidentialTransferFee {
        accounts: WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeAccounts,
        args: WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeArguments,
    },
    #[serde(rename = "ConfidentialTransferFee")]
    HarvestWithheldTokensToMintForConfidentialTransferFee {
        accounts: HarvestWithheldTokensToMintForConfidentialTransferFeeAccounts,
        args: HarvestWithheldTokensToMintForConfidentialTransferFeeArguments,
    },
    #[serde(rename = "ConfidentialTransferFee")]
    EnableHarvestToMint {
        accounts: EnableHarvestToMintAccounts,
        args: EnableHarvestToMintArguments,
    },
    #[serde(rename = "ConfidentialTransferFee")]
    DisableHarvestToMint {
        accounts: DisableHarvestToMintAccounts,
        args: DisableHarvestToMintArguments,
    },
    WithdrawExcessLamports {
        accounts: WithdrawExcessLamportsAccounts,
        args: WithdrawExcessLamportsArguments,
    },
    #[serde(rename = "MetadataPointer")]
    InitializeMetadataPointer {
        accounts: InitializeMetadataPointerAccounts,
        args: InitializeMetadataPointerArguments,
    },
    #[serde(rename = "MetadataPointer")]
    UpdateMetadataPointer {
        accounts: UpdateMetadataPointerAccounts,
        args: UpdateMetadataPointerArguments,
    },
    #[serde(rename = "GroupPointer")]
    InitializeGroupPointer {
        accounts: InitializeGroupPointerAccounts,
        args: InitializeGroupPointerArguments,
    },
    #[serde(rename = "GroupPointer")]
    UpdateGroupPointer {
        accounts: UpdateGroupPointerAccounts,
        args: UpdateGroupPointerArguments,
    },
    #[serde(rename = "GroupMemberPointer")]
    InitializeGroupMemberPointer {
        accounts: InitializeGroupMemberPointerAccounts,
        args: InitializeGroupMemberPointerArguments,
    },
    #[serde(rename = "GroupMemberPointer")]
    UpdateGroupMemberPointer {
        accounts: UpdateGroupMemberPointerAccounts,
        args: UpdateGroupMemberPointerArguments,
    },
    #[serde(rename = "ScaledUiAmountMint")]
    InitializeScaledUiAmountMint {
        accounts: InitializeScaledUiAmountMintAccounts,
        args: InitializeScaledUiAmountMintArguments,
    },
    #[serde(rename = "ScaledUiAmountMint")]
    UpdateMultiplierScaledUiMint {
        accounts: UpdateMultiplierScaledUiMintAccounts,
        args: UpdateMultiplierScaledUiMintArguments,
    },
    #[serde(rename = "Pausable")]
    InitializePausableConfig {
        accounts: InitializePausableConfigAccounts,
        args: InitializePausableConfigArguments,
    },
    #[serde(rename = "Pausable")]
    Pause {
        accounts: PauseAccounts,
        args: PauseArguments,
    },
    #[serde(rename = "Pausable")]
    Resume {
        accounts: ResumeAccounts,
        args: ResumeArguments,
    },
    InitializeTokenMetadata {
        accounts: InitializeTokenMetadataAccounts,
        args: InitializeTokenMetadataArguments,
    },
    UpdateTokenMetadataField {
        accounts: UpdateTokenMetadataFieldAccounts,
        args: UpdateTokenMetadataFieldArguments,
    },
    RemoveTokenMetadataKey {
        accounts: RemoveTokenMetadataKeyAccounts,
        args: RemoveTokenMetadataKeyArguments,
    },
    UpdateTokenMetadataUpdateAuthority {
        accounts: UpdateTokenMetadataUpdateAuthorityAccounts,
        args: UpdateTokenMetadataUpdateAuthorityArguments,
    },
    EmitTokenMetadata {
        accounts: EmitTokenMetadataAccounts,
        args: EmitTokenMetadataArguments,
    },
    InitializeTokenGroup {
        accounts: InitializeTokenGroupAccounts,
        args: InitializeTokenGroupArguments,
    },
    UpdateTokenGroupMaxSize {
        accounts: UpdateTokenGroupMaxSizeAccounts,
        args: UpdateTokenGroupMaxSizeArguments,
    },
    UpdateTokenGroupUpdateAuthority {
        accounts: UpdateTokenGroupUpdateAuthorityAccounts,
        args: UpdateTokenGroupUpdateAuthorityArguments,
    },
    InitializeTokenGroupMember {
        accounts: InitializeTokenGroupMemberAccounts,
        args: InitializeTokenGroupMemberArguments,
    },
}
impl Instruction {
    pub fn decode(data: &[u8], account_keys: &[String]) -> Result<Self, String> {
        if data.is_empty() {
            return Err("Empty instruction data".to_string());
        }
        if data.len() >= 8 {
            let (discriminator, rest) = data.split_at(8);
            match discriminator {
                [0u8] => {
                    let mut rdr: &[u8] = rest;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let mint_authority: [u8; 32usize] =
                        <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(mint_authority),
                                    e
                                )
                            })?;
                    let freeze_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(freeze_authority),
                                    e
                                )
                            })?;
                    let args = InitializeMintArguments {
                        decimals,
                        mint_authority,
                        freeze_authority,
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let rent = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        rent: rent.clone(),
                    };
                    return Ok(Instruction::InitializeMint { accounts, args });
                }
                [1u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeAccountArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let rent = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeAccountAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        owner: owner.clone(),
                        rent: rent.clone(),
                    };
                    return Ok(Instruction::InitializeAccount { accounts, args });
                }
                [2u8] => {
                    let mut rdr: &[u8] = rest;
                    let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                    let args = InitializeMultisigArguments { m };
                    let multisig = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "multisig" , "multisig" , account_index , provided_count) }) ?
                    };
                    let rent = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMultisigAccounts {
                        remaining: vec![],
                        multisig: multisig.clone(),
                        rent: rent.clone(),
                    };
                    return Ok(Instruction::InitializeMultisig { accounts, args });
                }
                [3u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = TransferArguments { amount };
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = TransferAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        destination: destination.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::Transfer { accounts, args });
                }
                [4u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = ApproveArguments { amount };
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let delegate = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "delegate" , "delegate" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = ApproveAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        delegate: delegate.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::Approve { accounts, args });
                }
                [5u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = RevokeArguments {};
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = RevokeAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::Revoke { accounts, args });
                }
                [6u8] => {
                    let mut rdr: &[u8] = rest;
                    let authority_type: AuthorityType =
                        <AuthorityType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(authority_type),
                                    e
                                )
                            })?;
                    let new_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(new_authority),
                                    e
                                )
                            })?;
                    let args = SetAuthorityArguments {
                        authority_type,
                        new_authority,
                    };
                    let owned = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owned" , "owned" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = SetAuthorityAccounts {
                        remaining: vec![],
                        owned: owned.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::SetAuthority { accounts, args });
                }
                [7u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = MintToArguments { amount };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = MintToAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        token: token.clone(),
                        mint_authority: mint_authority.clone(),
                    };
                    return Ok(Instruction::MintTo { accounts, args });
                }
                [8u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = BurnArguments { amount };
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = BurnAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::Burn { accounts, args });
                }
                [9u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = CloseAccountArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = CloseAccountAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        destination: destination.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::CloseAccount { accounts, args });
                }
                [10u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = FreezeAccountArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = FreezeAccountAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::FreezeAccount { accounts, args });
                }
                [11u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = ThawAccountArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = ThawAccountAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::ThawAccount { accounts, args });
                }
                [12u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = TransferCheckedArguments { amount, decimals };
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = TransferCheckedAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        mint: mint.clone(),
                        destination: destination.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::TransferChecked { accounts, args });
                }
                [13u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = ApproveCheckedArguments { amount, decimals };
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let delegate = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "delegate" , "delegate" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = ApproveCheckedAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        mint: mint.clone(),
                        delegate: delegate.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::ApproveChecked { accounts, args });
                }
                [14u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = MintToCheckedArguments { amount, decimals };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = MintToCheckedAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        token: token.clone(),
                        mint_authority: mint_authority.clone(),
                    };
                    return Ok(Instruction::MintToChecked { accounts, args });
                }
                [15u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = BurnCheckedArguments { amount, decimals };
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = BurnCheckedAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::BurnChecked { accounts, args });
                }
                [16u8] => {
                    let mut rdr: &[u8] = rest;
                    let owner: [u8; 32usize] =
                        <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(owner), e)
                            })?;
                    let args = InitializeAccount2Arguments { owner };
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let rent = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeAccount2Accounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        rent: rent.clone(),
                    };
                    return Ok(Instruction::InitializeAccount2 { accounts, args });
                }
                [17u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = SyncNativeArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let accounts = SyncNativeAccounts {
                        remaining: vec![],
                        account: account.clone(),
                    };
                    return Ok(Instruction::SyncNative { accounts, args });
                }
                [18u8] => {
                    let mut rdr: &[u8] = rest;
                    let owner: [u8; 32usize] =
                        <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(owner), e)
                            })?;
                    let args = InitializeAccount3Arguments { owner };
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeAccount3Accounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeAccount3 { accounts, args });
                }
                [19u8] => {
                    let mut rdr: &[u8] = rest;
                    let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                    let args = InitializeMultisig2Arguments { m };
                    let multisig = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "multisig" , "multisig" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMultisig2Accounts {
                        remaining: vec![],
                        multisig: multisig.clone(),
                    };
                    return Ok(Instruction::InitializeMultisig2 { accounts, args });
                }
                [20u8] => {
                    let mut rdr: &[u8] = rest;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let mint_authority: [u8; 32usize] =
                        <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(mint_authority),
                                    e
                                )
                            })?;
                    let freeze_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(freeze_authority),
                                    e
                                )
                            })?;
                    let args = InitializeMint2Arguments {
                        decimals,
                        mint_authority,
                        freeze_authority,
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMint2Accounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeMint2 { accounts, args });
                }
                [21u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = GetAccountDataSizeArguments {};
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = GetAccountDataSizeAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::GetAccountDataSize { accounts, args });
                }
                [22u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeImmutableOwnerArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeImmutableOwnerAccounts {
                        remaining: vec![],
                        account: account.clone(),
                    };
                    return Ok(Instruction::InitializeImmutableOwner { accounts, args });
                }
                [23u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = AmountToUiAmountArguments { amount };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = AmountToUiAmountAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::AmountToUiAmount { accounts, args });
                }
                [24u8] => {
                    let mut rdr: &[u8] = rest;
                    let ui_amount: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(ui_amount), e)
                        })?;
                    let args = UiAmountToAmountArguments { ui_amount };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = UiAmountToAmountAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::UiAmountToAmount { accounts, args });
                }
                [25u8] => {
                    let mut rdr: &[u8] = rest;
                    let close_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(close_authority),
                                    e
                                )
                            })?;
                    let args = InitializeMintCloseAuthorityArguments { close_authority };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMintCloseAuthorityAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeMintCloseAuthority { accounts, args });
                }
                [26u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let transfer_fee_config_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(transfer_fee_config_authority),
                                    e
                                )
                            })?;
                    let withdraw_withheld_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(withdraw_withheld_authority),
                                    e
                                )
                            })?;
                    let transfer_fee_basis_points: u16 =
                        <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(transfer_fee_basis_points),
                                e
                            )
                        })?;
                    let maximum_fee: u64 =
                        <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(maximum_fee), e)
                        })?;
                    let args = InitializeTransferFeeConfigArguments {
                        transfer_fee_config_authority,
                        withdraw_withheld_authority,
                        transfer_fee_basis_points,
                        maximum_fee,
                        extension_type: "initializeTransferFeeConfig".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeTransferFeeConfigAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeTransferFeeConfig { accounts, args });
                }
                [26u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let fee: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(fee), e))?;
                    let args = TransferCheckedWithFeeArguments {
                        amount,
                        decimals,
                        fee,
                        extension_type: "transferCheckedWithFee".to_string(),
                    };
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = TransferCheckedWithFeeAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        mint: mint.clone(),
                        destination: destination.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::TransferCheckedWithFee { accounts, args });
                }
                [26u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = WithdrawWithheldTokensFromMintArguments {
                        extension_type: "withdrawWithheldTokensFromMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let fee_receiver = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "feeReceiver" , "feeReceiver" , account_index , provided_count) }) ?
                    };
                    let withdraw_withheld_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "withdrawWithheldAuthority" , "withdrawWithheldAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = WithdrawWithheldTokensFromMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        fee_receiver: fee_receiver.clone(),
                        withdraw_withheld_authority: withdraw_withheld_authority.clone(),
                    };
                    return Ok(Instruction::WithdrawWithheldTokensFromMint { accounts, args });
                }
                [26u8, 3u8] => {
                    let mut rdr: &[u8] = rest;
                    let num_token_accounts: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(num_token_accounts),
                                e
                            )
                        })?;
                    let args = WithdrawWithheldTokensFromAccountsArguments {
                        num_token_accounts,
                        extension_type: "withdrawWithheldTokensFromAccounts".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let fee_receiver = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "feeReceiver" , "feeReceiver" , account_index , provided_count) }) ?
                    };
                    let withdraw_withheld_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "withdrawWithheldAuthority" , "withdrawWithheldAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = WithdrawWithheldTokensFromAccountsAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        fee_receiver: fee_receiver.clone(),
                        withdraw_withheld_authority: withdraw_withheld_authority.clone(),
                    };
                    return Ok(Instruction::WithdrawWithheldTokensFromAccounts { accounts, args });
                }
                [26u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = HarvestWithheldTokensToMintArguments {
                        extension_type: "harvestWithheldTokensToMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = HarvestWithheldTokensToMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::HarvestWithheldTokensToMint { accounts, args });
                }
                [26u8, 5u8] => {
                    let mut rdr: &[u8] = rest;
                    let transfer_fee_basis_points: u16 =
                        <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(transfer_fee_basis_points),
                                e
                            )
                        })?;
                    let maximum_fee: u64 =
                        <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(maximum_fee), e)
                        })?;
                    let args = SetTransferFeeArguments {
                        transfer_fee_basis_points,
                        maximum_fee,
                        extension_type: "setTransferFee".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let transfer_fee_config_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "transferFeeConfigAuthority" , "transferFeeConfigAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = SetTransferFeeAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        transfer_fee_config_authority: transfer_fee_config_authority.clone(),
                    };
                    return Ok(Instruction::SetTransferFee { accounts, args });
                }
                [27u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let auto_approve_new_accounts: bool =
                        <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                            |e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(auto_approve_new_accounts),
                                    e
                                )
                            },
                        )?;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(auditor_elgamal_pubkey),
                            e
                        )
                    })?;
                    let auditor_elgamal_pubkey = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeConfidentialTransferMintArguments {
                        authority,
                        auto_approve_new_accounts,
                        auditor_elgamal_pubkey,
                        extension_type: "initializeConfidentialTransferMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeConfidentialTransferMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeConfidentialTransferMint { accounts, args });
                }
                [27u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let auto_approve_new_accounts: bool =
                        <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                            |e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(auto_approve_new_accounts),
                                    e
                                )
                            },
                        )?;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(auditor_elgamal_pubkey),
                            e
                        )
                    })?;
                    let auditor_elgamal_pubkey = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateConfidentialTransferMintArguments {
                        auto_approve_new_accounts,
                        auditor_elgamal_pubkey,
                        extension_type: "updateConfidentialTransferMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateConfidentialTransferMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::UpdateConfidentialTransferMint { accounts, args });
                }
                [27u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut decryptable_zero_balance = [0u8; 36];
                    rdr.read_exact(&mut decryptable_zero_balance).map_err(|e| {
                        format!(
                            "Failed to read 36 bytes for {}: {}",
                            stringify!(decryptable_zero_balance),
                            e
                        )
                    })?;
                    let maximum_pending_balance_credit_counter: u64 =
                        <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(maximum_pending_balance_credit_counter),
                                e
                            )
                        })?;
                    let proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(proof_instruction_offset),
                                e
                            )
                        })?;
                    let args = ConfigureConfidentialTransferAccountArguments {
                        decryptable_zero_balance,
                        maximum_pending_balance_credit_counter,
                        proof_instruction_offset,
                        extension_type: "configureConfidentialTransferAccount".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar_or_context_state = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                    };
                    let record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            4usize
                        } else {
                            4usize - 1usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ConfigureConfidentialTransferAccountAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        mint: mint.clone(),
                        instructions_sysvar_or_context_state: instructions_sysvar_or_context_state
                            .clone(),
                        record: record,
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ConfigureConfidentialTransferAccount {
                        accounts,
                        args,
                    });
                }
                [27u8, 3u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = ApproveConfidentialTransferAccountArguments {
                        extension_type: "approveConfidentialTransferAccount".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ApproveConfidentialTransferAccountAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ApproveConfidentialTransferAccount { accounts, args });
                }
                [27u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(proof_instruction_offset),
                                e
                            )
                        })?;
                    let args = EmptyConfidentialTransferAccountArguments {
                        proof_instruction_offset,
                        extension_type: "emptyConfidentialTransferAccount".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar_or_context_state = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                    };
                    let record = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count > required_count && provided_count > 2usize {
                            account_keys.get(2usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 1usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = EmptyConfidentialTransferAccountAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        instructions_sysvar_or_context_state: instructions_sysvar_or_context_state
                            .clone(),
                        record: record,
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::EmptyConfidentialTransferAccount { accounts, args });
                }
                [27u8, 5u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = ConfidentialDepositArguments {
                        amount,
                        decimals,
                        extension_type: "confidentialDeposit".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ConfidentialDepositAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ConfidentialDeposit { accounts, args });
                }
                [27u8, 6u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let mut new_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_decryptable_available_balance),
                                e
                            )
                        })?;
                    let equality_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(equality_proof_instruction_offset),
                                e
                            )
                        })?;
                    let range_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(range_proof_instruction_offset),
                                e
                            )
                        })?;
                    let args = ConfidentialWithdrawArguments {
                        amount,
                        decimals,
                        new_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        range_proof_instruction_offset,
                        extension_type: "confidentialWithdraw".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 6usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 6usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count > required_count && provided_count > 2usize {
                            account_keys.get(2usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let equality_record = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let range_record = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count > required_count && provided_count > 4usize {
                            account_keys.get(4usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 6usize {
                            5usize
                        } else {
                            5usize - 3usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ConfidentialWithdrawAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        mint: mint.clone(),
                        instructions_sysvar: instructions_sysvar,
                        equality_record: equality_record,
                        range_record: range_record,
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ConfidentialWithdraw { accounts, args });
                }
                [27u8, 7u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut new_source_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_source_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_source_decryptable_available_balance),
                                e
                            )
                        })?;
                    let equality_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(equality_proof_instruction_offset),
                                e
                            )
                        })?;
                    let ciphertext_validity_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(ciphertext_validity_proof_instruction_offset),
                                e
                            )
                        })?;
                    let range_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(range_proof_instruction_offset),
                                e
                            )
                        })?;
                    let args = ConfidentialTransferArguments {
                        new_source_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        ciphertext_validity_proof_instruction_offset,
                        range_proof_instruction_offset,
                        extension_type: "confidentialTransfer".to_string(),
                    };
                    let source_token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 8usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "sourceToken" , "sourceToken" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 8usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination_token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 8usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destinationToken" , "destinationToken" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let equality_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 4usize {
                            account_keys.get(4usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let ciphertext_validity_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 5usize {
                            account_keys.get(5usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let range_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 6usize {
                            account_keys.get(6usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 8usize {
                            7usize
                        } else {
                            7usize - 4usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ConfidentialTransferAccounts {
                        remaining: vec![],
                        source_token: source_token.clone(),
                        mint: mint.clone(),
                        destination_token: destination_token.clone(),
                        instructions_sysvar: instructions_sysvar,
                        equality_record: equality_record,
                        ciphertext_validity_record: ciphertext_validity_record,
                        range_record: range_record,
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ConfidentialTransfer { accounts, args });
                }
                [27u8, 8u8] => {
                    let mut rdr: &[u8] = rest;
                    let expected_pending_balance_credit_counter: u64 =
                        <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(expected_pending_balance_credit_counter),
                                e
                            )
                        })?;
                    let mut new_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_decryptable_available_balance),
                                e
                            )
                        })?;
                    let args = ApplyConfidentialPendingBalanceArguments {
                        expected_pending_balance_credit_counter,
                        new_decryptable_available_balance,
                        extension_type: "applyConfidentialPendingBalance".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ApplyConfidentialPendingBalanceAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ApplyConfidentialPendingBalance { accounts, args });
                }
                [27u8, 9u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = EnableConfidentialCreditsArguments {
                        extension_type: "enableConfidentialCredits".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = EnableConfidentialCreditsAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::EnableConfidentialCredits { accounts, args });
                }
                [27u8, 10u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = DisableConfidentialCreditsArguments {
                        extension_type: "disableConfidentialCredits".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = DisableConfidentialCreditsAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::DisableConfidentialCredits { accounts, args });
                }
                [27u8, 11u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = EnableNonConfidentialCreditsArguments {
                        extension_type: "enableNonConfidentialCredits".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = EnableNonConfidentialCreditsAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::EnableNonConfidentialCredits { accounts, args });
                }
                [27u8, 12u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = DisableNonConfidentialCreditsArguments {
                        extension_type: "disableNonConfidentialCredits".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = DisableNonConfidentialCreditsAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::DisableNonConfidentialCredits { accounts, args });
                }
                [27u8, 13u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut new_source_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_source_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_source_decryptable_available_balance),
                                e
                            )
                        })?;
                    let equality_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(equality_proof_instruction_offset),
                                e
                            )
                        })?;
                    let transfer_amount_ciphertext_validity_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(
                                    transfer_amount_ciphertext_validity_proof_instruction_offset
                                ),
                                e
                            )
                        })?;
                    let fee_sigma_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(fee_sigma_proof_instruction_offset),
                                e
                            )
                        })?;
                    let fee_ciphertext_validity_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(fee_ciphertext_validity_proof_instruction_offset),
                                e
                            )
                        })?;
                    let range_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(range_proof_instruction_offset),
                                e
                            )
                        })?;
                    let args = ConfidentialTransferWithFeeArguments {
                        new_source_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        transfer_amount_ciphertext_validity_proof_instruction_offset,
                        fee_sigma_proof_instruction_offset,
                        fee_ciphertext_validity_proof_instruction_offset,
                        range_proof_instruction_offset,
                        extension_type: "confidentialTransferWithFee".to_string(),
                    };
                    let source_token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 10usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "sourceToken" , "sourceToken" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 10usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination_token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 10usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destinationToken" , "destinationToken" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let equality_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 4usize {
                            account_keys.get(4usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let transfer_amount_ciphertext_validity_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 5usize {
                            account_keys.get(5usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let fee_sigma_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 6usize {
                            account_keys.get(6usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let fee_ciphertext_validity_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 7usize {
                            account_keys.get(7usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let range_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 8usize {
                            account_keys.get(8usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 10usize {
                            9usize
                        } else {
                            9usize - 6usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ConfidentialTransferWithFeeAccounts {
                        remaining: vec![],
                        source_token: source_token.clone(),
                        mint: mint.clone(),
                        destination_token: destination_token.clone(),
                        instructions_sysvar: instructions_sysvar,
                        equality_record: equality_record,
                        transfer_amount_ciphertext_validity_record:
                            transfer_amount_ciphertext_validity_record,
                        fee_sigma_record: fee_sigma_record,
                        fee_ciphertext_validity_record: fee_ciphertext_validity_record,
                        range_record: range_record,
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ConfidentialTransferWithFee { accounts, args });
                }
                [28u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let state: AccountState =
                        <AccountState as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(state), e)
                            })?;
                    let args = InitializeDefaultAccountStateArguments {
                        state,
                        extension_type: "initializeDefaultAccountState".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeDefaultAccountStateAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeDefaultAccountState { accounts, args });
                }
                [28u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let state: AccountState =
                        <AccountState as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(state), e)
                            })?;
                    let args = UpdateDefaultAccountStateArguments {
                        state,
                        extension_type: "updateDefaultAccountState".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let freeze_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "freezeAuthority" , "freezeAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateDefaultAccountStateAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        freeze_authority: freeze_authority.clone(),
                    };
                    return Ok(Instruction::UpdateDefaultAccountState { accounts, args });
                }
                [29u8] => {
                    let mut rdr: &[u8] = rest;
                    let new_extension_types: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(new_extension_types),
                                e
                            )
                        })?;
                    let args = ReallocateArguments {
                        new_extension_types,
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let payer = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "payer" , "payer" , account_index , provided_count) }) ?
                    };
                    let system_program = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "systemProgram" , "systemProgram" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = ReallocateAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        payer: payer.clone(),
                        system_program: system_program.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::Reallocate { accounts, args });
                }
                [30u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = EnableMemoTransfersArguments {
                        extension_type: "enableMemoTransfers".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = EnableMemoTransfersAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::EnableMemoTransfers { accounts, args });
                }
                [30u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = DisableMemoTransfersArguments {
                        extension_type: "disableMemoTransfers".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = DisableMemoTransfersAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::DisableMemoTransfers { accounts, args });
                }
                [31u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = CreateNativeMintArguments {};
                    let payer = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "payer" , "payer" , account_index , provided_count) }) ?
                    };
                    let native_mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "nativeMint" , "nativeMint" , account_index , provided_count) }) ?
                    };
                    let system_program = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "systemProgram" , "systemProgram" , account_index , provided_count) }) ?
                    };
                    let accounts = CreateNativeMintAccounts {
                        remaining: vec![],
                        payer: payer.clone(),
                        native_mint: native_mint.clone(),
                        system_program: system_program.clone(),
                    };
                    return Ok(Instruction::CreateNativeMint { accounts, args });
                }
                [32u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeNonTransferableMintArguments {};
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeNonTransferableMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeNonTransferableMint { accounts, args });
                }
                [33u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(rate_authority),
                            e
                        )
                    })?;
                    let rate_authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(rate), e)
                        })?;
                    let args = InitializeInterestBearingMintArguments {
                        rate_authority,
                        rate,
                        extension_type: "initializeInterestBearingMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeInterestBearingMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeInterestBearingMint { accounts, args });
                }
                [33u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(rate), e)
                        })?;
                    let args = UpdateRateInterestBearingMintArguments {
                        rate,
                        extension_type: "updateRateInterestBearingMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let rate_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rateAuthority" , "rateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateRateInterestBearingMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        rate_authority: rate_authority.clone(),
                    };
                    return Ok(Instruction::UpdateRateInterestBearingMint { accounts, args });
                }
                [34u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = EnableCpiGuardArguments {
                        extension_type: "enableCpiGuard".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = EnableCpiGuardAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::EnableCpiGuard { accounts, args });
                }
                [34u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = DisableCpiGuardArguments {
                        extension_type: "disableCpiGuard".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = DisableCpiGuardAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::DisableCpiGuard { accounts, args });
                }
                [35u8] => {
                    let mut rdr: &[u8] = rest;
                    let delegate: [u8; 32usize] =
                        <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(delegate), e)
                            })?;
                    let args = InitializePermanentDelegateArguments { delegate };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializePermanentDelegateAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializePermanentDelegate { accounts, args });
                }
                [36u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(program_id),
                            e
                        )
                    })?;
                    let program_id = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeTransferHookArguments {
                        authority,
                        program_id,
                        extension_type: "initializeTransferHook".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeTransferHookAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeTransferHook { accounts, args });
                }
                [36u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(program_id),
                            e
                        )
                    })?;
                    let program_id = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateTransferHookArguments {
                        program_id,
                        extension_type: "updateTransferHook".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateTransferHookAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::UpdateTransferHook { accounts, args });
                }
                [37u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(withdraw_withheld_authority_el_gamal_pubkey),
                            e
                        )
                    })?;
                    let withdraw_withheld_authority_el_gamal_pubkey = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeConfidentialTransferFeeArguments {
                        authority,
                        withdraw_withheld_authority_el_gamal_pubkey,
                        extension_type: "initializeConfidentialTransferFee".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeConfidentialTransferFeeAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeConfidentialTransferFee { accounts, args });
                }
                [37u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(proof_instruction_offset),
                                e
                            )
                        })?;
                    let mut new_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_decryptable_available_balance),
                                e
                            )
                        })?;
                    let args = WithdrawWithheldTokensFromMintForConfidentialTransferFeeArguments {
                        proof_instruction_offset,
                        new_decryptable_available_balance,
                        extension_type: "withdrawWithheldTokensFromMintForConfidentialTransferFee"
                            .to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar_or_context_state = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                    };
                    let record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            4usize
                        } else {
                            4usize - 1usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts =
                        WithdrawWithheldTokensFromMintForConfidentialTransferFeeAccounts {
                            remaining: vec![],
                            mint: mint.clone(),
                            destination: destination.clone(),
                            instructions_sysvar_or_context_state:
                                instructions_sysvar_or_context_state.clone(),
                            record: record,
                            authority: authority.clone(),
                        };
                    return Ok(
                        Instruction::WithdrawWithheldTokensFromMintForConfidentialTransferFee {
                            accounts,
                            args,
                        },
                    );
                }
                [37u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let num_token_accounts: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(num_token_accounts),
                                e
                            )
                        })?;
                    let proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(proof_instruction_offset),
                                e
                            )
                        })?;
                    let mut new_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_decryptable_available_balance),
                                e
                            )
                        })?;
                    let args =
                        WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeArguments {
                            num_token_accounts,
                            proof_instruction_offset,
                            new_decryptable_available_balance,
                            extension_type:
                                "withdrawWithheldTokensFromAccountsForConfidentialTransferFee"
                                    .to_string(),
                        };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar_or_context_state = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                    };
                    let record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            4usize
                        } else {
                            4usize - 1usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts =
                        WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeAccounts {
                            remaining: vec![],
                            mint: mint.clone(),
                            destination: destination.clone(),
                            instructions_sysvar_or_context_state:
                                instructions_sysvar_or_context_state.clone(),
                            record: record,
                            authority: authority.clone(),
                        };
                    return Ok(
                        Instruction::WithdrawWithheldTokensFromAccountsForConfidentialTransferFee {
                            accounts,
                            args,
                        },
                    );
                }
                [37u8, 3u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = HarvestWithheldTokensToMintForConfidentialTransferFeeArguments {
                        extension_type: "harvestWithheldTokensToMintForConfidentialTransferFee"
                            .to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = HarvestWithheldTokensToMintForConfidentialTransferFeeAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(
                        Instruction::HarvestWithheldTokensToMintForConfidentialTransferFee {
                            accounts,
                            args,
                        },
                    );
                }
                [37u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = EnableHarvestToMintArguments {
                        extension_type: "enableHarvestToMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = EnableHarvestToMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::EnableHarvestToMint { accounts, args });
                }
                [37u8, 5u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = DisableHarvestToMintArguments {
                        extension_type: "disableHarvestToMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = DisableHarvestToMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::DisableHarvestToMint { accounts, args });
                }
                [38u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = WithdrawExcessLamportsArguments {};
                    let source_account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "sourceAccount" , "sourceAccount" , account_index , provided_count) }) ?
                    };
                    let destination_account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destinationAccount" , "destinationAccount" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = WithdrawExcessLamportsAccounts {
                        remaining: vec![],
                        source_account: source_account.clone(),
                        destination_account: destination_account.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::WithdrawExcessLamports { accounts, args });
                }
                [39u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(metadata_address),
                            e
                        )
                    })?;
                    let metadata_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeMetadataPointerArguments {
                        authority,
                        metadata_address,
                        extension_type: "initializeMetadataPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMetadataPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeMetadataPointer { accounts, args });
                }
                [39u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(metadata_address),
                            e
                        )
                    })?;
                    let metadata_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateMetadataPointerArguments {
                        metadata_address,
                        extension_type: "updateMetadataPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let metadata_pointer_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadataPointerAuthority" , "metadataPointerAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateMetadataPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        metadata_pointer_authority: metadata_pointer_authority.clone(),
                    };
                    return Ok(Instruction::UpdateMetadataPointer { accounts, args });
                }
                [40u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(group_address),
                            e
                        )
                    })?;
                    let group_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeGroupPointerArguments {
                        authority,
                        group_address,
                        extension_type: "initializeGroupPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeGroupPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeGroupPointer { accounts, args });
                }
                [40u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(group_address),
                            e
                        )
                    })?;
                    let group_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateGroupPointerArguments {
                        group_address,
                        extension_type: "updateGroupPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let group_pointer_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "groupPointerAuthority" , "groupPointerAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateGroupPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        group_pointer_authority: group_pointer_authority.clone(),
                    };
                    return Ok(Instruction::UpdateGroupPointer { accounts, args });
                }
                [41u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(member_address),
                            e
                        )
                    })?;
                    let member_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeGroupMemberPointerArguments {
                        authority,
                        member_address,
                        extension_type: "initializeGroupMemberPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeGroupMemberPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeGroupMemberPointer { accounts, args });
                }
                [41u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(member_address),
                            e
                        )
                    })?;
                    let member_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateGroupMemberPointerArguments {
                        member_address,
                        extension_type: "updateGroupMemberPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let group_member_pointer_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "groupMemberPointerAuthority" , "groupMemberPointerAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateGroupMemberPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        group_member_pointer_authority: group_member_pointer_authority.clone(),
                    };
                    return Ok(Instruction::UpdateGroupMemberPointer { accounts, args });
                }
                [43u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let multiplier: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(multiplier), e)
                    })?;
                    let args = InitializeScaledUiAmountMintArguments {
                        authority,
                        multiplier,
                        extension_type: "initializeScaledUiAmountMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeScaledUiAmountMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeScaledUiAmountMint { accounts, args });
                }
                [43u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let multiplier: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(multiplier), e)
                    })?;
                    let effective_timestamp: i64 =
                        <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(effective_timestamp),
                                e
                            )
                        })?;
                    let args = UpdateMultiplierScaledUiMintArguments {
                        multiplier,
                        effective_timestamp,
                        extension_type: "updateMultiplierScaledUiMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateMultiplierScaledUiMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::UpdateMultiplierScaledUiMint { accounts, args });
                }
                [44u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializePausableConfigArguments {
                        authority,
                        extension_type: "initializePausableConfig".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializePausableConfigAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializePausableConfig { accounts, args });
                }
                [44u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = PauseArguments {
                        extension_type: "pause".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = PauseAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::Pause { accounts, args });
                }
                [44u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = ResumeArguments {
                        extension_type: "resume".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ResumeAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::Resume { accounts, args });
                }
                [210u8, 225u8, 30u8, 162u8, 88u8, 184u8, 77u8, 141u8] => {
                    let mut rdr: &[u8] = rest;
                    let len = {
                        let mut len_bytes = [0u8; 4];
                        rdr.read_exact(&mut len_bytes).map_err(|e| {
                            format!("Failed to read length for {}: {}", stringify!(name), e)
                        })?;
                        u32::from_le_bytes(len_bytes) as usize
                    };
                    let bytes = {
                        let mut bytes = vec![0u8; len];
                        rdr.read_exact(&mut bytes).map_err(|e| {
                            format!(
                                "Failed to read {} bytes for {}: {}",
                                len,
                                stringify!(name),
                                e
                            )
                        })?;
                        bytes
                    };
                    let name = String::from_utf8(bytes).map_err(|e| {
                        format!("Failed to convert {} to string: {}", stringify!(name), e)
                    })?;
                    let len = {
                        let mut len_bytes = [0u8; 4];
                        rdr.read_exact(&mut len_bytes).map_err(|e| {
                            format!("Failed to read length for {}: {}", stringify!(symbol), e)
                        })?;
                        u32::from_le_bytes(len_bytes) as usize
                    };
                    let bytes = {
                        let mut bytes = vec![0u8; len];
                        rdr.read_exact(&mut bytes).map_err(|e| {
                            format!(
                                "Failed to read {} bytes for {}: {}",
                                len,
                                stringify!(symbol),
                                e
                            )
                        })?;
                        bytes
                    };
                    let symbol = String::from_utf8(bytes).map_err(|e| {
                        format!("Failed to convert {} to string: {}", stringify!(symbol), e)
                    })?;
                    let len = {
                        let mut len_bytes = [0u8; 4];
                        rdr.read_exact(&mut len_bytes).map_err(|e| {
                            format!("Failed to read length for {}: {}", stringify!(uri), e)
                        })?;
                        u32::from_le_bytes(len_bytes) as usize
                    };
                    let bytes = {
                        let mut bytes = vec![0u8; len];
                        rdr.read_exact(&mut bytes).map_err(|e| {
                            format!(
                                "Failed to read {} bytes for {}: {}",
                                len,
                                stringify!(uri),
                                e
                            )
                        })?;
                        bytes
                    };
                    let uri = String::from_utf8(bytes).map_err(|e| {
                        format!("Failed to convert {} to string: {}", stringify!(uri), e)
                    })?;
                    let args = InitializeTokenMetadataArguments {
                        name,
                        symbol,
                        uri,
                        extension_type: "initializeTokenMetadata".to_string(),
                    };
                    let metadata = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let mint_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeTokenMetadataAccounts {
                        remaining: vec![],
                        metadata: metadata.clone(),
                        update_authority: update_authority.clone(),
                        mint: mint.clone(),
                        mint_authority: mint_authority.clone(),
                    };
                    return Ok(Instruction::InitializeTokenMetadata { accounts, args });
                }
                [221u8, 233u8, 49u8, 45u8, 181u8, 202u8, 220u8, 200u8] => {
                    let mut rdr: &[u8] = rest;
                    let field: TokenMetadataField =
                        <TokenMetadataField as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(field), e)
                            })?;
                    let len = {
                        let mut len_bytes = [0u8; 4];
                        rdr.read_exact(&mut len_bytes).map_err(|e| {
                            format!("Failed to read length for {}: {}", stringify!(value), e)
                        })?;
                        u32::from_le_bytes(len_bytes) as usize
                    };
                    let bytes = {
                        let mut bytes = vec![0u8; len];
                        rdr.read_exact(&mut bytes).map_err(|e| {
                            format!(
                                "Failed to read {} bytes for {}: {}",
                                len,
                                stringify!(value),
                                e
                            )
                        })?;
                        bytes
                    };
                    let value = String::from_utf8(bytes).map_err(|e| {
                        format!("Failed to convert {} to string: {}", stringify!(value), e)
                    })?;
                    let args = UpdateTokenMetadataFieldArguments {
                        field,
                        value,
                        extension_type: "updateTokenMetadataField".to_string(),
                    };
                    let metadata = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateTokenMetadataFieldAccounts {
                        remaining: vec![],
                        metadata: metadata.clone(),
                        update_authority: update_authority.clone(),
                    };
                    return Ok(Instruction::UpdateTokenMetadataField { accounts, args });
                }
                [234u8, 18u8, 32u8, 56u8, 89u8, 141u8, 37u8, 181u8] => {
                    let mut rdr: &[u8] = rest;
                    let idempotent: bool =
                        <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                            |e| format!("Failed to deserialize {}: {}", stringify!(idempotent), e),
                        )?;
                    let len = {
                        let mut len_bytes = [0u8; 4];
                        rdr.read_exact(&mut len_bytes).map_err(|e| {
                            format!("Failed to read length for {}: {}", stringify!(key), e)
                        })?;
                        u32::from_le_bytes(len_bytes) as usize
                    };
                    let bytes = {
                        let mut bytes = vec![0u8; len];
                        rdr.read_exact(&mut bytes).map_err(|e| {
                            format!(
                                "Failed to read {} bytes for {}: {}",
                                len,
                                stringify!(key),
                                e
                            )
                        })?;
                        bytes
                    };
                    let key = String::from_utf8(bytes).map_err(|e| {
                        format!("Failed to convert {} to string: {}", stringify!(key), e)
                    })?;
                    let args = RemoveTokenMetadataKeyArguments {
                        idempotent,
                        key,
                        extension_type: "removeTokenMetadataKey".to_string(),
                    };
                    let metadata = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = RemoveTokenMetadataKeyAccounts {
                        remaining: vec![],
                        metadata: metadata.clone(),
                        update_authority: update_authority.clone(),
                    };
                    return Ok(Instruction::RemoveTokenMetadataKey { accounts, args });
                }
                [215u8, 228u8, 166u8, 228u8, 84u8, 100u8, 86u8, 123u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(new_update_authority),
                            e
                        )
                    })?;
                    let new_update_authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateTokenMetadataUpdateAuthorityArguments {
                        new_update_authority,
                        extension_type: "updateTokenMetadataUpdateAuthority".to_string(),
                    };
                    let metadata = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateTokenMetadataUpdateAuthorityAccounts {
                        remaining: vec![],
                        metadata: metadata.clone(),
                        update_authority: update_authority.clone(),
                    };
                    return Ok(Instruction::UpdateTokenMetadataUpdateAuthority { accounts, args });
                }
                [250u8, 166u8, 180u8, 250u8, 13u8, 12u8, 184u8, 70u8] => {
                    let mut rdr: &[u8] = rest;
                    let start: Option<u64> =
                        <Option<u64> as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                            |e| format!("Failed to deserialize {}: {}", stringify!(start), e),
                        )?;
                    let end: Option<u64> = <Option<u64> as ::borsh::BorshDeserialize>::deserialize(
                        &mut rdr,
                    )
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(end), e))?;
                    let args = EmitTokenMetadataArguments {
                        start,
                        end,
                        extension_type: "emitTokenMetadata".to_string(),
                    };
                    let metadata = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                    };
                    let accounts = EmitTokenMetadataAccounts {
                        remaining: vec![],
                        metadata: metadata.clone(),
                    };
                    return Ok(Instruction::EmitTokenMetadata { accounts, args });
                }
                [121u8, 113u8, 108u8, 39u8, 54u8, 51u8, 0u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(update_authority),
                            e
                        )
                    })?;
                    let update_authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                    let args = InitializeTokenGroupArguments {
                        update_authority,
                        max_size,
                        extension_type: "initializeTokenGroup".to_string(),
                    };
                    let group = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let mint_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeTokenGroupAccounts {
                        remaining: vec![],
                        group: group.clone(),
                        mint: mint.clone(),
                        mint_authority: mint_authority.clone(),
                    };
                    return Ok(Instruction::InitializeTokenGroup { accounts, args });
                }
                [108u8, 37u8, 171u8, 143u8, 248u8, 30u8, 18u8, 110u8] => {
                    let mut rdr: &[u8] = rest;
                    let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                    let args = UpdateTokenGroupMaxSizeArguments {
                        max_size,
                        extension_type: "updateTokenGroupMaxSize".to_string(),
                    };
                    let group = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateTokenGroupMaxSizeAccounts {
                        remaining: vec![],
                        group: group.clone(),
                        update_authority: update_authority.clone(),
                    };
                    return Ok(Instruction::UpdateTokenGroupMaxSize { accounts, args });
                }
                [161u8, 105u8, 88u8, 1u8, 237u8, 221u8, 216u8, 203u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(new_update_authority),
                            e
                        )
                    })?;
                    let new_update_authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateTokenGroupUpdateAuthorityArguments {
                        new_update_authority,
                        extension_type: "updateTokenGroupUpdateAuthority".to_string(),
                    };
                    let group = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateTokenGroupUpdateAuthorityAccounts {
                        remaining: vec![],
                        group: group.clone(),
                        update_authority: update_authority.clone(),
                    };
                    return Ok(Instruction::UpdateTokenGroupUpdateAuthority { accounts, args });
                }
                [152u8, 32u8, 222u8, 176u8, 223u8, 237u8, 116u8, 134u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeTokenGroupMemberArguments {
                        extension_type: "initializeTokenGroupMember".to_string(),
                    };
                    let member = {
                        let provided_count = account_keys.len();
                        let required_count = 5usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "member" , "member" , account_index , provided_count) }) ?
                    };
                    let member_mint = {
                        let provided_count = account_keys.len();
                        let required_count = 5usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "memberMint" , "memberMint" , account_index , provided_count) }) ?
                    };
                    let member_mint_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 5usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "memberMintAuthority" , "memberMintAuthority" , account_index , provided_count) }) ?
                    };
                    let group = {
                        let provided_count = account_keys.len();
                        let required_count = 5usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                    };
                    let group_update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 5usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            4usize
                        } else {
                            4usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "groupUpdateAuthority" , "groupUpdateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeTokenGroupMemberAccounts {
                        remaining: vec![],
                        member: member.clone(),
                        member_mint: member_mint.clone(),
                        member_mint_authority: member_mint_authority.clone(),
                        group: group.clone(),
                        group_update_authority: group_update_authority.clone(),
                    };
                    return Ok(Instruction::InitializeTokenGroupMember { accounts, args });
                }
                _ => {}
            }
        }
        if data.len() >= 2 {
            let (discriminator, rest) = data.split_at(2);
            match discriminator {
                [0u8] => {
                    let mut rdr: &[u8] = rest;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let mint_authority: [u8; 32usize] =
                        <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(mint_authority),
                                    e
                                )
                            })?;
                    let freeze_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(freeze_authority),
                                    e
                                )
                            })?;
                    let args = InitializeMintArguments {
                        decimals,
                        mint_authority,
                        freeze_authority,
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let rent = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        rent: rent.clone(),
                    };
                    return Ok(Instruction::InitializeMint { accounts, args });
                }
                [1u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeAccountArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let rent = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeAccountAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        owner: owner.clone(),
                        rent: rent.clone(),
                    };
                    return Ok(Instruction::InitializeAccount { accounts, args });
                }
                [2u8] => {
                    let mut rdr: &[u8] = rest;
                    let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                    let args = InitializeMultisigArguments { m };
                    let multisig = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "multisig" , "multisig" , account_index , provided_count) }) ?
                    };
                    let rent = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMultisigAccounts {
                        remaining: vec![],
                        multisig: multisig.clone(),
                        rent: rent.clone(),
                    };
                    return Ok(Instruction::InitializeMultisig { accounts, args });
                }
                [3u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = TransferArguments { amount };
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = TransferAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        destination: destination.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::Transfer { accounts, args });
                }
                [4u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = ApproveArguments { amount };
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let delegate = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "delegate" , "delegate" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = ApproveAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        delegate: delegate.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::Approve { accounts, args });
                }
                [5u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = RevokeArguments {};
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = RevokeAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::Revoke { accounts, args });
                }
                [6u8] => {
                    let mut rdr: &[u8] = rest;
                    let authority_type: AuthorityType =
                        <AuthorityType as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(authority_type),
                                    e
                                )
                            })?;
                    let new_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(new_authority),
                                    e
                                )
                            })?;
                    let args = SetAuthorityArguments {
                        authority_type,
                        new_authority,
                    };
                    let owned = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owned" , "owned" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = SetAuthorityAccounts {
                        remaining: vec![],
                        owned: owned.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::SetAuthority { accounts, args });
                }
                [7u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = MintToArguments { amount };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = MintToAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        token: token.clone(),
                        mint_authority: mint_authority.clone(),
                    };
                    return Ok(Instruction::MintTo { accounts, args });
                }
                [8u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = BurnArguments { amount };
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = BurnAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::Burn { accounts, args });
                }
                [9u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = CloseAccountArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = CloseAccountAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        destination: destination.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::CloseAccount { accounts, args });
                }
                [10u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = FreezeAccountArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = FreezeAccountAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::FreezeAccount { accounts, args });
                }
                [11u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = ThawAccountArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = ThawAccountAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::ThawAccount { accounts, args });
                }
                [12u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = TransferCheckedArguments { amount, decimals };
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = TransferCheckedAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        mint: mint.clone(),
                        destination: destination.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::TransferChecked { accounts, args });
                }
                [13u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = ApproveCheckedArguments { amount, decimals };
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let delegate = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "delegate" , "delegate" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = ApproveCheckedAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        mint: mint.clone(),
                        delegate: delegate.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::ApproveChecked { accounts, args });
                }
                [14u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = MintToCheckedArguments { amount, decimals };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = MintToCheckedAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        token: token.clone(),
                        mint_authority: mint_authority.clone(),
                    };
                    return Ok(Instruction::MintToChecked { accounts, args });
                }
                [15u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = BurnCheckedArguments { amount, decimals };
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = BurnCheckedAccounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::BurnChecked { accounts, args });
                }
                [16u8] => {
                    let mut rdr: &[u8] = rest;
                    let owner: [u8; 32usize] =
                        <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(owner), e)
                            })?;
                    let args = InitializeAccount2Arguments { owner };
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let rent = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeAccount2Accounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                        rent: rent.clone(),
                    };
                    return Ok(Instruction::InitializeAccount2 { accounts, args });
                }
                [17u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = SyncNativeArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let accounts = SyncNativeAccounts {
                        remaining: vec![],
                        account: account.clone(),
                    };
                    return Ok(Instruction::SyncNative { accounts, args });
                }
                [18u8] => {
                    let mut rdr: &[u8] = rest;
                    let owner: [u8; 32usize] =
                        <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(owner), e)
                            })?;
                    let args = InitializeAccount3Arguments { owner };
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeAccount3Accounts {
                        remaining: vec![],
                        account: account.clone(),
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeAccount3 { accounts, args });
                }
                [19u8] => {
                    let mut rdr: &[u8] = rest;
                    let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                    let args = InitializeMultisig2Arguments { m };
                    let multisig = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "multisig" , "multisig" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMultisig2Accounts {
                        remaining: vec![],
                        multisig: multisig.clone(),
                    };
                    return Ok(Instruction::InitializeMultisig2 { accounts, args });
                }
                [20u8] => {
                    let mut rdr: &[u8] = rest;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let mint_authority: [u8; 32usize] =
                        <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(mint_authority),
                                    e
                                )
                            })?;
                    let freeze_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(freeze_authority),
                                    e
                                )
                            })?;
                    let args = InitializeMint2Arguments {
                        decimals,
                        mint_authority,
                        freeze_authority,
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMint2Accounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeMint2 { accounts, args });
                }
                [21u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = GetAccountDataSizeArguments {};
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = GetAccountDataSizeAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::GetAccountDataSize { accounts, args });
                }
                [22u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeImmutableOwnerArguments {};
                    let account = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeImmutableOwnerAccounts {
                        remaining: vec![],
                        account: account.clone(),
                    };
                    return Ok(Instruction::InitializeImmutableOwner { accounts, args });
                }
                [23u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = AmountToUiAmountArguments { amount };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = AmountToUiAmountAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::AmountToUiAmount { accounts, args });
                }
                [24u8] => {
                    let mut rdr: &[u8] = rest;
                    let ui_amount: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(ui_amount), e)
                        })?;
                    let args = UiAmountToAmountArguments { ui_amount };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = UiAmountToAmountAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::UiAmountToAmount { accounts, args });
                }
                [25u8] => {
                    let mut rdr: &[u8] = rest;
                    let close_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(close_authority),
                                    e
                                )
                            })?;
                    let args = InitializeMintCloseAuthorityArguments { close_authority };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMintCloseAuthorityAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeMintCloseAuthority { accounts, args });
                }
                [26u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let transfer_fee_config_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(transfer_fee_config_authority),
                                    e
                                )
                            })?;
                    let withdraw_withheld_authority: Option<[u8; 32usize]> =
                        <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(withdraw_withheld_authority),
                                    e
                                )
                            })?;
                    let transfer_fee_basis_points: u16 =
                        <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(transfer_fee_basis_points),
                                e
                            )
                        })?;
                    let maximum_fee: u64 =
                        <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(maximum_fee), e)
                        })?;
                    let args = InitializeTransferFeeConfigArguments {
                        transfer_fee_config_authority,
                        withdraw_withheld_authority,
                        transfer_fee_basis_points,
                        maximum_fee,
                        extension_type: "initializeTransferFeeConfig".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeTransferFeeConfigAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeTransferFeeConfig { accounts, args });
                }
                [26u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let fee: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(fee), e))?;
                    let args = TransferCheckedWithFeeArguments {
                        amount,
                        decimals,
                        fee,
                        extension_type: "transferCheckedWithFee".to_string(),
                    };
                    let source = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = TransferCheckedWithFeeAccounts {
                        remaining: vec![],
                        source: source.clone(),
                        mint: mint.clone(),
                        destination: destination.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::TransferCheckedWithFee { accounts, args });
                }
                [26u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = WithdrawWithheldTokensFromMintArguments {
                        extension_type: "withdrawWithheldTokensFromMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let fee_receiver = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "feeReceiver" , "feeReceiver" , account_index , provided_count) }) ?
                    };
                    let withdraw_withheld_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "withdrawWithheldAuthority" , "withdrawWithheldAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = WithdrawWithheldTokensFromMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        fee_receiver: fee_receiver.clone(),
                        withdraw_withheld_authority: withdraw_withheld_authority.clone(),
                    };
                    return Ok(Instruction::WithdrawWithheldTokensFromMint { accounts, args });
                }
                [26u8, 3u8] => {
                    let mut rdr: &[u8] = rest;
                    let num_token_accounts: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(num_token_accounts),
                                e
                            )
                        })?;
                    let args = WithdrawWithheldTokensFromAccountsArguments {
                        num_token_accounts,
                        extension_type: "withdrawWithheldTokensFromAccounts".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let fee_receiver = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "feeReceiver" , "feeReceiver" , account_index , provided_count) }) ?
                    };
                    let withdraw_withheld_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "withdrawWithheldAuthority" , "withdrawWithheldAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = WithdrawWithheldTokensFromAccountsAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        fee_receiver: fee_receiver.clone(),
                        withdraw_withheld_authority: withdraw_withheld_authority.clone(),
                    };
                    return Ok(Instruction::WithdrawWithheldTokensFromAccounts { accounts, args });
                }
                [26u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = HarvestWithheldTokensToMintArguments {
                        extension_type: "harvestWithheldTokensToMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = HarvestWithheldTokensToMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::HarvestWithheldTokensToMint { accounts, args });
                }
                [26u8, 5u8] => {
                    let mut rdr: &[u8] = rest;
                    let transfer_fee_basis_points: u16 =
                        <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(transfer_fee_basis_points),
                                e
                            )
                        })?;
                    let maximum_fee: u64 =
                        <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(maximum_fee), e)
                        })?;
                    let args = SetTransferFeeArguments {
                        transfer_fee_basis_points,
                        maximum_fee,
                        extension_type: "setTransferFee".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let transfer_fee_config_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "transferFeeConfigAuthority" , "transferFeeConfigAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = SetTransferFeeAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        transfer_fee_config_authority: transfer_fee_config_authority.clone(),
                    };
                    return Ok(Instruction::SetTransferFee { accounts, args });
                }
                [27u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let auto_approve_new_accounts: bool =
                        <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                            |e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(auto_approve_new_accounts),
                                    e
                                )
                            },
                        )?;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(auditor_elgamal_pubkey),
                            e
                        )
                    })?;
                    let auditor_elgamal_pubkey = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeConfidentialTransferMintArguments {
                        authority,
                        auto_approve_new_accounts,
                        auditor_elgamal_pubkey,
                        extension_type: "initializeConfidentialTransferMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeConfidentialTransferMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeConfidentialTransferMint { accounts, args });
                }
                [27u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let auto_approve_new_accounts: bool =
                        <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                            |e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(auto_approve_new_accounts),
                                    e
                                )
                            },
                        )?;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(auditor_elgamal_pubkey),
                            e
                        )
                    })?;
                    let auditor_elgamal_pubkey = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateConfidentialTransferMintArguments {
                        auto_approve_new_accounts,
                        auditor_elgamal_pubkey,
                        extension_type: "updateConfidentialTransferMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateConfidentialTransferMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::UpdateConfidentialTransferMint { accounts, args });
                }
                [27u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut decryptable_zero_balance = [0u8; 36];
                    rdr.read_exact(&mut decryptable_zero_balance).map_err(|e| {
                        format!(
                            "Failed to read 36 bytes for {}: {}",
                            stringify!(decryptable_zero_balance),
                            e
                        )
                    })?;
                    let maximum_pending_balance_credit_counter: u64 =
                        <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(maximum_pending_balance_credit_counter),
                                e
                            )
                        })?;
                    let proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(proof_instruction_offset),
                                e
                            )
                        })?;
                    let args = ConfigureConfidentialTransferAccountArguments {
                        decryptable_zero_balance,
                        maximum_pending_balance_credit_counter,
                        proof_instruction_offset,
                        extension_type: "configureConfidentialTransferAccount".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar_or_context_state = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                    };
                    let record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            4usize
                        } else {
                            4usize - 1usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ConfigureConfidentialTransferAccountAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        mint: mint.clone(),
                        instructions_sysvar_or_context_state: instructions_sysvar_or_context_state
                            .clone(),
                        record: record,
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ConfigureConfidentialTransferAccount {
                        accounts,
                        args,
                    });
                }
                [27u8, 3u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = ApproveConfidentialTransferAccountArguments {
                        extension_type: "approveConfidentialTransferAccount".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ApproveConfidentialTransferAccountAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ApproveConfidentialTransferAccount { accounts, args });
                }
                [27u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(proof_instruction_offset),
                                e
                            )
                        })?;
                    let args = EmptyConfidentialTransferAccountArguments {
                        proof_instruction_offset,
                        extension_type: "emptyConfidentialTransferAccount".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar_or_context_state = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                    };
                    let record = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count > required_count && provided_count > 2usize {
                            account_keys.get(2usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 1usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = EmptyConfidentialTransferAccountAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        instructions_sysvar_or_context_state: instructions_sysvar_or_context_state
                            .clone(),
                        record: record,
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::EmptyConfidentialTransferAccount { accounts, args });
                }
                [27u8, 5u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = ConfidentialDepositArguments {
                        amount,
                        decimals,
                        extension_type: "confidentialDeposit".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ConfidentialDepositAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ConfidentialDeposit { accounts, args });
                }
                [27u8, 6u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let mut new_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_decryptable_available_balance),
                                e
                            )
                        })?;
                    let equality_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(equality_proof_instruction_offset),
                                e
                            )
                        })?;
                    let range_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(range_proof_instruction_offset),
                                e
                            )
                        })?;
                    let args = ConfidentialWithdrawArguments {
                        amount,
                        decimals,
                        new_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        range_proof_instruction_offset,
                        extension_type: "confidentialWithdraw".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 6usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 6usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count > required_count && provided_count > 2usize {
                            account_keys.get(2usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let equality_record = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let range_record = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count > required_count && provided_count > 4usize {
                            account_keys.get(4usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 6usize {
                            5usize
                        } else {
                            5usize - 3usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ConfidentialWithdrawAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        mint: mint.clone(),
                        instructions_sysvar: instructions_sysvar,
                        equality_record: equality_record,
                        range_record: range_record,
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ConfidentialWithdraw { accounts, args });
                }
                [27u8, 7u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut new_source_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_source_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_source_decryptable_available_balance),
                                e
                            )
                        })?;
                    let equality_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(equality_proof_instruction_offset),
                                e
                            )
                        })?;
                    let ciphertext_validity_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(ciphertext_validity_proof_instruction_offset),
                                e
                            )
                        })?;
                    let range_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(range_proof_instruction_offset),
                                e
                            )
                        })?;
                    let args = ConfidentialTransferArguments {
                        new_source_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        ciphertext_validity_proof_instruction_offset,
                        range_proof_instruction_offset,
                        extension_type: "confidentialTransfer".to_string(),
                    };
                    let source_token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 8usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "sourceToken" , "sourceToken" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 8usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination_token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 8usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destinationToken" , "destinationToken" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let equality_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 4usize {
                            account_keys.get(4usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let ciphertext_validity_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 5usize {
                            account_keys.get(5usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let range_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 6usize {
                            account_keys.get(6usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 8usize {
                            7usize
                        } else {
                            7usize - 4usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ConfidentialTransferAccounts {
                        remaining: vec![],
                        source_token: source_token.clone(),
                        mint: mint.clone(),
                        destination_token: destination_token.clone(),
                        instructions_sysvar: instructions_sysvar,
                        equality_record: equality_record,
                        ciphertext_validity_record: ciphertext_validity_record,
                        range_record: range_record,
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ConfidentialTransfer { accounts, args });
                }
                [27u8, 8u8] => {
                    let mut rdr: &[u8] = rest;
                    let expected_pending_balance_credit_counter: u64 =
                        <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(expected_pending_balance_credit_counter),
                                e
                            )
                        })?;
                    let mut new_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_decryptable_available_balance),
                                e
                            )
                        })?;
                    let args = ApplyConfidentialPendingBalanceArguments {
                        expected_pending_balance_credit_counter,
                        new_decryptable_available_balance,
                        extension_type: "applyConfidentialPendingBalance".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ApplyConfidentialPendingBalanceAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ApplyConfidentialPendingBalance { accounts, args });
                }
                [27u8, 9u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = EnableConfidentialCreditsArguments {
                        extension_type: "enableConfidentialCredits".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = EnableConfidentialCreditsAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::EnableConfidentialCredits { accounts, args });
                }
                [27u8, 10u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = DisableConfidentialCreditsArguments {
                        extension_type: "disableConfidentialCredits".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = DisableConfidentialCreditsAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::DisableConfidentialCredits { accounts, args });
                }
                [27u8, 11u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = EnableNonConfidentialCreditsArguments {
                        extension_type: "enableNonConfidentialCredits".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = EnableNonConfidentialCreditsAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::EnableNonConfidentialCredits { accounts, args });
                }
                [27u8, 12u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = DisableNonConfidentialCreditsArguments {
                        extension_type: "disableNonConfidentialCredits".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = DisableNonConfidentialCreditsAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::DisableNonConfidentialCredits { accounts, args });
                }
                [27u8, 13u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut new_source_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_source_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_source_decryptable_available_balance),
                                e
                            )
                        })?;
                    let equality_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(equality_proof_instruction_offset),
                                e
                            )
                        })?;
                    let transfer_amount_ciphertext_validity_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(
                                    transfer_amount_ciphertext_validity_proof_instruction_offset
                                ),
                                e
                            )
                        })?;
                    let fee_sigma_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(fee_sigma_proof_instruction_offset),
                                e
                            )
                        })?;
                    let fee_ciphertext_validity_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(fee_ciphertext_validity_proof_instruction_offset),
                                e
                            )
                        })?;
                    let range_proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(range_proof_instruction_offset),
                                e
                            )
                        })?;
                    let args = ConfidentialTransferWithFeeArguments {
                        new_source_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        transfer_amount_ciphertext_validity_proof_instruction_offset,
                        fee_sigma_proof_instruction_offset,
                        fee_ciphertext_validity_proof_instruction_offset,
                        range_proof_instruction_offset,
                        extension_type: "confidentialTransferWithFee".to_string(),
                    };
                    let source_token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 10usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "sourceToken" , "sourceToken" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 10usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination_token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 10usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destinationToken" , "destinationToken" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let equality_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 4usize {
                            account_keys.get(4usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let transfer_amount_ciphertext_validity_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 5usize {
                            account_keys.get(5usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let fee_sigma_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 6usize {
                            account_keys.get(6usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let fee_ciphertext_validity_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 7usize {
                            account_keys.get(7usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let range_record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 8usize {
                            account_keys.get(8usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 10usize {
                            9usize
                        } else {
                            9usize - 6usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ConfidentialTransferWithFeeAccounts {
                        remaining: vec![],
                        source_token: source_token.clone(),
                        mint: mint.clone(),
                        destination_token: destination_token.clone(),
                        instructions_sysvar: instructions_sysvar,
                        equality_record: equality_record,
                        transfer_amount_ciphertext_validity_record:
                            transfer_amount_ciphertext_validity_record,
                        fee_sigma_record: fee_sigma_record,
                        fee_ciphertext_validity_record: fee_ciphertext_validity_record,
                        range_record: range_record,
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::ConfidentialTransferWithFee { accounts, args });
                }
                [28u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let state: AccountState =
                        <AccountState as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(state), e)
                            })?;
                    let args = InitializeDefaultAccountStateArguments {
                        state,
                        extension_type: "initializeDefaultAccountState".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeDefaultAccountStateAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeDefaultAccountState { accounts, args });
                }
                [28u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let state: AccountState =
                        <AccountState as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(state), e)
                            })?;
                    let args = UpdateDefaultAccountStateArguments {
                        state,
                        extension_type: "updateDefaultAccountState".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let freeze_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "freezeAuthority" , "freezeAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateDefaultAccountStateAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        freeze_authority: freeze_authority.clone(),
                    };
                    return Ok(Instruction::UpdateDefaultAccountState { accounts, args });
                }
                [29u8] => {
                    let mut rdr: &[u8] = rest;
                    let new_extension_types: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(new_extension_types),
                                e
                            )
                        })?;
                    let args = ReallocateArguments {
                        new_extension_types,
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let payer = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "payer" , "payer" , account_index , provided_count) }) ?
                    };
                    let system_program = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "systemProgram" , "systemProgram" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = ReallocateAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        payer: payer.clone(),
                        system_program: system_program.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::Reallocate { accounts, args });
                }
                [30u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = EnableMemoTransfersArguments {
                        extension_type: "enableMemoTransfers".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = EnableMemoTransfersAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::EnableMemoTransfers { accounts, args });
                }
                [30u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = DisableMemoTransfersArguments {
                        extension_type: "disableMemoTransfers".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = DisableMemoTransfersAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::DisableMemoTransfers { accounts, args });
                }
                [31u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = CreateNativeMintArguments {};
                    let payer = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "payer" , "payer" , account_index , provided_count) }) ?
                    };
                    let native_mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "nativeMint" , "nativeMint" , account_index , provided_count) }) ?
                    };
                    let system_program = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "systemProgram" , "systemProgram" , account_index , provided_count) }) ?
                    };
                    let accounts = CreateNativeMintAccounts {
                        remaining: vec![],
                        payer: payer.clone(),
                        native_mint: native_mint.clone(),
                        system_program: system_program.clone(),
                    };
                    return Ok(Instruction::CreateNativeMint { accounts, args });
                }
                [32u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeNonTransferableMintArguments {};
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeNonTransferableMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeNonTransferableMint { accounts, args });
                }
                [33u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(rate_authority),
                            e
                        )
                    })?;
                    let rate_authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(rate), e)
                        })?;
                    let args = InitializeInterestBearingMintArguments {
                        rate_authority,
                        rate,
                        extension_type: "initializeInterestBearingMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeInterestBearingMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeInterestBearingMint { accounts, args });
                }
                [33u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(rate), e)
                        })?;
                    let args = UpdateRateInterestBearingMintArguments {
                        rate,
                        extension_type: "updateRateInterestBearingMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let rate_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rateAuthority" , "rateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateRateInterestBearingMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        rate_authority: rate_authority.clone(),
                    };
                    return Ok(Instruction::UpdateRateInterestBearingMint { accounts, args });
                }
                [34u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = EnableCpiGuardArguments {
                        extension_type: "enableCpiGuard".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = EnableCpiGuardAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::EnableCpiGuard { accounts, args });
                }
                [34u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = DisableCpiGuardArguments {
                        extension_type: "disableCpiGuard".to_string(),
                    };
                    let token = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                    };
                    let owner = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                    };
                    let accounts = DisableCpiGuardAccounts {
                        remaining: vec![],
                        token: token.clone(),
                        owner: owner.clone(),
                    };
                    return Ok(Instruction::DisableCpiGuard { accounts, args });
                }
                [35u8] => {
                    let mut rdr: &[u8] = rest;
                    let delegate: [u8; 32usize] =
                        <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(delegate), e)
                            })?;
                    let args = InitializePermanentDelegateArguments { delegate };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializePermanentDelegateAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializePermanentDelegate { accounts, args });
                }
                [36u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(program_id),
                            e
                        )
                    })?;
                    let program_id = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeTransferHookArguments {
                        authority,
                        program_id,
                        extension_type: "initializeTransferHook".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeTransferHookAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeTransferHook { accounts, args });
                }
                [36u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(program_id),
                            e
                        )
                    })?;
                    let program_id = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateTransferHookArguments {
                        program_id,
                        extension_type: "updateTransferHook".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateTransferHookAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::UpdateTransferHook { accounts, args });
                }
                [37u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(withdraw_withheld_authority_el_gamal_pubkey),
                            e
                        )
                    })?;
                    let withdraw_withheld_authority_el_gamal_pubkey = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeConfidentialTransferFeeArguments {
                        authority,
                        withdraw_withheld_authority_el_gamal_pubkey,
                        extension_type: "initializeConfidentialTransferFee".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeConfidentialTransferFeeAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeConfidentialTransferFee { accounts, args });
                }
                [37u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(proof_instruction_offset),
                                e
                            )
                        })?;
                    let mut new_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_decryptable_available_balance),
                                e
                            )
                        })?;
                    let args = WithdrawWithheldTokensFromMintForConfidentialTransferFeeArguments {
                        proof_instruction_offset,
                        new_decryptable_available_balance,
                        extension_type: "withdrawWithheldTokensFromMintForConfidentialTransferFee"
                            .to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar_or_context_state = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                    };
                    let record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            4usize
                        } else {
                            4usize - 1usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts =
                        WithdrawWithheldTokensFromMintForConfidentialTransferFeeAccounts {
                            remaining: vec![],
                            mint: mint.clone(),
                            destination: destination.clone(),
                            instructions_sysvar_or_context_state:
                                instructions_sysvar_or_context_state.clone(),
                            record: record,
                            authority: authority.clone(),
                        };
                    return Ok(
                        Instruction::WithdrawWithheldTokensFromMintForConfidentialTransferFee {
                            accounts,
                            args,
                        },
                    );
                }
                [37u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let num_token_accounts: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(num_token_accounts),
                                e
                            )
                        })?;
                    let proof_instruction_offset: i8 =
                        <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(proof_instruction_offset),
                                e
                            )
                        })?;
                    let mut new_decryptable_available_balance = [0u8; 36];
                    rdr.read_exact(&mut new_decryptable_available_balance)
                        .map_err(|e| {
                            format!(
                                "Failed to read 36 bytes for {}: {}",
                                stringify!(new_decryptable_available_balance),
                                e
                            )
                        })?;
                    let args =
                        WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeArguments {
                            num_token_accounts,
                            proof_instruction_offset,
                            new_decryptable_available_balance,
                            extension_type:
                                "withdrawWithheldTokensFromAccountsForConfidentialTransferFee"
                                    .to_string(),
                        };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let destination = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                    };
                    let instructions_sysvar_or_context_state = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                    };
                    let record = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count > required_count && provided_count > 3usize {
                            account_keys.get(3usize).map(|key| key.clone())
                        } else {
                            None
                        }
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            4usize
                        } else {
                            4usize - 1usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts =
                        WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeAccounts {
                            remaining: vec![],
                            mint: mint.clone(),
                            destination: destination.clone(),
                            instructions_sysvar_or_context_state:
                                instructions_sysvar_or_context_state.clone(),
                            record: record,
                            authority: authority.clone(),
                        };
                    return Ok(
                        Instruction::WithdrawWithheldTokensFromAccountsForConfidentialTransferFee {
                            accounts,
                            args,
                        },
                    );
                }
                [37u8, 3u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = HarvestWithheldTokensToMintForConfidentialTransferFeeArguments {
                        extension_type: "harvestWithheldTokensToMintForConfidentialTransferFee"
                            .to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = HarvestWithheldTokensToMintForConfidentialTransferFeeAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(
                        Instruction::HarvestWithheldTokensToMintForConfidentialTransferFee {
                            accounts,
                            args,
                        },
                    );
                }
                [37u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = EnableHarvestToMintArguments {
                        extension_type: "enableHarvestToMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = EnableHarvestToMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::EnableHarvestToMint { accounts, args });
                }
                [37u8, 5u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = DisableHarvestToMintArguments {
                        extension_type: "disableHarvestToMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = DisableHarvestToMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::DisableHarvestToMint { accounts, args });
                }
                [38u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = WithdrawExcessLamportsArguments {};
                    let source_account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "sourceAccount" , "sourceAccount" , account_index , provided_count) }) ?
                    };
                    let destination_account = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destinationAccount" , "destinationAccount" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = WithdrawExcessLamportsAccounts {
                        remaining: vec![],
                        source_account: source_account.clone(),
                        destination_account: destination_account.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::WithdrawExcessLamports { accounts, args });
                }
                [39u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(metadata_address),
                            e
                        )
                    })?;
                    let metadata_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeMetadataPointerArguments {
                        authority,
                        metadata_address,
                        extension_type: "initializeMetadataPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeMetadataPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeMetadataPointer { accounts, args });
                }
                [39u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(metadata_address),
                            e
                        )
                    })?;
                    let metadata_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateMetadataPointerArguments {
                        metadata_address,
                        extension_type: "updateMetadataPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let metadata_pointer_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadataPointerAuthority" , "metadataPointerAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateMetadataPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        metadata_pointer_authority: metadata_pointer_authority.clone(),
                    };
                    return Ok(Instruction::UpdateMetadataPointer { accounts, args });
                }
                [40u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(group_address),
                            e
                        )
                    })?;
                    let group_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeGroupPointerArguments {
                        authority,
                        group_address,
                        extension_type: "initializeGroupPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeGroupPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeGroupPointer { accounts, args });
                }
                [40u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(group_address),
                            e
                        )
                    })?;
                    let group_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateGroupPointerArguments {
                        group_address,
                        extension_type: "updateGroupPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let group_pointer_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "groupPointerAuthority" , "groupPointerAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateGroupPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        group_pointer_authority: group_pointer_authority.clone(),
                    };
                    return Ok(Instruction::UpdateGroupPointer { accounts, args });
                }
                [41u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(member_address),
                            e
                        )
                    })?;
                    let member_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializeGroupMemberPointerArguments {
                        authority,
                        member_address,
                        extension_type: "initializeGroupMemberPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeGroupMemberPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeGroupMemberPointer { accounts, args });
                }
                [41u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(member_address),
                            e
                        )
                    })?;
                    let member_address = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateGroupMemberPointerArguments {
                        member_address,
                        extension_type: "updateGroupMemberPointer".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let group_member_pointer_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "groupMemberPointerAuthority" , "groupMemberPointerAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateGroupMemberPointerAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        group_member_pointer_authority: group_member_pointer_authority.clone(),
                    };
                    return Ok(Instruction::UpdateGroupMemberPointer { accounts, args });
                }
                [43u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let multiplier: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(multiplier), e)
                    })?;
                    let args = InitializeScaledUiAmountMintArguments {
                        authority,
                        multiplier,
                        extension_type: "initializeScaledUiAmountMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeScaledUiAmountMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializeScaledUiAmountMint { accounts, args });
                }
                [43u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let multiplier: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(multiplier), e)
                    })?;
                    let effective_timestamp: i64 =
                        <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(effective_timestamp),
                                e
                            )
                        })?;
                    let args = UpdateMultiplierScaledUiMintArguments {
                        multiplier,
                        effective_timestamp,
                        extension_type: "updateMultiplierScaledUiMint".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateMultiplierScaledUiMintAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::UpdateMultiplierScaledUiMint { accounts, args });
                }
                [44u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = InitializePausableConfigArguments {
                        authority,
                        extension_type: "initializePausableConfig".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializePausableConfigAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                    };
                    return Ok(Instruction::InitializePausableConfig { accounts, args });
                }
                [44u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = PauseArguments {
                        extension_type: "pause".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = PauseAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::Pause { accounts, args });
                }
                [44u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = ResumeArguments {
                        extension_type: "resume".to_string(),
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                    };
                    let accounts = ResumeAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        authority: authority.clone(),
                    };
                    return Ok(Instruction::Resume { accounts, args });
                }
                [210u8, 225u8, 30u8, 162u8, 88u8, 184u8, 77u8, 141u8] => {
                    let mut rdr: &[u8] = rest;
                    let len = {
                        let mut len_bytes = [0u8; 4];
                        rdr.read_exact(&mut len_bytes).map_err(|e| {
                            format!("Failed to read length for {}: {}", stringify!(name), e)
                        })?;
                        u32::from_le_bytes(len_bytes) as usize
                    };
                    let bytes = {
                        let mut bytes = vec![0u8; len];
                        rdr.read_exact(&mut bytes).map_err(|e| {
                            format!(
                                "Failed to read {} bytes for {}: {}",
                                len,
                                stringify!(name),
                                e
                            )
                        })?;
                        bytes
                    };
                    let name = String::from_utf8(bytes).map_err(|e| {
                        format!("Failed to convert {} to string: {}", stringify!(name), e)
                    })?;
                    let len = {
                        let mut len_bytes = [0u8; 4];
                        rdr.read_exact(&mut len_bytes).map_err(|e| {
                            format!("Failed to read length for {}: {}", stringify!(symbol), e)
                        })?;
                        u32::from_le_bytes(len_bytes) as usize
                    };
                    let bytes = {
                        let mut bytes = vec![0u8; len];
                        rdr.read_exact(&mut bytes).map_err(|e| {
                            format!(
                                "Failed to read {} bytes for {}: {}",
                                len,
                                stringify!(symbol),
                                e
                            )
                        })?;
                        bytes
                    };
                    let symbol = String::from_utf8(bytes).map_err(|e| {
                        format!("Failed to convert {} to string: {}", stringify!(symbol), e)
                    })?;
                    let len = {
                        let mut len_bytes = [0u8; 4];
                        rdr.read_exact(&mut len_bytes).map_err(|e| {
                            format!("Failed to read length for {}: {}", stringify!(uri), e)
                        })?;
                        u32::from_le_bytes(len_bytes) as usize
                    };
                    let bytes = {
                        let mut bytes = vec![0u8; len];
                        rdr.read_exact(&mut bytes).map_err(|e| {
                            format!(
                                "Failed to read {} bytes for {}: {}",
                                len,
                                stringify!(uri),
                                e
                            )
                        })?;
                        bytes
                    };
                    let uri = String::from_utf8(bytes).map_err(|e| {
                        format!("Failed to convert {} to string: {}", stringify!(uri), e)
                    })?;
                    let args = InitializeTokenMetadataArguments {
                        name,
                        symbol,
                        uri,
                        extension_type: "initializeTokenMetadata".to_string(),
                    };
                    let metadata = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let mint_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 4usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 4usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeTokenMetadataAccounts {
                        remaining: vec![],
                        metadata: metadata.clone(),
                        update_authority: update_authority.clone(),
                        mint: mint.clone(),
                        mint_authority: mint_authority.clone(),
                    };
                    return Ok(Instruction::InitializeTokenMetadata { accounts, args });
                }
                [221u8, 233u8, 49u8, 45u8, 181u8, 202u8, 220u8, 200u8] => {
                    let mut rdr: &[u8] = rest;
                    let field: TokenMetadataField =
                        <TokenMetadataField as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(field), e)
                            })?;
                    let len = {
                        let mut len_bytes = [0u8; 4];
                        rdr.read_exact(&mut len_bytes).map_err(|e| {
                            format!("Failed to read length for {}: {}", stringify!(value), e)
                        })?;
                        u32::from_le_bytes(len_bytes) as usize
                    };
                    let bytes = {
                        let mut bytes = vec![0u8; len];
                        rdr.read_exact(&mut bytes).map_err(|e| {
                            format!(
                                "Failed to read {} bytes for {}: {}",
                                len,
                                stringify!(value),
                                e
                            )
                        })?;
                        bytes
                    };
                    let value = String::from_utf8(bytes).map_err(|e| {
                        format!("Failed to convert {} to string: {}", stringify!(value), e)
                    })?;
                    let args = UpdateTokenMetadataFieldArguments {
                        field,
                        value,
                        extension_type: "updateTokenMetadataField".to_string(),
                    };
                    let metadata = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateTokenMetadataFieldAccounts {
                        remaining: vec![],
                        metadata: metadata.clone(),
                        update_authority: update_authority.clone(),
                    };
                    return Ok(Instruction::UpdateTokenMetadataField { accounts, args });
                }
                [234u8, 18u8, 32u8, 56u8, 89u8, 141u8, 37u8, 181u8] => {
                    let mut rdr: &[u8] = rest;
                    let idempotent: bool =
                        <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                            |e| format!("Failed to deserialize {}: {}", stringify!(idempotent), e),
                        )?;
                    let len = {
                        let mut len_bytes = [0u8; 4];
                        rdr.read_exact(&mut len_bytes).map_err(|e| {
                            format!("Failed to read length for {}: {}", stringify!(key), e)
                        })?;
                        u32::from_le_bytes(len_bytes) as usize
                    };
                    let bytes = {
                        let mut bytes = vec![0u8; len];
                        rdr.read_exact(&mut bytes).map_err(|e| {
                            format!(
                                "Failed to read {} bytes for {}: {}",
                                len,
                                stringify!(key),
                                e
                            )
                        })?;
                        bytes
                    };
                    let key = String::from_utf8(bytes).map_err(|e| {
                        format!("Failed to convert {} to string: {}", stringify!(key), e)
                    })?;
                    let args = RemoveTokenMetadataKeyArguments {
                        idempotent,
                        key,
                        extension_type: "removeTokenMetadataKey".to_string(),
                    };
                    let metadata = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = RemoveTokenMetadataKeyAccounts {
                        remaining: vec![],
                        metadata: metadata.clone(),
                        update_authority: update_authority.clone(),
                    };
                    return Ok(Instruction::RemoveTokenMetadataKey { accounts, args });
                }
                [215u8, 228u8, 166u8, 228u8, 84u8, 100u8, 86u8, 123u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(new_update_authority),
                            e
                        )
                    })?;
                    let new_update_authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateTokenMetadataUpdateAuthorityArguments {
                        new_update_authority,
                        extension_type: "updateTokenMetadataUpdateAuthority".to_string(),
                    };
                    let metadata = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateTokenMetadataUpdateAuthorityAccounts {
                        remaining: vec![],
                        metadata: metadata.clone(),
                        update_authority: update_authority.clone(),
                    };
                    return Ok(Instruction::UpdateTokenMetadataUpdateAuthority { accounts, args });
                }
                [250u8, 166u8, 180u8, 250u8, 13u8, 12u8, 184u8, 70u8] => {
                    let mut rdr: &[u8] = rest;
                    let start: Option<u64> =
                        <Option<u64> as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                            |e| format!("Failed to deserialize {}: {}", stringify!(start), e),
                        )?;
                    let end: Option<u64> = <Option<u64> as ::borsh::BorshDeserialize>::deserialize(
                        &mut rdr,
                    )
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(end), e))?;
                    let args = EmitTokenMetadataArguments {
                        start,
                        end,
                        extension_type: "emitTokenMetadata".to_string(),
                    };
                    let metadata = {
                        let provided_count = account_keys.len();
                        let required_count = 1usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 1usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                    };
                    let accounts = EmitTokenMetadataAccounts {
                        remaining: vec![],
                        metadata: metadata.clone(),
                    };
                    return Ok(Instruction::EmitTokenMetadata { accounts, args });
                }
                [121u8, 113u8, 108u8, 39u8, 54u8, 51u8, 0u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(update_authority),
                            e
                        )
                    })?;
                    let update_authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                    let args = InitializeTokenGroupArguments {
                        update_authority,
                        max_size,
                        extension_type: "initializeTokenGroup".to_string(),
                    };
                    let group = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                    };
                    let mint = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                    };
                    let mint_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 3usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 3usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeTokenGroupAccounts {
                        remaining: vec![],
                        group: group.clone(),
                        mint: mint.clone(),
                        mint_authority: mint_authority.clone(),
                    };
                    return Ok(Instruction::InitializeTokenGroup { accounts, args });
                }
                [108u8, 37u8, 171u8, 143u8, 248u8, 30u8, 18u8, 110u8] => {
                    let mut rdr: &[u8] = rest;
                    let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                    let args = UpdateTokenGroupMaxSizeArguments {
                        max_size,
                        extension_type: "updateTokenGroupMaxSize".to_string(),
                    };
                    let group = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateTokenGroupMaxSizeAccounts {
                        remaining: vec![],
                        group: group.clone(),
                        update_authority: update_authority.clone(),
                    };
                    return Ok(Instruction::UpdateTokenGroupMaxSize { accounts, args });
                }
                [161u8, 105u8, 88u8, 1u8, 237u8, 221u8, 216u8, 203u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut bytes = [0u8; 32];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(new_update_authority),
                            e
                        )
                    })?;
                    let new_update_authority = if bytes == [0u8; 32] {
                        None
                    } else {
                        Some(bytes)
                    };
                    let args = UpdateTokenGroupUpdateAuthorityArguments {
                        new_update_authority,
                        extension_type: "updateTokenGroupUpdateAuthority".to_string(),
                    };
                    let group = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                    };
                    let update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 2usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 2usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = UpdateTokenGroupUpdateAuthorityAccounts {
                        remaining: vec![],
                        group: group.clone(),
                        update_authority: update_authority.clone(),
                    };
                    return Ok(Instruction::UpdateTokenGroupUpdateAuthority { accounts, args });
                }
                [152u8, 32u8, 222u8, 176u8, 223u8, 237u8, 116u8, 134u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeTokenGroupMemberArguments {
                        extension_type: "initializeTokenGroupMember".to_string(),
                    };
                    let member = {
                        let provided_count = account_keys.len();
                        let required_count = 5usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            0usize
                        } else {
                            0usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "member" , "member" , account_index , provided_count) }) ?
                    };
                    let member_mint = {
                        let provided_count = account_keys.len();
                        let required_count = 5usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            1usize
                        } else {
                            1usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "memberMint" , "memberMint" , account_index , provided_count) }) ?
                    };
                    let member_mint_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 5usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            2usize
                        } else {
                            2usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "memberMintAuthority" , "memberMintAuthority" , account_index , provided_count) }) ?
                    };
                    let group = {
                        let provided_count = account_keys.len();
                        let required_count = 5usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            3usize
                        } else {
                            3usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                    };
                    let group_update_authority = {
                        let provided_count = account_keys.len();
                        let required_count = 5usize;
                        if provided_count < required_count {
                            return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                        }
                        let account_index = if provided_count == 5usize {
                            4usize
                        } else {
                            4usize - 0usize
                        };
                        account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "groupUpdateAuthority" , "groupUpdateAuthority" , account_index , provided_count) }) ?
                    };
                    let accounts = InitializeTokenGroupMemberAccounts {
                        remaining: vec![],
                        member: member.clone(),
                        member_mint: member_mint.clone(),
                        member_mint_authority: member_mint_authority.clone(),
                        group: group.clone(),
                        group_update_authority: group_update_authority.clone(),
                    };
                    return Ok(Instruction::InitializeTokenGroupMember { accounts, args });
                }
                _ => {}
            }
        }
        let (discriminator, rest) = data.split_at(1);
        match discriminator {
            [0u8] => {
                let mut rdr: &[u8] = rest;
                let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                    })?;
                let mint_authority: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                        |e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(mint_authority),
                                e
                            )
                        },
                    )?;
                let freeze_authority: Option<[u8; 32usize]> =
                    <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(freeze_authority),
                                e
                            )
                        })?;
                let args = InitializeMintArguments {
                    decimals,
                    mint_authority,
                    freeze_authority,
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let rent = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                };
                let accounts = InitializeMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    rent: rent.clone(),
                };
                return Ok(Instruction::InitializeMint { accounts, args });
            }
            [1u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeAccountArguments {};
                let account = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let rent = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        3usize
                    } else {
                        3usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                };
                let accounts = InitializeAccountAccounts {
                    remaining: vec![],
                    account: account.clone(),
                    mint: mint.clone(),
                    owner: owner.clone(),
                    rent: rent.clone(),
                };
                return Ok(Instruction::InitializeAccount { accounts, args });
            }
            [2u8] => {
                let mut rdr: &[u8] = rest;
                let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                let args = InitializeMultisigArguments { m };
                let multisig = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "multisig" , "multisig" , account_index , provided_count) }) ?
                };
                let rent = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                };
                let accounts = InitializeMultisigAccounts {
                    remaining: vec![],
                    multisig: multisig.clone(),
                    rent: rent.clone(),
                };
                return Ok(Instruction::InitializeMultisig { accounts, args });
            }
            [3u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let args = TransferArguments { amount };
                let source = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                };
                let destination = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = TransferAccounts {
                    remaining: vec![],
                    source: source.clone(),
                    destination: destination.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::Transfer { accounts, args });
            }
            [4u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let args = ApproveArguments { amount };
                let source = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                };
                let delegate = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "delegate" , "delegate" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = ApproveAccounts {
                    remaining: vec![],
                    source: source.clone(),
                    delegate: delegate.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::Approve { accounts, args });
            }
            [5u8] => {
                let mut rdr: &[u8] = rest;
                let args = RevokeArguments {};
                let source = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = RevokeAccounts {
                    remaining: vec![],
                    source: source.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::Revoke { accounts, args });
            }
            [6u8] => {
                let mut rdr: &[u8] = rest;
                let authority_type: AuthorityType =
                    <AuthorityType as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                        |e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(authority_type),
                                e
                            )
                        },
                    )?;
                let new_authority: Option<[u8; 32usize]> =
                    <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(new_authority), e)
                        })?;
                let args = SetAuthorityArguments {
                    authority_type,
                    new_authority,
                };
                let owned = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owned" , "owned" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = SetAuthorityAccounts {
                    remaining: vec![],
                    owned: owned.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::SetAuthority { accounts, args });
            }
            [7u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let args = MintToArguments { amount };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let mint_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                };
                let accounts = MintToAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    token: token.clone(),
                    mint_authority: mint_authority.clone(),
                };
                return Ok(Instruction::MintTo { accounts, args });
            }
            [8u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let args = BurnArguments { amount };
                let account = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = BurnAccounts {
                    remaining: vec![],
                    account: account.clone(),
                    mint: mint.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::Burn { accounts, args });
            }
            [9u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseAccountArguments {};
                let account = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                };
                let destination = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = CloseAccountAccounts {
                    remaining: vec![],
                    account: account.clone(),
                    destination: destination.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::CloseAccount { accounts, args });
            }
            [10u8] => {
                let mut rdr: &[u8] = rest;
                let args = FreezeAccountArguments {};
                let account = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = FreezeAccountAccounts {
                    remaining: vec![],
                    account: account.clone(),
                    mint: mint.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::FreezeAccount { accounts, args });
            }
            [11u8] => {
                let mut rdr: &[u8] = rest;
                let args = ThawAccountArguments {};
                let account = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = ThawAccountAccounts {
                    remaining: vec![],
                    account: account.clone(),
                    mint: mint.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::ThawAccount { accounts, args });
            }
            [12u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                    })?;
                let args = TransferCheckedArguments { amount, decimals };
                let source = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let destination = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        3usize
                    } else {
                        3usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = TransferCheckedAccounts {
                    remaining: vec![],
                    source: source.clone(),
                    mint: mint.clone(),
                    destination: destination.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::TransferChecked { accounts, args });
            }
            [13u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                    })?;
                let args = ApproveCheckedArguments { amount, decimals };
                let source = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let delegate = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "delegate" , "delegate" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        3usize
                    } else {
                        3usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = ApproveCheckedAccounts {
                    remaining: vec![],
                    source: source.clone(),
                    mint: mint.clone(),
                    delegate: delegate.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::ApproveChecked { accounts, args });
            }
            [14u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                    })?;
                let args = MintToCheckedArguments { amount, decimals };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let mint_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                };
                let accounts = MintToCheckedAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    token: token.clone(),
                    mint_authority: mint_authority.clone(),
                };
                return Ok(Instruction::MintToChecked { accounts, args });
            }
            [15u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                    })?;
                let args = BurnCheckedArguments { amount, decimals };
                let account = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = BurnCheckedAccounts {
                    remaining: vec![],
                    account: account.clone(),
                    mint: mint.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::BurnChecked { accounts, args });
            }
            [16u8] => {
                let mut rdr: &[u8] = rest;
                let owner: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                        |e| format!("Failed to deserialize {}: {}", stringify!(owner), e),
                    )?;
                let args = InitializeAccount2Arguments { owner };
                let account = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let rent = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rent" , "rent" , account_index , provided_count) }) ?
                };
                let accounts = InitializeAccount2Accounts {
                    remaining: vec![],
                    account: account.clone(),
                    mint: mint.clone(),
                    rent: rent.clone(),
                };
                return Ok(Instruction::InitializeAccount2 { accounts, args });
            }
            [17u8] => {
                let mut rdr: &[u8] = rest;
                let args = SyncNativeArguments {};
                let account = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                };
                let accounts = SyncNativeAccounts {
                    remaining: vec![],
                    account: account.clone(),
                };
                return Ok(Instruction::SyncNative { accounts, args });
            }
            [18u8] => {
                let mut rdr: &[u8] = rest;
                let owner: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                        |e| format!("Failed to deserialize {}: {}", stringify!(owner), e),
                    )?;
                let args = InitializeAccount3Arguments { owner };
                let account = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeAccount3Accounts {
                    remaining: vec![],
                    account: account.clone(),
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeAccount3 { accounts, args });
            }
            [19u8] => {
                let mut rdr: &[u8] = rest;
                let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                let args = InitializeMultisig2Arguments { m };
                let multisig = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "multisig" , "multisig" , account_index , provided_count) }) ?
                };
                let accounts = InitializeMultisig2Accounts {
                    remaining: vec![],
                    multisig: multisig.clone(),
                };
                return Ok(Instruction::InitializeMultisig2 { accounts, args });
            }
            [20u8] => {
                let mut rdr: &[u8] = rest;
                let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                    })?;
                let mint_authority: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                        |e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(mint_authority),
                                e
                            )
                        },
                    )?;
                let freeze_authority: Option<[u8; 32usize]> =
                    <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(freeze_authority),
                                e
                            )
                        })?;
                let args = InitializeMint2Arguments {
                    decimals,
                    mint_authority,
                    freeze_authority,
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeMint2Accounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeMint2 { accounts, args });
            }
            [21u8] => {
                let mut rdr: &[u8] = rest;
                let args = GetAccountDataSizeArguments {};
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = GetAccountDataSizeAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::GetAccountDataSize { accounts, args });
            }
            [22u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeImmutableOwnerArguments {};
                let account = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "account" , "account" , account_index , provided_count) }) ?
                };
                let accounts = InitializeImmutableOwnerAccounts {
                    remaining: vec![],
                    account: account.clone(),
                };
                return Ok(Instruction::InitializeImmutableOwner { accounts, args });
            }
            [23u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let args = AmountToUiAmountArguments { amount };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = AmountToUiAmountAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::AmountToUiAmount { accounts, args });
            }
            [24u8] => {
                let mut rdr: &[u8] = rest;
                let ui_amount: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(ui_amount), e)
                    })?;
                let args = UiAmountToAmountArguments { ui_amount };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = UiAmountToAmountAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::UiAmountToAmount { accounts, args });
            }
            [25u8] => {
                let mut rdr: &[u8] = rest;
                let close_authority: Option<[u8; 32usize]> =
                    <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(close_authority),
                                e
                            )
                        })?;
                let args = InitializeMintCloseAuthorityArguments { close_authority };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeMintCloseAuthorityAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeMintCloseAuthority { accounts, args });
            }
            [26u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let transfer_fee_config_authority: Option<[u8; 32usize]> =
                    <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(transfer_fee_config_authority),
                                e
                            )
                        })?;
                let withdraw_withheld_authority: Option<[u8; 32usize]> =
                    <Option<[u8; 32usize]> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(withdraw_withheld_authority),
                                e
                            )
                        })?;
                let transfer_fee_basis_points: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(transfer_fee_basis_points),
                            e
                        )
                    })?;
                let maximum_fee: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(maximum_fee), e)
                    })?;
                let args = InitializeTransferFeeConfigArguments {
                    transfer_fee_config_authority,
                    withdraw_withheld_authority,
                    transfer_fee_basis_points,
                    maximum_fee,
                    extension_type: "initializeTransferFeeConfig".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeTransferFeeConfigAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeTransferFeeConfig { accounts, args });
            }
            [26u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                    })?;
                let fee: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(fee), e))?;
                let args = TransferCheckedWithFeeArguments {
                    amount,
                    decimals,
                    fee,
                    extension_type: "transferCheckedWithFee".to_string(),
                };
                let source = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "source" , "source" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let destination = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        3usize
                    } else {
                        3usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = TransferCheckedWithFeeAccounts {
                    remaining: vec![],
                    source: source.clone(),
                    mint: mint.clone(),
                    destination: destination.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::TransferCheckedWithFee { accounts, args });
            }
            [26u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawWithheldTokensFromMintArguments {
                    extension_type: "withdrawWithheldTokensFromMint".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let fee_receiver = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "feeReceiver" , "feeReceiver" , account_index , provided_count) }) ?
                };
                let withdraw_withheld_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "withdrawWithheldAuthority" , "withdrawWithheldAuthority" , account_index , provided_count) }) ?
                };
                let accounts = WithdrawWithheldTokensFromMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    fee_receiver: fee_receiver.clone(),
                    withdraw_withheld_authority: withdraw_withheld_authority.clone(),
                };
                return Ok(Instruction::WithdrawWithheldTokensFromMint { accounts, args });
            }
            [26u8, 3u8] => {
                let mut rdr: &[u8] = rest;
                let num_token_accounts: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(num_token_accounts),
                            e
                        )
                    })?;
                let args = WithdrawWithheldTokensFromAccountsArguments {
                    num_token_accounts,
                    extension_type: "withdrawWithheldTokensFromAccounts".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let fee_receiver = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "feeReceiver" , "feeReceiver" , account_index , provided_count) }) ?
                };
                let withdraw_withheld_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "withdrawWithheldAuthority" , "withdrawWithheldAuthority" , account_index , provided_count) }) ?
                };
                let accounts = WithdrawWithheldTokensFromAccountsAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    fee_receiver: fee_receiver.clone(),
                    withdraw_withheld_authority: withdraw_withheld_authority.clone(),
                };
                return Ok(Instruction::WithdrawWithheldTokensFromAccounts { accounts, args });
            }
            [26u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let args = HarvestWithheldTokensToMintArguments {
                    extension_type: "harvestWithheldTokensToMint".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = HarvestWithheldTokensToMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::HarvestWithheldTokensToMint { accounts, args });
            }
            [26u8, 5u8] => {
                let mut rdr: &[u8] = rest;
                let transfer_fee_basis_points: u16 =
                    <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(transfer_fee_basis_points),
                            e
                        )
                    })?;
                let maximum_fee: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(maximum_fee), e)
                    })?;
                let args = SetTransferFeeArguments {
                    transfer_fee_basis_points,
                    maximum_fee,
                    extension_type: "setTransferFee".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let transfer_fee_config_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "transferFeeConfigAuthority" , "transferFeeConfigAuthority" , account_index , provided_count) }) ?
                };
                let accounts = SetTransferFeeAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    transfer_fee_config_authority: transfer_fee_config_authority.clone(),
                };
                return Ok(Instruction::SetTransferFee { accounts, args });
            }
            [27u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let auto_approve_new_accounts: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(auto_approve_new_accounts),
                            e
                        )
                    })?;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(auditor_elgamal_pubkey),
                        e
                    )
                })?;
                let auditor_elgamal_pubkey = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = InitializeConfidentialTransferMintArguments {
                    authority,
                    auto_approve_new_accounts,
                    auditor_elgamal_pubkey,
                    extension_type: "initializeConfidentialTransferMint".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeConfidentialTransferMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeConfidentialTransferMint { accounts, args });
            }
            [27u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let auto_approve_new_accounts: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(auto_approve_new_accounts),
                            e
                        )
                    })?;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(auditor_elgamal_pubkey),
                        e
                    )
                })?;
                let auditor_elgamal_pubkey = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = UpdateConfidentialTransferMintArguments {
                    auto_approve_new_accounts,
                    auditor_elgamal_pubkey,
                    extension_type: "updateConfidentialTransferMint".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateConfidentialTransferMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::UpdateConfidentialTransferMint { accounts, args });
            }
            [27u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let mut decryptable_zero_balance = [0u8; 36];
                rdr.read_exact(&mut decryptable_zero_balance).map_err(|e| {
                    format!(
                        "Failed to read 36 bytes for {}: {}",
                        stringify!(decryptable_zero_balance),
                        e
                    )
                })?;
                let maximum_pending_balance_credit_counter: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(maximum_pending_balance_credit_counter),
                            e
                        )
                    })?;
                let proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(proof_instruction_offset),
                            e
                        )
                    })?;
                let args = ConfigureConfidentialTransferAccountArguments {
                    decryptable_zero_balance,
                    maximum_pending_balance_credit_counter,
                    proof_instruction_offset,
                    extension_type: "configureConfidentialTransferAccount".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let instructions_sysvar_or_context_state = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                };
                let record = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 3usize {
                        account_keys.get(3usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        4usize
                    } else {
                        4usize - 1usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = ConfigureConfidentialTransferAccountAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    mint: mint.clone(),
                    instructions_sysvar_or_context_state: instructions_sysvar_or_context_state
                        .clone(),
                    record: record,
                    authority: authority.clone(),
                };
                return Ok(Instruction::ConfigureConfidentialTransferAccount { accounts, args });
            }
            [27u8, 3u8] => {
                let mut rdr: &[u8] = rest;
                let args = ApproveConfidentialTransferAccountArguments {
                    extension_type: "approveConfidentialTransferAccount".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = ApproveConfidentialTransferAccountAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    mint: mint.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::ApproveConfidentialTransferAccount { accounts, args });
            }
            [27u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(proof_instruction_offset),
                            e
                        )
                    })?;
                let args = EmptyConfidentialTransferAccountArguments {
                    proof_instruction_offset,
                    extension_type: "emptyConfidentialTransferAccount".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let instructions_sysvar_or_context_state = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                };
                let record = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count > required_count && provided_count > 2usize {
                        account_keys.get(2usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        3usize
                    } else {
                        3usize - 1usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = EmptyConfidentialTransferAccountAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    instructions_sysvar_or_context_state: instructions_sysvar_or_context_state
                        .clone(),
                    record: record,
                    authority: authority.clone(),
                };
                return Ok(Instruction::EmptyConfidentialTransferAccount { accounts, args });
            }
            [27u8, 5u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                    })?;
                let args = ConfidentialDepositArguments {
                    amount,
                    decimals,
                    extension_type: "confidentialDeposit".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = ConfidentialDepositAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    mint: mint.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::ConfidentialDeposit { accounts, args });
            }
            [27u8, 6u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                    })?;
                let mut new_decryptable_available_balance = [0u8; 36];
                rdr.read_exact(&mut new_decryptable_available_balance)
                    .map_err(|e| {
                        format!(
                            "Failed to read 36 bytes for {}: {}",
                            stringify!(new_decryptable_available_balance),
                            e
                        )
                    })?;
                let equality_proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(equality_proof_instruction_offset),
                            e
                        )
                    })?;
                let range_proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(range_proof_instruction_offset),
                            e
                        )
                    })?;
                let args = ConfidentialWithdrawArguments {
                    amount,
                    decimals,
                    new_decryptable_available_balance,
                    equality_proof_instruction_offset,
                    range_proof_instruction_offset,
                    extension_type: "confidentialWithdraw".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 6usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 6usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let instructions_sysvar = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count > required_count && provided_count > 2usize {
                        account_keys.get(2usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let equality_record = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count > required_count && provided_count > 3usize {
                        account_keys.get(3usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let range_record = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count > required_count && provided_count > 4usize {
                        account_keys.get(4usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 6usize {
                        5usize
                    } else {
                        5usize - 3usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = ConfidentialWithdrawAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    mint: mint.clone(),
                    instructions_sysvar: instructions_sysvar,
                    equality_record: equality_record,
                    range_record: range_record,
                    authority: authority.clone(),
                };
                return Ok(Instruction::ConfidentialWithdraw { accounts, args });
            }
            [27u8, 7u8] => {
                let mut rdr: &[u8] = rest;
                let mut new_source_decryptable_available_balance = [0u8; 36];
                rdr.read_exact(&mut new_source_decryptable_available_balance)
                    .map_err(|e| {
                        format!(
                            "Failed to read 36 bytes for {}: {}",
                            stringify!(new_source_decryptable_available_balance),
                            e
                        )
                    })?;
                let equality_proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(equality_proof_instruction_offset),
                            e
                        )
                    })?;
                let ciphertext_validity_proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(ciphertext_validity_proof_instruction_offset),
                            e
                        )
                    })?;
                let range_proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(range_proof_instruction_offset),
                            e
                        )
                    })?;
                let args = ConfidentialTransferArguments {
                    new_source_decryptable_available_balance,
                    equality_proof_instruction_offset,
                    ciphertext_validity_proof_instruction_offset,
                    range_proof_instruction_offset,
                    extension_type: "confidentialTransfer".to_string(),
                };
                let source_token = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 8usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "sourceToken" , "sourceToken" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 8usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let destination_token = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 8usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destinationToken" , "destinationToken" , account_index , provided_count) }) ?
                };
                let instructions_sysvar = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 3usize {
                        account_keys.get(3usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let equality_record = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 4usize {
                        account_keys.get(4usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let ciphertext_validity_record = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 5usize {
                        account_keys.get(5usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let range_record = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 6usize {
                        account_keys.get(6usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 8usize {
                        7usize
                    } else {
                        7usize - 4usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = ConfidentialTransferAccounts {
                    remaining: vec![],
                    source_token: source_token.clone(),
                    mint: mint.clone(),
                    destination_token: destination_token.clone(),
                    instructions_sysvar: instructions_sysvar,
                    equality_record: equality_record,
                    ciphertext_validity_record: ciphertext_validity_record,
                    range_record: range_record,
                    authority: authority.clone(),
                };
                return Ok(Instruction::ConfidentialTransfer { accounts, args });
            }
            [27u8, 8u8] => {
                let mut rdr: &[u8] = rest;
                let expected_pending_balance_credit_counter: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(expected_pending_balance_credit_counter),
                            e
                        )
                    })?;
                let mut new_decryptable_available_balance = [0u8; 36];
                rdr.read_exact(&mut new_decryptable_available_balance)
                    .map_err(|e| {
                        format!(
                            "Failed to read 36 bytes for {}: {}",
                            stringify!(new_decryptable_available_balance),
                            e
                        )
                    })?;
                let args = ApplyConfidentialPendingBalanceArguments {
                    expected_pending_balance_credit_counter,
                    new_decryptable_available_balance,
                    extension_type: "applyConfidentialPendingBalance".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = ApplyConfidentialPendingBalanceAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::ApplyConfidentialPendingBalance { accounts, args });
            }
            [27u8, 9u8] => {
                let mut rdr: &[u8] = rest;
                let args = EnableConfidentialCreditsArguments {
                    extension_type: "enableConfidentialCredits".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = EnableConfidentialCreditsAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::EnableConfidentialCredits { accounts, args });
            }
            [27u8, 10u8] => {
                let mut rdr: &[u8] = rest;
                let args = DisableConfidentialCreditsArguments {
                    extension_type: "disableConfidentialCredits".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = DisableConfidentialCreditsAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::DisableConfidentialCredits { accounts, args });
            }
            [27u8, 11u8] => {
                let mut rdr: &[u8] = rest;
                let args = EnableNonConfidentialCreditsArguments {
                    extension_type: "enableNonConfidentialCredits".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = EnableNonConfidentialCreditsAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::EnableNonConfidentialCredits { accounts, args });
            }
            [27u8, 12u8] => {
                let mut rdr: &[u8] = rest;
                let args = DisableNonConfidentialCreditsArguments {
                    extension_type: "disableNonConfidentialCredits".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = DisableNonConfidentialCreditsAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::DisableNonConfidentialCredits { accounts, args });
            }
            [27u8, 13u8] => {
                let mut rdr: &[u8] = rest;
                let mut new_source_decryptable_available_balance = [0u8; 36];
                rdr.read_exact(&mut new_source_decryptable_available_balance)
                    .map_err(|e| {
                        format!(
                            "Failed to read 36 bytes for {}: {}",
                            stringify!(new_source_decryptable_available_balance),
                            e
                        )
                    })?;
                let equality_proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(equality_proof_instruction_offset),
                            e
                        )
                    })?;
                let transfer_amount_ciphertext_validity_proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(
                                transfer_amount_ciphertext_validity_proof_instruction_offset
                            ),
                            e
                        )
                    })?;
                let fee_sigma_proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(fee_sigma_proof_instruction_offset),
                            e
                        )
                    })?;
                let fee_ciphertext_validity_proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(fee_ciphertext_validity_proof_instruction_offset),
                            e
                        )
                    })?;
                let range_proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(range_proof_instruction_offset),
                            e
                        )
                    })?;
                let args = ConfidentialTransferWithFeeArguments {
                    new_source_decryptable_available_balance,
                    equality_proof_instruction_offset,
                    transfer_amount_ciphertext_validity_proof_instruction_offset,
                    fee_sigma_proof_instruction_offset,
                    fee_ciphertext_validity_proof_instruction_offset,
                    range_proof_instruction_offset,
                    extension_type: "confidentialTransferWithFee".to_string(),
                };
                let source_token = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 10usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "sourceToken" , "sourceToken" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 10usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let destination_token = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 10usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destinationToken" , "destinationToken" , account_index , provided_count) }) ?
                };
                let instructions_sysvar = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 3usize {
                        account_keys.get(3usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let equality_record = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 4usize {
                        account_keys.get(4usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let transfer_amount_ciphertext_validity_record = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 5usize {
                        account_keys.get(5usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let fee_sigma_record = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 6usize {
                        account_keys.get(6usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let fee_ciphertext_validity_record = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 7usize {
                        account_keys.get(7usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let range_record = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 8usize {
                        account_keys.get(8usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 10usize {
                        9usize
                    } else {
                        9usize - 6usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = ConfidentialTransferWithFeeAccounts {
                    remaining: vec![],
                    source_token: source_token.clone(),
                    mint: mint.clone(),
                    destination_token: destination_token.clone(),
                    instructions_sysvar: instructions_sysvar,
                    equality_record: equality_record,
                    transfer_amount_ciphertext_validity_record:
                        transfer_amount_ciphertext_validity_record,
                    fee_sigma_record: fee_sigma_record,
                    fee_ciphertext_validity_record: fee_ciphertext_validity_record,
                    range_record: range_record,
                    authority: authority.clone(),
                };
                return Ok(Instruction::ConfidentialTransferWithFee { accounts, args });
            }
            [28u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let state: AccountState = <AccountState as ::borsh::BorshDeserialize>::deserialize(
                    &mut rdr,
                )
                .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(state), e))?;
                let args = InitializeDefaultAccountStateArguments {
                    state,
                    extension_type: "initializeDefaultAccountState".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeDefaultAccountStateAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeDefaultAccountState { accounts, args });
            }
            [28u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let state: AccountState = <AccountState as ::borsh::BorshDeserialize>::deserialize(
                    &mut rdr,
                )
                .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(state), e))?;
                let args = UpdateDefaultAccountStateArguments {
                    state,
                    extension_type: "updateDefaultAccountState".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let freeze_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "freezeAuthority" , "freezeAuthority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateDefaultAccountStateAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    freeze_authority: freeze_authority.clone(),
                };
                return Ok(Instruction::UpdateDefaultAccountState { accounts, args });
            }
            [29u8] => {
                let mut rdr: &[u8] = rest;
                let new_extension_types: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(new_extension_types),
                            e
                        )
                    })?;
                let args = ReallocateArguments {
                    new_extension_types,
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let payer = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "payer" , "payer" , account_index , provided_count) }) ?
                };
                let system_program = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "systemProgram" , "systemProgram" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        3usize
                    } else {
                        3usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = ReallocateAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    payer: payer.clone(),
                    system_program: system_program.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::Reallocate { accounts, args });
            }
            [30u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let args = EnableMemoTransfersArguments {
                    extension_type: "enableMemoTransfers".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = EnableMemoTransfersAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::EnableMemoTransfers { accounts, args });
            }
            [30u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let args = DisableMemoTransfersArguments {
                    extension_type: "disableMemoTransfers".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = DisableMemoTransfersAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::DisableMemoTransfers { accounts, args });
            }
            [31u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateNativeMintArguments {};
                let payer = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "payer" , "payer" , account_index , provided_count) }) ?
                };
                let native_mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "nativeMint" , "nativeMint" , account_index , provided_count) }) ?
                };
                let system_program = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "systemProgram" , "systemProgram" , account_index , provided_count) }) ?
                };
                let accounts = CreateNativeMintAccounts {
                    remaining: vec![],
                    payer: payer.clone(),
                    native_mint: native_mint.clone(),
                    system_program: system_program.clone(),
                };
                return Ok(Instruction::CreateNativeMint { accounts, args });
            }
            [32u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeNonTransferableMintArguments {};
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeNonTransferableMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeNonTransferableMint { accounts, args });
            }
            [33u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(rate_authority),
                        e
                    )
                })?;
                let rate_authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(rate), e))?;
                let args = InitializeInterestBearingMintArguments {
                    rate_authority,
                    rate,
                    extension_type: "initializeInterestBearingMint".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeInterestBearingMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeInterestBearingMint { accounts, args });
            }
            [33u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(rate), e))?;
                let args = UpdateRateInterestBearingMintArguments {
                    rate,
                    extension_type: "updateRateInterestBearingMint".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let rate_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "rateAuthority" , "rateAuthority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateRateInterestBearingMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    rate_authority: rate_authority.clone(),
                };
                return Ok(Instruction::UpdateRateInterestBearingMint { accounts, args });
            }
            [34u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let args = EnableCpiGuardArguments {
                    extension_type: "enableCpiGuard".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = EnableCpiGuardAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::EnableCpiGuard { accounts, args });
            }
            [34u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let args = DisableCpiGuardArguments {
                    extension_type: "disableCpiGuard".to_string(),
                };
                let token = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "token" , "token" , account_index , provided_count) }) ?
                };
                let owner = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "owner" , "owner" , account_index , provided_count) }) ?
                };
                let accounts = DisableCpiGuardAccounts {
                    remaining: vec![],
                    token: token.clone(),
                    owner: owner.clone(),
                };
                return Ok(Instruction::DisableCpiGuard { accounts, args });
            }
            [35u8] => {
                let mut rdr: &[u8] = rest;
                let delegate: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                        |e| format!("Failed to deserialize {}: {}", stringify!(delegate), e),
                    )?;
                let args = InitializePermanentDelegateArguments { delegate };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializePermanentDelegateAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializePermanentDelegate { accounts, args });
            }
            [36u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(program_id),
                        e
                    )
                })?;
                let program_id = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = InitializeTransferHookArguments {
                    authority,
                    program_id,
                    extension_type: "initializeTransferHook".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeTransferHookAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeTransferHook { accounts, args });
            }
            [36u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(program_id),
                        e
                    )
                })?;
                let program_id = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = UpdateTransferHookArguments {
                    program_id,
                    extension_type: "updateTransferHook".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateTransferHookAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::UpdateTransferHook { accounts, args });
            }
            [37u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(withdraw_withheld_authority_el_gamal_pubkey),
                        e
                    )
                })?;
                let withdraw_withheld_authority_el_gamal_pubkey = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = InitializeConfidentialTransferFeeArguments {
                    authority,
                    withdraw_withheld_authority_el_gamal_pubkey,
                    extension_type: "initializeConfidentialTransferFee".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeConfidentialTransferFeeAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeConfidentialTransferFee { accounts, args });
            }
            [37u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(proof_instruction_offset),
                            e
                        )
                    })?;
                let mut new_decryptable_available_balance = [0u8; 36];
                rdr.read_exact(&mut new_decryptable_available_balance)
                    .map_err(|e| {
                        format!(
                            "Failed to read 36 bytes for {}: {}",
                            stringify!(new_decryptable_available_balance),
                            e
                        )
                    })?;
                let args = WithdrawWithheldTokensFromMintForConfidentialTransferFeeArguments {
                    proof_instruction_offset,
                    new_decryptable_available_balance,
                    extension_type: "withdrawWithheldTokensFromMintForConfidentialTransferFee"
                        .to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let destination = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                };
                let instructions_sysvar_or_context_state = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                };
                let record = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 3usize {
                        account_keys.get(3usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        4usize
                    } else {
                        4usize - 1usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = WithdrawWithheldTokensFromMintForConfidentialTransferFeeAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    destination: destination.clone(),
                    instructions_sysvar_or_context_state: instructions_sysvar_or_context_state
                        .clone(),
                    record: record,
                    authority: authority.clone(),
                };
                return Ok(
                    Instruction::WithdrawWithheldTokensFromMintForConfidentialTransferFee {
                        accounts,
                        args,
                    },
                );
            }
            [37u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let num_token_accounts: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(num_token_accounts),
                            e
                        )
                    })?;
                let proof_instruction_offset: i8 =
                    <i8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(proof_instruction_offset),
                            e
                        )
                    })?;
                let mut new_decryptable_available_balance = [0u8; 36];
                rdr.read_exact(&mut new_decryptable_available_balance)
                    .map_err(|e| {
                        format!(
                            "Failed to read 36 bytes for {}: {}",
                            stringify!(new_decryptable_available_balance),
                            e
                        )
                    })?;
                let args = WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeArguments {
                    num_token_accounts,
                    proof_instruction_offset,
                    new_decryptable_available_balance,
                    extension_type: "withdrawWithheldTokensFromAccountsForConfidentialTransferFee"
                        .to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let destination = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destination" , "destination" , account_index , provided_count) }) ?
                };
                let instructions_sysvar_or_context_state = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "instructionsSysvarOrContextState" , "instructionsSysvarOrContextState" , account_index , provided_count) }) ?
                };
                let record = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count > required_count && provided_count > 3usize {
                        account_keys.get(3usize).map(|key| key.clone())
                    } else {
                        None
                    }
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        4usize
                    } else {
                        4usize - 1usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts =
                    WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeAccounts {
                        remaining: vec![],
                        mint: mint.clone(),
                        destination: destination.clone(),
                        instructions_sysvar_or_context_state: instructions_sysvar_or_context_state
                            .clone(),
                        record: record,
                        authority: authority.clone(),
                    };
                return Ok(
                    Instruction::WithdrawWithheldTokensFromAccountsForConfidentialTransferFee {
                        accounts,
                        args,
                    },
                );
            }
            [37u8, 3u8] => {
                let mut rdr: &[u8] = rest;
                let args = HarvestWithheldTokensToMintForConfidentialTransferFeeArguments {
                    extension_type: "harvestWithheldTokensToMintForConfidentialTransferFee"
                        .to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = HarvestWithheldTokensToMintForConfidentialTransferFeeAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(
                    Instruction::HarvestWithheldTokensToMintForConfidentialTransferFee {
                        accounts,
                        args,
                    },
                );
            }
            [37u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let args = EnableHarvestToMintArguments {
                    extension_type: "enableHarvestToMint".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = EnableHarvestToMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::EnableHarvestToMint { accounts, args });
            }
            [37u8, 5u8] => {
                let mut rdr: &[u8] = rest;
                let args = DisableHarvestToMintArguments {
                    extension_type: "disableHarvestToMint".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = DisableHarvestToMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::DisableHarvestToMint { accounts, args });
            }
            [38u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawExcessLamportsArguments {};
                let source_account = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "sourceAccount" , "sourceAccount" , account_index , provided_count) }) ?
                };
                let destination_account = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "destinationAccount" , "destinationAccount" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = WithdrawExcessLamportsAccounts {
                    remaining: vec![],
                    source_account: source_account.clone(),
                    destination_account: destination_account.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::WithdrawExcessLamports { accounts, args });
            }
            [39u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(metadata_address),
                        e
                    )
                })?;
                let metadata_address = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = InitializeMetadataPointerArguments {
                    authority,
                    metadata_address,
                    extension_type: "initializeMetadataPointer".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeMetadataPointerAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeMetadataPointer { accounts, args });
            }
            [39u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(metadata_address),
                        e
                    )
                })?;
                let metadata_address = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = UpdateMetadataPointerArguments {
                    metadata_address,
                    extension_type: "updateMetadataPointer".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let metadata_pointer_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadataPointerAuthority" , "metadataPointerAuthority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateMetadataPointerAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    metadata_pointer_authority: metadata_pointer_authority.clone(),
                };
                return Ok(Instruction::UpdateMetadataPointer { accounts, args });
            }
            [40u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(group_address),
                        e
                    )
                })?;
                let group_address = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = InitializeGroupPointerArguments {
                    authority,
                    group_address,
                    extension_type: "initializeGroupPointer".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeGroupPointerAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeGroupPointer { accounts, args });
            }
            [40u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(group_address),
                        e
                    )
                })?;
                let group_address = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = UpdateGroupPointerArguments {
                    group_address,
                    extension_type: "updateGroupPointer".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let group_pointer_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "groupPointerAuthority" , "groupPointerAuthority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateGroupPointerAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    group_pointer_authority: group_pointer_authority.clone(),
                };
                return Ok(Instruction::UpdateGroupPointer { accounts, args });
            }
            [41u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(member_address),
                        e
                    )
                })?;
                let member_address = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = InitializeGroupMemberPointerArguments {
                    authority,
                    member_address,
                    extension_type: "initializeGroupMemberPointer".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeGroupMemberPointerAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeGroupMemberPointer { accounts, args });
            }
            [41u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(member_address),
                        e
                    )
                })?;
                let member_address = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = UpdateGroupMemberPointerArguments {
                    member_address,
                    extension_type: "updateGroupMemberPointer".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let group_member_pointer_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "groupMemberPointerAuthority" , "groupMemberPointerAuthority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateGroupMemberPointerAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    group_member_pointer_authority: group_member_pointer_authority.clone(),
                };
                return Ok(Instruction::UpdateGroupMemberPointer { accounts, args });
            }
            [43u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let multiplier: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(multiplier), e)
                    })?;
                let args = InitializeScaledUiAmountMintArguments {
                    authority,
                    multiplier,
                    extension_type: "initializeScaledUiAmountMint".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializeScaledUiAmountMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializeScaledUiAmountMint { accounts, args });
            }
            [43u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let multiplier: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(multiplier), e)
                    })?;
                let effective_timestamp: i64 =
                    <i64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(effective_timestamp),
                            e
                        )
                    })?;
                let args = UpdateMultiplierScaledUiMintArguments {
                    multiplier,
                    effective_timestamp,
                    extension_type: "updateMultiplierScaledUiMint".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateMultiplierScaledUiMintAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::UpdateMultiplierScaledUiMint { accounts, args });
            }
            [44u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = InitializePausableConfigArguments {
                    authority,
                    extension_type: "initializePausableConfig".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let accounts = InitializePausableConfigAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                };
                return Ok(Instruction::InitializePausableConfig { accounts, args });
            }
            [44u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let args = PauseArguments {
                    extension_type: "pause".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = PauseAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::Pause { accounts, args });
            }
            [44u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let args = ResumeArguments {
                    extension_type: "resume".to_string(),
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "authority" , "authority" , account_index , provided_count) }) ?
                };
                let accounts = ResumeAccounts {
                    remaining: vec![],
                    mint: mint.clone(),
                    authority: authority.clone(),
                };
                return Ok(Instruction::Resume { accounts, args });
            }
            [210u8, 225u8, 30u8, 162u8, 88u8, 184u8, 77u8, 141u8] => {
                let mut rdr: &[u8] = rest;
                let len = {
                    let mut len_bytes = [0u8; 4];
                    rdr.read_exact(&mut len_bytes).map_err(|e| {
                        format!("Failed to read length for {}: {}", stringify!(name), e)
                    })?;
                    u32::from_le_bytes(len_bytes) as usize
                };
                let bytes = {
                    let mut bytes = vec![0u8; len];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read {} bytes for {}: {}",
                            len,
                            stringify!(name),
                            e
                        )
                    })?;
                    bytes
                };
                let name = String::from_utf8(bytes).map_err(|e| {
                    format!("Failed to convert {} to string: {}", stringify!(name), e)
                })?;
                let len = {
                    let mut len_bytes = [0u8; 4];
                    rdr.read_exact(&mut len_bytes).map_err(|e| {
                        format!("Failed to read length for {}: {}", stringify!(symbol), e)
                    })?;
                    u32::from_le_bytes(len_bytes) as usize
                };
                let bytes = {
                    let mut bytes = vec![0u8; len];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read {} bytes for {}: {}",
                            len,
                            stringify!(symbol),
                            e
                        )
                    })?;
                    bytes
                };
                let symbol = String::from_utf8(bytes).map_err(|e| {
                    format!("Failed to convert {} to string: {}", stringify!(symbol), e)
                })?;
                let len = {
                    let mut len_bytes = [0u8; 4];
                    rdr.read_exact(&mut len_bytes).map_err(|e| {
                        format!("Failed to read length for {}: {}", stringify!(uri), e)
                    })?;
                    u32::from_le_bytes(len_bytes) as usize
                };
                let bytes = {
                    let mut bytes = vec![0u8; len];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read {} bytes for {}: {}",
                            len,
                            stringify!(uri),
                            e
                        )
                    })?;
                    bytes
                };
                let uri = String::from_utf8(bytes).map_err(|e| {
                    format!("Failed to convert {} to string: {}", stringify!(uri), e)
                })?;
                let args = InitializeTokenMetadataArguments {
                    name,
                    symbol,
                    uri,
                    extension_type: "initializeTokenMetadata".to_string(),
                };
                let metadata = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                };
                let update_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let mint_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 4usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 4usize {
                        3usize
                    } else {
                        3usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                };
                let accounts = InitializeTokenMetadataAccounts {
                    remaining: vec![],
                    metadata: metadata.clone(),
                    update_authority: update_authority.clone(),
                    mint: mint.clone(),
                    mint_authority: mint_authority.clone(),
                };
                return Ok(Instruction::InitializeTokenMetadata { accounts, args });
            }
            [221u8, 233u8, 49u8, 45u8, 181u8, 202u8, 220u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let field: TokenMetadataField =
                    <TokenMetadataField as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(field), e)
                        })?;
                let len = {
                    let mut len_bytes = [0u8; 4];
                    rdr.read_exact(&mut len_bytes).map_err(|e| {
                        format!("Failed to read length for {}: {}", stringify!(value), e)
                    })?;
                    u32::from_le_bytes(len_bytes) as usize
                };
                let bytes = {
                    let mut bytes = vec![0u8; len];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read {} bytes for {}: {}",
                            len,
                            stringify!(value),
                            e
                        )
                    })?;
                    bytes
                };
                let value = String::from_utf8(bytes).map_err(|e| {
                    format!("Failed to convert {} to string: {}", stringify!(value), e)
                })?;
                let args = UpdateTokenMetadataFieldArguments {
                    field,
                    value,
                    extension_type: "updateTokenMetadataField".to_string(),
                };
                let metadata = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                };
                let update_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateTokenMetadataFieldAccounts {
                    remaining: vec![],
                    metadata: metadata.clone(),
                    update_authority: update_authority.clone(),
                };
                return Ok(Instruction::UpdateTokenMetadataField { accounts, args });
            }
            [234u8, 18u8, 32u8, 56u8, 89u8, 141u8, 37u8, 181u8] => {
                let mut rdr: &[u8] = rest;
                let idempotent: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                    format!("Failed to deserialize {}: {}", stringify!(idempotent), e)
                })?;
                let len = {
                    let mut len_bytes = [0u8; 4];
                    rdr.read_exact(&mut len_bytes).map_err(|e| {
                        format!("Failed to read length for {}: {}", stringify!(key), e)
                    })?;
                    u32::from_le_bytes(len_bytes) as usize
                };
                let bytes = {
                    let mut bytes = vec![0u8; len];
                    rdr.read_exact(&mut bytes).map_err(|e| {
                        format!(
                            "Failed to read {} bytes for {}: {}",
                            len,
                            stringify!(key),
                            e
                        )
                    })?;
                    bytes
                };
                let key = String::from_utf8(bytes).map_err(|e| {
                    format!("Failed to convert {} to string: {}", stringify!(key), e)
                })?;
                let args = RemoveTokenMetadataKeyArguments {
                    idempotent,
                    key,
                    extension_type: "removeTokenMetadataKey".to_string(),
                };
                let metadata = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                };
                let update_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                };
                let accounts = RemoveTokenMetadataKeyAccounts {
                    remaining: vec![],
                    metadata: metadata.clone(),
                    update_authority: update_authority.clone(),
                };
                return Ok(Instruction::RemoveTokenMetadataKey { accounts, args });
            }
            [215u8, 228u8, 166u8, 228u8, 84u8, 100u8, 86u8, 123u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(new_update_authority),
                        e
                    )
                })?;
                let new_update_authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = UpdateTokenMetadataUpdateAuthorityArguments {
                    new_update_authority,
                    extension_type: "updateTokenMetadataUpdateAuthority".to_string(),
                };
                let metadata = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                };
                let update_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateTokenMetadataUpdateAuthorityAccounts {
                    remaining: vec![],
                    metadata: metadata.clone(),
                    update_authority: update_authority.clone(),
                };
                return Ok(Instruction::UpdateTokenMetadataUpdateAuthority { accounts, args });
            }
            [250u8, 166u8, 180u8, 250u8, 13u8, 12u8, 184u8, 70u8] => {
                let mut rdr: &[u8] = rest;
                let start: Option<u64> = <Option<u64> as ::borsh::BorshDeserialize>::deserialize(
                    &mut rdr,
                )
                .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(start), e))?;
                let end: Option<u64> =
                    <Option<u64> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(end), e))?;
                let args = EmitTokenMetadataArguments {
                    start,
                    end,
                    extension_type: "emitTokenMetadata".to_string(),
                };
                let metadata = {
                    let provided_count = account_keys.len();
                    let required_count = 1usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 1usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "metadata" , "metadata" , account_index , provided_count) }) ?
                };
                let accounts = EmitTokenMetadataAccounts {
                    remaining: vec![],
                    metadata: metadata.clone(),
                };
                return Ok(Instruction::EmitTokenMetadata { accounts, args });
            }
            [121u8, 113u8, 108u8, 39u8, 54u8, 51u8, 0u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(update_authority),
                        e
                    )
                })?;
                let update_authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                let args = InitializeTokenGroupArguments {
                    update_authority,
                    max_size,
                    extension_type: "initializeTokenGroup".to_string(),
                };
                let group = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                };
                let mint = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mint" , "mint" , account_index , provided_count) }) ?
                };
                let mint_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 3usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 3usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "mintAuthority" , "mintAuthority" , account_index , provided_count) }) ?
                };
                let accounts = InitializeTokenGroupAccounts {
                    remaining: vec![],
                    group: group.clone(),
                    mint: mint.clone(),
                    mint_authority: mint_authority.clone(),
                };
                return Ok(Instruction::InitializeTokenGroup { accounts, args });
            }
            [108u8, 37u8, 171u8, 143u8, 248u8, 30u8, 18u8, 110u8] => {
                let mut rdr: &[u8] = rest;
                let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                let args = UpdateTokenGroupMaxSizeArguments {
                    max_size,
                    extension_type: "updateTokenGroupMaxSize".to_string(),
                };
                let group = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                };
                let update_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateTokenGroupMaxSizeAccounts {
                    remaining: vec![],
                    group: group.clone(),
                    update_authority: update_authority.clone(),
                };
                return Ok(Instruction::UpdateTokenGroupMaxSize { accounts, args });
            }
            [161u8, 105u8, 88u8, 1u8, 237u8, 221u8, 216u8, 203u8] => {
                let mut rdr: &[u8] = rest;
                let mut bytes = [0u8; 32];
                rdr.read_exact(&mut bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(new_update_authority),
                        e
                    )
                })?;
                let new_update_authority = if bytes == [0u8; 32] {
                    None
                } else {
                    Some(bytes)
                };
                let args = UpdateTokenGroupUpdateAuthorityArguments {
                    new_update_authority,
                    extension_type: "updateTokenGroupUpdateAuthority".to_string(),
                };
                let group = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                };
                let update_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 2usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 2usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "updateAuthority" , "updateAuthority" , account_index , provided_count) }) ?
                };
                let accounts = UpdateTokenGroupUpdateAuthorityAccounts {
                    remaining: vec![],
                    group: group.clone(),
                    update_authority: update_authority.clone(),
                };
                return Ok(Instruction::UpdateTokenGroupUpdateAuthority { accounts, args });
            }
            [152u8, 32u8, 222u8, 176u8, 223u8, 237u8, 116u8, 134u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeTokenGroupMemberArguments {
                    extension_type: "initializeTokenGroupMember".to_string(),
                };
                let member = {
                    let provided_count = account_keys.len();
                    let required_count = 5usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        0usize
                    } else {
                        0usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "member" , "member" , account_index , provided_count) }) ?
                };
                let member_mint = {
                    let provided_count = account_keys.len();
                    let required_count = 5usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        1usize
                    } else {
                        1usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "memberMint" , "memberMint" , account_index , provided_count) }) ?
                };
                let member_mint_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 5usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        2usize
                    } else {
                        2usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "memberMintAuthority" , "memberMintAuthority" , account_index , provided_count) }) ?
                };
                let group = {
                    let provided_count = account_keys.len();
                    let required_count = 5usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        3usize
                    } else {
                        3usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "group" , "group" , account_index , provided_count) }) ?
                };
                let group_update_authority = {
                    let provided_count = account_keys.len();
                    let required_count = 5usize;
                    if provided_count < required_count {
                        return Err (format ! ("Insufficient accounts: provided {}, need at least {} required accounts" , provided_count , required_count)) ;
                    }
                    let account_index = if provided_count == 5usize {
                        4usize
                    } else {
                        4usize - 0usize
                    };
                    account_keys . get (account_index) . ok_or_else (|| { format ! ("Missing required account {}: {} at calculated index {} (provided {} accounts)" , "groupUpdateAuthority" , "groupUpdateAuthority" , account_index , provided_count) }) ?
                };
                let accounts = InitializeTokenGroupMemberAccounts {
                    remaining: vec![],
                    member: member.clone(),
                    member_mint: member_mint.clone(),
                    member_mint_authority: member_mint_authority.clone(),
                    group: group.clone(),
                    group_update_authority: group_update_authority.clone(),
                };
                return Ok(Instruction::InitializeTokenGroupMember { accounts, args });
            }
            _ => Err(format!(
                "Unknown instruction discriminator: {:?}",
                discriminator
            )),
        }
    }
}
