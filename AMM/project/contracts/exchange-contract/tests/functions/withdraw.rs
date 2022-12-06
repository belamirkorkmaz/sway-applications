use crate::utils::{setup, setup_and_construct};
use fuels::prelude::*;
use test_utils::abi::exchange::{balance, deposit, withdraw};

mod success {
    use super::*;

    #[tokio::test]
    async fn withdraws_entire_deposit_of_asset_a() {
        let (exchange, wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;
        let withdraw_amount = deposit_amount;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(deposit_amount), Some(exchange.pair.0), None),
        )
        .await;

        let initial_contract_balance = balance(&exchange.instance, exchange.pair.0).await.value;
        let initial_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        withdraw(&exchange.instance, deposit_amount, exchange.pair.0).await;

        let final_contract_balance = balance(&exchange.instance, exchange.pair.0).await.value;
        let final_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        assert_eq!(
            final_contract_balance,
            initial_contract_balance - withdraw_amount
        );
        assert_eq!(
            final_wallet_balance,
            initial_wallet_balance + withdraw_amount
        );
    }

    #[tokio::test]
    async fn withdraws_asset_a_partially() {
        let (exchange, wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;
        let withdraw_amount = 50;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(deposit_amount), Some(exchange.pair.0), None),
        )
        .await;

        let initial_contract_balance = balance(&exchange.instance, exchange.pair.0).await.value;
        let initial_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        withdraw(&exchange.instance, withdraw_amount, exchange.pair.0).await;

        let final_contract_balance = balance(&exchange.instance, exchange.pair.0).await.value;
        let final_wallet_balance = wallet.get_asset_balance(&exchange.pair.0).await.unwrap();

        assert_eq!(
            final_contract_balance,
            initial_contract_balance - withdraw_amount
        );
        assert_eq!(
            final_wallet_balance,
            initial_wallet_balance + withdraw_amount
        );
    }
}

mod revert {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"NotInitialized\"")]
    async fn on_unitialized() {
        // call setup instead of setup_and_construct
        let (exchange_instance, _wallet, _pool_asset_id, asset_a_id, _asset_b_id, _asset_c_id) =
            setup().await;

        withdraw(&exchange_instance, 0, asset_a_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"InvalidAsset\"")]
    async fn on_invalid_asset() {
        let (exchange, _wallet, _liquidity_parameters, asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(deposit_amount), Some(exchange.pair.0), None),
        )
        .await;

        // sending invalid asset
        withdraw(&exchange.instance, 0, asset_c_id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "RevertTransactionError(\"DesiredAmountTooHigh(101)\"")]
    async fn on_withdraw_more_than_deposited() {
        let (exchange, _wallet, _liquidity_parameters, _asset_c_id) =
            setup_and_construct(false, false).await;
        let deposit_amount = 100;

        deposit(
            &exchange.instance,
            CallParameters::new(Some(deposit_amount), Some(exchange.pair.0), None),
        )
        .await;

        // attempting to withdraw more than deposit amount
        withdraw(&exchange.instance, deposit_amount + 1, exchange.pair.0).await;
    }
}
