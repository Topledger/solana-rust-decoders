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
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SetCollectionSizeArgs {
        pub size: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreateMasterEditionArgs {
        pub max_supply: Option<u64>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MintPrintingTokensViaTokenArgs {
        pub supply: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct MintNewEditionFromMasterEditionViaTokenArgs {
        pub edition: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct TransferOutOfEscrowArgs {
        pub amount: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct CreateMetadataAccountArgsV3 {
        pub data: DataV2,
        pub is_mutable: bool,
        pub collection_details: Option<CollectionDetails>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UpdateMetadataAccountArgsV2 {
        pub data: Option<DataV2>,
        #[serde(with = "pubkey_serde_option")]
        pub update_authority: Option<[u8; 32usize]>,
        pub primary_sale_happened: Option<bool>,
        pub is_mutable: Option<bool>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ApproveUseAuthorityArgs {
        pub number_of_uses: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct UtilizeArgs {
        pub number_of_uses: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AuthorizationData {
        pub payload: Payload,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct AssetData {
        pub name: String,
        pub symbol: String,
        pub uri: String,
        pub seller_fee_basis_points: u16,
        pub creators: Option<Vec<Creator>>,
        pub primary_sale_happened: bool,
        pub is_mutable: bool,
        pub token_standard: TokenStandard,
        pub collection: Option<Collection>,
        pub uses: Option<Uses>,
        pub collection_details: Option<CollectionDetails>,
        #[serde(with = "pubkey_serde_option")]
        pub rule_set: Option<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Collection {
        pub verified: bool,
        #[serde(with = "pubkey_serde")]
        pub key: [u8; 32usize],
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Creator {
        #[serde(with = "pubkey_serde")]
        pub address: [u8; 32usize],
        pub verified: bool,
        pub share: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Data {
        pub name: String,
        pub symbol: String,
        pub uri: String,
        pub seller_fee_basis_points: u16,
        pub creators: Option<Vec<Creator>>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct DataV2 {
        pub name: String,
        pub symbol: String,
        pub uri: String,
        pub seller_fee_basis_points: u16,
        pub creators: Option<Vec<Creator>>,
        pub collection: Option<Collection>,
        pub uses: Option<Uses>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Reservation {
        #[serde(with = "pubkey_serde")]
        pub address: [u8; 32usize],
        pub spots_remaining: u64,
        pub total_spots: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ReservationV1 {
        #[serde(with = "pubkey_serde")]
        pub address: [u8; 32usize],
        pub spots_remaining: u8,
        pub total_spots: u8,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct SeedsVec {
        pub seeds: Vec<Vec<u8>>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct ProofInfo {
        pub proof: Vec<[u8; 32usize]>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Payload {
        pub map: Vec<PayloadEntry>,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct PayloadEntry {
        pub key: String,
        pub value: PayloadType,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub struct Uses {
        pub use_method: UseMethod,
        pub remaining: u64,
        pub total: u64,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum BurnArgs {
        V1 { amount: u64 },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum DelegateArgs {
        CollectionV1 {
            authorization_data: Option<AuthorizationData>,
        },
        SaleV1 {
            amount: u64,
            authorization_data: Option<AuthorizationData>,
        },
        TransferV1 {
            amount: u64,
            authorization_data: Option<AuthorizationData>,
        },
        DataV1 {
            authorization_data: Option<AuthorizationData>,
        },
        UtilityV1 {
            amount: u64,
            authorization_data: Option<AuthorizationData>,
        },
        StakingV1 {
            amount: u64,
            authorization_data: Option<AuthorizationData>,
        },
        StandardV1 {
            amount: u64,
        },
        LockedTransferV1 {
            amount: u64,
            #[serde(with = "pubkey_serde")]
            locked_address: [u8; 32usize],
            authorization_data: Option<AuthorizationData>,
        },
        ProgrammableConfigV1 {
            authorization_data: Option<AuthorizationData>,
        },
        AuthorityItemV1 {
            authorization_data: Option<AuthorizationData>,
        },
        DataItemV1 {
            authorization_data: Option<AuthorizationData>,
        },
        CollectionItemV1 {
            authorization_data: Option<AuthorizationData>,
        },
        ProgrammableConfigItemV1 {
            authorization_data: Option<AuthorizationData>,
        },
        PrintDelegateV1 {
            authorization_data: Option<AuthorizationData>,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RevokeArgs {
        CollectionV1,
        SaleV1,
        TransferV1,
        DataV1,
        UtilityV1,
        StakingV1,
        StandardV1,
        LockedTransferV1,
        ProgrammableConfigV1,
        MigrationV1,
        AuthorityItemV1,
        DataItemV1,
        CollectionItemV1,
        ProgrammableConfigItemV1,
        PrintDelegateV1,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum MetadataDelegateRole {
        AuthorityItem,
        Collection,
        Use,
        Data,
        ProgrammableConfig,
        DataItem,
        CollectionItem,
        ProgrammableConfigItem,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum HolderDelegateRole {
        PrintDelegate,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum CreateArgs {
        V1 {
            asset_data: AssetData,
            decimals: Option<u8>,
            print_supply: Option<PrintSupply>,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum MintArgs {
        V1 {
            amount: u64,
            authorization_data: Option<AuthorizationData>,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum TransferArgs {
        V1 {
            amount: u64,
            authorization_data: Option<AuthorizationData>,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateArgs {
        V1 {
            #[serde(with = "pubkey_serde_option")]
            new_update_authority: Option<[u8; 32usize]>,
            data: Option<Data>,
            primary_sale_happened: Option<bool>,
            is_mutable: Option<bool>,
            collection: CollectionToggle,
            collection_details: CollectionDetailsToggle,
            uses: UsesToggle,
            rule_set: RuleSetToggle,
            authorization_data: Option<AuthorizationData>,
        },
        AsUpdateAuthorityV2 {
            #[serde(with = "pubkey_serde_option")]
            new_update_authority: Option<[u8; 32usize]>,
            data: Option<Data>,
            primary_sale_happened: Option<bool>,
            is_mutable: Option<bool>,
            collection: CollectionToggle,
            collection_details: CollectionDetailsToggle,
            uses: UsesToggle,
            rule_set: RuleSetToggle,
            token_standard: Option<TokenStandard>,
            authorization_data: Option<AuthorizationData>,
        },
        AsAuthorityItemDelegateV2 {
            #[serde(with = "pubkey_serde_option")]
            new_update_authority: Option<[u8; 32usize]>,
            primary_sale_happened: Option<bool>,
            is_mutable: Option<bool>,
            token_standard: Option<TokenStandard>,
            authorization_data: Option<AuthorizationData>,
        },
        AsCollectionDelegateV2 {
            collection: CollectionToggle,
            authorization_data: Option<AuthorizationData>,
        },
        AsDataDelegateV2 {
            data: Option<Data>,
            authorization_data: Option<AuthorizationData>,
        },
        AsProgrammableConfigDelegateV2 {
            rule_set: RuleSetToggle,
            authorization_data: Option<AuthorizationData>,
        },
        AsDataItemDelegateV2 {
            data: Option<Data>,
            authorization_data: Option<AuthorizationData>,
        },
        AsCollectionItemDelegateV2 {
            collection: CollectionToggle,
            authorization_data: Option<AuthorizationData>,
        },
        AsProgrammableConfigItemDelegateV2 {
            rule_set: RuleSetToggle,
            authorization_data: Option<AuthorizationData>,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum CollectionToggle {
        None,
        Clear,
        Set(Collection),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UsesToggle {
        None,
        Clear,
        Set(Uses),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum CollectionDetailsToggle {
        None,
        Clear,
        Set(CollectionDetails),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum RuleSetToggle {
        None,
        Clear,
        Set([u8; 32usize]),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PrintArgs {
        V1 { edition: u64 },
        V2 { edition: u64 },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum LockArgs {
        V1 {
            authorization_data: Option<AuthorizationData>,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UnlockArgs {
        V1 {
            authorization_data: Option<AuthorizationData>,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UseArgs {
        V1 {
            authorization_data: Option<AuthorizationData>,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum VerificationArgs {
        CreatorV1,
        CollectionV1,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum TokenStandard {
        NonFungible,
        FungibleAsset,
        Fungible,
        NonFungibleEdition,
        ProgrammableNonFungible,
        ProgrammableNonFungibleEdition,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum Key {
        Uninitialized,
        EditionV1,
        MasterEditionV1,
        ReservationListV1,
        MetadataV1,
        ReservationListV2,
        MasterEditionV2,
        EditionMarker,
        UseAuthorityRecord,
        CollectionAuthorityRecord,
        TokenOwnedEscrow,
        TokenRecord,
        MetadataDelegate,
        EditionMarkerV2,
        HolderDelegate,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum CollectionDetails {
        V1 { size: u64 },
        V2 { padding: [u8; 8usize] },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum EscrowAuthority {
        TokenOwner,
        Creator([u8; 32usize]),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PrintSupply {
        Zero,
        Limited(u64),
        Unlimited,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum ProgrammableConfig {
        V1 {
            #[serde(with = "pubkey_serde_option")]
            rule_set: Option<[u8; 32usize]>,
        },
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum MigrationType {
        CollectionV1,
        ProgrammableV1,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum TokenState {
        Unlocked,
        Locked,
        Listed,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum TokenDelegateRole {
        Sale,
        Transfer,
        Utility,
        Staking,
        Standard,
        LockedTransfer,
        Migration,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum AuthorityType {
        None,
        Metadata,
        Holder,
        MetadataDelegate,
        TokenDelegate,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PayloadKey {
        Amount,
        Authority,
        AuthoritySeeds,
        Delegate,
        DelegateSeeds,
        Destination,
        DestinationSeeds,
        Holder,
        Source,
        SourceSeeds,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum PayloadType {
        Pubkey([u8; 32usize]),
        Seeds(SeedsVec),
        MerkleProof(ProofInfo),
        Number(u64),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UseMethod {
        Burn,
        Multiple,
        Single,
    }
}
pub mod accounts_data {
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct CreateMetadataAccountAccounts {
        pub metadata: String,
        pub mint: String,
        pub mintAuthority: String,
        pub payer: String,
        pub updateAuthority: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateMetadataAccountAccounts {
        pub metadata: String,
        pub updateAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeprecatedCreateMasterEditionAccounts {
        pub edition: String,
        pub mint: String,
        pub printingMint: String,
        pub oneTimePrintingAuthorizationMint: String,
        pub updateAuthority: String,
        pub printingMintAuthority: String,
        pub mintAuthority: String,
        pub metadata: String,
        pub payer: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub oneTimePrintingAuthorizationMintAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts {
        pub metadata: String,
        pub edition: String,
        pub masterEdition: String,
        pub mint: String,
        pub mintAuthority: String,
        pub printingMint: String,
        pub masterTokenAccount: String,
        pub editionMarker: String,
        pub burnAuthority: String,
        pub payer: String,
        pub masterUpdateAuthority: String,
        pub masterMetadata: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub reservationList: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdatePrimarySaleHappenedViaTokenAccounts {
        pub metadata: String,
        pub owner: String,
        pub token: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeprecatedSetReservationListAccounts {
        pub masterEdition: String,
        pub reservationList: String,
        pub resource: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeprecatedCreateReservationListAccounts {
        pub reservationList: String,
        pub payer: String,
        pub updateAuthority: String,
        pub masterEdition: String,
        pub resource: String,
        pub metadata: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SignMetadataAccounts {
        pub metadata: String,
        pub creator: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeprecatedMintPrintingTokensViaTokenAccounts {
        pub destination: String,
        pub token: String,
        pub oneTimePrintingAuthorizationMint: String,
        pub printingMint: String,
        pub burnAuthority: String,
        pub metadata: String,
        pub masterEdition: String,
        pub tokenProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DeprecatedMintPrintingTokensAccounts {
        pub destination: String,
        pub printingMint: String,
        pub updateAuthority: String,
        pub metadata: String,
        pub masterEdition: String,
        pub tokenProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateMasterEditionAccounts {
        pub edition: String,
        pub mint: String,
        pub updateAuthority: String,
        pub mintAuthority: String,
        pub payer: String,
        pub metadata: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MintNewEditionFromMasterEditionViaTokenAccounts {
        pub newMetadata: String,
        pub newEdition: String,
        pub masterEdition: String,
        pub newMint: String,
        pub editionMarkPda: String,
        pub newMintAuthority: String,
        pub payer: String,
        pub tokenAccountOwner: String,
        pub tokenAccount: String,
        pub newMetadataUpdateAuthority: String,
        pub metadata: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ConvertMasterEditionV1ToV2Accounts {
        pub masterEdition: String,
        pub oneTimeAuth: String,
        pub printingMint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MintNewEditionFromMasterEditionViaVaultProxyAccounts {
        pub newMetadata: String,
        pub newEdition: String,
        pub masterEdition: String,
        pub newMint: String,
        pub editionMarkPda: String,
        pub newMintAuthority: String,
        pub payer: String,
        pub vaultAuthority: String,
        pub safetyDepositStore: String,
        pub safetyDepositBox: String,
        pub vault: String,
        pub newMetadataUpdateAuthority: String,
        pub metadata: String,
        pub tokenProgram: String,
        pub tokenVaultProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PuffMetadataAccounts {
        pub metadata: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateMetadataAccountV2Accounts {
        pub metadata: String,
        pub updateAuthority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateMetadataAccountV2Accounts {
        pub metadata: String,
        pub mint: String,
        pub mintAuthority: String,
        pub payer: String,
        pub updateAuthority: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateMasterEditionV3Accounts {
        pub edition: String,
        pub mint: String,
        pub updateAuthority: String,
        pub mintAuthority: String,
        pub payer: String,
        pub metadata: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct VerifyCollectionAccounts {
        pub metadata: String,
        pub collectionAuthority: String,
        pub payer: String,
        pub collectionMint: String,
        pub collection: String,
        pub collectionMasterEditionAccount: String,
        pub collectionAuthorityRecord: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UtilizeAccounts {
        pub metadata: String,
        pub tokenAccount: String,
        pub mint: String,
        pub useAuthority: String,
        pub owner: String,
        pub tokenProgram: String,
        pub ataProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub useAuthorityRecord: String,
        pub burner: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ApproveUseAuthorityAccounts {
        pub useAuthorityRecord: String,
        pub owner: String,
        pub payer: String,
        pub user: String,
        pub ownerTokenAccount: String,
        pub metadata: String,
        pub mint: String,
        pub burner: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RevokeUseAuthorityAccounts {
        pub useAuthorityRecord: String,
        pub owner: String,
        pub user: String,
        pub ownerTokenAccount: String,
        pub mint: String,
        pub metadata: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UnverifyCollectionAccounts {
        pub metadata: String,
        pub collectionAuthority: String,
        pub collectionMint: String,
        pub collection: String,
        pub collectionMasterEditionAccount: String,
        pub collectionAuthorityRecord: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ApproveCollectionAuthorityAccounts {
        pub collectionAuthorityRecord: String,
        pub newCollectionAuthority: String,
        pub updateAuthority: String,
        pub payer: String,
        pub metadata: String,
        pub mint: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RevokeCollectionAuthorityAccounts {
        pub collectionAuthorityRecord: String,
        pub delegateAuthority: String,
        pub revokeAuthority: String,
        pub metadata: String,
        pub mint: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetAndVerifyCollectionAccounts {
        pub metadata: String,
        pub collectionAuthority: String,
        pub payer: String,
        pub updateAuthority: String,
        pub collectionMint: String,
        pub collection: String,
        pub collectionMasterEditionAccount: String,
        pub collectionAuthorityRecord: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FreezeDelegatedAccountAccounts {
        pub delegate: String,
        pub tokenAccount: String,
        pub edition: String,
        pub mint: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ThawDelegatedAccountAccounts {
        pub delegate: String,
        pub tokenAccount: String,
        pub edition: String,
        pub mint: String,
        pub tokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RemoveCreatorVerificationAccounts {
        pub metadata: String,
        pub creator: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BurnNftAccounts {
        pub metadata: String,
        pub owner: String,
        pub mint: String,
        pub tokenAccount: String,
        pub masterEditionAccount: String,
        pub splTokenProgram: String,
        pub collectionMetadata: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct VerifySizedCollectionItemAccounts {
        pub metadata: String,
        pub collectionAuthority: String,
        pub payer: String,
        pub collectionMint: String,
        pub collection: String,
        pub collectionMasterEditionAccount: String,
        pub collectionAuthorityRecord: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UnverifySizedCollectionItemAccounts {
        pub metadata: String,
        pub collectionAuthority: String,
        pub payer: String,
        pub collectionMint: String,
        pub collection: String,
        pub collectionMasterEditionAccount: String,
        pub collectionAuthorityRecord: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetAndVerifySizedCollectionItemAccounts {
        pub metadata: String,
        pub collectionAuthority: String,
        pub payer: String,
        pub updateAuthority: String,
        pub collectionMint: String,
        pub collection: String,
        pub collectionMasterEditionAccount: String,
        pub collectionAuthorityRecord: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateMetadataAccountV3Accounts {
        pub metadata: String,
        pub mint: String,
        pub mintAuthority: String,
        pub payer: String,
        pub updateAuthority: String,
        pub systemProgram: String,
        pub rent: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetCollectionSizeAccounts {
        pub collectionMetadata: String,
        pub collectionAuthority: String,
        pub collectionMint: String,
        pub collectionAuthorityRecord: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SetTokenStandardAccounts {
        pub metadata: String,
        pub updateAuthority: String,
        pub mint: String,
        pub edition: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BubblegumSetCollectionSizeAccounts {
        pub collectionMetadata: String,
        pub collectionAuthority: String,
        pub collectionMint: String,
        pub bubblegumSigner: String,
        pub collectionAuthorityRecord: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BurnEditionNftAccounts {
        pub metadata: String,
        pub owner: String,
        pub printEditionMint: String,
        pub masterEditionMint: String,
        pub printEditionTokenAccount: String,
        pub masterEditionTokenAccount: String,
        pub masterEditionAccount: String,
        pub printEditionAccount: String,
        pub editionMarkerAccount: String,
        pub splTokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateEscrowAccountAccounts {
        pub escrow: String,
        pub metadata: String,
        pub mint: String,
        pub tokenAccount: String,
        pub edition: String,
        pub payer: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseEscrowAccountAccounts {
        pub escrow: String,
        pub metadata: String,
        pub mint: String,
        pub tokenAccount: String,
        pub edition: String,
        pub payer: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferOutOfEscrowAccounts {
        pub escrow: String,
        pub metadata: String,
        pub payer: String,
        pub attributeMint: String,
        pub attributeSrc: String,
        pub attributeDst: String,
        pub escrowMint: String,
        pub escrowAccount: String,
        pub systemProgram: String,
        pub ataProgram: String,
        pub tokenProgram: String,
        pub sysvarInstructions: String,
        pub authority: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct BurnAccounts {
        pub authority: String,
        pub collectionMetadata: String,
        pub metadata: String,
        pub edition: String,
        pub mint: String,
        pub token: String,
        pub masterEdition: String,
        pub masterEditionMint: String,
        pub masterEditionToken: String,
        pub editionMarker: String,
        pub tokenRecord: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub splTokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateAccounts {
        pub metadata: String,
        pub masterEdition: String,
        pub mint: String,
        pub authority: String,
        pub payer: String,
        pub updateAuthority: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub splTokenProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MintAccounts {
        pub token: String,
        pub tokenOwner: String,
        pub metadata: String,
        pub masterEdition: String,
        pub tokenRecord: String,
        pub mint: String,
        pub authority: String,
        pub delegateRecord: String,
        pub payer: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub splTokenProgram: String,
        pub splAtaProgram: String,
        pub authorizationRulesProgram: String,
        pub authorizationRules: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct DelegateAccounts {
        pub delegateRecord: String,
        pub delegate: String,
        pub metadata: String,
        pub masterEdition: String,
        pub tokenRecord: String,
        pub mint: String,
        pub token: String,
        pub authority: String,
        pub payer: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub splTokenProgram: String,
        pub authorizationRulesProgram: String,
        pub authorizationRules: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct RevokeAccounts {
        pub delegateRecord: String,
        pub delegate: String,
        pub metadata: String,
        pub masterEdition: String,
        pub tokenRecord: String,
        pub mint: String,
        pub token: String,
        pub authority: String,
        pub payer: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub splTokenProgram: String,
        pub authorizationRulesProgram: String,
        pub authorizationRules: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LockAccounts {
        pub authority: String,
        pub tokenOwner: String,
        pub token: String,
        pub mint: String,
        pub metadata: String,
        pub edition: String,
        pub tokenRecord: String,
        pub payer: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub splTokenProgram: String,
        pub authorizationRulesProgram: String,
        pub authorizationRules: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UnlockAccounts {
        pub authority: String,
        pub tokenOwner: String,
        pub token: String,
        pub mint: String,
        pub metadata: String,
        pub edition: String,
        pub tokenRecord: String,
        pub payer: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub splTokenProgram: String,
        pub authorizationRulesProgram: String,
        pub authorizationRules: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct MigrateAccounts {
        pub metadata: String,
        pub edition: String,
        pub token: String,
        pub tokenOwner: String,
        pub mint: String,
        pub payer: String,
        pub authority: String,
        pub collectionMetadata: String,
        pub delegateRecord: String,
        pub tokenRecord: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub splTokenProgram: String,
        pub authorizationRulesProgram: String,
        pub authorizationRules: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TransferAccounts {
        pub token: String,
        pub tokenOwner: String,
        pub destination: String,
        pub destinationOwner: String,
        pub mint: String,
        pub metadata: String,
        pub edition: String,
        pub ownerTokenRecord: String,
        pub destinationTokenRecord: String,
        pub authority: String,
        pub payer: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub splTokenProgram: String,
        pub splAtaProgram: String,
        pub authorizationRulesProgram: String,
        pub authorizationRules: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateAccounts {
        pub authority: String,
        pub delegateRecord: String,
        pub token: String,
        pub mint: String,
        pub metadata: String,
        pub edition: String,
        pub payer: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub authorizationRulesProgram: String,
        pub authorizationRules: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UseAccounts {
        pub authority: String,
        pub delegateRecord: String,
        pub token: String,
        pub mint: String,
        pub metadata: String,
        pub edition: String,
        pub payer: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub splTokenProgram: String,
        pub authorizationRulesProgram: String,
        pub authorizationRules: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct VerifyAccounts {
        pub authority: String,
        pub delegateRecord: String,
        pub metadata: String,
        pub collectionMint: String,
        pub collectionMetadata: String,
        pub collectionMasterEdition: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UnverifyAccounts {
        pub authority: String,
        pub delegateRecord: String,
        pub metadata: String,
        pub collectionMint: String,
        pub collectionMetadata: String,
        pub systemProgram: String,
        pub sysvarInstructions: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CollectAccounts {
        pub authority: String,
        pub recipient: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct PrintAccounts {
        pub editionMetadata: String,
        pub edition: String,
        pub editionMint: String,
        pub editionTokenAccountOwner: String,
        pub editionTokenAccount: String,
        pub editionMintAuthority: String,
        pub editionTokenRecord: String,
        pub masterEdition: String,
        pub editionMarkerPda: String,
        pub payer: String,
        pub masterTokenAccountOwner: String,
        pub masterTokenAccount: String,
        pub masterMetadata: String,
        pub updateAuthority: String,
        pub splTokenProgram: String,
        pub splAtaProgram: String,
        pub sysvarInstructions: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ResizeAccounts {
        pub metadata: String,
        pub edition: String,
        pub mint: String,
        pub payer: String,
        pub authority: String,
        pub token: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseAccountsAccounts {
        pub metadata: String,
        pub edition: String,
        pub mint: String,
        pub authority: String,
        pub destination: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateMetadataAccountArguments {
        pub data: Data,
        pub is_mutable: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateMetadataAccountArguments {
        pub data: Option<Data>,
        #[serde(with = "pubkey_serde_option")]
        pub update_authority: Option<[u8; 32usize]>,
        pub primary_sale_happened: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeprecatedCreateMasterEditionArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdatePrimarySaleHappenedViaTokenArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeprecatedSetReservationListArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeprecatedCreateReservationListArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SignMetadataArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeprecatedMintPrintingTokensViaTokenArguments {
        pub mint_printing_tokens_via_token_args: MintPrintingTokensViaTokenArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DeprecatedMintPrintingTokensArguments {
        pub mint_printing_tokens_via_token_args: MintPrintingTokensViaTokenArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateMasterEditionArguments {
        pub create_master_edition_args: CreateMasterEditionArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MintNewEditionFromMasterEditionViaTokenArguments {
        pub mint_new_edition_from_master_edition_via_token_args:
            MintNewEditionFromMasterEditionViaTokenArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ConvertMasterEditionV1ToV2Arguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MintNewEditionFromMasterEditionViaVaultProxyArguments {
        pub mint_new_edition_from_master_edition_via_token_args:
            MintNewEditionFromMasterEditionViaTokenArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PuffMetadataArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateMetadataAccountV2Arguments {
        pub update_metadata_account_args_v2: UpdateMetadataAccountArgsV2,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateMetadataAccountV2Arguments {
        pub data_v2: DataV2,
        pub is_mutable: bool,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateMasterEditionV3Arguments {
        pub create_master_edition_args: CreateMasterEditionArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct VerifyCollectionArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UtilizeArguments {
        pub utilize_args: UtilizeArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ApproveUseAuthorityArguments {
        pub approve_use_authority_args: ApproveUseAuthorityArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RevokeUseAuthorityArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnverifyCollectionArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ApproveCollectionAuthorityArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RevokeCollectionAuthorityArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetAndVerifyCollectionArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FreezeDelegatedAccountArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ThawDelegatedAccountArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RemoveCreatorVerificationArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BurnNftArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct VerifySizedCollectionItemArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnverifySizedCollectionItemArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetAndVerifySizedCollectionItemArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateMetadataAccountV3Arguments {
        pub create_metadata_account_args_v3: CreateMetadataAccountArgsV3,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetCollectionSizeArguments {
        pub set_collection_size_args: SetCollectionSizeArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SetTokenStandardArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BubblegumSetCollectionSizeArguments {
        pub set_collection_size_args: SetCollectionSizeArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BurnEditionNftArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateEscrowAccountArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseEscrowAccountArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferOutOfEscrowArguments {
        pub transfer_out_of_escrow_args: TransferOutOfEscrowArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct BurnArguments {
        pub burn_args: BurnArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateArguments {
        pub create_args: CreateArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MintArguments {
        pub mint_args: MintArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct DelegateArguments {
        pub delegate_args: DelegateArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct RevokeArguments {
        pub revoke_args: RevokeArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LockArguments {
        pub lock_args: LockArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnlockArguments {
        pub unlock_args: UnlockArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct MigrateArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TransferArguments {
        pub transfer_args: TransferArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateArguments {
        pub update_args: UpdateArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UseArguments {
        pub use_args: UseArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct VerifyArguments {
        pub verification_args: VerificationArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UnverifyArguments {
        pub verification_args: VerificationArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CollectArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct PrintArguments {
        pub print_args: PrintArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ResizeArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseAccountsArguments {}
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    CreateMetadataAccount {
        accounts: CreateMetadataAccountAccounts,
        args: CreateMetadataAccountArguments,
    },
    UpdateMetadataAccount {
        accounts: UpdateMetadataAccountAccounts,
        args: UpdateMetadataAccountArguments,
    },
    DeprecatedCreateMasterEdition {
        accounts: DeprecatedCreateMasterEditionAccounts,
        args: DeprecatedCreateMasterEditionArguments,
    },
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken {
        accounts: DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts,
        args: DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenArguments,
    },
    UpdatePrimarySaleHappenedViaToken {
        accounts: UpdatePrimarySaleHappenedViaTokenAccounts,
        args: UpdatePrimarySaleHappenedViaTokenArguments,
    },
    DeprecatedSetReservationList {
        accounts: DeprecatedSetReservationListAccounts,
        args: DeprecatedSetReservationListArguments,
    },
    DeprecatedCreateReservationList {
        accounts: DeprecatedCreateReservationListAccounts,
        args: DeprecatedCreateReservationListArguments,
    },
    SignMetadata {
        accounts: SignMetadataAccounts,
        args: SignMetadataArguments,
    },
    DeprecatedMintPrintingTokensViaToken {
        accounts: DeprecatedMintPrintingTokensViaTokenAccounts,
        args: DeprecatedMintPrintingTokensViaTokenArguments,
    },
    DeprecatedMintPrintingTokens {
        accounts: DeprecatedMintPrintingTokensAccounts,
        args: DeprecatedMintPrintingTokensArguments,
    },
    CreateMasterEdition {
        accounts: CreateMasterEditionAccounts,
        args: CreateMasterEditionArguments,
    },
    MintNewEditionFromMasterEditionViaToken {
        accounts: MintNewEditionFromMasterEditionViaTokenAccounts,
        args: MintNewEditionFromMasterEditionViaTokenArguments,
    },
    ConvertMasterEditionV1ToV2 {
        accounts: ConvertMasterEditionV1ToV2Accounts,
        args: ConvertMasterEditionV1ToV2Arguments,
    },
    MintNewEditionFromMasterEditionViaVaultProxy {
        accounts: MintNewEditionFromMasterEditionViaVaultProxyAccounts,
        args: MintNewEditionFromMasterEditionViaVaultProxyArguments,
    },
    PuffMetadata {
        accounts: PuffMetadataAccounts,
        args: PuffMetadataArguments,
    },
    UpdateMetadataAccountV2 {
        accounts: UpdateMetadataAccountV2Accounts,
        args: UpdateMetadataAccountV2Arguments,
    },
    CreateMetadataAccountV2 {
        accounts: CreateMetadataAccountV2Accounts,
        args: CreateMetadataAccountV2Arguments,
    },
    CreateMasterEditionV3 {
        accounts: CreateMasterEditionV3Accounts,
        args: CreateMasterEditionV3Arguments,
    },
    VerifyCollection {
        accounts: VerifyCollectionAccounts,
        args: VerifyCollectionArguments,
    },
    Utilize {
        accounts: UtilizeAccounts,
        args: UtilizeArguments,
    },
    ApproveUseAuthority {
        accounts: ApproveUseAuthorityAccounts,
        args: ApproveUseAuthorityArguments,
    },
    RevokeUseAuthority {
        accounts: RevokeUseAuthorityAccounts,
        args: RevokeUseAuthorityArguments,
    },
    UnverifyCollection {
        accounts: UnverifyCollectionAccounts,
        args: UnverifyCollectionArguments,
    },
    ApproveCollectionAuthority {
        accounts: ApproveCollectionAuthorityAccounts,
        args: ApproveCollectionAuthorityArguments,
    },
    RevokeCollectionAuthority {
        accounts: RevokeCollectionAuthorityAccounts,
        args: RevokeCollectionAuthorityArguments,
    },
    SetAndVerifyCollection {
        accounts: SetAndVerifyCollectionAccounts,
        args: SetAndVerifyCollectionArguments,
    },
    FreezeDelegatedAccount {
        accounts: FreezeDelegatedAccountAccounts,
        args: FreezeDelegatedAccountArguments,
    },
    ThawDelegatedAccount {
        accounts: ThawDelegatedAccountAccounts,
        args: ThawDelegatedAccountArguments,
    },
    RemoveCreatorVerification {
        accounts: RemoveCreatorVerificationAccounts,
        args: RemoveCreatorVerificationArguments,
    },
    BurnNft {
        accounts: BurnNftAccounts,
        args: BurnNftArguments,
    },
    VerifySizedCollectionItem {
        accounts: VerifySizedCollectionItemAccounts,
        args: VerifySizedCollectionItemArguments,
    },
    UnverifySizedCollectionItem {
        accounts: UnverifySizedCollectionItemAccounts,
        args: UnverifySizedCollectionItemArguments,
    },
    SetAndVerifySizedCollectionItem {
        accounts: SetAndVerifySizedCollectionItemAccounts,
        args: SetAndVerifySizedCollectionItemArguments,
    },
    CreateMetadataAccountV3 {
        accounts: CreateMetadataAccountV3Accounts,
        args: CreateMetadataAccountV3Arguments,
    },
    SetCollectionSize {
        accounts: SetCollectionSizeAccounts,
        args: SetCollectionSizeArguments,
    },
    SetTokenStandard {
        accounts: SetTokenStandardAccounts,
        args: SetTokenStandardArguments,
    },
    BubblegumSetCollectionSize {
        accounts: BubblegumSetCollectionSizeAccounts,
        args: BubblegumSetCollectionSizeArguments,
    },
    BurnEditionNft {
        accounts: BurnEditionNftAccounts,
        args: BurnEditionNftArguments,
    },
    CreateEscrowAccount {
        accounts: CreateEscrowAccountAccounts,
        args: CreateEscrowAccountArguments,
    },
    CloseEscrowAccount {
        accounts: CloseEscrowAccountAccounts,
        args: CloseEscrowAccountArguments,
    },
    TransferOutOfEscrow {
        accounts: TransferOutOfEscrowAccounts,
        args: TransferOutOfEscrowArguments,
    },
    Burn {
        accounts: BurnAccounts,
        args: BurnArguments,
    },
    Create {
        accounts: CreateAccounts,
        args: CreateArguments,
    },
    Mint {
        accounts: MintAccounts,
        args: MintArguments,
    },
    Delegate {
        accounts: DelegateAccounts,
        args: DelegateArguments,
    },
    Revoke {
        accounts: RevokeAccounts,
        args: RevokeArguments,
    },
    Lock {
        accounts: LockAccounts,
        args: LockArguments,
    },
    Unlock {
        accounts: UnlockAccounts,
        args: UnlockArguments,
    },
    Migrate {
        accounts: MigrateAccounts,
        args: MigrateArguments,
    },
    Transfer {
        accounts: TransferAccounts,
        args: TransferArguments,
    },
    Update {
        accounts: UpdateAccounts,
        args: UpdateArguments,
    },
    Use {
        accounts: UseAccounts,
        args: UseArguments,
    },
    Verify {
        accounts: VerifyAccounts,
        args: VerifyArguments,
    },
    Unverify {
        accounts: UnverifyAccounts,
        args: UnverifyArguments,
    },
    Collect {
        accounts: CollectAccounts,
        args: CollectArguments,
    },
    Print {
        accounts: PrintAccounts,
        args: PrintArguments,
    },
    Resize {
        accounts: ResizeAccounts,
        args: ResizeArguments,
    },
    CloseAccounts {
        accounts: CloseAccountsAccounts,
        args: CloseAccountsArguments,
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
                let args = CreateMetadataAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let mintAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateMetadataAccountAccounts {
                    metadata,
                    mint,
                    mintAuthority,
                    payer,
                    updateAuthority,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::CreateMetadataAccount { accounts, args });
            }
            [1u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateMetadataAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateMetadataAccountAccounts {
                    metadata,
                    updateAuthority,
                    remaining,
                };
                return Ok(Instruction::UpdateMetadataAccount { accounts, args });
            }
            [2u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeprecatedCreateMasterEditionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let edition = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let printingMint = keys.next().unwrap().clone();
                let oneTimePrintingAuthorizationMint = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let printingMintAuthority = keys.next().unwrap().clone();
                let mintAuthority = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let oneTimePrintingAuthorizationMintAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeprecatedCreateMasterEditionAccounts {
                    edition,
                    mint,
                    printingMint,
                    oneTimePrintingAuthorizationMint,
                    updateAuthority,
                    printingMintAuthority,
                    mintAuthority,
                    metadata,
                    payer,
                    tokenProgram,
                    systemProgram,
                    rent,
                    oneTimePrintingAuthorizationMintAuthority,
                    remaining,
                };
                return Ok(Instruction::DeprecatedCreateMasterEdition { accounts, args });
            }
            [3u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenArguments :: deserialize (& mut rdr) ? ;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let mintAuthority = keys.next().unwrap().clone();
                let printingMint = keys.next().unwrap().clone();
                let masterTokenAccount = keys.next().unwrap().clone();
                let editionMarker = keys.next().unwrap().clone();
                let burnAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let masterUpdateAuthority = keys.next().unwrap().clone();
                let masterMetadata = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let reservationList = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts {
                    metadata,
                    edition,
                    masterEdition,
                    mint,
                    mintAuthority,
                    printingMint,
                    masterTokenAccount,
                    editionMarker,
                    burnAuthority,
                    payer,
                    masterUpdateAuthority,
                    masterMetadata,
                    tokenProgram,
                    systemProgram,
                    rent,
                    reservationList,
                    remaining,
                };
                return Ok(
                    Instruction::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken {
                        accounts,
                        args,
                    },
                );
            }
            [4u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdatePrimarySaleHappenedViaTokenArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdatePrimarySaleHappenedViaTokenAccounts {
                    metadata,
                    owner,
                    token,
                    remaining,
                };
                return Ok(Instruction::UpdatePrimarySaleHappenedViaToken { accounts, args });
            }
            [5u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeprecatedSetReservationListArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let masterEdition = keys.next().unwrap().clone();
                let reservationList = keys.next().unwrap().clone();
                let resource = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeprecatedSetReservationListAccounts {
                    masterEdition,
                    reservationList,
                    resource,
                    remaining,
                };
                return Ok(Instruction::DeprecatedSetReservationList { accounts, args });
            }
            [6u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeprecatedCreateReservationListArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let reservationList = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let resource = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeprecatedCreateReservationListAccounts {
                    reservationList,
                    payer,
                    updateAuthority,
                    masterEdition,
                    resource,
                    metadata,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::DeprecatedCreateReservationList { accounts, args });
            }
            [7u8] => {
                let mut rdr: &[u8] = rest;
                let args = SignMetadataArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let creator = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SignMetadataAccounts {
                    metadata,
                    creator,
                    remaining,
                };
                return Ok(Instruction::SignMetadata { accounts, args });
            }
            [8u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeprecatedMintPrintingTokensViaTokenArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let destination = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let oneTimePrintingAuthorizationMint = keys.next().unwrap().clone();
                let printingMint = keys.next().unwrap().clone();
                let burnAuthority = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeprecatedMintPrintingTokensViaTokenAccounts {
                    destination,
                    token,
                    oneTimePrintingAuthorizationMint,
                    printingMint,
                    burnAuthority,
                    metadata,
                    masterEdition,
                    tokenProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::DeprecatedMintPrintingTokensViaToken { accounts, args });
            }
            [9u8] => {
                let mut rdr: &[u8] = rest;
                let args = DeprecatedMintPrintingTokensArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let destination = keys.next().unwrap().clone();
                let printingMint = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DeprecatedMintPrintingTokensAccounts {
                    destination,
                    printingMint,
                    updateAuthority,
                    metadata,
                    masterEdition,
                    tokenProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::DeprecatedMintPrintingTokens { accounts, args });
            }
            [10u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateMasterEditionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let edition = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let mintAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateMasterEditionAccounts {
                    edition,
                    mint,
                    updateAuthority,
                    mintAuthority,
                    payer,
                    metadata,
                    tokenProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::CreateMasterEdition { accounts, args });
            }
            [11u8] => {
                let mut rdr: &[u8] = rest;
                let args = MintNewEditionFromMasterEditionViaTokenArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let newMetadata = keys.next().unwrap().clone();
                let newEdition = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let newMint = keys.next().unwrap().clone();
                let editionMarkPda = keys.next().unwrap().clone();
                let newMintAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let tokenAccountOwner = keys.next().unwrap().clone();
                let tokenAccount = keys.next().unwrap().clone();
                let newMetadataUpdateAuthority = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MintNewEditionFromMasterEditionViaTokenAccounts {
                    newMetadata,
                    newEdition,
                    masterEdition,
                    newMint,
                    editionMarkPda,
                    newMintAuthority,
                    payer,
                    tokenAccountOwner,
                    tokenAccount,
                    newMetadataUpdateAuthority,
                    metadata,
                    tokenProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::MintNewEditionFromMasterEditionViaToken { accounts, args });
            }
            [12u8] => {
                let mut rdr: &[u8] = rest;
                let args = ConvertMasterEditionV1ToV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let masterEdition = keys.next().unwrap().clone();
                let oneTimeAuth = keys.next().unwrap().clone();
                let printingMint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ConvertMasterEditionV1ToV2Accounts {
                    masterEdition,
                    oneTimeAuth,
                    printingMint,
                    remaining,
                };
                return Ok(Instruction::ConvertMasterEditionV1ToV2 { accounts, args });
            }
            [13u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    MintNewEditionFromMasterEditionViaVaultProxyArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let newMetadata = keys.next().unwrap().clone();
                let newEdition = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let newMint = keys.next().unwrap().clone();
                let editionMarkPda = keys.next().unwrap().clone();
                let newMintAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let vaultAuthority = keys.next().unwrap().clone();
                let safetyDepositStore = keys.next().unwrap().clone();
                let safetyDepositBox = keys.next().unwrap().clone();
                let vault = keys.next().unwrap().clone();
                let newMetadataUpdateAuthority = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let tokenVaultProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MintNewEditionFromMasterEditionViaVaultProxyAccounts {
                    newMetadata,
                    newEdition,
                    masterEdition,
                    newMint,
                    editionMarkPda,
                    newMintAuthority,
                    payer,
                    vaultAuthority,
                    safetyDepositStore,
                    safetyDepositBox,
                    vault,
                    newMetadataUpdateAuthority,
                    metadata,
                    tokenProgram,
                    tokenVaultProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::MintNewEditionFromMasterEditionViaVaultProxy {
                    accounts,
                    args,
                });
            }
            [14u8] => {
                let mut rdr: &[u8] = rest;
                let args = PuffMetadataArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PuffMetadataAccounts {
                    metadata,
                    remaining,
                };
                return Ok(Instruction::PuffMetadata { accounts, args });
            }
            [15u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateMetadataAccountV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateMetadataAccountV2Accounts {
                    metadata,
                    updateAuthority,
                    remaining,
                };
                return Ok(Instruction::UpdateMetadataAccountV2 { accounts, args });
            }
            [16u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateMetadataAccountV2Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let mintAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateMetadataAccountV2Accounts {
                    metadata,
                    mint,
                    mintAuthority,
                    payer,
                    updateAuthority,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::CreateMetadataAccountV2 { accounts, args });
            }
            [17u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateMasterEditionV3Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let edition = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let mintAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateMasterEditionV3Accounts {
                    edition,
                    mint,
                    updateAuthority,
                    mintAuthority,
                    payer,
                    metadata,
                    tokenProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::CreateMasterEditionV3 { accounts, args });
            }
            [18u8] => {
                let mut rdr: &[u8] = rest;
                let args = VerifyCollectionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let collectionAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let collectionMint = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let collectionMasterEditionAccount = keys.next().unwrap().clone();
                let collectionAuthorityRecord = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = VerifyCollectionAccounts {
                    metadata,
                    collectionAuthority,
                    payer,
                    collectionMint,
                    collection,
                    collectionMasterEditionAccount,
                    collectionAuthorityRecord,
                    remaining,
                };
                return Ok(Instruction::VerifyCollection { accounts, args });
            }
            [19u8] => {
                let mut rdr: &[u8] = rest;
                let args = UtilizeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let tokenAccount = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let useAuthority = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let ataProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let useAuthorityRecord = keys.next().unwrap().clone();
                let burner = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UtilizeAccounts {
                    metadata,
                    tokenAccount,
                    mint,
                    useAuthority,
                    owner,
                    tokenProgram,
                    ataProgram,
                    systemProgram,
                    rent,
                    useAuthorityRecord,
                    burner,
                    remaining,
                };
                return Ok(Instruction::Utilize { accounts, args });
            }
            [20u8] => {
                let mut rdr: &[u8] = rest;
                let args = ApproveUseAuthorityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let useAuthorityRecord = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let ownerTokenAccount = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let burner = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ApproveUseAuthorityAccounts {
                    useAuthorityRecord,
                    owner,
                    payer,
                    user,
                    ownerTokenAccount,
                    metadata,
                    mint,
                    burner,
                    tokenProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::ApproveUseAuthority { accounts, args });
            }
            [21u8] => {
                let mut rdr: &[u8] = rest;
                let args = RevokeUseAuthorityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let useAuthorityRecord = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let user = keys.next().unwrap().clone();
                let ownerTokenAccount = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RevokeUseAuthorityAccounts {
                    useAuthorityRecord,
                    owner,
                    user,
                    ownerTokenAccount,
                    mint,
                    metadata,
                    tokenProgram,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::RevokeUseAuthority { accounts, args });
            }
            [22u8] => {
                let mut rdr: &[u8] = rest;
                let args = UnverifyCollectionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let collectionAuthority = keys.next().unwrap().clone();
                let collectionMint = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let collectionMasterEditionAccount = keys.next().unwrap().clone();
                let collectionAuthorityRecord = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UnverifyCollectionAccounts {
                    metadata,
                    collectionAuthority,
                    collectionMint,
                    collection,
                    collectionMasterEditionAccount,
                    collectionAuthorityRecord,
                    remaining,
                };
                return Ok(Instruction::UnverifyCollection { accounts, args });
            }
            [23u8] => {
                let mut rdr: &[u8] = rest;
                let args = ApproveCollectionAuthorityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collectionAuthorityRecord = keys.next().unwrap().clone();
                let newCollectionAuthority = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ApproveCollectionAuthorityAccounts {
                    collectionAuthorityRecord,
                    newCollectionAuthority,
                    updateAuthority,
                    payer,
                    metadata,
                    mint,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::ApproveCollectionAuthority { accounts, args });
            }
            [24u8] => {
                let mut rdr: &[u8] = rest;
                let args = RevokeCollectionAuthorityArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collectionAuthorityRecord = keys.next().unwrap().clone();
                let delegateAuthority = keys.next().unwrap().clone();
                let revokeAuthority = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RevokeCollectionAuthorityAccounts {
                    collectionAuthorityRecord,
                    delegateAuthority,
                    revokeAuthority,
                    metadata,
                    mint,
                    remaining,
                };
                return Ok(Instruction::RevokeCollectionAuthority { accounts, args });
            }
            [25u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetAndVerifyCollectionArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let collectionAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let collectionMint = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let collectionMasterEditionAccount = keys.next().unwrap().clone();
                let collectionAuthorityRecord = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetAndVerifyCollectionAccounts {
                    metadata,
                    collectionAuthority,
                    payer,
                    updateAuthority,
                    collectionMint,
                    collection,
                    collectionMasterEditionAccount,
                    collectionAuthorityRecord,
                    remaining,
                };
                return Ok(Instruction::SetAndVerifyCollection { accounts, args });
            }
            [26u8] => {
                let mut rdr: &[u8] = rest;
                let args = FreezeDelegatedAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let delegate = keys.next().unwrap().clone();
                let tokenAccount = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = FreezeDelegatedAccountAccounts {
                    delegate,
                    tokenAccount,
                    edition,
                    mint,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::FreezeDelegatedAccount { accounts, args });
            }
            [27u8] => {
                let mut rdr: &[u8] = rest;
                let args = ThawDelegatedAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let delegate = keys.next().unwrap().clone();
                let tokenAccount = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ThawDelegatedAccountAccounts {
                    delegate,
                    tokenAccount,
                    edition,
                    mint,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::ThawDelegatedAccount { accounts, args });
            }
            [28u8] => {
                let mut rdr: &[u8] = rest;
                let args = RemoveCreatorVerificationArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let creator = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RemoveCreatorVerificationAccounts {
                    metadata,
                    creator,
                    remaining,
                };
                return Ok(Instruction::RemoveCreatorVerification { accounts, args });
            }
            [29u8] => {
                let mut rdr: &[u8] = rest;
                let args = BurnNftArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let tokenAccount = keys.next().unwrap().clone();
                let masterEditionAccount = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let collectionMetadata = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BurnNftAccounts {
                    metadata,
                    owner,
                    mint,
                    tokenAccount,
                    masterEditionAccount,
                    splTokenProgram,
                    collectionMetadata,
                    remaining,
                };
                return Ok(Instruction::BurnNft { accounts, args });
            }
            [30u8] => {
                let mut rdr: &[u8] = rest;
                let args = VerifySizedCollectionItemArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let collectionAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let collectionMint = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let collectionMasterEditionAccount = keys.next().unwrap().clone();
                let collectionAuthorityRecord = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = VerifySizedCollectionItemAccounts {
                    metadata,
                    collectionAuthority,
                    payer,
                    collectionMint,
                    collection,
                    collectionMasterEditionAccount,
                    collectionAuthorityRecord,
                    remaining,
                };
                return Ok(Instruction::VerifySizedCollectionItem { accounts, args });
            }
            [31u8] => {
                let mut rdr: &[u8] = rest;
                let args = UnverifySizedCollectionItemArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let collectionAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let collectionMint = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let collectionMasterEditionAccount = keys.next().unwrap().clone();
                let collectionAuthorityRecord = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UnverifySizedCollectionItemAccounts {
                    metadata,
                    collectionAuthority,
                    payer,
                    collectionMint,
                    collection,
                    collectionMasterEditionAccount,
                    collectionAuthorityRecord,
                    remaining,
                };
                return Ok(Instruction::UnverifySizedCollectionItem { accounts, args });
            }
            [32u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetAndVerifySizedCollectionItemArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let collectionAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let collectionMint = keys.next().unwrap().clone();
                let collection = keys.next().unwrap().clone();
                let collectionMasterEditionAccount = keys.next().unwrap().clone();
                let collectionAuthorityRecord = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetAndVerifySizedCollectionItemAccounts {
                    metadata,
                    collectionAuthority,
                    payer,
                    updateAuthority,
                    collectionMint,
                    collection,
                    collectionMasterEditionAccount,
                    collectionAuthorityRecord,
                    remaining,
                };
                return Ok(Instruction::SetAndVerifySizedCollectionItem { accounts, args });
            }
            [33u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateMetadataAccountV3Arguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let mintAuthority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateMetadataAccountV3Accounts {
                    metadata,
                    mint,
                    mintAuthority,
                    payer,
                    updateAuthority,
                    systemProgram,
                    rent,
                    remaining,
                };
                return Ok(Instruction::CreateMetadataAccountV3 { accounts, args });
            }
            [34u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetCollectionSizeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collectionMetadata = keys.next().unwrap().clone();
                let collectionAuthority = keys.next().unwrap().clone();
                let collectionMint = keys.next().unwrap().clone();
                let collectionAuthorityRecord = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetCollectionSizeAccounts {
                    collectionMetadata,
                    collectionAuthority,
                    collectionMint,
                    collectionAuthorityRecord,
                    remaining,
                };
                return Ok(Instruction::SetCollectionSize { accounts, args });
            }
            [35u8] => {
                let mut rdr: &[u8] = rest;
                let args = SetTokenStandardArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = SetTokenStandardAccounts {
                    metadata,
                    updateAuthority,
                    mint,
                    edition,
                    remaining,
                };
                return Ok(Instruction::SetTokenStandard { accounts, args });
            }
            [36u8] => {
                let mut rdr: &[u8] = rest;
                let args = BubblegumSetCollectionSizeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let collectionMetadata = keys.next().unwrap().clone();
                let collectionAuthority = keys.next().unwrap().clone();
                let collectionMint = keys.next().unwrap().clone();
                let bubblegumSigner = keys.next().unwrap().clone();
                let collectionAuthorityRecord = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BubblegumSetCollectionSizeAccounts {
                    collectionMetadata,
                    collectionAuthority,
                    collectionMint,
                    bubblegumSigner,
                    collectionAuthorityRecord,
                    remaining,
                };
                return Ok(Instruction::BubblegumSetCollectionSize { accounts, args });
            }
            [37u8] => {
                let mut rdr: &[u8] = rest;
                let args = BurnEditionNftArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let owner = keys.next().unwrap().clone();
                let printEditionMint = keys.next().unwrap().clone();
                let masterEditionMint = keys.next().unwrap().clone();
                let printEditionTokenAccount = keys.next().unwrap().clone();
                let masterEditionTokenAccount = keys.next().unwrap().clone();
                let masterEditionAccount = keys.next().unwrap().clone();
                let printEditionAccount = keys.next().unwrap().clone();
                let editionMarkerAccount = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BurnEditionNftAccounts {
                    metadata,
                    owner,
                    printEditionMint,
                    masterEditionMint,
                    printEditionTokenAccount,
                    masterEditionTokenAccount,
                    masterEditionAccount,
                    printEditionAccount,
                    editionMarkerAccount,
                    splTokenProgram,
                    remaining,
                };
                return Ok(Instruction::BurnEditionNft { accounts, args });
            }
            [38u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateEscrowAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let escrow = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let tokenAccount = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateEscrowAccountAccounts {
                    escrow,
                    metadata,
                    mint,
                    tokenAccount,
                    edition,
                    payer,
                    systemProgram,
                    sysvarInstructions,
                    authority,
                    remaining,
                };
                return Ok(Instruction::CreateEscrowAccount { accounts, args });
            }
            [39u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseEscrowAccountArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let escrow = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let tokenAccount = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseEscrowAccountAccounts {
                    escrow,
                    metadata,
                    mint,
                    tokenAccount,
                    edition,
                    payer,
                    systemProgram,
                    sysvarInstructions,
                    remaining,
                };
                return Ok(Instruction::CloseEscrowAccount { accounts, args });
            }
            [40u8] => {
                let mut rdr: &[u8] = rest;
                let args = TransferOutOfEscrowArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let escrow = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let attributeMint = keys.next().unwrap().clone();
                let attributeSrc = keys.next().unwrap().clone();
                let attributeDst = keys.next().unwrap().clone();
                let escrowMint = keys.next().unwrap().clone();
                let escrowAccount = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let ataProgram = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferOutOfEscrowAccounts {
                    escrow,
                    metadata,
                    payer,
                    attributeMint,
                    attributeSrc,
                    attributeDst,
                    escrowMint,
                    escrowAccount,
                    systemProgram,
                    ataProgram,
                    tokenProgram,
                    sysvarInstructions,
                    authority,
                    remaining,
                };
                return Ok(Instruction::TransferOutOfEscrow { accounts, args });
            }
            [41u8] => {
                let mut rdr: &[u8] = rest;
                let args = BurnArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let collectionMetadata = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let masterEditionMint = keys.next().unwrap().clone();
                let masterEditionToken = keys.next().unwrap().clone();
                let editionMarker = keys.next().unwrap().clone();
                let tokenRecord = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = BurnAccounts {
                    authority,
                    collectionMetadata,
                    metadata,
                    edition,
                    mint,
                    token,
                    masterEdition,
                    masterEditionMint,
                    masterEditionToken,
                    editionMarker,
                    tokenRecord,
                    systemProgram,
                    sysvarInstructions,
                    splTokenProgram,
                    remaining,
                };
                return Ok(Instruction::Burn { accounts, args });
            }
            [42u8] => {
                let mut rdr: &[u8] = rest;
                let args = CreateArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateAccounts {
                    metadata,
                    masterEdition,
                    mint,
                    authority,
                    payer,
                    updateAuthority,
                    systemProgram,
                    sysvarInstructions,
                    splTokenProgram,
                    remaining,
                };
                return Ok(Instruction::Create { accounts, args });
            }
            [43u8] => {
                let mut rdr: &[u8] = rest;
                let args = MintArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token = keys.next().unwrap().clone();
                let tokenOwner = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let tokenRecord = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let delegateRecord = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let splAtaProgram = keys.next().unwrap().clone();
                let authorizationRulesProgram = keys.next().unwrap().clone();
                let authorizationRules = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MintAccounts {
                    token,
                    tokenOwner,
                    metadata,
                    masterEdition,
                    tokenRecord,
                    mint,
                    authority,
                    delegateRecord,
                    payer,
                    systemProgram,
                    sysvarInstructions,
                    splTokenProgram,
                    splAtaProgram,
                    authorizationRulesProgram,
                    authorizationRules,
                    remaining,
                };
                return Ok(Instruction::Mint { accounts, args });
            }
            [44u8] => {
                let mut rdr: &[u8] = rest;
                let args = DelegateArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let delegateRecord = keys.next().unwrap().clone();
                let delegate = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let tokenRecord = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let authorizationRulesProgram = keys.next().unwrap().clone();
                let authorizationRules = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = DelegateAccounts {
                    delegateRecord,
                    delegate,
                    metadata,
                    masterEdition,
                    tokenRecord,
                    mint,
                    token,
                    authority,
                    payer,
                    systemProgram,
                    sysvarInstructions,
                    splTokenProgram,
                    authorizationRulesProgram,
                    authorizationRules,
                    remaining,
                };
                return Ok(Instruction::Delegate { accounts, args });
            }
            [45u8] => {
                let mut rdr: &[u8] = rest;
                let args = RevokeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let delegateRecord = keys.next().unwrap().clone();
                let delegate = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let tokenRecord = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let authorizationRulesProgram = keys.next().unwrap().clone();
                let authorizationRules = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = RevokeAccounts {
                    delegateRecord,
                    delegate,
                    metadata,
                    masterEdition,
                    tokenRecord,
                    mint,
                    token,
                    authority,
                    payer,
                    systemProgram,
                    sysvarInstructions,
                    splTokenProgram,
                    authorizationRulesProgram,
                    authorizationRules,
                    remaining,
                };
                return Ok(Instruction::Revoke { accounts, args });
            }
            [46u8] => {
                let mut rdr: &[u8] = rest;
                let args = LockArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let tokenOwner = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let tokenRecord = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let authorizationRulesProgram = keys.next().unwrap().clone();
                let authorizationRules = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LockAccounts {
                    authority,
                    tokenOwner,
                    token,
                    mint,
                    metadata,
                    edition,
                    tokenRecord,
                    payer,
                    systemProgram,
                    sysvarInstructions,
                    splTokenProgram,
                    authorizationRulesProgram,
                    authorizationRules,
                    remaining,
                };
                return Ok(Instruction::Lock { accounts, args });
            }
            [47u8] => {
                let mut rdr: &[u8] = rest;
                let args = UnlockArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let tokenOwner = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let tokenRecord = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let authorizationRulesProgram = keys.next().unwrap().clone();
                let authorizationRules = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UnlockAccounts {
                    authority,
                    tokenOwner,
                    token,
                    mint,
                    metadata,
                    edition,
                    tokenRecord,
                    payer,
                    systemProgram,
                    sysvarInstructions,
                    splTokenProgram,
                    authorizationRulesProgram,
                    authorizationRules,
                    remaining,
                };
                return Ok(Instruction::Unlock { accounts, args });
            }
            [48u8] => {
                let mut rdr: &[u8] = rest;
                let args = MigrateArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let tokenOwner = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let collectionMetadata = keys.next().unwrap().clone();
                let delegateRecord = keys.next().unwrap().clone();
                let tokenRecord = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let authorizationRulesProgram = keys.next().unwrap().clone();
                let authorizationRules = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = MigrateAccounts {
                    metadata,
                    edition,
                    token,
                    tokenOwner,
                    mint,
                    payer,
                    authority,
                    collectionMetadata,
                    delegateRecord,
                    tokenRecord,
                    systemProgram,
                    sysvarInstructions,
                    splTokenProgram,
                    authorizationRulesProgram,
                    authorizationRules,
                    remaining,
                };
                return Ok(Instruction::Migrate { accounts, args });
            }
            [49u8] => {
                let mut rdr: &[u8] = rest;
                let args = TransferArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let token = keys.next().unwrap().clone();
                let tokenOwner = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let destinationOwner = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let ownerTokenRecord = keys.next().unwrap().clone();
                let destinationTokenRecord = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let splAtaProgram = keys.next().unwrap().clone();
                let authorizationRulesProgram = keys.next().unwrap().clone();
                let authorizationRules = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = TransferAccounts {
                    token,
                    tokenOwner,
                    destination,
                    destinationOwner,
                    mint,
                    metadata,
                    edition,
                    ownerTokenRecord,
                    destinationTokenRecord,
                    authority,
                    payer,
                    systemProgram,
                    sysvarInstructions,
                    splTokenProgram,
                    splAtaProgram,
                    authorizationRulesProgram,
                    authorizationRules,
                    remaining,
                };
                return Ok(Instruction::Transfer { accounts, args });
            }
            [50u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let delegateRecord = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let authorizationRulesProgram = keys.next().unwrap().clone();
                let authorizationRules = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateAccounts {
                    authority,
                    delegateRecord,
                    token,
                    mint,
                    metadata,
                    edition,
                    payer,
                    systemProgram,
                    sysvarInstructions,
                    authorizationRulesProgram,
                    authorizationRules,
                    remaining,
                };
                return Ok(Instruction::Update { accounts, args });
            }
            [51u8] => {
                let mut rdr: &[u8] = rest;
                let args = UseArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let delegateRecord = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let authorizationRulesProgram = keys.next().unwrap().clone();
                let authorizationRules = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UseAccounts {
                    authority,
                    delegateRecord,
                    token,
                    mint,
                    metadata,
                    edition,
                    payer,
                    systemProgram,
                    sysvarInstructions,
                    splTokenProgram,
                    authorizationRulesProgram,
                    authorizationRules,
                    remaining,
                };
                return Ok(Instruction::Use { accounts, args });
            }
            [52u8] => {
                let mut rdr: &[u8] = rest;
                let args = VerifyArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let delegateRecord = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let collectionMint = keys.next().unwrap().clone();
                let collectionMetadata = keys.next().unwrap().clone();
                let collectionMasterEdition = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = VerifyAccounts {
                    authority,
                    delegateRecord,
                    metadata,
                    collectionMint,
                    collectionMetadata,
                    collectionMasterEdition,
                    systemProgram,
                    sysvarInstructions,
                    remaining,
                };
                return Ok(Instruction::Verify { accounts, args });
            }
            [53u8] => {
                let mut rdr: &[u8] = rest;
                let args = UnverifyArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let delegateRecord = keys.next().unwrap().clone();
                let metadata = keys.next().unwrap().clone();
                let collectionMint = keys.next().unwrap().clone();
                let collectionMetadata = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UnverifyAccounts {
                    authority,
                    delegateRecord,
                    metadata,
                    collectionMint,
                    collectionMetadata,
                    systemProgram,
                    sysvarInstructions,
                    remaining,
                };
                return Ok(Instruction::Unverify { accounts, args });
            }
            [54u8] => {
                let mut rdr: &[u8] = rest;
                let args = CollectArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let authority = keys.next().unwrap().clone();
                let recipient = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CollectAccounts {
                    authority,
                    recipient,
                    remaining,
                };
                return Ok(Instruction::Collect { accounts, args });
            }
            [55u8] => {
                let mut rdr: &[u8] = rest;
                let args = PrintArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let editionMetadata = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let editionMint = keys.next().unwrap().clone();
                let editionTokenAccountOwner = keys.next().unwrap().clone();
                let editionTokenAccount = keys.next().unwrap().clone();
                let editionMintAuthority = keys.next().unwrap().clone();
                let editionTokenRecord = keys.next().unwrap().clone();
                let masterEdition = keys.next().unwrap().clone();
                let editionMarkerPda = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let masterTokenAccountOwner = keys.next().unwrap().clone();
                let masterTokenAccount = keys.next().unwrap().clone();
                let masterMetadata = keys.next().unwrap().clone();
                let updateAuthority = keys.next().unwrap().clone();
                let splTokenProgram = keys.next().unwrap().clone();
                let splAtaProgram = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = PrintAccounts {
                    editionMetadata,
                    edition,
                    editionMint,
                    editionTokenAccountOwner,
                    editionTokenAccount,
                    editionMintAuthority,
                    editionTokenRecord,
                    masterEdition,
                    editionMarkerPda,
                    payer,
                    masterTokenAccountOwner,
                    masterTokenAccount,
                    masterMetadata,
                    updateAuthority,
                    splTokenProgram,
                    splAtaProgram,
                    sysvarInstructions,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::Print { accounts, args });
            }
            [56u8] => {
                let mut rdr: &[u8] = rest;
                let args = ResizeArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let payer = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let token = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = ResizeAccounts {
                    metadata,
                    edition,
                    mint,
                    payer,
                    authority,
                    token,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::Resize { accounts, args });
            }
            [57u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseAccountsArguments::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let metadata = keys.next().unwrap().clone();
                let edition = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let authority = keys.next().unwrap().clone();
                let destination = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = CloseAccountsAccounts {
                    metadata,
                    edition,
                    mint,
                    authority,
                    destination,
                    remaining,
                };
                return Ok(Instruction::CloseAccounts { accounts, args });
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
