use crate::utils;
use color_eyre::eyre::{Report, Result};
use ethers::providers::{Http, Provider};
use std::sync::Arc;

fn get_rpc_provider(network: utils::Network) -> Result<String, Report> {
    let provider = match network {
        utils::Network::Mainnet => std::env::var("RPC").expect("RPC not set"),
    };
    Ok(provider)
}

pub fn get_provider(network: utils::Network) -> Result<Arc<Provider<Http>>, Report> {
    let rpc = get_rpc_provider(network)?;
    let provider = Provider::<Http>::try_from(rpc)?;
    let client = Arc::new(provider);
    Ok(client)
}
