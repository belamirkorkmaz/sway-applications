use super::abi::{Exchange, AMM};
use fuels::prelude::*;
use std::collections::HashMap;

pub struct AMMContract {
    pub id: ContractId,
    pub instance: AMM,
    pub pools: HashMap<(AssetId, AssetId), ExchangeContract>,
}

pub struct ExchangeContract {
    pub bytecode_root: Option<ContractId>,
    pub id: ContractId,
    pub instance: Exchange,
    pub pair: (AssetId, AssetId),
}

pub struct ExchangeContractConfiguration {
    pub compute_bytecode_root: bool,
    pub malicious: bool,
    pub salt: [u8; 32],
}

impl Default for ExchangeContractConfiguration {
    fn default() -> Self {
        Self {
            compute_bytecode_root: false,
            malicious: false,
            salt: [0u8; 32],
        }
    }
}

impl ExchangeContractConfiguration {
    pub fn new(
        compute_bytecode_root: Option<bool>,
        malicious: Option<bool>,
        salt: Option<[u8; 32]>,
    ) -> Self {
        Self {
            compute_bytecode_root: compute_bytecode_root.unwrap_or_default(),
            malicious: malicious.unwrap_or_default(),
            salt: salt.unwrap_or_default(),
        }
    }
}

pub struct LiquidityParameters {
    pub amounts: (u64, u64),
    pub deadline: u64,
    pub liquidity: u64,
}

impl Default for LiquidityParameters {
    fn default() -> Self {
        Self {
            amounts: (100, 400),
            deadline: 1000,
            liquidity: 200,
        }
    }
}

impl LiquidityParameters {
    pub fn new(amounts: Option<(u64, u64)>, deadline: Option<u64>, liquidity: Option<u64>) -> Self {
        Self {
            amounts: amounts.unwrap_or((100, 400)),
            deadline: deadline.unwrap_or(1000),
            liquidity: liquidity.unwrap_or(200),
        }
    }
}

pub struct WalletAssetParameters {
    pub num_assets: u64,
    pub coins_per_asset: u64,
    pub amount_per_coin: u64,
}
