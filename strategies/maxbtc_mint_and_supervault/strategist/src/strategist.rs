use crate::strategy_config::Strategy;
use alloy::eips::BlockNumberOrTag::Finalized;
use alloy::eips::{BlockId, BlockNumberOrTag};
use alloy::network::primitives::BlockTransactionsKind;
use alloy::providers::Provider;
use async_trait::async_trait;
use log::info;
use packages::phases::VALENCE_WORKER;
use packages::types::sol_types::OneWayVault;
use packages::utils::wait_for_block_to_finalize;
use std::error::Error;
use tokio::time::sleep;
use valence_domain_clients::evm::base_client::EvmBaseClient;
use valence_domain_clients::evm::{
    base_client::CustomProvider, request_provider_client::RequestProviderClient,
};
use valence_strategist_utils::worker::ValenceWorker;

// implement the ValenceWorker trait for the Strategy struct.
// This trait defines the main loop of the strategy and inherits
// the default implementation for spawning the worker.
#[async_trait]
impl ValenceWorker for Strategy {
    fn get_name(&self) -> String {
        format!("Valence X-Vault: {}", self.label)
    }

    async fn cycle(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!(target: VALENCE_WORKER, "{}: Starting cycle...", self.get_name());

        // go into sentry (pre-flight) phase
        self.sentry().await?;

        let eth_rp: CustomProvider = self.eth_client.get_request_provider().await?;

        let one_way_vault_contract =
            OneWayVault::new(self.cfg.ethereum.libraries.one_way_vault, &eth_rp);

        let min_rate_update_delay = self
            .eth_client
            .query(one_way_vault_contract.config())
            .await?
            .minRateUpdateDelay;
        let last_rr_update_time = self
            .eth_client
            .query(one_way_vault_contract.lastRateUpdateTimestamp())
            .await?
            ._0;

        if last_rr_update_time + min_rate_update_delay
            <= std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                .as_secs()
        {
            info!("Rate update required");

            if !self.eth_client.query(one_way_vault_contract.vaultState()).await?.paused {
                info!("Pausing vault...");
                let pause_request = one_way_vault_contract.pause().into_transaction_request();
                let pause_vault_exec_response = self.eth_client.sign_and_send(pause_request).await?;
                eth_rp
                    .get_transaction_receipt(pause_vault_exec_response.transaction_hash)
                    .await?;
                info!("Vault paused");
            } else {
                info!("Vault is already paused, skipping pausing...");
            }

            let last_block_after_pause = self.eth_client.latest_block_height().await?;

            wait_for_block_to_finalize(last_block_after_pause, &eth_rp).await?;

            // first we carry out the deposit flow
            self.deposit(&eth_rp).await?;

            // after deposit flow is complete, we process the new obligations
            self.register_withdraw_obligations().await?;

            // with new obligations registered into the clearing queue, we
            // carry out the settlements
            self.settlement().await?;

            info!("Unpausing vault...");
            let unpause_request = one_way_vault_contract.unpause().into_transaction_request();
            let unpause_vault_exec_response =
                self.eth_client.sign_and_send(unpause_request).await?;
            eth_rp
                .get_transaction_receipt(unpause_vault_exec_response.transaction_hash)
                .await?;
            info!("Vault unpaused");

            // having processed all new exit requests after the deposit flow,
            // the epoch is ready to be concluded.
            // we perform the final accounting flow and post vault update.
            self.update(&eth_rp).await?;
        } else {
            info!("Rate update not required");
            // first we carry out the deposit flow
            self.deposit(&eth_rp).await?;

            // after deposit flow is complete, we process the new obligations
            self.register_withdraw_obligations().await?;

            // with new obligations registered into the clearing queue, we
            // carry out the settlements
            self.settlement().await?;
        }
        Ok(())
    }
}
