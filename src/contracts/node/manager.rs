use crate::{rpc, utils::Network};
use color_eyre::eyre::Result;
use ethers::contract::{abigen, Multicall};
use ethers::types::{Address, U256};
use std::str::FromStr;

pub async fn get_node_count(network: Network) -> Result<U256> {
    let address_from_var =
        std::env::var("ROCKET_NODE_MANAGER_ADDRESS").expect("ROCKET_NODE_MANAGER_ADDRESS not set");
    let contract_address = Address::from_str(&address_from_var)?;

    abigen!(MANAGER, "./src/contracts/abi/rocketnodemanager.json");
    let contract = MANAGER::new(contract_address, rpc::get_provider(network)?);

    let node_count = contract
        .method::<_, U256>("getNodeCount", ())?
        .call()
        .await
        .unwrap();

    Ok(node_count)
}

pub async fn get_node_addresses(network: Network) -> Result<Vec<Address>> {
    let client = rpc::get_provider(network)?;
    let mut multicall = Multicall::new(client.clone(), None).await?;

    let address_from_var =
        std::env::var("ROCKET_NODE_MANAGER_ADDRESS").expect("ROCKET_NODE_MANAGER_ADDRESS not set");
    let contract_address = Address::from_str(&address_from_var)?;
    abigen!(MANAGER, "./src/contracts/abi/rocketnodemanager.json");
    let contract = MANAGER::new(contract_address, rpc::get_provider(network)?);

    let node_count = get_node_count(network).await?;

    let mut node_addresses: Vec<Address> = Vec::new();
    let indices: Vec<_> = (0..node_count.as_u64()).collect();
    for batches in indices.chunks(500) {
        for batch in batches {
            multicall.add_call(
                contract
                    .clone()
                    .method::<_, Address>("getNodeAt", U256::from(*batch))?,
                false,
            );
        }
        let node_addresses_batch: Vec<Address> = multicall.call_array().await.unwrap();
        node_addresses.extend(node_addresses_batch);
        multicall.clear_calls();
    }

    Ok(node_addresses)
}
