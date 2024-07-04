#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use rocketpool_api::utils::Network;

    #[tokio::test]
    async fn test_network_path() {
        dotenv().ok();
        let network = Network::Mainnet;
        let path = rocketpool_api::utils::network_path(network);
        assert_eq!(path, "mainnet");
    }

    #[tokio::test]
    async fn test_network_from_string() {
        dotenv().ok();
        let network = rocketpool_api::utils::network_from_string("mainnet".to_string());
        assert_eq!(network, Network::Mainnet);
    }

    #[tokio::test]
    async fn test_display_banner() {
        dotenv().ok();
        let banner = rocketpool_api::utils::display_banner();
        assert!(banner.contains("DECENTRALISED STAKING PROTOCOL FOR ETHEREUM"));
    }
}
