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
fn serialize_u64_as_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}
pub use accounts_data::*;
pub use typedefs::*;
pub mod typedefs {
    use super::serialize_u64_as_string;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use anchor_lang::prelude::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use serde::Serialize;
    use serde::Serializer;
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum Dex {
        SplTokenSwap,
        StableSwap,
        Whirlpool,
        MeteoraDynamicpool,
        RaydiumSwap,
        RaydiumStableSwap,
        RaydiumClmmSwap,
        AldrinExchangeV1,
        AldrinExchangeV2,
        LifinityV1,
        LifinityV2,
        RaydiumClmmSwapV2,
        FluxBeam,
        MeteoraDlmm,
        RaydiumCpmmSwap,
        OpenBookV2,
        WhirlpoolV2,
        Phoenix,
        ObricV2,
        SanctumAddLiq,
        SanctumRemoveLiq,
        SanctumNonWsolSwap,
        SanctumWsolSwap,
        PumpfunBuy,
        PumpfunSell,
        StabbleSwap,
        SanctumRouter,
        MeteoraVaultDeposit,
        MeteoraVaultWithdraw,
        Saros,
        MeteoraLst,
        Solfi,
        QualiaSwap,
        Zerofi,
        PumpfunammBuy,
        PumpfunammSell,
        Virtuals,
        VertigoBuy,
        VertigoSell,
        PerpetualsAddLiq,
        PerpetualsRemoveLiq,
        PerpetualsSwap,
        RaydiumLaunchpad,
        LetsBonkFun,
        Woofi,
        MeteoraDbc,
        MeteoraDlmmSwap2,
        MeteoraDAMMV2,
        Gavel,
        BoopfunBuy,
        BoopfunSell,
        MeteoraDbc2,
        GooseFX,
        Dooar,
        Numeraire,
        SaberDecimalWrapperDeposit,
        SaberDecimalWrapperWithdraw,
        SarosDlmm,
        OneDexSwap,
        Manifest,
        ByrealClmm,
        PancakeSwapV3Swap,
        PancakeSwapV3SwapV2,
        Tessera,
        SolRfq {
            rfq_id: u64,
            expected_maker_amount: u64,
            expected_taker_amount: u64,
            maker_send_amount: u64,
            taker_send_amount: u64,
            expiry: u64,
            maker_use_native_sol: bool,
            taker_use_native_sol: bool,
        },
        Humidifi,
        HeavenBuy,
        HeavenSell,
        SolfiV2,
        Goonfi,
        MoonitBuy,
        MoonitSell,
        RaydiumSwapV2,
        Swaap,
        SugarMoneyBuy {
            bonding_curve_bump: u8,
            bonding_curve_sol_associated_account_bump: u8,
        },
        SugarMoneySell {
            bonding_curve_bump: u8,
            bonding_curve_sol_associated_account_bump: u8,
        },
        MeteoraDAMMV2Swap2,
        AlphaQ,
    }
    impl Default for Dex {
        fn default() -> Self {
            Self::SplTokenSwap
        }
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct PlatformFeeWrapUnwrapArgs {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount_in: u64,
        pub commission_info: u32,
        pub platform_fee_rate: u16,
        pub tob: bool,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct Route {
        pub dex: Dex,
        pub weight: u16,
        pub index: u8,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
    pub struct SwapArgs {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub expect_amount_out: u64,
        pub slippage: u16,
        pub routes: Vec<Route>,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SwapCpiEvent {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
        #[serde(with = "pubkey_serde")]
        pub source_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub destination_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub source_token_account_owner: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub destination_token_account_owner: Pubkey,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub source_token_change: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub destination_token_change: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SwapEvent {
        pub dex: Dex,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount_in: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub amount_out: u64,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SwapWithFeesCpiEvent {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
        #[serde(with = "pubkey_serde")]
        pub source_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub destination_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub source_token_account_owner: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub destination_token_account_owner: Pubkey,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub source_token_change: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub destination_token_change: u64,
        pub commission_direction: bool,
        pub commission_rate: u32,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub commission_amount: u64,
        #[serde(with = "pubkey_serde")]
        pub commission_account: Pubkey,
        pub platform_fee_rate: u16,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub platform_fee_amount: u64,
        #[serde(with = "pubkey_serde")]
        pub platform_fee_account: Pubkey,
        pub trim_rate: u8,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub trim_amount: u64,
        #[serde(with = "pubkey_serde")]
        pub trim_account: Pubkey,
    }
    #[derive(Serialize, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct SwapWithFeesCpiEventEnhanced {
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub order_id: u64,
        #[serde(with = "pubkey_serde")]
        pub source_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub destination_mint: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub source_token_account_owner: Pubkey,
        #[serde(with = "pubkey_serde")]
        pub destination_token_account_owner: Pubkey,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub source_token_change: u64,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub destination_token_change: u64,
        pub commission_direction: bool,
        pub commission_rate: u32,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub commission_amount: u64,
        #[serde(with = "pubkey_serde")]
        pub commission_account: Pubkey,
        pub platform_fee_rate: u16,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub platform_fee_amount: u64,
        #[serde(with = "pubkey_serde")]
        pub platform_fee_account: Pubkey,
        pub trim_rate: u8,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub trim_amount: u64,
        #[serde(with = "pubkey_serde")]
        pub trim_account: Pubkey,
        pub charge_rate: u16,
        #[serde(serialize_with = "serialize_u64_as_string")]
        pub charge_amount: u64,
        #[serde(with = "pubkey_serde")]
        pub charge_account: Pubkey,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct ClaimAccounts {
        pub signer: String,
        pub receiver: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub sa_authority: String,
        pub token_mint: String,
        pub token_program: String,
        pub system_program: String,
        pub associated_token_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateTokenAccountAccounts {
        pub payer: String,
        pub owner: String,
        pub token_account: String,
        pub token_mint: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateTokenAccountWithSeedAccounts {
        pub payer: String,
        pub owner: String,
        pub token_account: String,
        pub token_mint: String,
        pub token_program: String,
        pub system_program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct ProxySwapAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapTobAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_account: String,
        pub platform_fee_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapTobEnhancedAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_account: String,
        pub platform_fee_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapTobWithReceiverAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_account: String,
        pub platform_fee_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub sol_receiver: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct SwapTocAccounts {
        pub payer: String,
        pub source_token_account: String,
        pub destination_token_account: String,
        pub source_mint: String,
        pub destination_mint: String,
        pub commission_account: String,
        pub platform_fee_account: String,
        pub sa_authority: String,
        pub source_token_sa: String,
        pub destination_token_sa: String,
        pub source_token_program: String,
        pub destination_token_program: String,
        pub associated_token_program: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WrapUnwrapAccounts {
        pub payer: String,
        pub payer_wsol_account: String,
        pub wsol_mint: String,
        pub temp_wsol_account: String,
        pub commission_account: String,
        pub platform_fee_account: String,
        pub authority_pda: String,
        pub wsol_sa: String,
        pub token_program: String,
        pub system_program: String,
        pub event_authority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::serialize_u64_as_string;
    use super::typedefs::*;
    use serde::Serialize;
    use serde::Serializer;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize, Default)]
    pub struct ClaimInstructionArgs {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateTokenAccountInstructionArgs {
        pub bump: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateTokenAccountWithSeedInstructionArgs {
        pub bump: u8,
        pub seed: u32,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct ProxySwapInstructionArgs {
        pub args: SwapArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapInstructionArgs {
        pub args: SwapArgs,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapTobInstructionArgs {
        pub args: SwapArgs,
        pub commission_info: u32,
        pub platform_fee_rate: u16,
        pub trim_rate: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapTobEnhancedInstructionArgs {
        pub args: SwapArgs,
        pub commission_info: u32,
        pub platform_fee_rate: u16,
        pub trim_rate: u8,
        pub charge_rate: u16,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapTobWithReceiverInstructionArgs {
        pub args: SwapArgs,
        pub commission_info: u32,
        pub platform_fee_rate: u16,
        pub trim_rate: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct SwapTocInstructionArgs {
        pub args: SwapArgs,
        pub commission_info: u32,
        pub platform_fee_rate: u16,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WrapUnwrapInstructionArgs {
        pub args: PlatformFeeWrapUnwrapArgs,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    Claim {
        accounts: ClaimAccounts,
        args: ix_data::ClaimInstructionArgs,
    },
    CreateTokenAccount {
        accounts: CreateTokenAccountAccounts,
        args: ix_data::CreateTokenAccountInstructionArgs,
    },
    CreateTokenAccountWithSeed {
        accounts: CreateTokenAccountWithSeedAccounts,
        args: ix_data::CreateTokenAccountWithSeedInstructionArgs,
    },
    ProxySwap {
        accounts: ProxySwapAccounts,
        args: ix_data::ProxySwapInstructionArgs,
    },
    Swap {
        accounts: SwapAccounts,
        args: ix_data::SwapInstructionArgs,
    },
    SwapTob {
        accounts: SwapTobAccounts,
        args: ix_data::SwapTobInstructionArgs,
    },
    SwapTobEnhanced {
        accounts: SwapTobEnhancedAccounts,
        args: ix_data::SwapTobEnhancedInstructionArgs,
    },
    SwapTobWithReceiver {
        accounts: SwapTobWithReceiverAccounts,
        args: ix_data::SwapTobWithReceiverInstructionArgs,
    },
    SwapToc {
        accounts: SwapTocAccounts,
        args: ix_data::SwapTocInstructionArgs,
    },
    WrapUnwrap {
        accounts: WrapUnwrapAccounts,
        args: ix_data::WrapUnwrapInstructionArgs,
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
            [62u8, 198u8, 214u8, 193u8, 213u8, 159u8, 108u8, 210u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::ClaimInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let signer = keys.next().unwrap_or(&String::new()).clone();
                let receiver = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let token_mint = keys.next().unwrap_or(&String::new()).clone();
                let token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = ClaimAccounts {
                    signer,
                    receiver,
                    source_token_account,
                    destination_token_account,
                    sa_authority,
                    token_mint,
                    token_program,
                    system_program,
                    associated_token_program,
                    remaining,
                };
                return Ok(Instruction::Claim { accounts, args });
            }
            [147u8, 241u8, 123u8, 100u8, 244u8, 132u8, 174u8, 118u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::CreateTokenAccountInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let owner = keys.next().unwrap_or(&String::new()).clone();
                let token_account = keys.next().unwrap_or(&String::new()).clone();
                let token_mint = keys.next().unwrap_or(&String::new()).clone();
                let token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateTokenAccountAccounts {
                    payer,
                    owner,
                    token_account,
                    token_mint,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateTokenAccount { accounts, args });
            }
            [125u8, 191u8, 239u8, 140u8, 66u8, 8u8, 9u8, 228u8] => {
                let mut rdr: &[u8] = rest;
                let args =
                    ix_data::CreateTokenAccountWithSeedInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let owner = keys.next().unwrap_or(&String::new()).clone();
                let token_account = keys.next().unwrap_or(&String::new()).clone();
                let token_mint = keys.next().unwrap_or(&String::new()).clone();
                let token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = CreateTokenAccountWithSeedAccounts {
                    payer,
                    owner,
                    token_account,
                    token_mint,
                    token_program,
                    system_program,
                    remaining,
                };
                return Ok(Instruction::CreateTokenAccountWithSeed { accounts, args });
            }
            [19u8, 44u8, 130u8, 148u8, 72u8, 56u8, 44u8, 238u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::ProxySwapInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let event_authority = keys.next().unwrap_or(&String::new()).clone();
                let program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = ProxySwapAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::ProxySwap { accounts, args });
            }
            [248u8, 198u8, 158u8, 145u8, 225u8, 117u8, 135u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::SwapInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let event_authority = keys.next().unwrap_or(&String::new()).clone();
                let program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::Swap { accounts, args });
            }
            [170u8, 41u8, 85u8, 177u8, 132u8, 80u8, 31u8, 53u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::SwapTobInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let platform_fee_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let event_authority = keys.next().unwrap_or(&String::new()).clone();
                let program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapTobAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_account,
                    platform_fee_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SwapTob { accounts, args });
            }
            [190u8, 156u8, 169u8, 176u8, 149u8, 154u8, 161u8, 108u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::SwapTobEnhancedInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let platform_fee_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let event_authority = keys.next().unwrap_or(&String::new()).clone();
                let program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapTobEnhancedAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_account,
                    platform_fee_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SwapTobEnhanced { accounts, args });
            }
            [223u8, 170u8, 216u8, 234u8, 204u8, 6u8, 241u8, 25u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::SwapTobWithReceiverInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let platform_fee_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let sol_receiver = keys.next().unwrap_or(&String::new()).clone();
                let event_authority = keys.next().unwrap_or(&String::new()).clone();
                let program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapTobWithReceiverAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_account,
                    platform_fee_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    sol_receiver,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SwapTobWithReceiver { accounts, args });
            }
            [187u8, 201u8, 212u8, 51u8, 16u8, 155u8, 236u8, 60u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::SwapTocInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let source_token_account = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_account = keys.next().unwrap_or(&String::new()).clone();
                let source_mint = keys.next().unwrap_or(&String::new()).clone();
                let destination_mint = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let platform_fee_account = keys.next().unwrap_or(&String::new()).clone();
                let sa_authority = keys.next().unwrap_or(&String::new()).clone();
                let source_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_sa = keys.next().unwrap_or(&String::new()).clone();
                let source_token_program = keys.next().unwrap_or(&String::new()).clone();
                let destination_token_program = keys.next().unwrap_or(&String::new()).clone();
                let associated_token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let event_authority = keys.next().unwrap_or(&String::new()).clone();
                let program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = SwapTocAccounts {
                    payer,
                    source_token_account,
                    destination_token_account,
                    source_mint,
                    destination_mint,
                    commission_account,
                    platform_fee_account,
                    sa_authority,
                    source_token_sa,
                    destination_token_sa,
                    source_token_program,
                    destination_token_program,
                    associated_token_program,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::SwapToc { accounts, args });
            }
            [220u8, 101u8, 139u8, 249u8, 41u8, 190u8, 118u8, 199u8] => {
                let mut rdr: &[u8] = rest;
                let args = ix_data::WrapUnwrapInstructionArgs::deserialize(&mut rdr)?;
                let mut keys = account_keys.iter();
                let payer = keys.next().unwrap_or(&String::new()).clone();
                let payer_wsol_account = keys.next().unwrap_or(&String::new()).clone();
                let wsol_mint = keys.next().unwrap_or(&String::new()).clone();
                let temp_wsol_account = keys.next().unwrap_or(&String::new()).clone();
                let commission_account = keys.next().unwrap_or(&String::new()).clone();
                let platform_fee_account = keys.next().unwrap_or(&String::new()).clone();
                let authority_pda = keys.next().unwrap_or(&String::new()).clone();
                let wsol_sa = keys.next().unwrap_or(&String::new()).clone();
                let token_program = keys.next().unwrap_or(&String::new()).clone();
                let system_program = keys.next().unwrap_or(&String::new()).clone();
                let event_authority = keys.next().unwrap_or(&String::new()).clone();
                let program = keys.next().unwrap_or(&String::new()).clone();
                let remaining = keys.cloned().collect();
                let accounts = WrapUnwrapAccounts {
                    payer,
                    payer_wsol_account,
                    wsol_mint,
                    temp_wsol_account,
                    commission_account,
                    platform_fee_account,
                    authority_pda,
                    wsol_sa,
                    token_program,
                    system_program,
                    event_authority,
                    program,
                    remaining,
                };
                return Ok(Instruction::WrapUnwrap { accounts, args });
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
        SwapCpiEvent { args: SwapCpiEvent },
        SwapEvent { args: SwapEvent },
        SwapWithFeesCpiEvent { args: SwapWithFeesCpiEvent },
        SwapWithFeesCpiEventEnhanced { args: SwapWithFeesCpiEventEnhanced },
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
            match disc {
                [85u8, 81u8, 149u8, 239u8, 163u8, 74u8, 158u8, 111u8] => {
                    let mut rdr = &data[16..];
                    let args = SwapCpiEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SwapCpiEvent { args });
                }
                [64u8, 198u8, 205u8, 232u8, 38u8, 8u8, 113u8, 226u8] => {
                    let mut rdr = &data[16..];
                    let args = SwapEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SwapEvent { args });
                }
                [189u8, 97u8, 67u8, 12u8, 37u8, 209u8, 247u8, 29u8] => {
                    let mut rdr = &data[16..];
                    let args = SwapWithFeesCpiEvent::deserialize(&mut rdr)?;
                    return Ok(Event::SwapWithFeesCpiEvent { args });
                }
                [37u8, 72u8, 219u8, 67u8, 50u8, 244u8, 1u8, 213u8] => {
                    let mut rdr = &data[16..];
                    let args = SwapWithFeesCpiEventEnhanced::deserialize(&mut rdr)?;
                    return Ok(Event::SwapWithFeesCpiEventEnhanced { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
