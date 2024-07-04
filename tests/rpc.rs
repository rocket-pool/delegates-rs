#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use ethers::providers::Middleware;
    use ethers::types::U64;
    use rocketpool_api::rpc::get_provider;
    use rocketpool_api::utils::Network;

    #[tokio::test]
    async fn test_get_mainnet_provider() {
        dotenv().ok();
        let provider = get_provider(Network::Mainnet).unwrap();
        let block = provider.get_block_number().await.unwrap();
        println!("block: {}", block);
        assert!(block > U64::from(0));
    }
}
