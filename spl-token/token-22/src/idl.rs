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
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeMintArguments {
    pub decimals: u8,
    #[serde(with = "pubkey_serde")]
    pub mint_authority: [u8; 32usize],
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
    pub close_authority: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeTransferFeeConfigArguments {
    pub transfer_fee_config_authority: Option<[u8; 32usize]>,
    pub withdraw_withheld_authority: Option<[u8; 32usize]>,
    pub transfer_fee_basis_points: u16,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub maximum_fee: u64,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct TransferCheckedWithFeeArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
    pub decimals: u8,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub fee: u64,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct WithdrawWithheldTokensFromMintArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct WithdrawWithheldTokensFromAccountsArguments {
    pub num_token_accounts: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct HarvestWithheldTokensToMintArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct SetTransferFeeArguments {
    pub transfer_fee_basis_points: u16,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub maximum_fee: u64,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeConfidentialTransferMintArguments {
    pub confidential_transfer_discriminator: u8,
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    pub auto_approve_new_accounts: bool,
    #[serde(with = "pubkey_serde_option")]
    pub auditor_elgamal_pubkey: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateConfidentialTransferMintArguments {
    pub confidential_transfer_discriminator: u8,
    pub auto_approve_new_accounts: bool,
    #[serde(with = "pubkey_serde_option")]
    pub auditor_elgamal_pubkey: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ConfigureConfidentialTransferAccountArguments {
    pub confidential_transfer_discriminator: u8,
    pub decryptable_zero_balance: DecryptableBalance,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub maximum_pending_balance_credit_counter: u64,
    pub proof_instruction_offset: i8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ApproveConfidentialTransferAccountArguments {
    pub confidential_transfer_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EmptyConfidentialTransferAccountArguments {
    pub confidential_transfer_discriminator: u8,
    pub proof_instruction_offset: i8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ConfidentialDepositArguments {
    pub confidential_transfer_discriminator: u8,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
    pub decimals: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ConfidentialWithdrawArguments {
    pub confidential_transfer_discriminator: u8,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub amount: u64,
    pub decimals: u8,
    pub new_decryptable_available_balance: DecryptableBalance,
    pub equality_proof_instruction_offset: i8,
    pub range_proof_instruction_offset: i8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ConfidentialTransferArguments {
    pub confidential_transfer_discriminator: u8,
    pub new_source_decryptable_available_balance: DecryptableBalance,
    pub equality_proof_instruction_offset: i8,
    pub ciphertext_validity_proof_instruction_offset: i8,
    pub range_proof_instruction_offset: i8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ApplyConfidentialPendingBalanceArguments {
    pub confidential_transfer_discriminator: u8,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub expected_pending_balance_credit_counter: u64,
    pub new_decryptable_available_balance: DecryptableBalance,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EnableConfidentialCreditsArguments {
    pub confidential_transfer_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct DisableConfidentialCreditsArguments {
    pub confidential_transfer_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EnableNonConfidentialCreditsArguments {
    pub confidential_transfer_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct DisableNonConfidentialCreditsArguments {
    pub confidential_transfer_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ConfidentialTransferWithFeeArguments {
    pub confidential_transfer_discriminator: u8,
    pub new_source_decryptable_available_balance: DecryptableBalance,
    pub equality_proof_instruction_offset: i8,
    pub transfer_amount_ciphertext_validity_proof_instruction_offset: i8,
    pub fee_sigma_proof_instruction_offset: i8,
    pub fee_ciphertext_validity_proof_instruction_offset: i8,
    pub range_proof_instruction_offset: i8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeDefaultAccountStateArguments {
    pub default_account_state_discriminator: u8,
    pub state: AccountState,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateDefaultAccountStateArguments {
    pub default_account_state_discriminator: u8,
    pub state: AccountState,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ReallocateArguments {
    pub new_extension_types: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EnableMemoTransfersArguments {
    pub memo_transfers_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct DisableMemoTransfersArguments {
    pub memo_transfers_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct CreateNativeMintArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeNonTransferableMintArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeInterestBearingMintArguments {
    pub interest_bearing_mint_discriminator: u8,
    #[serde(with = "pubkey_serde_option")]
    pub rate_authority: Option<[u8; 32usize]>,
    pub rate: i16,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateRateInterestBearingMintArguments {
    pub interest_bearing_mint_discriminator: u8,
    pub rate: i16,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EnableCpiGuardArguments {
    pub cpi_guard_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct DisableCpiGuardArguments {
    pub cpi_guard_discriminator: u8,
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
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateTransferHookArguments {
    #[serde(with = "pubkey_serde_option")]
    pub program_id: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeConfidentialTransferFeeArguments {
    pub confidential_transfer_fee_discriminator: u8,
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    #[serde(with = "pubkey_serde_option")]
    pub withdraw_withheld_authority_el_gamal_pubkey: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct WithdrawWithheldTokensFromMintForConfidentialTransferFeeArguments {
    pub confidential_transfer_fee_discriminator: u8,
    pub proof_instruction_offset: i8,
    pub new_decryptable_available_balance: DecryptableBalance,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeArguments {
    pub confidential_transfer_fee_discriminator: u8,
    pub num_token_accounts: u8,
    pub proof_instruction_offset: i8,
    pub new_decryptable_available_balance: DecryptableBalance,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct HarvestWithheldTokensToMintForConfidentialTransferFeeArguments {
    pub confidential_transfer_fee_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EnableHarvestToMintArguments {
    pub confidential_transfer_fee_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct DisableHarvestToMintArguments {
    pub confidential_transfer_fee_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct WithdrawExcessLamportsArguments {}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeMetadataPointerArguments {
    pub metadata_pointer_discriminator: u8,
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    #[serde(with = "pubkey_serde_option")]
    pub metadata_address: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateMetadataPointerArguments {
    pub metadata_pointer_discriminator: u8,
    #[serde(with = "pubkey_serde_option")]
    pub metadata_address: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeGroupPointerArguments {
    pub group_pointer_discriminator: u8,
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    #[serde(with = "pubkey_serde_option")]
    pub group_address: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateGroupPointerArguments {
    pub group_pointer_discriminator: u8,
    #[serde(with = "pubkey_serde_option")]
    pub group_address: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeGroupMemberPointerArguments {
    pub group_member_pointer_discriminator: u8,
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    #[serde(with = "pubkey_serde_option")]
    pub member_address: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateGroupMemberPointerArguments {
    pub group_member_pointer_discriminator: u8,
    #[serde(with = "pubkey_serde_option")]
    pub member_address: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeScaledUiAmountMintArguments {
    pub scaled_ui_amount_mint_discriminator: u8,
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
    pub multiplier: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateMultiplierScaledUiMintArguments {
    pub scaled_ui_amount_mint_discriminator: u8,
    pub multiplier: u8,
    pub effective_timestamp: i64,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializePausableConfigArguments {
    pub pausable_discriminator: u8,
    #[serde(with = "pubkey_serde_option")]
    pub authority: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct PauseArguments {
    pub pausable_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct ResumeArguments {
    pub pausable_discriminator: u8,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeTokenMetadataArguments {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateTokenMetadataFieldArguments {
    pub field: TokenMetadataField,
    pub value: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct RemoveTokenMetadataKeyArguments {
    pub idempotent: bool,
    pub key: String,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateTokenMetadataUpdateAuthorityArguments {
    #[serde(with = "pubkey_serde_option")]
    pub new_update_authority: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct EmitTokenMetadataArguments {
    pub start: Option<u64>,
    pub end: Option<u64>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeTokenGroupArguments {
    #[serde(with = "pubkey_serde_option")]
    pub update_authority: Option<[u8; 32usize]>,
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub max_size: u64,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateTokenGroupMaxSizeArguments {
    #[serde(serialize_with = "crate::serialize_to_string")]
    pub max_size: u64,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct UpdateTokenGroupUpdateAuthorityArguments {
    #[serde(with = "pubkey_serde_option")]
    pub new_update_authority: Option<[u8; 32usize]>,
}
#[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
pub struct InitializeTokenGroupMemberArguments {}
#[derive(Debug, Serialize)]
pub struct InitializeMintAccounts {
    pub mint: String,
    pub rent: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeAccountAccounts {
    pub account: String,
    pub mint: String,
    pub owner: String,
    pub rent: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeMultisigAccounts {
    pub multisig: String,
    pub rent: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct TransferAccounts {
    pub source: String,
    pub destination: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ApproveAccounts {
    pub source: String,
    pub delegate: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct RevokeAccounts {
    pub source: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct SetAuthorityAccounts {
    pub owned: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct MintToAccounts {
    pub mint: String,
    pub token: String,
    pub mint_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct BurnAccounts {
    pub account: String,
    pub mint: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct CloseAccountAccounts {
    pub account: String,
    pub destination: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct FreezeAccountAccounts {
    pub account: String,
    pub mint: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ThawAccountAccounts {
    pub account: String,
    pub mint: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct TransferCheckedAccounts {
    pub source: String,
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ApproveCheckedAccounts {
    pub source: String,
    pub mint: String,
    pub delegate: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct MintToCheckedAccounts {
    pub mint: String,
    pub token: String,
    pub mint_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct BurnCheckedAccounts {
    pub account: String,
    pub mint: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeAccount2Accounts {
    pub account: String,
    pub mint: String,
    pub rent: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct SyncNativeAccounts {
    pub account: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeAccount3Accounts {
    pub account: String,
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeMultisig2Accounts {
    pub multisig: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeMint2Accounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct GetAccountDataSizeAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeImmutableOwnerAccounts {
    pub account: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct AmountToUiAmountAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UiAmountToAmountAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeMintCloseAuthorityAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeTransferFeeConfigAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct TransferCheckedWithFeeAccounts {
    pub source: String,
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct WithdrawWithheldTokensFromMintAccounts {
    pub mint: String,
    pub fee_receiver: String,
    pub withdraw_withheld_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct WithdrawWithheldTokensFromAccountsAccounts {
    pub mint: String,
    pub fee_receiver: String,
    pub withdraw_withheld_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct HarvestWithheldTokensToMintAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct SetTransferFeeAccounts {
    pub mint: String,
    pub transfer_fee_config_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeConfidentialTransferMintAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateConfidentialTransferMintAccounts {
    pub mint: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ConfigureConfidentialTransferAccountAccounts {
    pub token: String,
    pub mint: String,
    pub instructions_sysvar_or_context_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ApproveConfidentialTransferAccountAccounts {
    pub token: String,
    pub mint: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct EmptyConfidentialTransferAccountAccounts {
    pub token: String,
    pub instructions_sysvar_or_context_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ConfidentialDepositAccounts {
    pub token: String,
    pub mint: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ConfidentialWithdrawAccounts {
    pub token: String,
    pub mint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_sysvar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equality_record: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_record: Option<String>,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ConfidentialTransferAccounts {
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
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ApplyConfidentialPendingBalanceAccounts {
    pub token: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct EnableConfidentialCreditsAccounts {
    pub token: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct DisableConfidentialCreditsAccounts {
    pub token: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct EnableNonConfidentialCreditsAccounts {
    pub token: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct DisableNonConfidentialCreditsAccounts {
    pub token: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ConfidentialTransferWithFeeAccounts {
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
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeDefaultAccountStateAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateDefaultAccountStateAccounts {
    pub mint: String,
    pub freeze_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ReallocateAccounts {
    pub token: String,
    pub payer: String,
    pub system_program: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct EnableMemoTransfersAccounts {
    pub token: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct DisableMemoTransfersAccounts {
    pub token: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct CreateNativeMintAccounts {
    pub payer: String,
    pub native_mint: String,
    pub system_program: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeNonTransferableMintAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeInterestBearingMintAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateRateInterestBearingMintAccounts {
    pub mint: String,
    pub rate_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct EnableCpiGuardAccounts {
    pub token: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct DisableCpiGuardAccounts {
    pub token: String,
    pub owner: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializePermanentDelegateAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeTransferHookAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateTransferHookAccounts {
    pub mint: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeConfidentialTransferFeeAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct WithdrawWithheldTokensFromMintForConfidentialTransferFeeAccounts {
    pub mint: String,
    pub destination: String,
    pub instructions_sysvar_or_context_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeAccounts {
    pub mint: String,
    pub destination: String,
    pub instructions_sysvar_or_context_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct HarvestWithheldTokensToMintForConfidentialTransferFeeAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct EnableHarvestToMintAccounts {
    pub mint: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct DisableHarvestToMintAccounts {
    pub mint: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct WithdrawExcessLamportsAccounts {
    pub source_account: String,
    pub destination_account: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeMetadataPointerAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateMetadataPointerAccounts {
    pub mint: String,
    pub metadata_pointer_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeGroupPointerAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateGroupPointerAccounts {
    pub mint: String,
    pub group_pointer_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeGroupMemberPointerAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateGroupMemberPointerAccounts {
    pub mint: String,
    pub group_member_pointer_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeScaledUiAmountMintAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateMultiplierScaledUiMintAccounts {
    pub mint: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializePausableConfigAccounts {
    pub mint: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct PauseAccounts {
    pub mint: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct ResumeAccounts {
    pub mint: String,
    pub authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeTokenMetadataAccounts {
    pub metadata: String,
    pub update_authority: String,
    pub mint: String,
    pub mint_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateTokenMetadataFieldAccounts {
    pub metadata: String,
    pub update_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct RemoveTokenMetadataKeyAccounts {
    pub metadata: String,
    pub update_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateTokenMetadataUpdateAuthorityAccounts {
    pub metadata: String,
    pub update_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct EmitTokenMetadataAccounts {
    pub metadata: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeTokenGroupAccounts {
    pub group: String,
    pub mint: String,
    pub mint_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateTokenGroupMaxSizeAccounts {
    pub group: String,
    pub update_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct UpdateTokenGroupUpdateAuthorityAccounts {
    pub group: String,
    pub update_authority: String,
    pub remaining: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct InitializeTokenGroupMemberAccounts {
    pub member: String,
    pub member_mint: String,
    pub member_mint_authority: String,
    pub group: String,
    pub group_update_authority: String,
    pub remaining: Vec<String>,
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
    InitializeTransferFeeConfig {
        accounts: InitializeTransferFeeConfigAccounts,
        args: InitializeTransferFeeConfigArguments,
    },
    TransferCheckedWithFee {
        accounts: TransferCheckedWithFeeAccounts,
        args: TransferCheckedWithFeeArguments,
    },
    WithdrawWithheldTokensFromMint {
        accounts: WithdrawWithheldTokensFromMintAccounts,
        args: WithdrawWithheldTokensFromMintArguments,
    },
    WithdrawWithheldTokensFromAccounts {
        accounts: WithdrawWithheldTokensFromAccountsAccounts,
        args: WithdrawWithheldTokensFromAccountsArguments,
    },
    HarvestWithheldTokensToMint {
        accounts: HarvestWithheldTokensToMintAccounts,
        args: HarvestWithheldTokensToMintArguments,
    },
    SetTransferFee {
        accounts: SetTransferFeeAccounts,
        args: SetTransferFeeArguments,
    },
    InitializeConfidentialTransferMint {
        accounts: InitializeConfidentialTransferMintAccounts,
        args: InitializeConfidentialTransferMintArguments,
    },
    UpdateConfidentialTransferMint {
        accounts: UpdateConfidentialTransferMintAccounts,
        args: UpdateConfidentialTransferMintArguments,
    },
    ConfigureConfidentialTransferAccount {
        accounts: ConfigureConfidentialTransferAccountAccounts,
        args: ConfigureConfidentialTransferAccountArguments,
    },
    ApproveConfidentialTransferAccount {
        accounts: ApproveConfidentialTransferAccountAccounts,
        args: ApproveConfidentialTransferAccountArguments,
    },
    EmptyConfidentialTransferAccount {
        accounts: EmptyConfidentialTransferAccountAccounts,
        args: EmptyConfidentialTransferAccountArguments,
    },
    ConfidentialDeposit {
        accounts: ConfidentialDepositAccounts,
        args: ConfidentialDepositArguments,
    },
    ConfidentialWithdraw {
        accounts: ConfidentialWithdrawAccounts,
        args: ConfidentialWithdrawArguments,
    },
    ConfidentialTransfer {
        accounts: ConfidentialTransferAccounts,
        args: ConfidentialTransferArguments,
    },
    ApplyConfidentialPendingBalance {
        accounts: ApplyConfidentialPendingBalanceAccounts,
        args: ApplyConfidentialPendingBalanceArguments,
    },
    EnableConfidentialCredits {
        accounts: EnableConfidentialCreditsAccounts,
        args: EnableConfidentialCreditsArguments,
    },
    DisableConfidentialCredits {
        accounts: DisableConfidentialCreditsAccounts,
        args: DisableConfidentialCreditsArguments,
    },
    EnableNonConfidentialCredits {
        accounts: EnableNonConfidentialCreditsAccounts,
        args: EnableNonConfidentialCreditsArguments,
    },
    DisableNonConfidentialCredits {
        accounts: DisableNonConfidentialCreditsAccounts,
        args: DisableNonConfidentialCreditsArguments,
    },
    ConfidentialTransferWithFee {
        accounts: ConfidentialTransferWithFeeAccounts,
        args: ConfidentialTransferWithFeeArguments,
    },
    InitializeDefaultAccountState {
        accounts: InitializeDefaultAccountStateAccounts,
        args: InitializeDefaultAccountStateArguments,
    },
    UpdateDefaultAccountState {
        accounts: UpdateDefaultAccountStateAccounts,
        args: UpdateDefaultAccountStateArguments,
    },
    Reallocate {
        accounts: ReallocateAccounts,
        args: ReallocateArguments,
    },
    EnableMemoTransfers {
        accounts: EnableMemoTransfersAccounts,
        args: EnableMemoTransfersArguments,
    },
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
    InitializeInterestBearingMint {
        accounts: InitializeInterestBearingMintAccounts,
        args: InitializeInterestBearingMintArguments,
    },
    UpdateRateInterestBearingMint {
        accounts: UpdateRateInterestBearingMintAccounts,
        args: UpdateRateInterestBearingMintArguments,
    },
    EnableCpiGuard {
        accounts: EnableCpiGuardAccounts,
        args: EnableCpiGuardArguments,
    },
    DisableCpiGuard {
        accounts: DisableCpiGuardAccounts,
        args: DisableCpiGuardArguments,
    },
    InitializePermanentDelegate {
        accounts: InitializePermanentDelegateAccounts,
        args: InitializePermanentDelegateArguments,
    },
    InitializeTransferHook {
        accounts: InitializeTransferHookAccounts,
        args: InitializeTransferHookArguments,
    },
    UpdateTransferHook {
        accounts: UpdateTransferHookAccounts,
        args: UpdateTransferHookArguments,
    },
    InitializeConfidentialTransferFee {
        accounts: InitializeConfidentialTransferFeeAccounts,
        args: InitializeConfidentialTransferFeeArguments,
    },
    WithdrawWithheldTokensFromMintForConfidentialTransferFee {
        accounts: WithdrawWithheldTokensFromMintForConfidentialTransferFeeAccounts,
        args: WithdrawWithheldTokensFromMintForConfidentialTransferFeeArguments,
    },
    WithdrawWithheldTokensFromAccountsForConfidentialTransferFee {
        accounts: WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeAccounts,
        args: WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeArguments,
    },
    HarvestWithheldTokensToMintForConfidentialTransferFee {
        accounts: HarvestWithheldTokensToMintForConfidentialTransferFeeAccounts,
        args: HarvestWithheldTokensToMintForConfidentialTransferFeeArguments,
    },
    EnableHarvestToMint {
        accounts: EnableHarvestToMintAccounts,
        args: EnableHarvestToMintArguments,
    },
    DisableHarvestToMint {
        accounts: DisableHarvestToMintAccounts,
        args: DisableHarvestToMintArguments,
    },
    WithdrawExcessLamports {
        accounts: WithdrawExcessLamportsAccounts,
        args: WithdrawExcessLamportsArguments,
    },
    InitializeMetadataPointer {
        accounts: InitializeMetadataPointerAccounts,
        args: InitializeMetadataPointerArguments,
    },
    UpdateMetadataPointer {
        accounts: UpdateMetadataPointerAccounts,
        args: UpdateMetadataPointerArguments,
    },
    InitializeGroupPointer {
        accounts: InitializeGroupPointerAccounts,
        args: InitializeGroupPointerArguments,
    },
    UpdateGroupPointer {
        accounts: UpdateGroupPointerAccounts,
        args: UpdateGroupPointerArguments,
    },
    InitializeGroupMemberPointer {
        accounts: InitializeGroupMemberPointerAccounts,
        args: InitializeGroupMemberPointerArguments,
    },
    UpdateGroupMemberPointer {
        accounts: UpdateGroupMemberPointerAccounts,
        args: UpdateGroupMemberPointerArguments,
    },
    InitializeScaledUiAmountMint {
        accounts: InitializeScaledUiAmountMintAccounts,
        args: InitializeScaledUiAmountMintArguments,
    },
    UpdateMultiplierScaledUiMint {
        accounts: UpdateMultiplierScaledUiMintAccounts,
        args: UpdateMultiplierScaledUiMintArguments,
    },
    InitializePausableConfig {
        accounts: InitializePausableConfigAccounts,
        args: InitializePausableConfigArguments,
    },
    Pause {
        accounts: PauseAccounts,
        args: PauseArguments,
    },
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let rent = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMintAccounts {
                        remaining,
                        mint,
                        rent,
                    };
                    return Ok(Instruction::InitializeMint { accounts, args });
                }
                [1u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeAccountArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let rent = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeAccountAccounts {
                        remaining,
                        account,
                        mint,
                        owner,
                        rent,
                    };
                    return Ok(Instruction::InitializeAccount { accounts, args });
                }
                [2u8] => {
                    let mut rdr: &[u8] = rest;
                    let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                    let args = InitializeMultisigArguments { m };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let multisig = keys.next().unwrap().clone();
                    let rent = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMultisigAccounts {
                        remaining,
                        multisig,
                        rent,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let source = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = TransferAccounts {
                        remaining,
                        source,
                        destination,
                        authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let source = keys.next().unwrap().clone();
                    let delegate = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ApproveAccounts {
                        remaining,
                        source,
                        delegate,
                        owner,
                    };
                    return Ok(Instruction::Approve { accounts, args });
                }
                [5u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = RevokeArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let source = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = RevokeAccounts {
                        remaining,
                        source,
                        owner,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let owned = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = SetAuthorityAccounts {
                        remaining,
                        owned,
                        owner,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let mint = keys.next().unwrap().clone();
                    let token = keys.next().unwrap().clone();
                    let mint_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = MintToAccounts {
                        remaining,
                        mint,
                        token,
                        mint_authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = BurnAccounts {
                        remaining,
                        account,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::Burn { accounts, args });
                }
                [9u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = CloseAccountArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = CloseAccountAccounts {
                        remaining,
                        account,
                        destination,
                        owner,
                    };
                    return Ok(Instruction::CloseAccount { accounts, args });
                }
                [10u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = FreezeAccountArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = FreezeAccountAccounts {
                        remaining,
                        account,
                        mint,
                        owner,
                    };
                    return Ok(Instruction::FreezeAccount { accounts, args });
                }
                [11u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = ThawAccountArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ThawAccountAccounts {
                        remaining,
                        account,
                        mint,
                        owner,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let source = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = TransferCheckedAccounts {
                        remaining,
                        source,
                        mint,
                        destination,
                        authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let source = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let delegate = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ApproveCheckedAccounts {
                        remaining,
                        source,
                        mint,
                        delegate,
                        owner,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let mint = keys.next().unwrap().clone();
                    let token = keys.next().unwrap().clone();
                    let mint_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = MintToCheckedAccounts {
                        remaining,
                        mint,
                        token,
                        mint_authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = BurnCheckedAccounts {
                        remaining,
                        account,
                        mint,
                        authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let rent = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeAccount2Accounts {
                        remaining,
                        account,
                        mint,
                        rent,
                    };
                    return Ok(Instruction::InitializeAccount2 { accounts, args });
                }
                [17u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = SyncNativeArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let account = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = SyncNativeAccounts { remaining, account };
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeAccount3Accounts {
                        remaining,
                        account,
                        mint,
                    };
                    return Ok(Instruction::InitializeAccount3 { accounts, args });
                }
                [19u8] => {
                    let mut rdr: &[u8] = rest;
                    let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                    let args = InitializeMultisig2Arguments { m };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let multisig = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMultisig2Accounts {
                        remaining,
                        multisig,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMint2Accounts { remaining, mint };
                    return Ok(Instruction::InitializeMint2 { accounts, args });
                }
                [21u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = GetAccountDataSizeArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = GetAccountDataSizeAccounts { remaining, mint };
                    return Ok(Instruction::GetAccountDataSize { accounts, args });
                }
                [22u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeImmutableOwnerArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let account = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeImmutableOwnerAccounts { remaining, account };
                    return Ok(Instruction::InitializeImmutableOwner { accounts, args });
                }
                [23u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = AmountToUiAmountArguments { amount };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = AmountToUiAmountAccounts { remaining, mint };
                    return Ok(Instruction::AmountToUiAmount { accounts, args });
                }
                [24u8] => {
                    let mut rdr: &[u8] = rest;
                    let ui_amount: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(ui_amount), e)
                        })?;
                    let args = UiAmountToAmountArguments { ui_amount };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UiAmountToAmountAccounts { remaining, mint };
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMintCloseAuthorityAccounts { remaining, mint };
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
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeTransferFeeConfigAccounts { remaining, mint };
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
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let source = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = TransferCheckedWithFeeAccounts {
                        remaining,
                        source,
                        mint,
                        destination,
                        authority,
                    };
                    return Ok(Instruction::TransferCheckedWithFee { accounts, args });
                }
                [26u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = WithdrawWithheldTokensFromMintArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let mint = keys.next().unwrap().clone();
                    let fee_receiver = keys.next().unwrap().clone();
                    let withdraw_withheld_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = WithdrawWithheldTokensFromMintAccounts {
                        remaining,
                        mint,
                        fee_receiver,
                        withdraw_withheld_authority,
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
                    let args = WithdrawWithheldTokensFromAccountsArguments { num_token_accounts };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let mint = keys.next().unwrap().clone();
                    let fee_receiver = keys.next().unwrap().clone();
                    let withdraw_withheld_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = WithdrawWithheldTokensFromAccountsAccounts {
                        remaining,
                        mint,
                        fee_receiver,
                        withdraw_withheld_authority,
                    };
                    return Ok(Instruction::WithdrawWithheldTokensFromAccounts { accounts, args });
                }
                [26u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = HarvestWithheldTokensToMintArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = HarvestWithheldTokensToMintAccounts { remaining, mint };
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
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let transfer_fee_config_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = SetTransferFeeAccounts {
                        remaining,
                        mint,
                        transfer_fee_config_authority,
                    };
                    return Ok(Instruction::SetTransferFee { accounts, args });
                }
                [27u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
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
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(auditor_elgamal_pubkey),
                            e
                        )
                    })?;
                    let auditor_elgamal_pubkey: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32]
                    {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializeConfidentialTransferMintArguments {
                        confidential_transfer_discriminator,
                        authority,
                        auto_approve_new_accounts,
                        auditor_elgamal_pubkey,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeConfidentialTransferMintAccounts { remaining, mint };
                    return Ok(Instruction::InitializeConfidentialTransferMint { accounts, args });
                }
                [27u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
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
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(auditor_elgamal_pubkey),
                            e
                        )
                    })?;
                    let auditor_elgamal_pubkey: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32]
                    {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateConfidentialTransferMintArguments {
                        confidential_transfer_discriminator,
                        auto_approve_new_accounts,
                        auditor_elgamal_pubkey,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateConfidentialTransferMintAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::UpdateConfidentialTransferMint { accounts, args });
                }
                [27u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let decryptable_zero_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
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
                        confidential_transfer_discriminator,
                        decryptable_zero_balance,
                        maximum_pending_balance_credit_counter,
                        proof_instruction_offset,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                    let record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ConfigureConfidentialTransferAccountAccounts {
                        remaining,
                        token,
                        mint,
                        instructions_sysvar_or_context_state,
                        record,
                        authority,
                    };
                    return Ok(Instruction::ConfigureConfidentialTransferAccount {
                        accounts,
                        args,
                    });
                }
                [27u8, 3u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let args = ApproveConfidentialTransferAccountArguments {
                        confidential_transfer_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ApproveConfidentialTransferAccountAccounts {
                        remaining,
                        token,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::ApproveConfidentialTransferAccount { accounts, args });
                }
                [27u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
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
                    let args = EmptyConfidentialTransferAccountArguments {
                        confidential_transfer_discriminator,
                        proof_instruction_offset,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let token = keys.next().unwrap().clone();
                    let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                    let record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EmptyConfidentialTransferAccountAccounts {
                        remaining,
                        token,
                        instructions_sysvar_or_context_state,
                        record,
                        authority,
                    };
                    return Ok(Instruction::EmptyConfidentialTransferAccount { accounts, args });
                }
                [27u8, 5u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = ConfidentialDepositArguments {
                        confidential_transfer_discriminator,
                        amount,
                        decimals,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ConfidentialDepositAccounts {
                        remaining,
                        token,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::ConfidentialDeposit { accounts, args });
                }
                [27u8, 6u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let new_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
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
                        confidential_transfer_discriminator,
                        amount,
                        decimals,
                        new_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        range_proof_instruction_offset,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let instructions_sysvar = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let equality_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let range_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ConfidentialWithdrawAccounts {
                        remaining,
                        token,
                        mint,
                        instructions_sysvar,
                        equality_record,
                        range_record,
                        authority,
                    };
                    return Ok(Instruction::ConfidentialWithdraw { accounts, args });
                }
                [27u8, 7u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let new_source_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
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
                        confidential_transfer_discriminator,
                        new_source_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        ciphertext_validity_proof_instruction_offset,
                        range_proof_instruction_offset,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let source_token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let destination_token = keys.next().unwrap().clone();
                    let instructions_sysvar = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let equality_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let ciphertext_validity_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let range_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ConfidentialTransferAccounts {
                        remaining,
                        source_token,
                        mint,
                        destination_token,
                        instructions_sysvar,
                        equality_record,
                        ciphertext_validity_record,
                        range_record,
                        authority,
                    };
                    return Ok(Instruction::ConfidentialTransfer { accounts, args });
                }
                [27u8, 8u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let expected_pending_balance_credit_counter: u64 =
                        <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(expected_pending_balance_credit_counter),
                                e
                            )
                        })?;
                    let new_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(new_decryptable_available_balance),
                                    e
                                )
                            })?;
                    let args = ApplyConfidentialPendingBalanceArguments {
                        confidential_transfer_discriminator,
                        expected_pending_balance_credit_counter,
                        new_decryptable_available_balance,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ApplyConfidentialPendingBalanceAccounts {
                        remaining,
                        token,
                        authority,
                    };
                    return Ok(Instruction::ApplyConfidentialPendingBalance { accounts, args });
                }
                [27u8, 9u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let args = EnableConfidentialCreditsArguments {
                        confidential_transfer_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EnableConfidentialCreditsAccounts {
                        remaining,
                        token,
                        authority,
                    };
                    return Ok(Instruction::EnableConfidentialCredits { accounts, args });
                }
                [27u8, 10u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let args = DisableConfidentialCreditsArguments {
                        confidential_transfer_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = DisableConfidentialCreditsAccounts {
                        remaining,
                        token,
                        authority,
                    };
                    return Ok(Instruction::DisableConfidentialCredits { accounts, args });
                }
                [27u8, 11u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let args = EnableNonConfidentialCreditsArguments {
                        confidential_transfer_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EnableNonConfidentialCreditsAccounts {
                        remaining,
                        token,
                        authority,
                    };
                    return Ok(Instruction::EnableNonConfidentialCredits { accounts, args });
                }
                [27u8, 12u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let args = DisableNonConfidentialCreditsArguments {
                        confidential_transfer_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = DisableNonConfidentialCreditsAccounts {
                        remaining,
                        token,
                        authority,
                    };
                    return Ok(Instruction::DisableNonConfidentialCredits { accounts, args });
                }
                [27u8, 13u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let new_source_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
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
                        confidential_transfer_discriminator,
                        new_source_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        transfer_amount_ciphertext_validity_proof_instruction_offset,
                        fee_sigma_proof_instruction_offset,
                        fee_ciphertext_validity_proof_instruction_offset,
                        range_proof_instruction_offset,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let source_token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let destination_token = keys.next().unwrap().clone();
                    let instructions_sysvar = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let equality_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let transfer_amount_ciphertext_validity_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let fee_sigma_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let fee_ciphertext_validity_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let range_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ConfidentialTransferWithFeeAccounts {
                        remaining,
                        source_token,
                        mint,
                        destination_token,
                        instructions_sysvar,
                        equality_record,
                        transfer_amount_ciphertext_validity_record,
                        fee_sigma_record,
                        fee_ciphertext_validity_record,
                        range_record,
                        authority,
                    };
                    return Ok(Instruction::ConfidentialTransferWithFee { accounts, args });
                }
                [28u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let default_account_state_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(default_account_state_discriminator),
                                e
                            )
                        })?;
                    let state: AccountState =
                        <AccountState as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(state), e)
                            })?;
                    let args = InitializeDefaultAccountStateArguments {
                        default_account_state_discriminator,
                        state,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeDefaultAccountStateAccounts { remaining, mint };
                    return Ok(Instruction::InitializeDefaultAccountState { accounts, args });
                }
                [28u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let default_account_state_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(default_account_state_discriminator),
                                e
                            )
                        })?;
                    let state: AccountState =
                        <AccountState as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(state), e)
                            })?;
                    let args = UpdateDefaultAccountStateArguments {
                        default_account_state_discriminator,
                        state,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let freeze_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateDefaultAccountStateAccounts {
                        remaining,
                        mint,
                        freeze_authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let token = keys.next().unwrap().clone();
                    let payer = keys.next().unwrap().clone();
                    let system_program = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ReallocateAccounts {
                        remaining,
                        token,
                        payer,
                        system_program,
                        owner,
                    };
                    return Ok(Instruction::Reallocate { accounts, args });
                }
                [30u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let memo_transfers_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(memo_transfers_discriminator),
                                e
                            )
                        })?;
                    let args = EnableMemoTransfersArguments {
                        memo_transfers_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EnableMemoTransfersAccounts {
                        remaining,
                        token,
                        owner,
                    };
                    return Ok(Instruction::EnableMemoTransfers { accounts, args });
                }
                [30u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let memo_transfers_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(memo_transfers_discriminator),
                                e
                            )
                        })?;
                    let args = DisableMemoTransfersArguments {
                        memo_transfers_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = DisableMemoTransfersAccounts {
                        remaining,
                        token,
                        owner,
                    };
                    return Ok(Instruction::DisableMemoTransfers { accounts, args });
                }
                [31u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = CreateNativeMintArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let payer = keys.next().unwrap().clone();
                    let native_mint = keys.next().unwrap().clone();
                    let system_program = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = CreateNativeMintAccounts {
                        remaining,
                        payer,
                        native_mint,
                        system_program,
                    };
                    return Ok(Instruction::CreateNativeMint { accounts, args });
                }
                [32u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeNonTransferableMintArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeNonTransferableMintAccounts { remaining, mint };
                    return Ok(Instruction::InitializeNonTransferableMint { accounts, args });
                }
                [33u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let interest_bearing_mint_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(interest_bearing_mint_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(rate_authority),
                            e
                        )
                    })?;
                    let rate_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(rate), e)
                        })?;
                    let args = InitializeInterestBearingMintArguments {
                        interest_bearing_mint_discriminator,
                        rate_authority,
                        rate,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeInterestBearingMintAccounts { remaining, mint };
                    return Ok(Instruction::InitializeInterestBearingMint { accounts, args });
                }
                [33u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let interest_bearing_mint_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(interest_bearing_mint_discriminator),
                                e
                            )
                        })?;
                    let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(rate), e)
                        })?;
                    let args = UpdateRateInterestBearingMintArguments {
                        interest_bearing_mint_discriminator,
                        rate,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let rate_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateRateInterestBearingMintAccounts {
                        remaining,
                        mint,
                        rate_authority,
                    };
                    return Ok(Instruction::UpdateRateInterestBearingMint { accounts, args });
                }
                [34u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let cpi_guard_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(cpi_guard_discriminator),
                                e
                            )
                        })?;
                    let args = EnableCpiGuardArguments {
                        cpi_guard_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EnableCpiGuardAccounts {
                        remaining,
                        token,
                        owner,
                    };
                    return Ok(Instruction::EnableCpiGuard { accounts, args });
                }
                [34u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let cpi_guard_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(cpi_guard_discriminator),
                                e
                            )
                        })?;
                    let args = DisableCpiGuardArguments {
                        cpi_guard_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = DisableCpiGuardAccounts {
                        remaining,
                        token,
                        owner,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializePermanentDelegateAccounts { remaining, mint };
                    return Ok(Instruction::InitializePermanentDelegate { accounts, args });
                }
                [36u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(program_id),
                            e
                        )
                    })?;
                    let program_id: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializeTransferHookArguments {
                        authority,
                        program_id,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeTransferHookAccounts { remaining, mint };
                    return Ok(Instruction::InitializeTransferHook { accounts, args });
                }
                [36u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(program_id),
                            e
                        )
                    })?;
                    let program_id: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateTransferHookArguments { program_id };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateTransferHookAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::UpdateTransferHook { accounts, args });
                }
                [37u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(withdraw_withheld_authority_el_gamal_pubkey),
                            e
                        )
                    })?;
                    let withdraw_withheld_authority_el_gamal_pubkey: Option<[u8; 32usize]> =
                        if pubkey_bytes == [0u8; 32] {
                            None
                        } else {
                            Some(pubkey_bytes)
                        };
                    let args = InitializeConfidentialTransferFeeArguments {
                        confidential_transfer_fee_discriminator,
                        authority,
                        withdraw_withheld_authority_el_gamal_pubkey,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeConfidentialTransferFeeAccounts { remaining, mint };
                    return Ok(Instruction::InitializeConfidentialTransferFee { accounts, args });
                }
                [37u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
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
                    let new_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(new_decryptable_available_balance),
                                    e
                                )
                            })?;
                    let args = WithdrawWithheldTokensFromMintForConfidentialTransferFeeArguments {
                        confidential_transfer_fee_discriminator,
                        proof_instruction_offset,
                        new_decryptable_available_balance,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let mint = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                    let record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts =
                        WithdrawWithheldTokensFromMintForConfidentialTransferFeeAccounts {
                            remaining,
                            mint,
                            destination,
                            instructions_sysvar_or_context_state,
                            record,
                            authority,
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
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
                                e
                            )
                        })?;
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
                    let new_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(new_decryptable_available_balance),
                                    e
                                )
                            })?;
                    let args =
                        WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeArguments {
                            confidential_transfer_fee_discriminator,
                            num_token_accounts,
                            proof_instruction_offset,
                            new_decryptable_available_balance,
                        };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let mint = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                    let record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts =
                        WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeAccounts {
                            remaining,
                            mint,
                            destination,
                            instructions_sysvar_or_context_state,
                            record,
                            authority,
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
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
                                e
                            )
                        })?;
                    let args = HarvestWithheldTokensToMintForConfidentialTransferFeeArguments {
                        confidential_transfer_fee_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = HarvestWithheldTokensToMintForConfidentialTransferFeeAccounts {
                        remaining,
                        mint,
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
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
                                e
                            )
                        })?;
                    let args = EnableHarvestToMintArguments {
                        confidential_transfer_fee_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EnableHarvestToMintAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::EnableHarvestToMint { accounts, args });
                }
                [37u8, 5u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
                                e
                            )
                        })?;
                    let args = DisableHarvestToMintArguments {
                        confidential_transfer_fee_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = DisableHarvestToMintAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::DisableHarvestToMint { accounts, args });
                }
                [38u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = WithdrawExcessLamportsArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let source_account = keys.next().unwrap().clone();
                    let destination_account = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = WithdrawExcessLamportsAccounts {
                        remaining,
                        source_account,
                        destination_account,
                        authority,
                    };
                    return Ok(Instruction::WithdrawExcessLamports { accounts, args });
                }
                [39u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let metadata_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(metadata_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(metadata_address),
                            e
                        )
                    })?;
                    let metadata_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializeMetadataPointerArguments {
                        metadata_pointer_discriminator,
                        authority,
                        metadata_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMetadataPointerAccounts { remaining, mint };
                    return Ok(Instruction::InitializeMetadataPointer { accounts, args });
                }
                [39u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let metadata_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(metadata_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(metadata_address),
                            e
                        )
                    })?;
                    let metadata_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateMetadataPointerArguments {
                        metadata_pointer_discriminator,
                        metadata_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let metadata_pointer_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateMetadataPointerAccounts {
                        remaining,
                        mint,
                        metadata_pointer_authority,
                    };
                    return Ok(Instruction::UpdateMetadataPointer { accounts, args });
                }
                [40u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let group_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(group_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(group_address),
                            e
                        )
                    })?;
                    let group_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializeGroupPointerArguments {
                        group_pointer_discriminator,
                        authority,
                        group_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeGroupPointerAccounts { remaining, mint };
                    return Ok(Instruction::InitializeGroupPointer { accounts, args });
                }
                [40u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let group_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(group_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(group_address),
                            e
                        )
                    })?;
                    let group_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateGroupPointerArguments {
                        group_pointer_discriminator,
                        group_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let group_pointer_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateGroupPointerAccounts {
                        remaining,
                        mint,
                        group_pointer_authority,
                    };
                    return Ok(Instruction::UpdateGroupPointer { accounts, args });
                }
                [41u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let group_member_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(group_member_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(member_address),
                            e
                        )
                    })?;
                    let member_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializeGroupMemberPointerArguments {
                        group_member_pointer_discriminator,
                        authority,
                        member_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeGroupMemberPointerAccounts { remaining, mint };
                    return Ok(Instruction::InitializeGroupMemberPointer { accounts, args });
                }
                [41u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let group_member_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(group_member_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(member_address),
                            e
                        )
                    })?;
                    let member_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateGroupMemberPointerArguments {
                        group_member_pointer_discriminator,
                        member_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let group_member_pointer_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateGroupMemberPointerAccounts {
                        remaining,
                        mint,
                        group_member_pointer_authority,
                    };
                    return Ok(Instruction::UpdateGroupMemberPointer { accounts, args });
                }
                [43u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let scaled_ui_amount_mint_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(scaled_ui_amount_mint_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let multiplier: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(multiplier), e)
                    })?;
                    let args = InitializeScaledUiAmountMintArguments {
                        scaled_ui_amount_mint_discriminator,
                        authority,
                        multiplier,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeScaledUiAmountMintAccounts { remaining, mint };
                    return Ok(Instruction::InitializeScaledUiAmountMint { accounts, args });
                }
                [43u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let scaled_ui_amount_mint_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(scaled_ui_amount_mint_discriminator),
                                e
                            )
                        })?;
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
                        scaled_ui_amount_mint_discriminator,
                        multiplier,
                        effective_timestamp,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateMultiplierScaledUiMintAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::UpdateMultiplierScaledUiMint { accounts, args });
                }
                [44u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let pausable_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(pausable_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializePausableConfigArguments {
                        pausable_discriminator,
                        authority,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializePausableConfigAccounts { remaining, mint };
                    return Ok(Instruction::InitializePausableConfig { accounts, args });
                }
                [44u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let pausable_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(pausable_discriminator),
                                e
                            )
                        })?;
                    let args = PauseArguments {
                        pausable_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = PauseAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::Pause { accounts, args });
                }
                [44u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let pausable_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(pausable_discriminator),
                                e
                            )
                        })?;
                    let args = ResumeArguments {
                        pausable_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ResumeAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::Resume { accounts, args });
                }
                [210u8, 225u8, 30u8, 162u8, 88u8, 184u8, 77u8, 141u8] => {
                    let mut rdr: &[u8] = rest;
                    let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize string length for {}: {}",
                                stringify!(name),
                                e
                            )
                        })?;
                    let mut string_bytes = vec![0u8; string_len as usize];
                    rdr.read_exact(&mut string_bytes).map_err(|e| {
                        format!(
                            "Failed to read string bytes for {}: {}",
                            stringify!(name),
                            e
                        )
                    })?;
                    let name: String = String::from_utf8(string_bytes).map_err(|e| {
                        format!(
                            "Failed to parse UTF-8 string for {}: {}",
                            stringify!(name),
                            e
                        )
                    })?;
                    let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize string length for {}: {}",
                                stringify!(symbol),
                                e
                            )
                        })?;
                    let mut string_bytes = vec![0u8; string_len as usize];
                    rdr.read_exact(&mut string_bytes).map_err(|e| {
                        format!(
                            "Failed to read string bytes for {}: {}",
                            stringify!(symbol),
                            e
                        )
                    })?;
                    let symbol: String = String::from_utf8(string_bytes).map_err(|e| {
                        format!(
                            "Failed to parse UTF-8 string for {}: {}",
                            stringify!(symbol),
                            e
                        )
                    })?;
                    let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize string length for {}: {}",
                                stringify!(uri),
                                e
                            )
                        })?;
                    let mut string_bytes = vec![0u8; string_len as usize];
                    rdr.read_exact(&mut string_bytes).map_err(|e| {
                        format!("Failed to read string bytes for {}: {}", stringify!(uri), e)
                    })?;
                    let uri: String = String::from_utf8(string_bytes).map_err(|e| {
                        format!(
                            "Failed to parse UTF-8 string for {}: {}",
                            stringify!(uri),
                            e
                        )
                    })?;
                    let args = InitializeTokenMetadataArguments { name, symbol, uri };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let metadata = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let mint_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeTokenMetadataAccounts {
                        remaining,
                        metadata,
                        update_authority,
                        mint,
                        mint_authority,
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
                    let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize string length for {}: {}",
                                stringify!(value),
                                e
                            )
                        })?;
                    let mut string_bytes = vec![0u8; string_len as usize];
                    rdr.read_exact(&mut string_bytes).map_err(|e| {
                        format!(
                            "Failed to read string bytes for {}: {}",
                            stringify!(value),
                            e
                        )
                    })?;
                    let value: String = String::from_utf8(string_bytes).map_err(|e| {
                        format!(
                            "Failed to parse UTF-8 string for {}: {}",
                            stringify!(value),
                            e
                        )
                    })?;
                    let args = UpdateTokenMetadataFieldArguments { field, value };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let metadata = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateTokenMetadataFieldAccounts {
                        remaining,
                        metadata,
                        update_authority,
                    };
                    return Ok(Instruction::UpdateTokenMetadataField { accounts, args });
                }
                [234u8, 18u8, 32u8, 56u8, 89u8, 141u8, 37u8, 181u8] => {
                    let mut rdr: &[u8] = rest;
                    let idempotent: bool =
                        <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                            |e| format!("Failed to deserialize {}: {}", stringify!(idempotent), e),
                        )?;
                    let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize string length for {}: {}",
                                stringify!(key),
                                e
                            )
                        })?;
                    let mut string_bytes = vec![0u8; string_len as usize];
                    rdr.read_exact(&mut string_bytes).map_err(|e| {
                        format!("Failed to read string bytes for {}: {}", stringify!(key), e)
                    })?;
                    let key: String = String::from_utf8(string_bytes).map_err(|e| {
                        format!(
                            "Failed to parse UTF-8 string for {}: {}",
                            stringify!(key),
                            e
                        )
                    })?;
                    let args = RemoveTokenMetadataKeyArguments { idempotent, key };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let metadata = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = RemoveTokenMetadataKeyAccounts {
                        remaining,
                        metadata,
                        update_authority,
                    };
                    return Ok(Instruction::RemoveTokenMetadataKey { accounts, args });
                }
                [215u8, 228u8, 166u8, 228u8, 84u8, 100u8, 86u8, 123u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(new_update_authority),
                            e
                        )
                    })?;
                    let new_update_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateTokenMetadataUpdateAuthorityArguments {
                        new_update_authority,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let metadata = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateTokenMetadataUpdateAuthorityAccounts {
                        remaining,
                        metadata,
                        update_authority,
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
                    let args = EmitTokenMetadataArguments { start, end };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let metadata = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EmitTokenMetadataAccounts {
                        remaining,
                        metadata,
                    };
                    return Ok(Instruction::EmitTokenMetadata { accounts, args });
                }
                [121u8, 113u8, 108u8, 39u8, 54u8, 51u8, 0u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(update_authority),
                            e
                        )
                    })?;
                    let update_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                    let args = InitializeTokenGroupArguments {
                        update_authority,
                        max_size,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let group = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let mint_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeTokenGroupAccounts {
                        remaining,
                        group,
                        mint,
                        mint_authority,
                    };
                    return Ok(Instruction::InitializeTokenGroup { accounts, args });
                }
                [108u8, 37u8, 171u8, 143u8, 248u8, 30u8, 18u8, 110u8] => {
                    let mut rdr: &[u8] = rest;
                    let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                    let args = UpdateTokenGroupMaxSizeArguments { max_size };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let group = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateTokenGroupMaxSizeAccounts {
                        remaining,
                        group,
                        update_authority,
                    };
                    return Ok(Instruction::UpdateTokenGroupMaxSize { accounts, args });
                }
                [161u8, 105u8, 88u8, 1u8, 237u8, 221u8, 216u8, 203u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(new_update_authority),
                            e
                        )
                    })?;
                    let new_update_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateTokenGroupUpdateAuthorityArguments {
                        new_update_authority,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let group = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateTokenGroupUpdateAuthorityAccounts {
                        remaining,
                        group,
                        update_authority,
                    };
                    return Ok(Instruction::UpdateTokenGroupUpdateAuthority { accounts, args });
                }
                [152u8, 32u8, 222u8, 176u8, 223u8, 237u8, 116u8, 134u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeTokenGroupMemberArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(5usize);
                    let member = keys.next().unwrap().clone();
                    let member_mint = keys.next().unwrap().clone();
                    let member_mint_authority = keys.next().unwrap().clone();
                    let group = keys.next().unwrap().clone();
                    let group_update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeTokenGroupMemberAccounts {
                        remaining,
                        member,
                        member_mint,
                        member_mint_authority,
                        group,
                        group_update_authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let rent = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMintAccounts {
                        remaining,
                        mint,
                        rent,
                    };
                    return Ok(Instruction::InitializeMint { accounts, args });
                }
                [1u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeAccountArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let rent = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeAccountAccounts {
                        remaining,
                        account,
                        mint,
                        owner,
                        rent,
                    };
                    return Ok(Instruction::InitializeAccount { accounts, args });
                }
                [2u8] => {
                    let mut rdr: &[u8] = rest;
                    let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                    let args = InitializeMultisigArguments { m };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let multisig = keys.next().unwrap().clone();
                    let rent = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMultisigAccounts {
                        remaining,
                        multisig,
                        rent,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let source = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = TransferAccounts {
                        remaining,
                        source,
                        destination,
                        authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let source = keys.next().unwrap().clone();
                    let delegate = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ApproveAccounts {
                        remaining,
                        source,
                        delegate,
                        owner,
                    };
                    return Ok(Instruction::Approve { accounts, args });
                }
                [5u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = RevokeArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let source = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = RevokeAccounts {
                        remaining,
                        source,
                        owner,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let owned = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = SetAuthorityAccounts {
                        remaining,
                        owned,
                        owner,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let mint = keys.next().unwrap().clone();
                    let token = keys.next().unwrap().clone();
                    let mint_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = MintToAccounts {
                        remaining,
                        mint,
                        token,
                        mint_authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = BurnAccounts {
                        remaining,
                        account,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::Burn { accounts, args });
                }
                [9u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = CloseAccountArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = CloseAccountAccounts {
                        remaining,
                        account,
                        destination,
                        owner,
                    };
                    return Ok(Instruction::CloseAccount { accounts, args });
                }
                [10u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = FreezeAccountArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = FreezeAccountAccounts {
                        remaining,
                        account,
                        mint,
                        owner,
                    };
                    return Ok(Instruction::FreezeAccount { accounts, args });
                }
                [11u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = ThawAccountArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ThawAccountAccounts {
                        remaining,
                        account,
                        mint,
                        owner,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let source = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = TransferCheckedAccounts {
                        remaining,
                        source,
                        mint,
                        destination,
                        authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let source = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let delegate = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ApproveCheckedAccounts {
                        remaining,
                        source,
                        mint,
                        delegate,
                        owner,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let mint = keys.next().unwrap().clone();
                    let token = keys.next().unwrap().clone();
                    let mint_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = MintToCheckedAccounts {
                        remaining,
                        mint,
                        token,
                        mint_authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = BurnCheckedAccounts {
                        remaining,
                        account,
                        mint,
                        authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let rent = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeAccount2Accounts {
                        remaining,
                        account,
                        mint,
                        rent,
                    };
                    return Ok(Instruction::InitializeAccount2 { accounts, args });
                }
                [17u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = SyncNativeArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let account = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = SyncNativeAccounts { remaining, account };
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let account = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeAccount3Accounts {
                        remaining,
                        account,
                        mint,
                    };
                    return Ok(Instruction::InitializeAccount3 { accounts, args });
                }
                [19u8] => {
                    let mut rdr: &[u8] = rest;
                    let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                    let args = InitializeMultisig2Arguments { m };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let multisig = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMultisig2Accounts {
                        remaining,
                        multisig,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMint2Accounts { remaining, mint };
                    return Ok(Instruction::InitializeMint2 { accounts, args });
                }
                [21u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = GetAccountDataSizeArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = GetAccountDataSizeAccounts { remaining, mint };
                    return Ok(Instruction::GetAccountDataSize { accounts, args });
                }
                [22u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeImmutableOwnerArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let account = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeImmutableOwnerAccounts { remaining, account };
                    return Ok(Instruction::InitializeImmutableOwner { accounts, args });
                }
                [23u8] => {
                    let mut rdr: &[u8] = rest;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let args = AmountToUiAmountArguments { amount };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = AmountToUiAmountAccounts { remaining, mint };
                    return Ok(Instruction::AmountToUiAmount { accounts, args });
                }
                [24u8] => {
                    let mut rdr: &[u8] = rest;
                    let ui_amount: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(ui_amount), e)
                        })?;
                    let args = UiAmountToAmountArguments { ui_amount };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UiAmountToAmountAccounts { remaining, mint };
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMintCloseAuthorityAccounts { remaining, mint };
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
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeTransferFeeConfigAccounts { remaining, mint };
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
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let source = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = TransferCheckedWithFeeAccounts {
                        remaining,
                        source,
                        mint,
                        destination,
                        authority,
                    };
                    return Ok(Instruction::TransferCheckedWithFee { accounts, args });
                }
                [26u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = WithdrawWithheldTokensFromMintArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let mint = keys.next().unwrap().clone();
                    let fee_receiver = keys.next().unwrap().clone();
                    let withdraw_withheld_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = WithdrawWithheldTokensFromMintAccounts {
                        remaining,
                        mint,
                        fee_receiver,
                        withdraw_withheld_authority,
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
                    let args = WithdrawWithheldTokensFromAccountsArguments { num_token_accounts };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let mint = keys.next().unwrap().clone();
                    let fee_receiver = keys.next().unwrap().clone();
                    let withdraw_withheld_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = WithdrawWithheldTokensFromAccountsAccounts {
                        remaining,
                        mint,
                        fee_receiver,
                        withdraw_withheld_authority,
                    };
                    return Ok(Instruction::WithdrawWithheldTokensFromAccounts { accounts, args });
                }
                [26u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = HarvestWithheldTokensToMintArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = HarvestWithheldTokensToMintAccounts { remaining, mint };
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
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let transfer_fee_config_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = SetTransferFeeAccounts {
                        remaining,
                        mint,
                        transfer_fee_config_authority,
                    };
                    return Ok(Instruction::SetTransferFee { accounts, args });
                }
                [27u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
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
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(auditor_elgamal_pubkey),
                            e
                        )
                    })?;
                    let auditor_elgamal_pubkey: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32]
                    {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializeConfidentialTransferMintArguments {
                        confidential_transfer_discriminator,
                        authority,
                        auto_approve_new_accounts,
                        auditor_elgamal_pubkey,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeConfidentialTransferMintAccounts { remaining, mint };
                    return Ok(Instruction::InitializeConfidentialTransferMint { accounts, args });
                }
                [27u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
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
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(auditor_elgamal_pubkey),
                            e
                        )
                    })?;
                    let auditor_elgamal_pubkey: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32]
                    {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateConfidentialTransferMintArguments {
                        confidential_transfer_discriminator,
                        auto_approve_new_accounts,
                        auditor_elgamal_pubkey,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateConfidentialTransferMintAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::UpdateConfidentialTransferMint { accounts, args });
                }
                [27u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let decryptable_zero_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
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
                        confidential_transfer_discriminator,
                        decryptable_zero_balance,
                        maximum_pending_balance_credit_counter,
                        proof_instruction_offset,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                    let record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ConfigureConfidentialTransferAccountAccounts {
                        remaining,
                        token,
                        mint,
                        instructions_sysvar_or_context_state,
                        record,
                        authority,
                    };
                    return Ok(Instruction::ConfigureConfidentialTransferAccount {
                        accounts,
                        args,
                    });
                }
                [27u8, 3u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let args = ApproveConfidentialTransferAccountArguments {
                        confidential_transfer_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ApproveConfidentialTransferAccountAccounts {
                        remaining,
                        token,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::ApproveConfidentialTransferAccount { accounts, args });
                }
                [27u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
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
                    let args = EmptyConfidentialTransferAccountArguments {
                        confidential_transfer_discriminator,
                        proof_instruction_offset,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let token = keys.next().unwrap().clone();
                    let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                    let record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EmptyConfidentialTransferAccountAccounts {
                        remaining,
                        token,
                        instructions_sysvar_or_context_state,
                        record,
                        authority,
                    };
                    return Ok(Instruction::EmptyConfidentialTransferAccount { accounts, args });
                }
                [27u8, 5u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let args = ConfidentialDepositArguments {
                        confidential_transfer_discriminator,
                        amount,
                        decimals,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ConfidentialDepositAccounts {
                        remaining,
                        token,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::ConfidentialDeposit { accounts, args });
                }
                [27u8, 6u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(amount), e)
                        })?;
                    let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                        })?;
                    let new_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
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
                        confidential_transfer_discriminator,
                        amount,
                        decimals,
                        new_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        range_proof_instruction_offset,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let instructions_sysvar = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let equality_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let range_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ConfidentialWithdrawAccounts {
                        remaining,
                        token,
                        mint,
                        instructions_sysvar,
                        equality_record,
                        range_record,
                        authority,
                    };
                    return Ok(Instruction::ConfidentialWithdraw { accounts, args });
                }
                [27u8, 7u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let new_source_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
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
                        confidential_transfer_discriminator,
                        new_source_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        ciphertext_validity_proof_instruction_offset,
                        range_proof_instruction_offset,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let source_token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let destination_token = keys.next().unwrap().clone();
                    let instructions_sysvar = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let equality_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let ciphertext_validity_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let range_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ConfidentialTransferAccounts {
                        remaining,
                        source_token,
                        mint,
                        destination_token,
                        instructions_sysvar,
                        equality_record,
                        ciphertext_validity_record,
                        range_record,
                        authority,
                    };
                    return Ok(Instruction::ConfidentialTransfer { accounts, args });
                }
                [27u8, 8u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let expected_pending_balance_credit_counter: u64 =
                        <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(expected_pending_balance_credit_counter),
                                e
                            )
                        })?;
                    let new_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(new_decryptable_available_balance),
                                    e
                                )
                            })?;
                    let args = ApplyConfidentialPendingBalanceArguments {
                        confidential_transfer_discriminator,
                        expected_pending_balance_credit_counter,
                        new_decryptable_available_balance,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ApplyConfidentialPendingBalanceAccounts {
                        remaining,
                        token,
                        authority,
                    };
                    return Ok(Instruction::ApplyConfidentialPendingBalance { accounts, args });
                }
                [27u8, 9u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let args = EnableConfidentialCreditsArguments {
                        confidential_transfer_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EnableConfidentialCreditsAccounts {
                        remaining,
                        token,
                        authority,
                    };
                    return Ok(Instruction::EnableConfidentialCredits { accounts, args });
                }
                [27u8, 10u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let args = DisableConfidentialCreditsArguments {
                        confidential_transfer_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = DisableConfidentialCreditsAccounts {
                        remaining,
                        token,
                        authority,
                    };
                    return Ok(Instruction::DisableConfidentialCredits { accounts, args });
                }
                [27u8, 11u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let args = EnableNonConfidentialCreditsArguments {
                        confidential_transfer_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EnableNonConfidentialCreditsAccounts {
                        remaining,
                        token,
                        authority,
                    };
                    return Ok(Instruction::EnableNonConfidentialCredits { accounts, args });
                }
                [27u8, 12u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let args = DisableNonConfidentialCreditsArguments {
                        confidential_transfer_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = DisableNonConfidentialCreditsAccounts {
                        remaining,
                        token,
                        authority,
                    };
                    return Ok(Instruction::DisableNonConfidentialCredits { accounts, args });
                }
                [27u8, 13u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_discriminator),
                                e
                            )
                        })?;
                    let new_source_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
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
                        confidential_transfer_discriminator,
                        new_source_decryptable_available_balance,
                        equality_proof_instruction_offset,
                        transfer_amount_ciphertext_validity_proof_instruction_offset,
                        fee_sigma_proof_instruction_offset,
                        fee_ciphertext_validity_proof_instruction_offset,
                        range_proof_instruction_offset,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let source_token = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let destination_token = keys.next().unwrap().clone();
                    let instructions_sysvar = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let equality_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let transfer_amount_ciphertext_validity_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let fee_sigma_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let fee_ciphertext_validity_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let range_record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ConfidentialTransferWithFeeAccounts {
                        remaining,
                        source_token,
                        mint,
                        destination_token,
                        instructions_sysvar,
                        equality_record,
                        transfer_amount_ciphertext_validity_record,
                        fee_sigma_record,
                        fee_ciphertext_validity_record,
                        range_record,
                        authority,
                    };
                    return Ok(Instruction::ConfidentialTransferWithFee { accounts, args });
                }
                [28u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let default_account_state_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(default_account_state_discriminator),
                                e
                            )
                        })?;
                    let state: AccountState =
                        <AccountState as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(state), e)
                            })?;
                    let args = InitializeDefaultAccountStateArguments {
                        default_account_state_discriminator,
                        state,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeDefaultAccountStateAccounts { remaining, mint };
                    return Ok(Instruction::InitializeDefaultAccountState { accounts, args });
                }
                [28u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let default_account_state_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(default_account_state_discriminator),
                                e
                            )
                        })?;
                    let state: AccountState =
                        <AccountState as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!("Failed to deserialize {}: {}", stringify!(state), e)
                            })?;
                    let args = UpdateDefaultAccountStateArguments {
                        default_account_state_discriminator,
                        state,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let freeze_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateDefaultAccountStateAccounts {
                        remaining,
                        mint,
                        freeze_authority,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let token = keys.next().unwrap().clone();
                    let payer = keys.next().unwrap().clone();
                    let system_program = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ReallocateAccounts {
                        remaining,
                        token,
                        payer,
                        system_program,
                        owner,
                    };
                    return Ok(Instruction::Reallocate { accounts, args });
                }
                [30u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let memo_transfers_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(memo_transfers_discriminator),
                                e
                            )
                        })?;
                    let args = EnableMemoTransfersArguments {
                        memo_transfers_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EnableMemoTransfersAccounts {
                        remaining,
                        token,
                        owner,
                    };
                    return Ok(Instruction::EnableMemoTransfers { accounts, args });
                }
                [30u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let memo_transfers_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(memo_transfers_discriminator),
                                e
                            )
                        })?;
                    let args = DisableMemoTransfersArguments {
                        memo_transfers_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = DisableMemoTransfersAccounts {
                        remaining,
                        token,
                        owner,
                    };
                    return Ok(Instruction::DisableMemoTransfers { accounts, args });
                }
                [31u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = CreateNativeMintArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let payer = keys.next().unwrap().clone();
                    let native_mint = keys.next().unwrap().clone();
                    let system_program = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = CreateNativeMintAccounts {
                        remaining,
                        payer,
                        native_mint,
                        system_program,
                    };
                    return Ok(Instruction::CreateNativeMint { accounts, args });
                }
                [32u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeNonTransferableMintArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeNonTransferableMintAccounts { remaining, mint };
                    return Ok(Instruction::InitializeNonTransferableMint { accounts, args });
                }
                [33u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let interest_bearing_mint_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(interest_bearing_mint_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(rate_authority),
                            e
                        )
                    })?;
                    let rate_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(rate), e)
                        })?;
                    let args = InitializeInterestBearingMintArguments {
                        interest_bearing_mint_discriminator,
                        rate_authority,
                        rate,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeInterestBearingMintAccounts { remaining, mint };
                    return Ok(Instruction::InitializeInterestBearingMint { accounts, args });
                }
                [33u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let interest_bearing_mint_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(interest_bearing_mint_discriminator),
                                e
                            )
                        })?;
                    let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!("Failed to deserialize {}: {}", stringify!(rate), e)
                        })?;
                    let args = UpdateRateInterestBearingMintArguments {
                        interest_bearing_mint_discriminator,
                        rate,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let rate_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateRateInterestBearingMintAccounts {
                        remaining,
                        mint,
                        rate_authority,
                    };
                    return Ok(Instruction::UpdateRateInterestBearingMint { accounts, args });
                }
                [34u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let cpi_guard_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(cpi_guard_discriminator),
                                e
                            )
                        })?;
                    let args = EnableCpiGuardArguments {
                        cpi_guard_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EnableCpiGuardAccounts {
                        remaining,
                        token,
                        owner,
                    };
                    return Ok(Instruction::EnableCpiGuard { accounts, args });
                }
                [34u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let cpi_guard_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(cpi_guard_discriminator),
                                e
                            )
                        })?;
                    let args = DisableCpiGuardArguments {
                        cpi_guard_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let token = keys.next().unwrap().clone();
                    let owner = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = DisableCpiGuardAccounts {
                        remaining,
                        token,
                        owner,
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
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializePermanentDelegateAccounts { remaining, mint };
                    return Ok(Instruction::InitializePermanentDelegate { accounts, args });
                }
                [36u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(program_id),
                            e
                        )
                    })?;
                    let program_id: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializeTransferHookArguments {
                        authority,
                        program_id,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeTransferHookAccounts { remaining, mint };
                    return Ok(Instruction::InitializeTransferHook { accounts, args });
                }
                [36u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(program_id),
                            e
                        )
                    })?;
                    let program_id: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateTransferHookArguments { program_id };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateTransferHookAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::UpdateTransferHook { accounts, args });
                }
                [37u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(withdraw_withheld_authority_el_gamal_pubkey),
                            e
                        )
                    })?;
                    let withdraw_withheld_authority_el_gamal_pubkey: Option<[u8; 32usize]> =
                        if pubkey_bytes == [0u8; 32] {
                            None
                        } else {
                            Some(pubkey_bytes)
                        };
                    let args = InitializeConfidentialTransferFeeArguments {
                        confidential_transfer_fee_discriminator,
                        authority,
                        withdraw_withheld_authority_el_gamal_pubkey,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeConfidentialTransferFeeAccounts { remaining, mint };
                    return Ok(Instruction::InitializeConfidentialTransferFee { accounts, args });
                }
                [37u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
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
                    let new_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(new_decryptable_available_balance),
                                    e
                                )
                            })?;
                    let args = WithdrawWithheldTokensFromMintForConfidentialTransferFeeArguments {
                        confidential_transfer_fee_discriminator,
                        proof_instruction_offset,
                        new_decryptable_available_balance,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let mint = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                    let record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts =
                        WithdrawWithheldTokensFromMintForConfidentialTransferFeeAccounts {
                            remaining,
                            mint,
                            destination,
                            instructions_sysvar_or_context_state,
                            record,
                            authority,
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
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
                                e
                            )
                        })?;
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
                    let new_decryptable_available_balance: DecryptableBalance =
                        <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                            .map_err(|e| {
                                format!(
                                    "Failed to deserialize {}: {}",
                                    stringify!(new_decryptable_available_balance),
                                    e
                                )
                            })?;
                    let args =
                        WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeArguments {
                            confidential_transfer_fee_discriminator,
                            num_token_accounts,
                            proof_instruction_offset,
                            new_decryptable_available_balance,
                        };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let mint = keys.next().unwrap().clone();
                    let destination = keys.next().unwrap().clone();
                    let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                    let record = if opt_budget > 0 {
                        opt_budget -= 1;
                        Some(keys.next().unwrap().clone())
                    } else {
                        None
                    };
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts =
                        WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeAccounts {
                            remaining,
                            mint,
                            destination,
                            instructions_sysvar_or_context_state,
                            record,
                            authority,
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
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
                                e
                            )
                        })?;
                    let args = HarvestWithheldTokensToMintForConfidentialTransferFeeArguments {
                        confidential_transfer_fee_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = HarvestWithheldTokensToMintForConfidentialTransferFeeAccounts {
                        remaining,
                        mint,
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
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
                                e
                            )
                        })?;
                    let args = EnableHarvestToMintArguments {
                        confidential_transfer_fee_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EnableHarvestToMintAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::EnableHarvestToMint { accounts, args });
                }
                [37u8, 5u8] => {
                    let mut rdr: &[u8] = rest;
                    let confidential_transfer_fee_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(confidential_transfer_fee_discriminator),
                                e
                            )
                        })?;
                    let args = DisableHarvestToMintArguments {
                        confidential_transfer_fee_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = DisableHarvestToMintAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::DisableHarvestToMint { accounts, args });
                }
                [38u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = WithdrawExcessLamportsArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let source_account = keys.next().unwrap().clone();
                    let destination_account = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = WithdrawExcessLamportsAccounts {
                        remaining,
                        source_account,
                        destination_account,
                        authority,
                    };
                    return Ok(Instruction::WithdrawExcessLamports { accounts, args });
                }
                [39u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let metadata_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(metadata_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(metadata_address),
                            e
                        )
                    })?;
                    let metadata_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializeMetadataPointerArguments {
                        metadata_pointer_discriminator,
                        authority,
                        metadata_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeMetadataPointerAccounts { remaining, mint };
                    return Ok(Instruction::InitializeMetadataPointer { accounts, args });
                }
                [39u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let metadata_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(metadata_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(metadata_address),
                            e
                        )
                    })?;
                    let metadata_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateMetadataPointerArguments {
                        metadata_pointer_discriminator,
                        metadata_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let metadata_pointer_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateMetadataPointerAccounts {
                        remaining,
                        mint,
                        metadata_pointer_authority,
                    };
                    return Ok(Instruction::UpdateMetadataPointer { accounts, args });
                }
                [40u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let group_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(group_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(group_address),
                            e
                        )
                    })?;
                    let group_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializeGroupPointerArguments {
                        group_pointer_discriminator,
                        authority,
                        group_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeGroupPointerAccounts { remaining, mint };
                    return Ok(Instruction::InitializeGroupPointer { accounts, args });
                }
                [40u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let group_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(group_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(group_address),
                            e
                        )
                    })?;
                    let group_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateGroupPointerArguments {
                        group_pointer_discriminator,
                        group_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let group_pointer_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateGroupPointerAccounts {
                        remaining,
                        mint,
                        group_pointer_authority,
                    };
                    return Ok(Instruction::UpdateGroupPointer { accounts, args });
                }
                [41u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let group_member_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(group_member_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(member_address),
                            e
                        )
                    })?;
                    let member_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializeGroupMemberPointerArguments {
                        group_member_pointer_discriminator,
                        authority,
                        member_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeGroupMemberPointerAccounts { remaining, mint };
                    return Ok(Instruction::InitializeGroupMemberPointer { accounts, args });
                }
                [41u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let group_member_pointer_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(group_member_pointer_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(member_address),
                            e
                        )
                    })?;
                    let member_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateGroupMemberPointerArguments {
                        group_member_pointer_discriminator,
                        member_address,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let group_member_pointer_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateGroupMemberPointerAccounts {
                        remaining,
                        mint,
                        group_member_pointer_authority,
                    };
                    return Ok(Instruction::UpdateGroupMemberPointer { accounts, args });
                }
                [43u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let scaled_ui_amount_mint_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(scaled_ui_amount_mint_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let multiplier: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(multiplier), e)
                    })?;
                    let args = InitializeScaledUiAmountMintArguments {
                        scaled_ui_amount_mint_discriminator,
                        authority,
                        multiplier,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeScaledUiAmountMintAccounts { remaining, mint };
                    return Ok(Instruction::InitializeScaledUiAmountMint { accounts, args });
                }
                [43u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let scaled_ui_amount_mint_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(scaled_ui_amount_mint_discriminator),
                                e
                            )
                        })?;
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
                        scaled_ui_amount_mint_discriminator,
                        multiplier,
                        effective_timestamp,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateMultiplierScaledUiMintAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::UpdateMultiplierScaledUiMint { accounts, args });
                }
                [44u8, 0u8] => {
                    let mut rdr: &[u8] = rest;
                    let pausable_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(pausable_discriminator),
                                e
                            )
                        })?;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(authority),
                            e
                        )
                    })?;
                    let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = InitializePausableConfigArguments {
                        pausable_discriminator,
                        authority,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let mint = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializePausableConfigAccounts { remaining, mint };
                    return Ok(Instruction::InitializePausableConfig { accounts, args });
                }
                [44u8, 1u8] => {
                    let mut rdr: &[u8] = rest;
                    let pausable_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(pausable_discriminator),
                                e
                            )
                        })?;
                    let args = PauseArguments {
                        pausable_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = PauseAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::Pause { accounts, args });
                }
                [44u8, 2u8] => {
                    let mut rdr: &[u8] = rest;
                    let pausable_discriminator: u8 =
                        <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(pausable_discriminator),
                                e
                            )
                        })?;
                    let args = ResumeArguments {
                        pausable_discriminator,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let mint = keys.next().unwrap().clone();
                    let authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = ResumeAccounts {
                        remaining,
                        mint,
                        authority,
                    };
                    return Ok(Instruction::Resume { accounts, args });
                }
                [210u8, 225u8, 30u8, 162u8, 88u8, 184u8, 77u8, 141u8] => {
                    let mut rdr: &[u8] = rest;
                    let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize string length for {}: {}",
                                stringify!(name),
                                e
                            )
                        })?;
                    let mut string_bytes = vec![0u8; string_len as usize];
                    rdr.read_exact(&mut string_bytes).map_err(|e| {
                        format!(
                            "Failed to read string bytes for {}: {}",
                            stringify!(name),
                            e
                        )
                    })?;
                    let name: String = String::from_utf8(string_bytes).map_err(|e| {
                        format!(
                            "Failed to parse UTF-8 string for {}: {}",
                            stringify!(name),
                            e
                        )
                    })?;
                    let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize string length for {}: {}",
                                stringify!(symbol),
                                e
                            )
                        })?;
                    let mut string_bytes = vec![0u8; string_len as usize];
                    rdr.read_exact(&mut string_bytes).map_err(|e| {
                        format!(
                            "Failed to read string bytes for {}: {}",
                            stringify!(symbol),
                            e
                        )
                    })?;
                    let symbol: String = String::from_utf8(string_bytes).map_err(|e| {
                        format!(
                            "Failed to parse UTF-8 string for {}: {}",
                            stringify!(symbol),
                            e
                        )
                    })?;
                    let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize string length for {}: {}",
                                stringify!(uri),
                                e
                            )
                        })?;
                    let mut string_bytes = vec![0u8; string_len as usize];
                    rdr.read_exact(&mut string_bytes).map_err(|e| {
                        format!("Failed to read string bytes for {}: {}", stringify!(uri), e)
                    })?;
                    let uri: String = String::from_utf8(string_bytes).map_err(|e| {
                        format!(
                            "Failed to parse UTF-8 string for {}: {}",
                            stringify!(uri),
                            e
                        )
                    })?;
                    let args = InitializeTokenMetadataArguments { name, symbol, uri };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(4usize);
                    let metadata = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let mint_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeTokenMetadataAccounts {
                        remaining,
                        metadata,
                        update_authority,
                        mint,
                        mint_authority,
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
                    let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize string length for {}: {}",
                                stringify!(value),
                                e
                            )
                        })?;
                    let mut string_bytes = vec![0u8; string_len as usize];
                    rdr.read_exact(&mut string_bytes).map_err(|e| {
                        format!(
                            "Failed to read string bytes for {}: {}",
                            stringify!(value),
                            e
                        )
                    })?;
                    let value: String = String::from_utf8(string_bytes).map_err(|e| {
                        format!(
                            "Failed to parse UTF-8 string for {}: {}",
                            stringify!(value),
                            e
                        )
                    })?;
                    let args = UpdateTokenMetadataFieldArguments { field, value };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let metadata = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateTokenMetadataFieldAccounts {
                        remaining,
                        metadata,
                        update_authority,
                    };
                    return Ok(Instruction::UpdateTokenMetadataField { accounts, args });
                }
                [234u8, 18u8, 32u8, 56u8, 89u8, 141u8, 37u8, 181u8] => {
                    let mut rdr: &[u8] = rest;
                    let idempotent: bool =
                        <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                            |e| format!("Failed to deserialize {}: {}", stringify!(idempotent), e),
                        )?;
                    let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize string length for {}: {}",
                                stringify!(key),
                                e
                            )
                        })?;
                    let mut string_bytes = vec![0u8; string_len as usize];
                    rdr.read_exact(&mut string_bytes).map_err(|e| {
                        format!("Failed to read string bytes for {}: {}", stringify!(key), e)
                    })?;
                    let key: String = String::from_utf8(string_bytes).map_err(|e| {
                        format!(
                            "Failed to parse UTF-8 string for {}: {}",
                            stringify!(key),
                            e
                        )
                    })?;
                    let args = RemoveTokenMetadataKeyArguments { idempotent, key };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let metadata = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = RemoveTokenMetadataKeyAccounts {
                        remaining,
                        metadata,
                        update_authority,
                    };
                    return Ok(Instruction::RemoveTokenMetadataKey { accounts, args });
                }
                [215u8, 228u8, 166u8, 228u8, 84u8, 100u8, 86u8, 123u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(new_update_authority),
                            e
                        )
                    })?;
                    let new_update_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateTokenMetadataUpdateAuthorityArguments {
                        new_update_authority,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let metadata = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateTokenMetadataUpdateAuthorityAccounts {
                        remaining,
                        metadata,
                        update_authority,
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
                    let args = EmitTokenMetadataArguments { start, end };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(1usize);
                    let metadata = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = EmitTokenMetadataAccounts {
                        remaining,
                        metadata,
                    };
                    return Ok(Instruction::EmitTokenMetadata { accounts, args });
                }
                [121u8, 113u8, 108u8, 39u8, 54u8, 51u8, 0u8, 4u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(update_authority),
                            e
                        )
                    })?;
                    let update_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                    let args = InitializeTokenGroupArguments {
                        update_authority,
                        max_size,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(3usize);
                    let group = keys.next().unwrap().clone();
                    let mint = keys.next().unwrap().clone();
                    let mint_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeTokenGroupAccounts {
                        remaining,
                        group,
                        mint,
                        mint_authority,
                    };
                    return Ok(Instruction::InitializeTokenGroup { accounts, args });
                }
                [108u8, 37u8, 171u8, 143u8, 248u8, 30u8, 18u8, 110u8] => {
                    let mut rdr: &[u8] = rest;
                    let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                    let args = UpdateTokenGroupMaxSizeArguments { max_size };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let group = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateTokenGroupMaxSizeAccounts {
                        remaining,
                        group,
                        update_authority,
                    };
                    return Ok(Instruction::UpdateTokenGroupMaxSize { accounts, args });
                }
                [161u8, 105u8, 88u8, 1u8, 237u8, 221u8, 216u8, 203u8] => {
                    let mut rdr: &[u8] = rest;
                    let mut pubkey_bytes = [0u8; 32];
                    rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                        format!(
                            "Failed to read pubkey bytes for {}: {}",
                            stringify!(new_update_authority),
                            e
                        )
                    })?;
                    let new_update_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                    let args = UpdateTokenGroupUpdateAuthorityArguments {
                        new_update_authority,
                    };
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(2usize);
                    let group = keys.next().unwrap().clone();
                    let update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = UpdateTokenGroupUpdateAuthorityAccounts {
                        remaining,
                        group,
                        update_authority,
                    };
                    return Ok(Instruction::UpdateTokenGroupUpdateAuthority { accounts, args });
                }
                [152u8, 32u8, 222u8, 176u8, 223u8, 237u8, 116u8, 134u8] => {
                    let mut rdr: &[u8] = rest;
                    let args = InitializeTokenGroupMemberArguments {};
                    let mut keys = account_keys.iter();
                    let mut opt_budget = account_keys.len().saturating_sub(5usize);
                    let member = keys.next().unwrap().clone();
                    let member_mint = keys.next().unwrap().clone();
                    let member_mint_authority = keys.next().unwrap().clone();
                    let group = keys.next().unwrap().clone();
                    let group_update_authority = keys.next().unwrap().clone();
                    let remaining = keys.cloned().collect();
                    let accounts = InitializeTokenGroupMemberAccounts {
                        remaining,
                        member,
                        member_mint,
                        member_mint_authority,
                        group,
                        group_update_authority,
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
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeMintAccounts {
                    remaining,
                    mint,
                    rent,
                };
                return Ok(Instruction::InitializeMint { accounts, args });
            }
            [1u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeAccountArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let account = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccountAccounts {
                    remaining,
                    account,
                    mint,
                    owner,
                    rent,
                };
                return Ok(Instruction::InitializeAccount { accounts, args });
            }
            [2u8] => {
                let mut rdr: &[u8] = rest;
                let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                let args = InitializeMultisigArguments { m };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let multisig = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeMultisigAccounts {
                    remaining,
                    multisig,
                    rent,
                };
                return Ok(Instruction::InitializeMultisig { accounts, args });
            }
            [3u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let args = TransferArguments { amount };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let source = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferAccounts {
                    remaining,
                    source,
                    destination,
                    authority,
                };
                return Ok(Instruction::Transfer { accounts, args });
            }
            [4u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let args = ApproveArguments { amount };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let source = keys.next().unwrap().clone();
                let delegate = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ApproveAccounts {
                    remaining,
                    source,
                    delegate,
                    owner,
                };
                return Ok(Instruction::Approve { accounts, args });
            }
            [5u8] => {
                let mut rdr: &[u8] = rest;
                let args = RevokeArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let source = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RevokeAccounts {
                    remaining,
                    source,
                    owner,
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
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let owned = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetAuthorityAccounts {
                    remaining,
                    owned,
                    owner,
                };
                return Ok(Instruction::SetAuthority { accounts, args });
            }
            [7u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let args = MintToArguments { amount };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let mint = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let mint_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MintToAccounts {
                    remaining,
                    mint,
                    token,
                    mint_authority,
                };
                return Ok(Instruction::MintTo { accounts, args });
            }
            [8u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let args = BurnArguments { amount };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let account = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BurnAccounts {
                    remaining,
                    account,
                    mint,
                    authority,
                };
                return Ok(Instruction::Burn { accounts, args });
            }
            [9u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseAccountArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let account = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseAccountAccounts {
                    remaining,
                    account,
                    destination,
                    owner,
                };
                return Ok(Instruction::CloseAccount { accounts, args });
            }
            [10u8] => {
                let mut rdr: &[u8] = rest;
                let args = FreezeAccountArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let account = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FreezeAccountAccounts {
                    remaining,
                    account,
                    mint,
                    owner,
                };
                return Ok(Instruction::FreezeAccount { accounts, args });
            }
            [11u8] => {
                let mut rdr: &[u8] = rest;
                let args = ThawAccountArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let account = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ThawAccountAccounts {
                    remaining,
                    account,
                    mint,
                    owner,
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
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let source = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferCheckedAccounts {
                    remaining,
                    source,
                    mint,
                    destination,
                    authority,
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
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let source = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let delegate = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ApproveCheckedAccounts {
                    remaining,
                    source,
                    mint,
                    delegate,
                    owner,
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
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let mint = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let mint_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MintToCheckedAccounts {
                    remaining,
                    mint,
                    token,
                    mint_authority,
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
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let account = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BurnCheckedAccounts {
                    remaining,
                    account,
                    mint,
                    authority,
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
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let account = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccount2Accounts {
                    remaining,
                    account,
                    mint,
                    rent,
                };
                return Ok(Instruction::InitializeAccount2 { accounts, args });
            }
            [17u8] => {
                let mut rdr: &[u8] = rest;
                let args = SyncNativeArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let account = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SyncNativeAccounts { remaining, account };
                return Ok(Instruction::SyncNative { accounts, args });
            }
            [18u8] => {
                let mut rdr: &[u8] = rest;
                let owner: [u8; 32usize] =
                    <[u8; 32usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(
                        |e| format!("Failed to deserialize {}: {}", stringify!(owner), e),
                    )?;
                let args = InitializeAccount3Arguments { owner };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let account = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccount3Accounts {
                    remaining,
                    account,
                    mint,
                };
                return Ok(Instruction::InitializeAccount3 { accounts, args });
            }
            [19u8] => {
                let mut rdr: &[u8] = rest;
                let m: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(m), e))?;
                let args = InitializeMultisig2Arguments { m };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let multisig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeMultisig2Accounts {
                    remaining,
                    multisig,
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
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeMint2Accounts { remaining, mint };
                return Ok(Instruction::InitializeMint2 { accounts, args });
            }
            [21u8] => {
                let mut rdr: &[u8] = rest;
                let args = GetAccountDataSizeArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = GetAccountDataSizeAccounts { remaining, mint };
                return Ok(Instruction::GetAccountDataSize { accounts, args });
            }
            [22u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeImmutableOwnerArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let account = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeImmutableOwnerAccounts { remaining, account };
                return Ok(Instruction::InitializeImmutableOwner { accounts, args });
            }
            [23u8] => {
                let mut rdr: &[u8] = rest;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let args = AmountToUiAmountArguments { amount };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AmountToUiAmountAccounts { remaining, mint };
                return Ok(Instruction::AmountToUiAmount { accounts, args });
            }
            [24u8] => {
                let mut rdr: &[u8] = rest;
                let ui_amount: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(ui_amount), e)
                    })?;
                let args = UiAmountToAmountArguments { ui_amount };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UiAmountToAmountAccounts { remaining, mint };
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
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeMintCloseAuthorityAccounts { remaining, mint };
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
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeTransferFeeConfigAccounts { remaining, mint };
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
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let source = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferCheckedWithFeeAccounts {
                    remaining,
                    source,
                    mint,
                    destination,
                    authority,
                };
                return Ok(Instruction::TransferCheckedWithFee { accounts, args });
            }
            [26u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawWithheldTokensFromMintArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let mint = keys.next().unwrap().clone();
                let fee_receiver = keys.next().unwrap().clone();
                let withdraw_withheld_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawWithheldTokensFromMintAccounts {
                    remaining,
                    mint,
                    fee_receiver,
                    withdraw_withheld_authority,
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
                let args = WithdrawWithheldTokensFromAccountsArguments { num_token_accounts };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let mint = keys.next().unwrap().clone();
                let fee_receiver = keys.next().unwrap().clone();
                let withdraw_withheld_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawWithheldTokensFromAccountsAccounts {
                    remaining,
                    mint,
                    fee_receiver,
                    withdraw_withheld_authority,
                };
                return Ok(Instruction::WithdrawWithheldTokensFromAccounts { accounts, args });
            }
            [26u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let args = HarvestWithheldTokensToMintArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = HarvestWithheldTokensToMintAccounts { remaining, mint };
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
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let transfer_fee_config_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetTransferFeeAccounts {
                    remaining,
                    mint,
                    transfer_fee_config_authority,
                };
                return Ok(Instruction::SetTransferFee { accounts, args });
            }
            [27u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let auto_approve_new_accounts: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(auto_approve_new_accounts),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(auditor_elgamal_pubkey),
                        e
                    )
                })?;
                let auditor_elgamal_pubkey: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = InitializeConfidentialTransferMintArguments {
                    confidential_transfer_discriminator,
                    authority,
                    auto_approve_new_accounts,
                    auditor_elgamal_pubkey,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeConfidentialTransferMintAccounts { remaining, mint };
                return Ok(Instruction::InitializeConfidentialTransferMint { accounts, args });
            }
            [27u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let auto_approve_new_accounts: bool =
                    <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(auto_approve_new_accounts),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(auditor_elgamal_pubkey),
                        e
                    )
                })?;
                let auditor_elgamal_pubkey: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = UpdateConfidentialTransferMintArguments {
                    confidential_transfer_discriminator,
                    auto_approve_new_accounts,
                    auditor_elgamal_pubkey,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateConfidentialTransferMintAccounts {
                    remaining,
                    mint,
                    authority,
                };
                return Ok(Instruction::UpdateConfidentialTransferMint { accounts, args });
            }
            [27u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let decryptable_zero_balance: DecryptableBalance =
                    <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
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
                    confidential_transfer_discriminator,
                    decryptable_zero_balance,
                    maximum_pending_balance_credit_counter,
                    proof_instruction_offset,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let token = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                let record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ConfigureConfidentialTransferAccountAccounts {
                    remaining,
                    token,
                    mint,
                    instructions_sysvar_or_context_state,
                    record,
                    authority,
                };
                return Ok(Instruction::ConfigureConfidentialTransferAccount { accounts, args });
            }
            [27u8, 3u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let args = ApproveConfidentialTransferAccountArguments {
                    confidential_transfer_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let token = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ApproveConfidentialTransferAccountAccounts {
                    remaining,
                    token,
                    mint,
                    authority,
                };
                return Ok(Instruction::ApproveConfidentialTransferAccount { accounts, args });
            }
            [27u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
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
                let args = EmptyConfidentialTransferAccountArguments {
                    confidential_transfer_discriminator,
                    proof_instruction_offset,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let token = keys.next().unwrap().clone();
                let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                let record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EmptyConfidentialTransferAccountAccounts {
                    remaining,
                    token,
                    instructions_sysvar_or_context_state,
                    record,
                    authority,
                };
                return Ok(Instruction::EmptyConfidentialTransferAccount { accounts, args });
            }
            [27u8, 5u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                    })?;
                let args = ConfidentialDepositArguments {
                    confidential_transfer_discriminator,
                    amount,
                    decimals,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let token = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ConfidentialDepositAccounts {
                    remaining,
                    token,
                    mint,
                    authority,
                };
                return Ok(Instruction::ConfidentialDeposit { accounts, args });
            }
            [27u8, 6u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(amount), e))?;
                let decimals: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(decimals), e)
                    })?;
                let new_decryptable_available_balance: DecryptableBalance =
                    <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
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
                    confidential_transfer_discriminator,
                    amount,
                    decimals,
                    new_decryptable_available_balance,
                    equality_proof_instruction_offset,
                    range_proof_instruction_offset,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let token = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let instructions_sysvar = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let equality_record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let range_record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ConfidentialWithdrawAccounts {
                    remaining,
                    token,
                    mint,
                    instructions_sysvar,
                    equality_record,
                    range_record,
                    authority,
                };
                return Ok(Instruction::ConfidentialWithdraw { accounts, args });
            }
            [27u8, 7u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let new_source_decryptable_available_balance: DecryptableBalance =
                    <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
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
                    confidential_transfer_discriminator,
                    new_source_decryptable_available_balance,
                    equality_proof_instruction_offset,
                    ciphertext_validity_proof_instruction_offset,
                    range_proof_instruction_offset,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let source_token = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let destination_token = keys.next().unwrap().clone();
                let instructions_sysvar = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let equality_record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let ciphertext_validity_record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let range_record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ConfidentialTransferAccounts {
                    remaining,
                    source_token,
                    mint,
                    destination_token,
                    instructions_sysvar,
                    equality_record,
                    ciphertext_validity_record,
                    range_record,
                    authority,
                };
                return Ok(Instruction::ConfidentialTransfer { accounts, args });
            }
            [27u8, 8u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let expected_pending_balance_credit_counter: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(expected_pending_balance_credit_counter),
                            e
                        )
                    })?;
                let new_decryptable_available_balance: DecryptableBalance =
                    <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(new_decryptable_available_balance),
                                e
                            )
                        })?;
                let args = ApplyConfidentialPendingBalanceArguments {
                    confidential_transfer_discriminator,
                    expected_pending_balance_credit_counter,
                    new_decryptable_available_balance,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let token = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ApplyConfidentialPendingBalanceAccounts {
                    remaining,
                    token,
                    authority,
                };
                return Ok(Instruction::ApplyConfidentialPendingBalance { accounts, args });
            }
            [27u8, 9u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let args = EnableConfidentialCreditsArguments {
                    confidential_transfer_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let token = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EnableConfidentialCreditsAccounts {
                    remaining,
                    token,
                    authority,
                };
                return Ok(Instruction::EnableConfidentialCredits { accounts, args });
            }
            [27u8, 10u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let args = DisableConfidentialCreditsArguments {
                    confidential_transfer_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let token = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DisableConfidentialCreditsAccounts {
                    remaining,
                    token,
                    authority,
                };
                return Ok(Instruction::DisableConfidentialCredits { accounts, args });
            }
            [27u8, 11u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let args = EnableNonConfidentialCreditsArguments {
                    confidential_transfer_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let token = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EnableNonConfidentialCreditsAccounts {
                    remaining,
                    token,
                    authority,
                };
                return Ok(Instruction::EnableNonConfidentialCredits { accounts, args });
            }
            [27u8, 12u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let args = DisableNonConfidentialCreditsArguments {
                    confidential_transfer_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let token = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DisableNonConfidentialCreditsAccounts {
                    remaining,
                    token,
                    authority,
                };
                return Ok(Instruction::DisableNonConfidentialCredits { accounts, args });
            }
            [27u8, 13u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_discriminator),
                            e
                        )
                    })?;
                let new_source_decryptable_available_balance: DecryptableBalance =
                    <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
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
                    confidential_transfer_discriminator,
                    new_source_decryptable_available_balance,
                    equality_proof_instruction_offset,
                    transfer_amount_ciphertext_validity_proof_instruction_offset,
                    fee_sigma_proof_instruction_offset,
                    fee_ciphertext_validity_proof_instruction_offset,
                    range_proof_instruction_offset,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let source_token = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let destination_token = keys.next().unwrap().clone();
                let instructions_sysvar = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let equality_record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let transfer_amount_ciphertext_validity_record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let fee_sigma_record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let fee_ciphertext_validity_record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let range_record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ConfidentialTransferWithFeeAccounts {
                    remaining,
                    source_token,
                    mint,
                    destination_token,
                    instructions_sysvar,
                    equality_record,
                    transfer_amount_ciphertext_validity_record,
                    fee_sigma_record,
                    fee_ciphertext_validity_record,
                    range_record,
                    authority,
                };
                return Ok(Instruction::ConfidentialTransferWithFee { accounts, args });
            }
            [28u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let default_account_state_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(default_account_state_discriminator),
                            e
                        )
                    })?;
                let state: AccountState = <AccountState as ::borsh::BorshDeserialize>::deserialize(
                    &mut rdr,
                )
                .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(state), e))?;
                let args = InitializeDefaultAccountStateArguments {
                    default_account_state_discriminator,
                    state,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeDefaultAccountStateAccounts { remaining, mint };
                return Ok(Instruction::InitializeDefaultAccountState { accounts, args });
            }
            [28u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let default_account_state_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(default_account_state_discriminator),
                            e
                        )
                    })?;
                let state: AccountState = <AccountState as ::borsh::BorshDeserialize>::deserialize(
                    &mut rdr,
                )
                .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(state), e))?;
                let args = UpdateDefaultAccountStateArguments {
                    default_account_state_discriminator,
                    state,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let freeze_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateDefaultAccountStateAccounts {
                    remaining,
                    mint,
                    freeze_authority,
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
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let token = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ReallocateAccounts {
                    remaining,
                    token,
                    payer,
                    system_program,
                    owner,
                };
                return Ok(Instruction::Reallocate { accounts, args });
            }
            [30u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let memo_transfers_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(memo_transfers_discriminator),
                            e
                        )
                    })?;
                let args = EnableMemoTransfersArguments {
                    memo_transfers_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let token = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EnableMemoTransfersAccounts {
                    remaining,
                    token,
                    owner,
                };
                return Ok(Instruction::EnableMemoTransfers { accounts, args });
            }
            [30u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let memo_transfers_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(memo_transfers_discriminator),
                            e
                        )
                    })?;
                let args = DisableMemoTransfersArguments {
                    memo_transfers_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let token = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DisableMemoTransfersAccounts {
                    remaining,
                    token,
                    owner,
                };
                return Ok(Instruction::DisableMemoTransfers { accounts, args });
            }
            [31u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateNativeMintArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let payer = keys.next().unwrap().clone();
                let native_mint = keys.next().unwrap().clone();
                let system_program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateNativeMintAccounts {
                    remaining,
                    payer,
                    native_mint,
                    system_program,
                };
                return Ok(Instruction::CreateNativeMint { accounts, args });
            }
            [32u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeNonTransferableMintArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeNonTransferableMintAccounts { remaining, mint };
                return Ok(Instruction::InitializeNonTransferableMint { accounts, args });
            }
            [33u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let interest_bearing_mint_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(interest_bearing_mint_discriminator),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(rate_authority),
                        e
                    )
                })?;
                let rate_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(rate), e))?;
                let args = InitializeInterestBearingMintArguments {
                    interest_bearing_mint_discriminator,
                    rate_authority,
                    rate,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeInterestBearingMintAccounts { remaining, mint };
                return Ok(Instruction::InitializeInterestBearingMint { accounts, args });
            }
            [33u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let interest_bearing_mint_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(interest_bearing_mint_discriminator),
                            e
                        )
                    })?;
                let rate: i16 = <i16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| format!("Failed to deserialize {}: {}", stringify!(rate), e))?;
                let args = UpdateRateInterestBearingMintArguments {
                    interest_bearing_mint_discriminator,
                    rate,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let rate_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateRateInterestBearingMintAccounts {
                    remaining,
                    mint,
                    rate_authority,
                };
                return Ok(Instruction::UpdateRateInterestBearingMint { accounts, args });
            }
            [34u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let cpi_guard_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(cpi_guard_discriminator),
                            e
                        )
                    })?;
                let args = EnableCpiGuardArguments {
                    cpi_guard_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let token = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EnableCpiGuardAccounts {
                    remaining,
                    token,
                    owner,
                };
                return Ok(Instruction::EnableCpiGuard { accounts, args });
            }
            [34u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let cpi_guard_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(cpi_guard_discriminator),
                            e
                        )
                    })?;
                let args = DisableCpiGuardArguments {
                    cpi_guard_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let token = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DisableCpiGuardAccounts {
                    remaining,
                    token,
                    owner,
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
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePermanentDelegateAccounts { remaining, mint };
                return Ok(Instruction::InitializePermanentDelegate { accounts, args });
            }
            [36u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(program_id),
                        e
                    )
                })?;
                let program_id: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = InitializeTransferHookArguments {
                    authority,
                    program_id,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeTransferHookAccounts { remaining, mint };
                return Ok(Instruction::InitializeTransferHook { accounts, args });
            }
            [36u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(program_id),
                        e
                    )
                })?;
                let program_id: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = UpdateTransferHookArguments { program_id };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateTransferHookAccounts {
                    remaining,
                    mint,
                    authority,
                };
                return Ok(Instruction::UpdateTransferHook { accounts, args });
            }
            [37u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_fee_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_fee_discriminator),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(withdraw_withheld_authority_el_gamal_pubkey),
                        e
                    )
                })?;
                let withdraw_withheld_authority_el_gamal_pubkey: Option<[u8; 32usize]> =
                    if pubkey_bytes == [0u8; 32] {
                        None
                    } else {
                        Some(pubkey_bytes)
                    };
                let args = InitializeConfidentialTransferFeeArguments {
                    confidential_transfer_fee_discriminator,
                    authority,
                    withdraw_withheld_authority_el_gamal_pubkey,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeConfidentialTransferFeeAccounts { remaining, mint };
                return Ok(Instruction::InitializeConfidentialTransferFee { accounts, args });
            }
            [37u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_fee_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_fee_discriminator),
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
                let new_decryptable_available_balance: DecryptableBalance =
                    <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(new_decryptable_available_balance),
                                e
                            )
                        })?;
                let args = WithdrawWithheldTokensFromMintForConfidentialTransferFeeArguments {
                    confidential_transfer_fee_discriminator,
                    proof_instruction_offset,
                    new_decryptable_available_balance,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let mint = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                let record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawWithheldTokensFromMintForConfidentialTransferFeeAccounts {
                    remaining,
                    mint,
                    destination,
                    instructions_sysvar_or_context_state,
                    record,
                    authority,
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
                let confidential_transfer_fee_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_fee_discriminator),
                            e
                        )
                    })?;
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
                let new_decryptable_available_balance: DecryptableBalance =
                    <DecryptableBalance as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                        .map_err(|e| {
                            format!(
                                "Failed to deserialize {}: {}",
                                stringify!(new_decryptable_available_balance),
                                e
                            )
                        })?;
                let args = WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeArguments {
                    confidential_transfer_fee_discriminator,
                    num_token_accounts,
                    proof_instruction_offset,
                    new_decryptable_available_balance,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let mint = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let instructions_sysvar_or_context_state = keys.next().unwrap().clone();
                let record = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts =
                    WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeAccounts {
                        remaining,
                        mint,
                        destination,
                        instructions_sysvar_or_context_state,
                        record,
                        authority,
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
                let confidential_transfer_fee_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_fee_discriminator),
                            e
                        )
                    })?;
                let args = HarvestWithheldTokensToMintForConfidentialTransferFeeArguments {
                    confidential_transfer_fee_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = HarvestWithheldTokensToMintForConfidentialTransferFeeAccounts {
                    remaining,
                    mint,
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
                let confidential_transfer_fee_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_fee_discriminator),
                            e
                        )
                    })?;
                let args = EnableHarvestToMintArguments {
                    confidential_transfer_fee_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EnableHarvestToMintAccounts {
                    remaining,
                    mint,
                    authority,
                };
                return Ok(Instruction::EnableHarvestToMint { accounts, args });
            }
            [37u8, 5u8] => {
                let mut rdr: &[u8] = rest;
                let confidential_transfer_fee_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(confidential_transfer_fee_discriminator),
                            e
                        )
                    })?;
                let args = DisableHarvestToMintArguments {
                    confidential_transfer_fee_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DisableHarvestToMintAccounts {
                    remaining,
                    mint,
                    authority,
                };
                return Ok(Instruction::DisableHarvestToMint { accounts, args });
            }
            [38u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawExcessLamportsArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let source_account = keys.next().unwrap().clone();
                let destination_account = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawExcessLamportsAccounts {
                    remaining,
                    source_account,
                    destination_account,
                    authority,
                };
                return Ok(Instruction::WithdrawExcessLamports { accounts, args });
            }
            [39u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let metadata_pointer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(metadata_pointer_discriminator),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(metadata_address),
                        e
                    )
                })?;
                let metadata_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = InitializeMetadataPointerArguments {
                    metadata_pointer_discriminator,
                    authority,
                    metadata_address,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeMetadataPointerAccounts { remaining, mint };
                return Ok(Instruction::InitializeMetadataPointer { accounts, args });
            }
            [39u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let metadata_pointer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(metadata_pointer_discriminator),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(metadata_address),
                        e
                    )
                })?;
                let metadata_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = UpdateMetadataPointerArguments {
                    metadata_pointer_discriminator,
                    metadata_address,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let metadata_pointer_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateMetadataPointerAccounts {
                    remaining,
                    mint,
                    metadata_pointer_authority,
                };
                return Ok(Instruction::UpdateMetadataPointer { accounts, args });
            }
            [40u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let group_pointer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(group_pointer_discriminator),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(group_address),
                        e
                    )
                })?;
                let group_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = InitializeGroupPointerArguments {
                    group_pointer_discriminator,
                    authority,
                    group_address,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeGroupPointerAccounts { remaining, mint };
                return Ok(Instruction::InitializeGroupPointer { accounts, args });
            }
            [40u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let group_pointer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(group_pointer_discriminator),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(group_address),
                        e
                    )
                })?;
                let group_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = UpdateGroupPointerArguments {
                    group_pointer_discriminator,
                    group_address,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let group_pointer_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGroupPointerAccounts {
                    remaining,
                    mint,
                    group_pointer_authority,
                };
                return Ok(Instruction::UpdateGroupPointer { accounts, args });
            }
            [41u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let group_member_pointer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(group_member_pointer_discriminator),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(member_address),
                        e
                    )
                })?;
                let member_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = InitializeGroupMemberPointerArguments {
                    group_member_pointer_discriminator,
                    authority,
                    member_address,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeGroupMemberPointerAccounts { remaining, mint };
                return Ok(Instruction::InitializeGroupMemberPointer { accounts, args });
            }
            [41u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let group_member_pointer_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(group_member_pointer_discriminator),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(member_address),
                        e
                    )
                })?;
                let member_address: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = UpdateGroupMemberPointerArguments {
                    group_member_pointer_discriminator,
                    member_address,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let group_member_pointer_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGroupMemberPointerAccounts {
                    remaining,
                    mint,
                    group_member_pointer_authority,
                };
                return Ok(Instruction::UpdateGroupMemberPointer { accounts, args });
            }
            [43u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let scaled_ui_amount_mint_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(scaled_ui_amount_mint_discriminator),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let multiplier: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(multiplier), e)
                    })?;
                let args = InitializeScaledUiAmountMintArguments {
                    scaled_ui_amount_mint_discriminator,
                    authority,
                    multiplier,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeScaledUiAmountMintAccounts { remaining, mint };
                return Ok(Instruction::InitializeScaledUiAmountMint { accounts, args });
            }
            [43u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let scaled_ui_amount_mint_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(scaled_ui_amount_mint_discriminator),
                            e
                        )
                    })?;
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
                    scaled_ui_amount_mint_discriminator,
                    multiplier,
                    effective_timestamp,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateMultiplierScaledUiMintAccounts {
                    remaining,
                    mint,
                    authority,
                };
                return Ok(Instruction::UpdateMultiplierScaledUiMint { accounts, args });
            }
            [44u8, 0u8] => {
                let mut rdr: &[u8] = rest;
                let pausable_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(pausable_discriminator),
                            e
                        )
                    })?;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(authority),
                        e
                    )
                })?;
                let authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = InitializePausableConfigArguments {
                    pausable_discriminator,
                    authority,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializePausableConfigAccounts { remaining, mint };
                return Ok(Instruction::InitializePausableConfig { accounts, args });
            }
            [44u8, 1u8] => {
                let mut rdr: &[u8] = rest;
                let pausable_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(pausable_discriminator),
                            e
                        )
                    })?;
                let args = PauseArguments {
                    pausable_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PauseAccounts {
                    remaining,
                    mint,
                    authority,
                };
                return Ok(Instruction::Pause { accounts, args });
            }
            [44u8, 2u8] => {
                let mut rdr: &[u8] = rest;
                let pausable_discriminator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr).map_err(|e| {
                        format!(
                            "Failed to deserialize {}: {}",
                            stringify!(pausable_discriminator),
                            e
                        )
                    })?;
                let args = ResumeArguments {
                    pausable_discriminator,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ResumeAccounts {
                    remaining,
                    mint,
                    authority,
                };
                return Ok(Instruction::Resume { accounts, args });
            }
            [210u8, 225u8, 30u8, 162u8, 88u8, 184u8, 77u8, 141u8] => {
                let mut rdr: &[u8] = rest;
                let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!(
                            "Failed to deserialize string length for {}: {}",
                            stringify!(name),
                            e
                        )
                    })?;
                let mut string_bytes = vec![0u8; string_len as usize];
                rdr.read_exact(&mut string_bytes).map_err(|e| {
                    format!(
                        "Failed to read string bytes for {}: {}",
                        stringify!(name),
                        e
                    )
                })?;
                let name: String = String::from_utf8(string_bytes).map_err(|e| {
                    format!(
                        "Failed to parse UTF-8 string for {}: {}",
                        stringify!(name),
                        e
                    )
                })?;
                let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!(
                            "Failed to deserialize string length for {}: {}",
                            stringify!(symbol),
                            e
                        )
                    })?;
                let mut string_bytes = vec![0u8; string_len as usize];
                rdr.read_exact(&mut string_bytes).map_err(|e| {
                    format!(
                        "Failed to read string bytes for {}: {}",
                        stringify!(symbol),
                        e
                    )
                })?;
                let symbol: String = String::from_utf8(string_bytes).map_err(|e| {
                    format!(
                        "Failed to parse UTF-8 string for {}: {}",
                        stringify!(symbol),
                        e
                    )
                })?;
                let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!(
                            "Failed to deserialize string length for {}: {}",
                            stringify!(uri),
                            e
                        )
                    })?;
                let mut string_bytes = vec![0u8; string_len as usize];
                rdr.read_exact(&mut string_bytes).map_err(|e| {
                    format!("Failed to read string bytes for {}: {}", stringify!(uri), e)
                })?;
                let uri: String = String::from_utf8(string_bytes).map_err(|e| {
                    format!(
                        "Failed to parse UTF-8 string for {}: {}",
                        stringify!(uri),
                        e
                    )
                })?;
                let args = InitializeTokenMetadataArguments { name, symbol, uri };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let metadata = keys.next().unwrap().clone();
                let update_authority = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let mint_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeTokenMetadataAccounts {
                    remaining,
                    metadata,
                    update_authority,
                    mint,
                    mint_authority,
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
                let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!(
                            "Failed to deserialize string length for {}: {}",
                            stringify!(value),
                            e
                        )
                    })?;
                let mut string_bytes = vec![0u8; string_len as usize];
                rdr.read_exact(&mut string_bytes).map_err(|e| {
                    format!(
                        "Failed to read string bytes for {}: {}",
                        stringify!(value),
                        e
                    )
                })?;
                let value: String = String::from_utf8(string_bytes).map_err(|e| {
                    format!(
                        "Failed to parse UTF-8 string for {}: {}",
                        stringify!(value),
                        e
                    )
                })?;
                let args = UpdateTokenMetadataFieldArguments { field, value };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let metadata = keys.next().unwrap().clone();
                let update_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateTokenMetadataFieldAccounts {
                    remaining,
                    metadata,
                    update_authority,
                };
                return Ok(Instruction::UpdateTokenMetadataField { accounts, args });
            }
            [234u8, 18u8, 32u8, 56u8, 89u8, 141u8, 37u8, 181u8] => {
                let mut rdr: &[u8] = rest;
                let idempotent: bool = <bool as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                    format!("Failed to deserialize {}: {}", stringify!(idempotent), e)
                })?;
                let string_len: u32 = <u32 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!(
                            "Failed to deserialize string length for {}: {}",
                            stringify!(key),
                            e
                        )
                    })?;
                let mut string_bytes = vec![0u8; string_len as usize];
                rdr.read_exact(&mut string_bytes).map_err(|e| {
                    format!("Failed to read string bytes for {}: {}", stringify!(key), e)
                })?;
                let key: String = String::from_utf8(string_bytes).map_err(|e| {
                    format!(
                        "Failed to parse UTF-8 string for {}: {}",
                        stringify!(key),
                        e
                    )
                })?;
                let args = RemoveTokenMetadataKeyArguments { idempotent, key };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let metadata = keys.next().unwrap().clone();
                let update_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveTokenMetadataKeyAccounts {
                    remaining,
                    metadata,
                    update_authority,
                };
                return Ok(Instruction::RemoveTokenMetadataKey { accounts, args });
            }
            [215u8, 228u8, 166u8, 228u8, 84u8, 100u8, 86u8, 123u8] => {
                let mut rdr: &[u8] = rest;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(new_update_authority),
                        e
                    )
                })?;
                let new_update_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = UpdateTokenMetadataUpdateAuthorityArguments {
                    new_update_authority,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let metadata = keys.next().unwrap().clone();
                let update_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateTokenMetadataUpdateAuthorityAccounts {
                    remaining,
                    metadata,
                    update_authority,
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
                let args = EmitTokenMetadataArguments { start, end };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(1usize);
                let metadata = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = EmitTokenMetadataAccounts {
                    remaining,
                    metadata,
                };
                return Ok(Instruction::EmitTokenMetadata { accounts, args });
            }
            [121u8, 113u8, 108u8, 39u8, 54u8, 51u8, 0u8, 4u8] => {
                let mut rdr: &[u8] = rest;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(update_authority),
                        e
                    )
                })?;
                let update_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                let args = InitializeTokenGroupArguments {
                    update_authority,
                    max_size,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let group = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let mint_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeTokenGroupAccounts {
                    remaining,
                    group,
                    mint,
                    mint_authority,
                };
                return Ok(Instruction::InitializeTokenGroup { accounts, args });
            }
            [108u8, 37u8, 171u8, 143u8, 248u8, 30u8, 18u8, 110u8] => {
                let mut rdr: &[u8] = rest;
                let max_size: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)
                    .map_err(|e| {
                        format!("Failed to deserialize {}: {}", stringify!(max_size), e)
                    })?;
                let args = UpdateTokenGroupMaxSizeArguments { max_size };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let group = keys.next().unwrap().clone();
                let update_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateTokenGroupMaxSizeAccounts {
                    remaining,
                    group,
                    update_authority,
                };
                return Ok(Instruction::UpdateTokenGroupMaxSize { accounts, args });
            }
            [161u8, 105u8, 88u8, 1u8, 237u8, 221u8, 216u8, 203u8] => {
                let mut rdr: &[u8] = rest;
                let mut pubkey_bytes = [0u8; 32];
                rdr.read_exact(&mut pubkey_bytes).map_err(|e| {
                    format!(
                        "Failed to read pubkey bytes for {}: {}",
                        stringify!(new_update_authority),
                        e
                    )
                })?;
                let new_update_authority: Option<[u8; 32usize]> = if pubkey_bytes == [0u8; 32] {
                    None
                } else {
                    Some(pubkey_bytes)
                };
                let args = UpdateTokenGroupUpdateAuthorityArguments {
                    new_update_authority,
                };
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let group = keys.next().unwrap().clone();
                let update_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateTokenGroupUpdateAuthorityAccounts {
                    remaining,
                    group,
                    update_authority,
                };
                return Ok(Instruction::UpdateTokenGroupUpdateAuthority { accounts, args });
            }
            [152u8, 32u8, 222u8, 176u8, 223u8, 237u8, 116u8, 134u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeTokenGroupMemberArguments {};
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(5usize);
                let member = keys.next().unwrap().clone();
                let member_mint = keys.next().unwrap().clone();
                let member_mint_authority = keys.next().unwrap().clone();
                let group = keys.next().unwrap().clone();
                let group_update_authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeTokenGroupMemberAccounts {
                    remaining,
                    member,
                    member_mint,
                    member_mint_authority,
                    group,
                    group_update_authority,
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
