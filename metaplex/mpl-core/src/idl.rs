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
    pub type __TupleHookableLifecycleEventExternalCheckResult =
        (HookableLifecycleEvent, ExternalCheckResult);
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PluginAuthorityPair {
        pub plugin: Plugin,
        pub authority: Option<Authority>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AppData {
        pub data_authority: Authority,
        pub schema: ExternalPluginAdapterSchema,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AppDataInitInfo {
        pub data_authority: Authority,
        pub init_plugin_authority: Option<Authority>,
        pub schema: Option<ExternalPluginAdapterSchema>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AppDataUpdateInfo {
        pub schema: Option<ExternalPluginAdapterSchema>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DataSection {
        pub parent_key: LinkedDataKey,
        pub schema: ExternalPluginAdapterSchema,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DataSectionInitInfo {
        pub parent_key: LinkedDataKey,
        pub schema: ExternalPluginAdapterSchema,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DataSectionUpdateInfo {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LifecycleHook {
        #[serde(with = "pubkey_serde")]
        pub hooked_program: [u8; 32usize],
        pub extra_accounts: Option<Vec<ExtraAccount>>,
        pub data_authority: Option<Authority>,
        pub schema: ExternalPluginAdapterSchema,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LifecycleHookInitInfo {
        #[serde(with = "pubkey_serde")]
        pub hooked_program: [u8; 32usize],
        pub init_plugin_authority: Option<Authority>,
        pub lifecycle_checks: Vec<__TupleHookableLifecycleEventExternalCheckResult>,
        pub extra_accounts: Option<Vec<ExtraAccount>>,
        pub data_authority: Option<Authority>,
        pub schema: Option<ExternalPluginAdapterSchema>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LifecycleHookUpdateInfo {
        pub lifecycle_checks: Option<Vec<__TupleHookableLifecycleEventExternalCheckResult>>,
        pub extra_accounts: Option<Vec<ExtraAccount>>,
        pub schema: Option<ExternalPluginAdapterSchema>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LinkedAppData {
        pub data_authority: Authority,
        pub schema: ExternalPluginAdapterSchema,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LinkedAppDataInitInfo {
        pub data_authority: Authority,
        pub init_plugin_authority: Option<Authority>,
        pub schema: Option<ExternalPluginAdapterSchema>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LinkedAppDataUpdateInfo {
        pub schema: Option<ExternalPluginAdapterSchema>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LinkedLifecycleHook {
        #[serde(with = "pubkey_serde")]
        pub hooked_program: [u8; 32usize],
        pub extra_accounts: Option<Vec<ExtraAccount>>,
        pub data_authority: Option<Authority>,
        pub schema: ExternalPluginAdapterSchema,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LinkedLifecycleHookInitInfo {
        #[serde(with = "pubkey_serde")]
        pub hooked_program: [u8; 32usize],
        pub init_plugin_authority: Option<Authority>,
        pub lifecycle_checks: Vec<__TupleHookableLifecycleEventExternalCheckResult>,
        pub extra_accounts: Option<Vec<ExtraAccount>>,
        pub data_authority: Option<Authority>,
        pub schema: Option<ExternalPluginAdapterSchema>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct LinkedLifecycleHookUpdateInfo {
        pub lifecycle_checks: Option<Vec<__TupleHookableLifecycleEventExternalCheckResult>>,
        pub extra_accounts: Option<Vec<ExtraAccount>>,
        pub schema: Option<ExternalPluginAdapterSchema>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Oracle {
        #[serde(with = "pubkey_serde")]
        pub base_address: [u8; 32usize],
        pub base_address_config: Option<ExtraAccount>,
        pub results_offset: ValidationResultsOffset,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OracleInitInfo {
        #[serde(with = "pubkey_serde")]
        pub base_address: [u8; 32usize],
        pub init_plugin_authority: Option<Authority>,
        pub lifecycle_checks: Vec<__TupleHookableLifecycleEventExternalCheckResult>,
        pub base_address_config: Option<ExtraAccount>,
        pub results_offset: Option<ValidationResultsOffset>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct OracleUpdateInfo {
        pub lifecycle_checks: Option<Vec<__TupleHookableLifecycleEventExternalCheckResult>>,
        pub base_address_config: Option<ExtraAccount>,
        pub results_offset: Option<ValidationResultsOffset>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AddBlocker {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Attribute {
        pub key: String,
        pub value: String,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Attributes {
        pub attribute_list: Vec<Attribute>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ImmutableMetadata {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MasterEdition {
        pub max_supply: Option<u32>,
        pub name: Option<String>,
        pub uri: Option<String>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Creator {
        #[serde(with = "pubkey_serde")]
        pub address: [u8; 32usize],
        pub percentage: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Royalties {
        pub basis_points: u16,
        pub creators: Vec<Creator>,
        pub rule_set: RuleSet,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateDelegate {
        pub additional_delegates: Vec<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct VerifiedCreatorsSignature {
        #[serde(with = "pubkey_serde")]
        pub address: [u8; 32usize],
        pub verified: bool,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct VerifiedCreators {
        pub signatures: Vec<VerifiedCreatorsSignature>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AutographSignature {
        #[serde(with = "pubkey_serde")]
        pub address: [u8; 32usize],
        pub message: String,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Autograph {
        pub signatures: Vec<AutographSignature>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BurnDelegate {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct FreezeDelegate {
        pub frozen: bool,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TransferDelegate {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BubblegumV2 {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Edition {
        pub number: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PermanentBurnDelegate {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PermanentFreezeDelegate {
        pub frozen: bool,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PermanentTransferDelegate {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ExternalCheckResult {
        pub flags: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RegistryRecord {
        pub plugin_type: PluginType,
        pub authority: Authority,
        pub offset: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ExternalRegistryRecord {
        pub plugin_type: ExternalPluginAdapterType,
        pub authority: Authority,
        pub lifecycle_checks: Option<Vec<__TupleHookableLifecycleEventExternalCheckResult>>,
        pub offset: u64,
        pub data_offset: Option<u64>,
        pub data_len: Option<u64>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AddExternalPluginAdapterV1Args {
        pub init_info: ExternalPluginAdapterInitInfo,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AddCollectionExternalPluginAdapterV1Args {
        pub init_info: ExternalPluginAdapterInitInfo,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AddPluginV1Args {
        pub plugin: Plugin,
        pub init_authority: Option<Authority>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AddCollectionPluginV1Args {
        pub plugin: Plugin,
        pub init_authority: Option<Authority>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ApprovePluginAuthorityV1Args {
        pub plugin_type: PluginType,
        pub new_authority: Authority,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ApproveCollectionPluginAuthorityV1Args {
        pub plugin_type: PluginType,
        pub new_authority: Authority,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BurnV1Args {
        pub compression_proof: Option<CompressionProof>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct BurnCollectionV1Args {
        pub compression_proof: Option<CompressionProof>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CompressV1Args {}
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreateV1Args {
        pub data_state: DataState,
        pub name: String,
        pub uri: String,
        pub plugins: Option<Vec<PluginAuthorityPair>>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreateV2Args {
        pub data_state: DataState,
        pub name: String,
        pub uri: String,
        pub plugins: Option<Vec<PluginAuthorityPair>>,
        pub external_plugin_adapters: Option<Vec<ExternalPluginAdapterInitInfo>>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreateCollectionV1Args {
        pub name: String,
        pub uri: String,
        pub plugins: Option<Vec<PluginAuthorityPair>>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreateCollectionV2Args {
        pub name: String,
        pub uri: String,
        pub plugins: Option<Vec<PluginAuthorityPair>>,
        pub external_plugin_adapters: Option<Vec<ExternalPluginAdapterInitInfo>>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DecompressV1Args {
        pub compression_proof: CompressionProof,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ExecuteV1Args {
        pub instruction_data: Vec<u8>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RemoveExternalPluginAdapterV1Args {
        pub key: ExternalPluginAdapterKey,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RemoveCollectionExternalPluginAdapterV1Args {
        pub key: ExternalPluginAdapterKey,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RemovePluginV1Args {
        pub plugin_type: PluginType,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RemoveCollectionPluginV1Args {
        pub plugin_type: PluginType,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RevokePluginAuthorityV1Args {
        pub plugin_type: PluginType,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct RevokeCollectionPluginAuthorityV1Args {
        pub plugin_type: PluginType,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TransferV1Args {
        pub compression_proof: Option<CompressionProof>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateV1Args {
        pub new_name: Option<String>,
        pub new_uri: Option<String>,
        pub new_update_authority: Option<UpdateAuthority>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateV2Args {
        pub new_name: Option<String>,
        pub new_uri: Option<String>,
        pub new_update_authority: Option<UpdateAuthority>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateCollectionV1Args {
        pub new_name: Option<String>,
        pub new_uri: Option<String>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateCollectionInfoV1Args {
        pub update_type: UpdateType,
        pub amount: u32,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateExternalPluginAdapterV1Args {
        pub key: ExternalPluginAdapterKey,
        pub update_info: ExternalPluginAdapterUpdateInfo,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateCollectionExternalPluginAdapterV1Args {
        pub key: ExternalPluginAdapterKey,
        pub update_info: ExternalPluginAdapterUpdateInfo,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdatePluginV1Args {
        pub plugin: Plugin,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateCollectionPluginV1Args {
        pub plugin: Plugin,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WriteExternalPluginAdapterDataV1Args {
        pub key: ExternalPluginAdapterKey,
        pub data: Option<Vec<u8>>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct WriteCollectionExternalPluginAdapterDataV1Args {
        pub key: ExternalPluginAdapterKey,
        pub data: Option<Vec<u8>>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CompressionProof {
        #[serde(with = "pubkey_serde")]
        pub owner: [u8; 32usize],
        pub update_authority: UpdateAuthority,
        pub name: String,
        pub uri: String,
        pub seq: u64,
        pub plugins: Vec<HashablePluginSchema>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct HashablePluginSchema {
        pub index: u64,
        pub authority: Authority,
        pub plugin: Plugin,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct HashedAssetSchema {
        pub asset_hash: [u8; 32usize],
        pub plugin_hashes: Vec<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Plugin {
        Royalties(Royalties),
        FreezeDelegate(FreezeDelegate),
        BurnDelegate(BurnDelegate),
        TransferDelegate(TransferDelegate),
        UpdateDelegate(UpdateDelegate),
        PermanentFreezeDelegate(PermanentFreezeDelegate),
        Attributes(Attributes),
        PermanentTransferDelegate(PermanentTransferDelegate),
        PermanentBurnDelegate(PermanentBurnDelegate),
        Edition(Edition),
        MasterEdition(MasterEdition),
        AddBlocker(AddBlocker),
        ImmutableMetadata(ImmutableMetadata),
        VerifiedCreators(VerifiedCreators),
        Autograph(Autograph),
        BubblegumV2(BubblegumV2),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PluginType {
        Royalties,
        FreezeDelegate,
        BurnDelegate,
        TransferDelegate,
        UpdateDelegate,
        PermanentFreezeDelegate,
        Attributes,
        PermanentTransferDelegate,
        PermanentBurnDelegate,
        Edition,
        MasterEdition,
        AddBlocker,
        ImmutableMetadata,
        VerifiedCreators,
        Autograph,
        BubblegumV2,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ValidationResultsOffset {
        NoOffset,
        Anchor,
        Custom(u64),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OracleValidation {
        Uninitialized,
        V1 {
            create: ExternalValidationResult,
            transfer: ExternalValidationResult,
            burn: ExternalValidationResult,
            update: ExternalValidationResult,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ExternalPluginAdapterType {
        LifecycleHook,
        Oracle,
        AppData,
        LinkedLifecycleHook,
        LinkedAppData,
        DataSection,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ExternalPluginAdapter {
        LifecycleHook(LifecycleHook),
        Oracle(Oracle),
        AppData(AppData),
        LinkedLifecycleHook(LinkedLifecycleHook),
        LinkedAppData(LinkedAppData),
        DataSection(DataSection),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum HookableLifecycleEvent {
        Create,
        Transfer,
        Burn,
        Update,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ExtraAccount {
        PreconfiguredProgram {
            is_signer: bool,
            is_writable: bool,
        },
        PreconfiguredCollection {
            is_signer: bool,
            is_writable: bool,
        },
        PreconfiguredOwner {
            is_signer: bool,
            is_writable: bool,
        },
        PreconfiguredRecipient {
            is_signer: bool,
            is_writable: bool,
        },
        PreconfiguredAsset {
            is_signer: bool,
            is_writable: bool,
        },
        CustomPda {
            seeds: Vec<Seed>,
            #[serde(with = "pubkey_serde_option")]
            custom_program_id: Option<[u8; 32usize]>,
            is_signer: bool,
            is_writable: bool,
        },
        Address {
            #[serde(with = "pubkey_serde")]
            address: [u8; 32usize],
            is_signer: bool,
            is_writable: bool,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Seed {
        Collection,
        Owner,
        Recipient,
        Asset,
        Address([u8; 32usize]),
        Bytes(Vec<u8>),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ExternalPluginAdapterSchema {
        Binary,
        Json,
        MsgPack,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ExternalPluginAdapterInitInfo {
        LifecycleHook(LifecycleHookInitInfo),
        Oracle(OracleInitInfo),
        AppData(AppDataInitInfo),
        LinkedLifecycleHook(LinkedLifecycleHookInitInfo),
        LinkedAppData(LinkedAppDataInitInfo),
        DataSection(DataSectionInitInfo),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ExternalPluginAdapterUpdateInfo {
        LifecycleHook(LifecycleHookUpdateInfo),
        Oracle(OracleUpdateInfo),
        AppData(AppDataUpdateInfo),
        LinkedLifecycleHook(LinkedLifecycleHookUpdateInfo),
        LinkedAppData(LinkedAppDataUpdateInfo),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ExternalPluginAdapterKey {
        LifecycleHook([u8; 32usize]),
        Oracle([u8; 32usize]),
        AppData(Authority),
        LinkedLifecycleHook([u8; 32usize]),
        LinkedAppData(Authority),
        DataSection(LinkedDataKey),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum LinkedDataKey {
        LinkedLifecycleHook([u8; 32usize]),
        LinkedAppData(Authority),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RuleSet {
        None,
        ProgramAllowList(Vec<[u8; 32usize]>),
        ProgramDenyList(Vec<[u8; 32usize]>),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ValidationResult {
        Approved,
        Rejected,
        Pass,
        ForceApproved,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ExternalValidationResult {
        Approved,
        Rejected,
        Pass,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateType {
        Mint,
        Add,
        Remove,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum DataState {
        AccountState,
        LedgerState,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Authority {
        None,
        Owner,
        UpdateAuthority,
        Address {
            #[serde(with = "pubkey_serde")]
            address: [u8; 32usize],
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Key {
        Uninitialized,
        AssetV1,
        HashedAssetV1,
        PluginHeaderV1,
        PluginRegistryV1,
        CollectionV1,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateAuthority {
        None,
        Address([u8; 32usize]),
        Collection([u8; 32usize]),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TupleHookableLifecycleEventExternalCheckResult {
        pub field0: HookableLifecycleEvent,
        pub field1: ExternalCheckResult,
    }
}
pub mod accounts_data {
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct CreateV1Accounts {
        pub asset: String,
        pub collection: String,
        pub authority: String,
        pub payer: String,
        pub owner: String,
        pub updateAuthority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateCollectionV1Accounts {
        pub collection: String,
        pub updateAuthority: String,
        pub payer: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddPluginV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddCollectionPluginV1Accounts {
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemovePluginV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveCollectionPluginV1Accounts {
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePluginV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateCollectionPluginV1Accounts {
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ApprovePluginAuthorityV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ApproveCollectionPluginAuthorityV1Accounts {
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RevokePluginAuthorityV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RevokeCollectionPluginAuthorityV1Accounts {
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BurnV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BurnCollectionV1Accounts {
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub newOwner: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateCollectionV1Accounts {
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub newUpdateAuthority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CompressV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DecompressV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectAccounts {
        pub recipient1: String,
        pub recipient2: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateV2Accounts {
        pub asset: String,
        pub collection: String,
        pub authority: String,
        pub payer: String,
        pub owner: String,
        pub updateAuthority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateCollectionV2Accounts {
        pub collection: String,
        pub updateAuthority: String,
        pub payer: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddExternalPluginAdapterV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AddCollectionExternalPluginAdapterV1Accounts {
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveExternalPluginAdapterV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveCollectionExternalPluginAdapterV1Accounts {
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateExternalPluginAdapterV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateCollectionExternalPluginAdapterV1Accounts {
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WriteExternalPluginAdapterDataV1Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub buffer: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WriteCollectionExternalPluginAdapterDataV1Accounts {
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub buffer: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateV2Accounts {
        pub asset: String,
        pub collection: String,
        pub payer: String,
        pub authority: String,
        pub newCollection: String,
        pub systemProgram: String,
        pub logWrapper: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ExecuteV1Accounts {
        pub asset: String,
        pub collection: String,
        pub assetSigner: String,
        pub payer: String,
        pub authority: String,
        pub systemProgram: String,
        pub programId: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateCollectionInfoV1Accounts {
        pub collection: String,
        pub bubblegumSigner: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateV1Arguments {
        pub create_v1_args: CreateV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateCollectionV1Arguments {
        pub create_collection_v1_args: CreateCollectionV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddPluginV1Arguments {
        pub add_plugin_v1_args: AddPluginV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddCollectionPluginV1Arguments {
        pub add_collection_plugin_v1_args: AddCollectionPluginV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemovePluginV1Arguments {
        pub remove_plugin_v1_args: RemovePluginV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveCollectionPluginV1Arguments {
        pub remove_collection_plugin_v1_args: RemoveCollectionPluginV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePluginV1Arguments {
        pub update_plugin_v1_args: UpdatePluginV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateCollectionPluginV1Arguments {
        pub update_collection_plugin_v1_args: UpdateCollectionPluginV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ApprovePluginAuthorityV1Arguments {
        pub approve_plugin_authority_v1_args: ApprovePluginAuthorityV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ApproveCollectionPluginAuthorityV1Arguments {
        pub approve_collection_plugin_authority_v1_args: ApproveCollectionPluginAuthorityV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RevokePluginAuthorityV1Arguments {
        pub revoke_plugin_authority_v1_args: RevokePluginAuthorityV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RevokeCollectionPluginAuthorityV1Arguments {
        pub revoke_collection_plugin_authority_v1_args: RevokeCollectionPluginAuthorityV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BurnV1Arguments {
        pub burn_v1_args: BurnV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BurnCollectionV1Arguments {
        pub burn_collection_v1_args: BurnCollectionV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferV1Arguments {
        pub transfer_v1_args: TransferV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateV1Arguments {
        pub update_v1_args: UpdateV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateCollectionV1Arguments {
        pub update_collection_v1_args: UpdateCollectionV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CompressV1Arguments {
        pub compress_v1_args: CompressV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DecompressV1Arguments {
        pub decompress_v1_args: DecompressV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateV2Arguments {
        pub create_v2_args: CreateV2Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateCollectionV2Arguments {
        pub create_collection_v2_args: CreateCollectionV2Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddExternalPluginAdapterV1Arguments {
        pub add_external_plugin_adapter_v1_args: AddExternalPluginAdapterV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AddCollectionExternalPluginAdapterV1Arguments {
        pub add_collection_external_plugin_adapter_v1_args:
            AddCollectionExternalPluginAdapterV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveExternalPluginAdapterV1Arguments {
        pub remove_external_plugin_adapter_v1_args: RemoveExternalPluginAdapterV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveCollectionExternalPluginAdapterV1Arguments {
        pub remove_collection_external_plugin_adapter_v1_args:
            RemoveCollectionExternalPluginAdapterV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateExternalPluginAdapterV1Arguments {
        pub update_external_plugin_adapter_v1_args: UpdateExternalPluginAdapterV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateCollectionExternalPluginAdapterV1Arguments {
        pub update_collection_external_plugin_adapter_v1_args:
            UpdateCollectionExternalPluginAdapterV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WriteExternalPluginAdapterDataV1Arguments {
        pub write_external_plugin_adapter_data_v1_args: WriteExternalPluginAdapterDataV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WriteCollectionExternalPluginAdapterDataV1Arguments {
        pub write_collection_external_plugin_adapter_data_v1_args:
            WriteCollectionExternalPluginAdapterDataV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateV2Arguments {
        pub update_v2_args: UpdateV2Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ExecuteV1Arguments {
        pub execute_v1_args: ExecuteV1Args,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateCollectionInfoV1Arguments {
        pub update_collection_info_v1_args: UpdateCollectionInfoV1Args,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    CreateV1 {
        accounts: CreateV1Accounts,
        args: CreateV1Arguments,
    },
    CreateCollectionV1 {
        accounts: CreateCollectionV1Accounts,
        args: CreateCollectionV1Arguments,
    },
    AddPluginV1 {
        accounts: AddPluginV1Accounts,
        args: AddPluginV1Arguments,
    },
    AddCollectionPluginV1 {
        accounts: AddCollectionPluginV1Accounts,
        args: AddCollectionPluginV1Arguments,
    },
    RemovePluginV1 {
        accounts: RemovePluginV1Accounts,
        args: RemovePluginV1Arguments,
    },
    RemoveCollectionPluginV1 {
        accounts: RemoveCollectionPluginV1Accounts,
        args: RemoveCollectionPluginV1Arguments,
    },
    UpdatePluginV1 {
        accounts: UpdatePluginV1Accounts,
        args: UpdatePluginV1Arguments,
    },
    UpdateCollectionPluginV1 {
        accounts: UpdateCollectionPluginV1Accounts,
        args: UpdateCollectionPluginV1Arguments,
    },
    ApprovePluginAuthorityV1 {
        accounts: ApprovePluginAuthorityV1Accounts,
        args: ApprovePluginAuthorityV1Arguments,
    },
    ApproveCollectionPluginAuthorityV1 {
        accounts: ApproveCollectionPluginAuthorityV1Accounts,
        args: ApproveCollectionPluginAuthorityV1Arguments,
    },
    RevokePluginAuthorityV1 {
        accounts: RevokePluginAuthorityV1Accounts,
        args: RevokePluginAuthorityV1Arguments,
    },
    RevokeCollectionPluginAuthorityV1 {
        accounts: RevokeCollectionPluginAuthorityV1Accounts,
        args: RevokeCollectionPluginAuthorityV1Arguments,
    },
    BurnV1 {
        accounts: BurnV1Accounts,
        args: BurnV1Arguments,
    },
    BurnCollectionV1 {
        accounts: BurnCollectionV1Accounts,
        args: BurnCollectionV1Arguments,
    },
    TransferV1 {
        accounts: TransferV1Accounts,
        args: TransferV1Arguments,
    },
    UpdateV1 {
        accounts: UpdateV1Accounts,
        args: UpdateV1Arguments,
    },
    UpdateCollectionV1 {
        accounts: UpdateCollectionV1Accounts,
        args: UpdateCollectionV1Arguments,
    },
    CompressV1 {
        accounts: CompressV1Accounts,
        args: CompressV1Arguments,
    },
    DecompressV1 {
        accounts: DecompressV1Accounts,
        args: DecompressV1Arguments,
    },
    Collect {
        accounts: CollectAccounts,
        args: CollectArguments,
    },
    CreateV2 {
        accounts: CreateV2Accounts,
        args: CreateV2Arguments,
    },
    CreateCollectionV2 {
        accounts: CreateCollectionV2Accounts,
        args: CreateCollectionV2Arguments,
    },
    AddExternalPluginAdapterV1 {
        accounts: AddExternalPluginAdapterV1Accounts,
        args: AddExternalPluginAdapterV1Arguments,
    },
    AddCollectionExternalPluginAdapterV1 {
        accounts: AddCollectionExternalPluginAdapterV1Accounts,
        args: AddCollectionExternalPluginAdapterV1Arguments,
    },
    RemoveExternalPluginAdapterV1 {
        accounts: RemoveExternalPluginAdapterV1Accounts,
        args: RemoveExternalPluginAdapterV1Arguments,
    },
    RemoveCollectionExternalPluginAdapterV1 {
        accounts: RemoveCollectionExternalPluginAdapterV1Accounts,
        args: RemoveCollectionExternalPluginAdapterV1Arguments,
    },
    UpdateExternalPluginAdapterV1 {
        accounts: UpdateExternalPluginAdapterV1Accounts,
        args: UpdateExternalPluginAdapterV1Arguments,
    },
    UpdateCollectionExternalPluginAdapterV1 {
        accounts: UpdateCollectionExternalPluginAdapterV1Accounts,
        args: UpdateCollectionExternalPluginAdapterV1Arguments,
    },
    WriteExternalPluginAdapterDataV1 {
        accounts: WriteExternalPluginAdapterDataV1Accounts,
        args: WriteExternalPluginAdapterDataV1Arguments,
    },
    WriteCollectionExternalPluginAdapterDataV1 {
        accounts: WriteCollectionExternalPluginAdapterDataV1Accounts,
        args: WriteCollectionExternalPluginAdapterDataV1Arguments,
    },
    UpdateV2 {
        accounts: UpdateV2Accounts,
        args: UpdateV2Arguments,
    },
    ExecuteV1 {
        accounts: ExecuteV1Accounts,
        args: ExecuteV1Arguments,
    },
    UpdateCollectionInfoV1 {
        accounts: UpdateCollectionInfoV1Accounts,
        args: UpdateCollectionInfoV1Arguments,
    },
}
impl Instruction {
    pub fn decode(account_keys: &[String], data: &[u8]) -> anyhow::Result<Self> {
        if data.len() < 1 {
            anyhow::bail!("Data too short: {}", data.len());
        }
        let (disc_slice, rest) = data.split_at(1);
        let disc: [u8; 1] = disc_slice.try_into().unwrap();
        match disc {
            [0u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateV1Accounts {
                    asset,
                    collection,
                    authority,
                    payer,
                    owner,
                    updateAuthority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::CreateV1 { accounts, args });
            }
            [1u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateCollectionV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateCollectionV1Accounts {
                    collection,
                    updateAuthority,
                    payer,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::CreateCollectionV1 { accounts, args });
            }
            [2u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddPluginV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddPluginV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::AddPluginV1 { accounts, args });
            }
            [3u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddCollectionPluginV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddCollectionPluginV1Accounts {
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::AddCollectionPluginV1 { accounts, args });
            }
            [4u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemovePluginV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemovePluginV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::RemovePluginV1 { accounts, args });
            }
            [5u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveCollectionPluginV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveCollectionPluginV1Accounts {
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::RemoveCollectionPluginV1 { accounts, args });
            }
            [6u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePluginV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePluginV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::UpdatePluginV1 { accounts, args });
            }
            [7u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateCollectionPluginV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateCollectionPluginV1Accounts {
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::UpdateCollectionPluginV1 { accounts, args });
            }
            [8u8] => {
                let mut rdr: &[u8] = rest;
                let args = ApprovePluginAuthorityV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ApprovePluginAuthorityV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::ApprovePluginAuthorityV1 { accounts, args });
            }
            [9u8] => {
                let mut rdr: &[u8] = rest;
                let args = ApproveCollectionPluginAuthorityV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ApproveCollectionPluginAuthorityV1Accounts {
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::ApproveCollectionPluginAuthorityV1 { accounts, args });
            }
            [10u8] => {
                let mut rdr: &[u8] = rest;
                let args = RevokePluginAuthorityV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RevokePluginAuthorityV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::RevokePluginAuthorityV1 { accounts, args });
            }
            [11u8] => {
                let mut rdr: &[u8] = rest;
                let args = RevokeCollectionPluginAuthorityV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RevokeCollectionPluginAuthorityV1Accounts {
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::RevokeCollectionPluginAuthorityV1 { accounts, args });
            }
            [12u8] => {
                let mut rdr: &[u8] = rest;
                let args = BurnV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BurnV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::BurnV1 { accounts, args });
            }
            [13u8] => {
                let mut rdr: &[u8] = rest;
                let args = BurnCollectionV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BurnCollectionV1Accounts {
                    collection,
                    payer,
                    authority,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::BurnCollectionV1 { accounts, args });
            }
            [14u8] => {
                let mut rdr: &[u8] = rest;
                let args = TransferV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let newOwner = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    newOwner,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::TransferV1 { accounts, args });
            }
            [15u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::UpdateV1 { accounts, args });
            }
            [16u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateCollectionV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let newUpdateAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateCollectionV1Accounts {
                    collection,
                    payer,
                    authority,
                    newUpdateAuthority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::UpdateCollectionV1 { accounts, args });
            }
            [17u8] => {
                let mut rdr: &[u8] = rest;
                let args = CompressV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CompressV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::CompressV1 { accounts, args });
            }
            [18u8] => {
                let mut rdr: &[u8] = rest;
                let args = DecompressV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DecompressV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::DecompressV1 { accounts, args });
            }
            [19u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let recipient1 = keys.next().unwrap().clone();
                let recipient2 = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectAccounts {
                    recipient1,
                    recipient2,
                    remaining,
                };
                return Ok(Instruction::Collect { accounts, args });
            }
            [20u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateV2Accounts {
                    asset,
                    collection,
                    authority,
                    payer,
                    owner,
                    updateAuthority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::CreateV2 { accounts, args });
            }
            [21u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateCollectionV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateCollectionV2Accounts {
                    collection,
                    updateAuthority,
                    payer,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::CreateCollectionV2 { accounts, args });
            }
            [22u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddExternalPluginAdapterV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddExternalPluginAdapterV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::AddExternalPluginAdapterV1 { accounts, args });
            }
            [23u8] => {
                let mut rdr: &[u8] = rest;
                let args = AddCollectionExternalPluginAdapterV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AddCollectionExternalPluginAdapterV1Accounts {
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::AddCollectionExternalPluginAdapterV1 { accounts, args });
            }
            [24u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveExternalPluginAdapterV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveExternalPluginAdapterV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::RemoveExternalPluginAdapterV1 { accounts, args });
            }
            [25u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveCollectionExternalPluginAdapterV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveCollectionExternalPluginAdapterV1Accounts {
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::RemoveCollectionExternalPluginAdapterV1 { accounts, args });
            }
            [26u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateExternalPluginAdapterV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateExternalPluginAdapterV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::UpdateExternalPluginAdapterV1 { accounts, args });
            }
            [27u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateCollectionExternalPluginAdapterV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateCollectionExternalPluginAdapterV1Accounts {
                    collection,
                    payer,
                    authority,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::UpdateCollectionExternalPluginAdapterV1 { accounts, args });
            }
            [28u8] => {
                let mut rdr: &[u8] = rest;
                let args = WriteExternalPluginAdapterDataV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let buffer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WriteExternalPluginAdapterDataV1Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    buffer,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::WriteExternalPluginAdapterDataV1 { accounts, args });
            }
            [29u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    WriteCollectionExternalPluginAdapterDataV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let buffer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WriteCollectionExternalPluginAdapterDataV1Accounts {
                    collection,
                    payer,
                    authority,
                    buffer,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::WriteCollectionExternalPluginAdapterDataV1 {
                    accounts,
                    args,
                });
            }
            [30u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let newCollection = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let logWrapper = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateV2Accounts {
                    asset,
                    collection,
                    payer,
                    authority,
                    newCollection,
                    systemProgram,
                    logWrapper,
                    remaining,
                };
                return Ok(Instruction::UpdateV2 { accounts, args });
            }
            [31u8] => {
                let mut rdr: &[u8] = rest;
                let args = ExecuteV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let asset = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let assetSigner = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let programId = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ExecuteV1Accounts {
                    asset,
                    collection,
                    assetSigner,
                    payer,
                    authority,
                    systemProgram,
                    programId,
                    remaining,
                };
                return Ok(Instruction::ExecuteV1 { accounts, args });
            }
            [32u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateCollectionInfoV1Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collection = keys.next().unwrap().clone();
                let bubblegumSigner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateCollectionInfoV1Accounts {
                    collection,
                    bubblegumSigner,
                    remaining,
                };
                return Ok(Instruction::UpdateCollectionInfoV1 { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    use borsh::BorshDeserialize;
    use serde::Serialize;
}
