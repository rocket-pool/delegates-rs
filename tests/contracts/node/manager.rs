#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use ethers::types::U256;
    use rocketpool_api::contracts::node::manager;
    use rocketpool_api::utils::Network;

    #[tokio::test]
    async fn test_get_node_count() {
        dotenv().ok();
        let network = Network::Mainnet;
        let node_count = manager::get_node_count(network).await.unwrap();
        println!("node_count: {}", node_count);
        assert!(node_count > U256::from(0));
    }

    #[tokio::test]
    async fn test_get_node_addresses() {
        dotenv().ok();
        let network = Network::Mainnet;
        let node_addresses = manager::get_node_addresses(network).await.unwrap();
        println!("node_addresses: {:?}", node_addresses);
        assert!(node_addresses.len() > 0);
    }
}
