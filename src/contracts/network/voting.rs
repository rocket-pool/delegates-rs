use crate::contracts::node::manager::get_node_addresses;
use crate::{rpc, utils::Network};
use bigdecimal::BigDecimal;
use color_eyre::eyre::Result;
use ethers::contract::abigen;
use ethers::contract::Multicall;
use ethers::types::{Address, U256};
use ethers::utils::format_ether;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateList {
    pub address: Address,
    pub voting_power: String,
    pub delegates_to: Address,
    pub delegators: Vec<Delegators>,
    pub block: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delegators {
    pub address: Address,
    pub voting_power: String,
    pub block: String,
}

pub async fn get_voting_power(network: Network, address: Address, block: U256) -> Result<U256> {
    let address_from_var = std::env::var("ROCKET_NETWORK_VOTING_ADDRESS")
        .expect("ROCKET_NETWORK_VOTING_ADDRESS not set");
    let contract_address = Address::from_str(&address_from_var)?;
    abigen!(VOTING, "./src/contracts/abi/rocketnetworkvoting.json");
    let contract = VOTING::new(contract_address, rpc::get_provider(network)?);

    let vote_power = contract
        .method::<_, U256>("getVotingPower", (address, block))?
        .call()
        .await
        .unwrap();

    Ok(vote_power)
}

pub async fn get_current_delegate(network: Network, address: Address) -> Result<Address> {
    let address_from_var = std::env::var("ROCKET_NETWORK_VOTING_ADDRESS")
        .expect("ROCKET_NETWORK_VOTING_ADDRESS not set");
    let contract_address = Address::from_str(&address_from_var)?;
    abigen!(VOTING, "./src/contracts/abi/rocketnetworkvoting.json");
    let contract = VOTING::new(contract_address, rpc::get_provider(network)?);

    let current_delegate = contract
        .method::<_, Address>("getCurrentDelegate", address)?
        .call()
        .await
        .unwrap();

    Ok(current_delegate)
}

pub async fn get_delegate(network: Network, address: Address, block: U256) -> Result<Address> {
    let address_from_var = std::env::var("ROCKET_NETWORK_VOTING_ADDRESS")
        .expect("ROCKET_NETWORK_VOTING_ADDRESS not set");
    let contract_address = Address::from_str(&address_from_var)?;
    abigen!(VOTING, "./src/contracts/abi/rocketnetworkvoting.json");
    let contract = VOTING::new(contract_address, rpc::get_provider(network)?);

    let delegate_address = contract
        .method::<_, Address>("getDelegate", (address, block))?
        .call()
        .await
        .unwrap();

    Ok(delegate_address)
}

pub async fn get_delegates_list(network: Network, block: U256) -> Result<Vec<DelegateList>> {
    let client = rpc::get_provider(network)?;
    let mut multicall = Multicall::new(client.clone(), None).await?;

    let address_from_var = std::env::var("ROCKET_NETWORK_VOTING_ADDRESS")
        .expect("ROCKET_NETWORK_VOTING_ADDRESS not set");
    let contract_address = Address::from_str(&address_from_var)?;
    abigen!(VOTING, "./src/contracts/abi/rocketnetworkvoting.json");
    let contract = VOTING::new(contract_address, rpc::get_provider(network)?);

    let node_addresses = get_node_addresses(network).await?;

    let batches: Vec<_> = node_addresses
        .chunks(500)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut nodes: Vec<DelegateList> = Vec::new();

    for (_i, batch) in batches.iter().enumerate() {
        for address in batch {
            multicall.add_call(
                contract
                    .clone()
                    .method::<_, U256>("getVotingPower", (Address::from(*address), block))?,
                false,
            );
        }
        let voting_power: Vec<U256> = multicall.call_array().await.unwrap();
        multicall.clear_calls();

        for address in batch {
            multicall.add_call(
                contract
                    .clone()
                    .method::<_, Address>("getDelegate", (Address::from(*address), block))?,
                false,
            );
        }
        let delegates_to: Vec<Address> = multicall.call_array().await.unwrap();
        multicall.clear_calls();

        for (index, value) in batch.iter().enumerate() {
            nodes.push(DelegateList {
                address: Address::from(*value),
                voting_power: format_ether(voting_power[index]),
                delegates_to: delegates_to[index],
                delegators: Vec::new(),
                block: block.to_string(),
            })
        }
        multicall.clear_calls();
    }

    let mut delegate_map: HashMap<Address, DelegateList> = HashMap::new();
    for node in nodes.clone() {
        delegate_map.insert(node.address.clone(), node);
    }

    for (_index, value) in nodes.clone().iter().enumerate() {
        if value.delegates_to != value.address && value.delegates_to != Address::zero() {
            let delegator = Delegators {
                address: value.address.clone(),
                voting_power: value.voting_power.clone(),
                block: value.block.clone(),
            };

            let delegate = delegate_map.get_mut(&value.delegates_to).unwrap();
            let node_address_voting_power = BigDecimal::from_str(&value.voting_power).unwrap();
            let delegate_voting_power =
                BigDecimal::from_str(&delegate.voting_power.to_string()).unwrap();

            let voting_power = delegate_voting_power
                .clone()
                .add(node_address_voting_power.clone());
            delegate.voting_power = voting_power.to_string();
            delegate.delegators.push(delegator);
        }
    }

    let updated_delegate_list: Vec<DelegateList> = delegate_map.values().cloned().collect();

    Ok(updated_delegate_list)
}
