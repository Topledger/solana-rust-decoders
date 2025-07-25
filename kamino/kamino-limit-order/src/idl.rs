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
#[doc = r" Parse an Option<T> in either old‑IDL (no tag) or new‑IDL (0x00/0x01 prefix) form"]
fn parse_option<T: ::borsh::BorshDeserialize>(rdr: &mut &[u8]) -> anyhow::Result<Option<T>> {
    if rdr.is_empty() {
        return Ok(None);
    }
    let tag = rdr[0];
    if tag == 0 {
        *rdr = &rdr[1..];
        return Ok(None);
    } else if tag == 1 {
        *rdr = &rdr[1..];
        let v = T::deserialize(rdr)?;
        return Ok(Some(v));
    }
    let v = T::deserialize(rdr)?;
    Ok(Some(v))
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
    serde_big_array::big_array! { BigArray ; 64 , 51 , 72 , 128 , 256 }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OrderStatus {
        Active,
        Filled,
        Cancelled,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum OrderType {
        Vanilla,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateGlobalConfigMode {
        UpdateEmergencyMode,
        UpdateFlashTakeOrderBlocked,
        UpdateBlockNewOrders,
        UpdateBlockOrderTaking,
        UpdateHostFeeBps,
        UpdateAdminAuthorityCached,
        UpdateOrderTakingPermissionless,
        UpdateOrderCloseDelaySeconds,
        UpdateTxnFeeCost,
        UpdateAtaCreationCost,
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateGlobalConfigValue {
        Bool(bool),
        U16(u16),
        U64(u64),
        Pubkey([u8; 32usize]),
    }
    #[derive(:: borsh :: BorshSerialize, :: borsh :: BorshDeserialize, Clone, Debug, Serialize)]
    pub enum UpdateOrderMode {
        UpdatePermissionless,
        UpdateCounterparty,
    }
}
pub mod accounts_data {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct InitializeGlobalConfigAccounts {
        pub adminAuthority: String,
        pub pdaAuthority: String,
        pub globalConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeVaultAccounts {
        pub payer: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub mint: String,
        pub vault: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CreateOrderAccounts {
        pub maker: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub order: String,
        pub inputMint: String,
        pub outputMint: String,
        pub makerAta: String,
        pub inputVault: String,
        pub inputTokenProgram: String,
        pub outputTokenProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub systemProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateOrderAccounts {
        pub maker: String,
        pub globalConfig: String,
        pub order: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct CloseOrderAndClaimTipAccounts {
        pub maker: String,
        pub order: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub inputMint: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub outputMint: Option<String>,
        pub makerInputAta: String,
        pub inputVault: String,
        pub inputTokenProgram: String,
        pub systemProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct InitializeTipVaultAccounts {
        pub admin_authority: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub mint: String,
        pub tip_vault: String,
        pub rent: String,
        pub tokenProgram: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct TakeOrderAccounts {
        pub taker: String,
        pub maker: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub order: String,
        pub inputMint: String,
        pub outputMint: String,
        pub inputVault: String,
        pub takerInputAta: String,
        pub takerOutputAta: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub intermediaryOutputTokenAccount: Option<String>,
        pub makerOutputAta: String,
        pub expressRelay: String,
        pub expressRelayMetadata: String,
        pub sysvarInstructions: String,
        pub permission: String,
        pub configRouter: String,
        pub inputTokenProgram: String,
        pub outputTokenProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rent: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub systemProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FlashTakeOrderStartAccounts {
        pub taker: String,
        pub maker: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub order: String,
        pub inputMint: String,
        pub outputMint: String,
        pub inputVault: String,
        pub takerInputAta: String,
        pub takerOutputAta: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub intermediaryOutputTokenAccount: Option<String>,
        pub makerOutputAta: String,
        pub expressRelay: String,
        pub expressRelayMetadata: String,
        pub sysvarInstructions: String,
        pub permission: String,
        pub configRouter: String,
        pub inputTokenProgram: String,
        pub outputTokenProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub systemProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rent: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct FlashTakeOrderEndAccounts {
        pub taker: String,
        pub maker: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub order: String,
        pub inputMint: String,
        pub outputMint: String,
        pub inputVault: String,
        pub takerInputAta: String,
        pub takerOutputAta: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub intermediaryOutputTokenAccount: Option<String>,
        pub makerOutputAta: String,
        pub expressRelay: String,
        pub expressRelayMetadata: String,
        pub sysvarInstructions: String,
        pub permission: String,
        pub configRouter: String,
        pub inputTokenProgram: String,
        pub outputTokenProgram: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub systemProgram: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rent: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eventAuthority: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub program: Option<String>,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateGlobalConfigAccounts {
        pub adminAuthority: String,
        pub globalConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct UpdateGlobalConfigAdminAccounts {
        pub adminAuthorityCached: String,
        pub globalConfig: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct WithdrawHostTipAccounts {
        pub adminAuthority: String,
        pub globalConfig: String,
        pub pdaAuthority: String,
        pub systemProgram: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LogUserSwapBalancesAccounts {
        pub maker: String,
        pub inputMint: String,
        pub outputMint: String,
        pub inputTa: String,
        pub outputTa: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LogUserSwapBalancesStartAccounts {
        pub maker: String,
        pub inputMint: String,
        pub outputMint: String,
        pub inputTa: String,
        pub outputTa: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pdaReferrer: Option<String>,
        pub swapProgramId: String,
        pub userSwapBalanceState: String,
        pub systemProgram: String,
        pub rent: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sysvarInstructions: Option<String>,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct LogUserSwapBalancesEndAccounts {
        pub maker: String,
        pub inputMint: String,
        pub outputMint: String,
        pub inputTa: String,
        pub outputTa: String,
        pub pdaReferrer: String,
        pub swapProgramId: String,
        pub userSwapBalanceState: String,
        pub systemProgram: String,
        pub rent: String,
        pub sysvarInstructions: String,
        pub eventAuthority: String,
        pub program: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AssertUserSwapBalancesStartAccounts {
        pub maker: String,
        pub inputTa: String,
        pub outputTa: String,
        pub userSwapBalanceState: String,
        pub systemProgram: String,
        pub rent: String,
        pub sysvarInstructions: String,
        pub remaining: Vec<String>,
    }
    #[derive(Debug, Serialize)]
    pub struct AssertUserSwapBalancesEndAccounts {
        pub maker: String,
        pub inputTa: String,
        pub outputTa: String,
        pub userSwapBalanceState: String,
        pub systemProgram: String,
        pub rent: String,
        pub sysvarInstructions: String,
        pub remaining: Vec<String>,
    }
}
pub mod ix_data {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use serde::Serialize;
    serde_big_array::big_array! { BigArray ; 64 , 51 , 72 , 128 , 256 }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeGlobalConfigArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeVaultArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CreateOrderArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub output_amount: u64,
        pub order_type: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateOrderArguments {
        pub mode: UpdateOrderMode,
        pub value: Vec<u8>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct CloseOrderAndClaimTipArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct InitializeTipVaultArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct TakeOrderArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_amount: u64,
        pub min_output_amount: Option<u64>,
        pub tip_amount_permissionless_taking: Option<u64>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FlashTakeOrderStartArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_output_amount: u64,
        pub tip_amount_permissionless_taking: Option<u64>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct FlashTakeOrderEndArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_output_amount: u64,
        pub tip_amount_permissionless_taking: Option<u64>,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGlobalConfigArguments {
        pub mode: u16,
        #[serde(with = "BigArray")]
        pub value: [u8; 128usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UpdateGlobalConfigAdminArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct WithdrawHostTipArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LogUserSwapBalancesArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LogUserSwapBalancesStartArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct LogUserSwapBalancesEndArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub simulated_swap_amount_out: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub simulated_ts: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_amount_out: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub swap_amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub simulated_amount_out_next_best: u64,
        pub aggregator: u8,
        pub next_best_aggregator: u8,
        pub padding: [u8; 2usize],
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AssertUserSwapBalancesStartArguments {}
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct AssertUserSwapBalancesEndArguments {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub max_input_amount_change: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub min_output_amount_change: u64,
    }
}
#[derive(Debug, Serialize)]
#[serde(tag = "instruction_type")]
pub enum Instruction {
    InitializeGlobalConfig {
        accounts: InitializeGlobalConfigAccounts,
        args: InitializeGlobalConfigArguments,
    },
    InitializeVault {
        accounts: InitializeVaultAccounts,
        args: InitializeVaultArguments,
    },
    CreateOrder {
        accounts: CreateOrderAccounts,
        args: CreateOrderArguments,
    },
    UpdateOrder {
        accounts: UpdateOrderAccounts,
        args: UpdateOrderArguments,
    },
    CloseOrderAndClaimTip {
        accounts: CloseOrderAndClaimTipAccounts,
        args: CloseOrderAndClaimTipArguments,
    },
    InitializeTipVault {
        accounts: InitializeTipVaultAccounts,
        args: InitializeTipVaultArguments,
    },
    TakeOrder {
        accounts: TakeOrderAccounts,
        args: TakeOrderArguments,
    },
    FlashTakeOrderStart {
        accounts: FlashTakeOrderStartAccounts,
        args: FlashTakeOrderStartArguments,
    },
    FlashTakeOrderEnd {
        accounts: FlashTakeOrderEndAccounts,
        args: FlashTakeOrderEndArguments,
    },
    UpdateGlobalConfig {
        accounts: UpdateGlobalConfigAccounts,
        args: UpdateGlobalConfigArguments,
    },
    UpdateGlobalConfigAdmin {
        accounts: UpdateGlobalConfigAdminAccounts,
        args: UpdateGlobalConfigAdminArguments,
    },
    WithdrawHostTip {
        accounts: WithdrawHostTipAccounts,
        args: WithdrawHostTipArguments,
    },
    LogUserSwapBalances {
        accounts: LogUserSwapBalancesAccounts,
        args: LogUserSwapBalancesArguments,
    },
    LogUserSwapBalancesStart {
        accounts: LogUserSwapBalancesStartAccounts,
        args: LogUserSwapBalancesStartArguments,
    },
    LogUserSwapBalancesEnd {
        accounts: LogUserSwapBalancesEndAccounts,
        args: LogUserSwapBalancesEndArguments,
    },
    AssertUserSwapBalancesStart {
        accounts: AssertUserSwapBalancesStartAccounts,
        args: AssertUserSwapBalancesStartArguments,
    },
    AssertUserSwapBalancesEnd {
        accounts: AssertUserSwapBalancesEndAccounts,
        args: AssertUserSwapBalancesEndArguments,
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
            [113u8, 216u8, 122u8, 131u8, 225u8, 209u8, 22u8, 55u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeGlobalConfigArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let adminAuthority = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeGlobalConfigAccounts {
                    remaining,
                    adminAuthority,
                    pdaAuthority,
                    globalConfig,
                };
                return Ok(Instruction::InitializeGlobalConfig { accounts, args });
            }
            [48u8, 191u8, 163u8, 44u8, 71u8, 129u8, 63u8, 164u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeVaultArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let payer = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let vault = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeVaultAccounts {
                    remaining,
                    payer,
                    globalConfig,
                    pdaAuthority,
                    mint,
                    vault,
                    tokenProgram,
                    systemProgram,
                };
                return Ok(Instruction::InitializeVault { accounts, args });
            }
            [141u8, 54u8, 37u8, 207u8, 237u8, 210u8, 250u8, 215u8] => {
                let mut rdr: &[u8] = rest;
                let input_amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let output_amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let order_type: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = CreateOrderArguments {
                    input_amount,
                    output_amount,
                    order_type,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(10usize);
                let maker = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let makerAta = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let outputTokenProgram = keys.next().unwrap().clone();
                let systemProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let eventAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let program = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = CreateOrderAccounts {
                    remaining,
                    maker,
                    globalConfig,
                    pdaAuthority,
                    order,
                    inputMint,
                    outputMint,
                    makerAta,
                    inputVault,
                    inputTokenProgram,
                    outputTokenProgram,
                    systemProgram,
                    eventAuthority,
                    program,
                };
                return Ok(Instruction::CreateOrder { accounts, args });
            }
            [54u8, 8u8, 208u8, 207u8, 34u8, 134u8, 239u8, 168u8] => {
                let mut rdr: &[u8] = rest;
                let mode: UpdateOrderMode =
                    <UpdateOrderMode as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let value: Vec<u8> = <Vec<u8> as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateOrderArguments { mode, value };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(3usize);
                let maker = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateOrderAccounts {
                    remaining,
                    maker,
                    globalConfig,
                    order,
                };
                return Ok(Instruction::UpdateOrder { accounts, args });
            }
            [244u8, 27u8, 12u8, 226u8, 45u8, 247u8, 230u8, 43u8] => {
                let mut rdr: &[u8] = rest;
                let args = CloseOrderAndClaimTipArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(9usize);
                let maker = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let makerInputAta = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let eventAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let program = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = CloseOrderAndClaimTipAccounts {
                    remaining,
                    maker,
                    order,
                    globalConfig,
                    pdaAuthority,
                    inputMint,
                    outputMint,
                    makerInputAta,
                    inputVault,
                    inputTokenProgram,
                    systemProgram,
                    eventAuthority,
                    program,
                };
                return Ok(Instruction::CloseOrderAndClaimTip { accounts, args });
            }
            [66u8, 199u8, 95u8, 180u8, 137u8, 159u8, 207u8, 228u8] => {
                let mut rdr: &[u8] = rest;
                let args = InitializeTipVaultArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(8usize);
                let admin_authority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let mint = keys.next().unwrap().clone();
                let tip_vault = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let tokenProgram = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = InitializeTipVaultAccounts {
                    remaining,
                    admin_authority,
                    globalConfig,
                    pdaAuthority,
                    mint,
                    tip_vault,
                    rent,
                    tokenProgram,
                    systemProgram,
                };
                return Ok(Instruction::InitializeTipVault { accounts, args });
            }
            [163u8, 208u8, 20u8, 172u8, 223u8, 65u8, 255u8, 228u8] => {
                let mut rdr: &[u8] = rest;
                let input_amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let min_output_amount: Option<u64> = parse_option(&mut rdr)?;
                let tip_amount_permissionless_taking: Option<u64> = parse_option(&mut rdr)?;
                let args = TakeOrderArguments {
                    input_amount,
                    min_output_amount,
                    tip_amount_permissionless_taking,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(18usize);
                let taker = keys.next().unwrap().clone();
                let maker = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let takerInputAta = keys.next().unwrap().clone();
                let takerOutputAta = keys.next().unwrap().clone();
                let intermediaryOutputTokenAccount = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let makerOutputAta = keys.next().unwrap().clone();
                let expressRelay = keys.next().unwrap().clone();
                let expressRelayMetadata = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let permission = keys.next().unwrap().clone();
                let configRouter = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let outputTokenProgram = keys.next().unwrap().clone();
                let rent = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let systemProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let eventAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let program = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = TakeOrderAccounts {
                    remaining,
                    taker,
                    maker,
                    globalConfig,
                    pdaAuthority,
                    order,
                    inputMint,
                    outputMint,
                    inputVault,
                    takerInputAta,
                    takerOutputAta,
                    intermediaryOutputTokenAccount,
                    makerOutputAta,
                    expressRelay,
                    expressRelayMetadata,
                    sysvarInstructions,
                    permission,
                    configRouter,
                    inputTokenProgram,
                    outputTokenProgram,
                    rent,
                    systemProgram,
                    eventAuthority,
                    program,
                };
                return Ok(Instruction::TakeOrder { accounts, args });
            }
            [126u8, 53u8, 176u8, 15u8, 39u8, 103u8, 97u8, 243u8] => {
                let mut rdr: &[u8] = rest;
                let input_amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let min_output_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let tip_amount_permissionless_taking: Option<u64> = parse_option(&mut rdr)?;
                let args = FlashTakeOrderStartArguments {
                    input_amount,
                    min_output_amount,
                    tip_amount_permissionless_taking,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(18usize);
                let taker = keys.next().unwrap().clone();
                let maker = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let takerInputAta = keys.next().unwrap().clone();
                let takerOutputAta = keys.next().unwrap().clone();
                let intermediaryOutputTokenAccount = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let makerOutputAta = keys.next().unwrap().clone();
                let expressRelay = keys.next().unwrap().clone();
                let expressRelayMetadata = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let permission = keys.next().unwrap().clone();
                let configRouter = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let outputTokenProgram = keys.next().unwrap().clone();
                let systemProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let rent = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let eventAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let program = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = FlashTakeOrderStartAccounts {
                    remaining,
                    taker,
                    maker,
                    globalConfig,
                    pdaAuthority,
                    order,
                    inputMint,
                    outputMint,
                    inputVault,
                    takerInputAta,
                    takerOutputAta,
                    intermediaryOutputTokenAccount,
                    makerOutputAta,
                    expressRelay,
                    expressRelayMetadata,
                    sysvarInstructions,
                    permission,
                    configRouter,
                    inputTokenProgram,
                    outputTokenProgram,
                    systemProgram,
                    rent,
                    eventAuthority,
                    program,
                };
                return Ok(Instruction::FlashTakeOrderStart { accounts, args });
            }
            [206u8, 242u8, 215u8, 187u8, 134u8, 33u8, 224u8, 148u8] => {
                let mut rdr: &[u8] = rest;
                let input_amount: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let min_output_amount: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let tip_amount_permissionless_taking: Option<u64> = parse_option(&mut rdr)?;
                let args = FlashTakeOrderEndArguments {
                    input_amount,
                    min_output_amount,
                    tip_amount_permissionless_taking,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(18usize);
                let taker = keys.next().unwrap().clone();
                let maker = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let order = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let inputVault = keys.next().unwrap().clone();
                let takerInputAta = keys.next().unwrap().clone();
                let takerOutputAta = keys.next().unwrap().clone();
                let intermediaryOutputTokenAccount = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let makerOutputAta = keys.next().unwrap().clone();
                let expressRelay = keys.next().unwrap().clone();
                let expressRelayMetadata = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let permission = keys.next().unwrap().clone();
                let configRouter = keys.next().unwrap().clone();
                let inputTokenProgram = keys.next().unwrap().clone();
                let outputTokenProgram = keys.next().unwrap().clone();
                let systemProgram = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let rent = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let eventAuthority = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let program = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let remaining = keys.cloned().collect();
                let accounts = FlashTakeOrderEndAccounts {
                    remaining,
                    taker,
                    maker,
                    globalConfig,
                    pdaAuthority,
                    order,
                    inputMint,
                    outputMint,
                    inputVault,
                    takerInputAta,
                    takerOutputAta,
                    intermediaryOutputTokenAccount,
                    makerOutputAta,
                    expressRelay,
                    expressRelayMetadata,
                    sysvarInstructions,
                    permission,
                    configRouter,
                    inputTokenProgram,
                    outputTokenProgram,
                    systemProgram,
                    rent,
                    eventAuthority,
                    program,
                };
                return Ok(Instruction::FlashTakeOrderEnd { accounts, args });
            }
            [164u8, 84u8, 130u8, 189u8, 111u8, 58u8, 250u8, 200u8] => {
                let mut rdr: &[u8] = rest;
                let mode: u16 = <u16 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let value: [u8; 128usize] =
                    <[u8; 128usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = UpdateGlobalConfigArguments { mode, value };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let adminAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGlobalConfigAccounts {
                    remaining,
                    adminAuthority,
                    globalConfig,
                };
                return Ok(Instruction::UpdateGlobalConfig { accounts, args });
            }
            [184u8, 87u8, 23u8, 193u8, 156u8, 238u8, 175u8, 119u8] => {
                let mut rdr: &[u8] = rest;
                let args = UpdateGlobalConfigAdminArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(2usize);
                let adminAuthorityCached = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = UpdateGlobalConfigAdminAccounts {
                    remaining,
                    adminAuthorityCached,
                    globalConfig,
                };
                return Ok(Instruction::UpdateGlobalConfigAdmin { accounts, args });
            }
            [140u8, 246u8, 105u8, 165u8, 80u8, 85u8, 143u8, 18u8] => {
                let mut rdr: &[u8] = rest;
                let args = WithdrawHostTipArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(4usize);
                let adminAuthority = keys.next().unwrap().clone();
                let globalConfig = keys.next().unwrap().clone();
                let pdaAuthority = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = WithdrawHostTipAccounts {
                    remaining,
                    adminAuthority,
                    globalConfig,
                    pdaAuthority,
                    systemProgram,
                };
                return Ok(Instruction::WithdrawHostTip { accounts, args });
            }
            [35u8, 118u8, 95u8, 77u8, 231u8, 46u8, 128u8, 38u8] => {
                let mut rdr: &[u8] = rest;
                let args = LogUserSwapBalancesArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let maker = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let inputTa = keys.next().unwrap().clone();
                let outputTa = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LogUserSwapBalancesAccounts {
                    remaining,
                    maker,
                    inputMint,
                    outputMint,
                    inputTa,
                    outputTa,
                    eventAuthority,
                    program,
                };
                return Ok(Instruction::LogUserSwapBalances { accounts, args });
            }
            [133u8, 108u8, 23u8, 15u8, 226u8, 215u8, 176u8, 95u8] => {
                let mut rdr: &[u8] = rest;
                let args = LogUserSwapBalancesStartArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(11usize);
                let maker = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let inputTa = keys.next().unwrap().clone();
                let outputTa = keys.next().unwrap().clone();
                let pdaReferrer = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let swapProgramId = keys.next().unwrap().clone();
                let userSwapBalanceState = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let sysvarInstructions = if opt_budget > 0 {
                    opt_budget -= 1;
                    Some(keys.next().unwrap().clone())
                } else {
                    None
                };
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LogUserSwapBalancesStartAccounts {
                    remaining,
                    maker,
                    inputMint,
                    outputMint,
                    inputTa,
                    outputTa,
                    pdaReferrer,
                    swapProgramId,
                    userSwapBalanceState,
                    systemProgram,
                    rent,
                    sysvarInstructions,
                    eventAuthority,
                    program,
                };
                return Ok(Instruction::LogUserSwapBalancesStart { accounts, args });
            }
            [140u8, 42u8, 198u8, 82u8, 147u8, 144u8, 44u8, 113u8] => {
                let mut rdr: &[u8] = rest;
                let simulated_swap_amount_out: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let simulated_ts: u64 = <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let minimum_amount_out: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let swap_amount_in: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let simulated_amount_out_next_best: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let aggregator: u8 = <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let next_best_aggregator: u8 =
                    <u8 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let padding: [u8; 2usize] =
                    <[u8; 2usize] as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = LogUserSwapBalancesEndArguments {
                    simulated_swap_amount_out,
                    simulated_ts,
                    minimum_amount_out,
                    swap_amount_in,
                    simulated_amount_out_next_best,
                    aggregator,
                    next_best_aggregator,
                    padding,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(13usize);
                let maker = keys.next().unwrap().clone();
                let inputMint = keys.next().unwrap().clone();
                let outputMint = keys.next().unwrap().clone();
                let inputTa = keys.next().unwrap().clone();
                let outputTa = keys.next().unwrap().clone();
                let pdaReferrer = keys.next().unwrap().clone();
                let swapProgramId = keys.next().unwrap().clone();
                let userSwapBalanceState = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let eventAuthority = keys.next().unwrap().clone();
                let program = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = LogUserSwapBalancesEndAccounts {
                    remaining,
                    maker,
                    inputMint,
                    outputMint,
                    inputTa,
                    outputTa,
                    pdaReferrer,
                    swapProgramId,
                    userSwapBalanceState,
                    systemProgram,
                    rent,
                    sysvarInstructions,
                    eventAuthority,
                    program,
                };
                return Ok(Instruction::LogUserSwapBalancesEnd { accounts, args });
            }
            [95u8, 241u8, 226u8, 193u8, 214u8, 175u8, 142u8, 139u8] => {
                let mut rdr: &[u8] = rest;
                let args = AssertUserSwapBalancesStartArguments {};
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let maker = keys.next().unwrap().clone();
                let inputTa = keys.next().unwrap().clone();
                let outputTa = keys.next().unwrap().clone();
                let userSwapBalanceState = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AssertUserSwapBalancesStartAccounts {
                    remaining,
                    maker,
                    inputTa,
                    outputTa,
                    userSwapBalanceState,
                    systemProgram,
                    rent,
                    sysvarInstructions,
                };
                return Ok(Instruction::AssertUserSwapBalancesStart { accounts, args });
            }
            [163u8, 157u8, 174u8, 93u8, 28u8, 127u8, 250u8, 136u8] => {
                let mut rdr: &[u8] = rest;
                let max_input_amount_change: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let min_output_amount_change: u64 =
                    <u64 as ::borsh::BorshDeserialize>::deserialize(&mut rdr)?;
                let args = AssertUserSwapBalancesEndArguments {
                    max_input_amount_change,
                    min_output_amount_change,
                };
                let mut keys = account_keys.iter();
                let mut keys = account_keys.iter();
                let mut opt_budget = account_keys.len().saturating_sub(7usize);
                let maker = keys.next().unwrap().clone();
                let inputTa = keys.next().unwrap().clone();
                let outputTa = keys.next().unwrap().clone();
                let userSwapBalanceState = keys.next().unwrap().clone();
                let systemProgram = keys.next().unwrap().clone();
                let rent = keys.next().unwrap().clone();
                let sysvarInstructions = keys.next().unwrap().clone();
                let remaining = keys.cloned().collect();
                let accounts = AssertUserSwapBalancesEndAccounts {
                    remaining,
                    maker,
                    inputTa,
                    outputTa,
                    userSwapBalanceState,
                    systemProgram,
                    rent,
                    sysvarInstructions,
                };
                return Ok(Instruction::AssertUserSwapBalancesEnd { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
pub mod events {
    use super::*;
    use crate::pubkey_serializer::pubkey_serde;
    use crate::pubkey_serializer::pubkey_serde_option;
    use borsh::BorshDeserialize;
    use serde::Serialize;
    pub use typedefs::*;
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct OrderDisplay {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub initial_input_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub expected_output_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub remaining_input_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub filled_output_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub tip_amount: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub number_of_fills: u64,
        pub order_type: u8,
        pub status: u8,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub last_updated_timestamp: u64,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UserSwapBalanceDiffs {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub user_lamports_before: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_ta_balance_before: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub output_ta_balance_before: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub user_lamports_after: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_ta_balance_after: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub output_ta_balance_after: u64,
        #[serde(with = "pubkey_serde")]
        pub swap_program: [u8; 32usize],
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub simulated_swap_amount_out: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub simulated_ts: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub minimum_amount_out: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub swap_amount_in: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub simulated_amount_out_next_best: u64,
        pub aggregator: u8,
        pub next_best_aggregator: u8,
    }
    #[derive(:: borsh :: BorshDeserialize, Debug, Serialize)]
    pub struct UserSwapBalances {
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub user_lamports: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub input_ta_balance: u64,
        #[serde(serialize_with = "crate::serialize_to_string")]
        pub output_ta_balance: u64,
    }
    #[derive(Debug, Serialize)]
    #[serde(tag = "event_type")]
    pub enum Event {
        OrderDisplay { args: OrderDisplay },
        UserSwapBalanceDiffs { args: UserSwapBalanceDiffs },
        UserSwapBalances { args: UserSwapBalances },
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
                [92u8, 101u8, 6u8, 158u8, 248u8, 152u8, 241u8, 60u8] => {
                    let mut rdr = &payload[..];
                    let args = OrderDisplay::deserialize(&mut rdr)?;
                    return Ok(Event::OrderDisplay { args });
                }
                [139u8, 203u8, 35u8, 31u8, 25u8, 8u8, 62u8, 143u8] => {
                    let mut rdr = &payload[..];
                    let args = UserSwapBalanceDiffs::deserialize(&mut rdr)?;
                    return Ok(Event::UserSwapBalanceDiffs { args });
                }
                [73u8, 107u8, 206u8, 225u8, 161u8, 59u8, 64u8, 15u8] => {
                    let mut rdr = &payload[..];
                    let args = UserSwapBalances::deserialize(&mut rdr)?;
                    return Ok(Event::UserSwapBalances { args });
                }
                _ => anyhow::bail!("Unknown event discriminator: {:?}", disc),
            }
        }
    }
}
