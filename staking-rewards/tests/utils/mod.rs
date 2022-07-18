use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(StakingRewards, "out/debug/staking-rewards-abi.json");

pub const ONE: u64 = 1_000_000_000;
pub const BASE_ASSET: AssetId = AssetId::new([0u8; 32]);

pub async fn get_balance(provider: &Provider, address: Address, asset: AssetId) -> u64 {
    let balance = provider.get_asset_balance(&address, asset).await.unwrap();
    balance
}

pub async fn setup(
    initial_stake: u64,
    initial_timestamp: u64,
) -> (StakingRewards, ContractId, LocalWallet) {
    // Launch a local network and deploy the contract

    let config = WalletsConfig::new_single(Some(1), Some(10000 * ONE));
    let wallet = &launch_custom_provider_and_get_wallets(config, None).await[0];

    let id = Contract::deploy(
        "./out/debug/staking-rewards.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/staking-rewards-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let staking_contract = StakingRewards::new(id.to_string(), wallet.clone());

    // Seed the contract with some reward tokens
    let seed_amount = 1000 * ONE;
    let _receipt = wallet
        .transfer(
            &Address::new(*id),
            seed_amount,
            BASE_ASSET,
            TxParameters::default(),
        )
        .await
        .unwrap();

    // Stake some tokens from the wallet
    let staking_call_params = CallParameters::new(Some(initial_stake), None, None);
    let _receipts = staking_contract
        .stake(initial_timestamp)
        .call_params(staking_call_params)
        .call()
        .await
        .unwrap();

    (staking_contract, id, wallet.clone())
}
