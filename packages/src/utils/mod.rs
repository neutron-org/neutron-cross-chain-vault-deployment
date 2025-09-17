use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;
use alloy::eips::BlockId;
use alloy::eips::BlockNumberOrTag::Finalized;
use alloy::network::primitives::BlockTransactionsKind;
use alloy::providers::Provider;
use cosmwasm_std::Uint128;
use log::info;
use tokio::time::sleep;
use valence_domain_clients::coprocessor::base_client::{Base64, Proof};
use valence_domain_clients::evm::base_client::CustomProvider;

pub mod crypto_provider;
pub mod logging;
pub mod mars;
pub mod maxbtc;
pub mod obligation;
pub mod skip;
pub mod supervaults;
pub mod valence_core;

/// Decodes the base64 bytes of the proof and public inputs.
pub fn decode(a: Proof) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let proof = Base64::decode(&a.proof)?;
    let inputs = Base64::decode(&a.inputs)?;

    Ok((proof, inputs))
}

pub async fn wait_for_block_to_finalize(block_number: u64, eth_rp: &CustomProvider) -> Result<(), Box<dyn Error + Send + Sync>> {
    loop {
        info!("Waiting for block {} to be finalized...", block_number);
        if let Some(block) = eth_rp.get_block(BlockId::Number(Finalized), BlockTransactionsKind::Hashes).await? {
            if block.header.number >= block_number {
                sleep(std::time::Duration::from_secs(60)).await;
                return Ok(());
            }
        }
        sleep(std::time::Duration::from_secs(10)).await;
    }
}


pub async fn lock_transfer(label:&str, name: &str, amount: u128) -> anyhow::Result<()> {
    info!("locking transfer for {} {}", label, name);
    let mut file = File::create(format!("{}_{}_transfer", label, name))?;
    file.write_all(amount.to_string().as_bytes())
        .expect(
            "failed to write to file",
        );
    Ok(())
}

pub async fn unlock_transfer(label:&str, name: &str) -> anyhow::Result<()> {
    info!("unlocking transfer for {} {}", label, name);
    std::fs::remove_file(format!("{}_{}_transfer", label, name))?;
    Ok(())
}

pub async fn check_transfer_lock(label:&str, name: &str) -> Option<Uint128> {
    info!("checking transfer lock for {} {}", label, name);
    if let Ok(mut file) = File::open(format!("{}_{}_transfer", label, name)) {
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect(
                "failed to read file",
            );

        let amount_to_wait_for = Uint128::from_str(&buf.strip_suffix("\n").unwrap()).unwrap();

        return Some(amount_to_wait_for);
    }

    None
}