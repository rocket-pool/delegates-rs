use color_eyre::eyre::{Report, Result};
use dotenv::dotenv;
use ethers::prelude::U256;
use rocketpool_api::contracts::network::voting;
use rocketpool_api::utils;
use rocketpool_api::utils::{network_path, Network};
use std::fs::File;
use tokio::time::Instant;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "srv=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("{}", utils::display_banner());

    let block_number = std::env::var("BLOCK_NUMBER").expect("BLOCK_NUMBER not set");
    let block_u256 = U256::from_dec_str(&block_number).expect("Invalid block number format");
    tracing::info!("Capturing data for block {}", block_u256);

    capture_data(Network::Mainnet, block_u256).await?;
    Ok(())
}

pub async fn capture_data(network: Network, block: U256) -> Result<()> {
    let network_path = network_path(network);
    let start = Instant::now();
    let data = voting::get_delegates_list(network, block).await?;
    let filename = format!("{}-{}.json", network_path, block.to_string());
    let file = File::create(filename)?;
    serde_json::to_writer_pretty(file, &data)?;
    let duration = start.elapsed();
    tracing::info!(
        "Captured delegates data for {:?} on block {:?} in {:?}",
        network,
        block,
        duration,
    );
    Ok(())
}
