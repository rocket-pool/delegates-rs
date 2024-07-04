#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use ethers::types::{Address, U256};
    use rocketpool_api::contracts::network::voting;
    use rocketpool_api::utils::Network;

    #[tokio::test]
    async fn test_get_voting_power() {
        dotenv().ok();
        let network = Network::Mainnet;

        let address = "0xC76DCF5ed10555C417d3c85aF987F2b7D2f63415"
            .parse::<Address>()
            .unwrap();

        let block_number = std::env::var("BLOCK_NUMBER").expect("BLOCK_NUMBER not set");
        let block = U256::from_dec_str(&block_number).expect("Invalid block number format");

        let voting_power = voting::get_voting_power(network, address, block)
            .await
            .unwrap();
        println!("voting_power: {}", voting_power);
        assert!(voting_power > U256::from(0));
    }

    #[tokio::test]
    async fn test_get_current_delegate() {
        dotenv().ok();
        let network = Network::Mainnet;

        let address = "0xc942b5aa63a3410a13358a7a3aedf33d9e9d3ac3"
            .parse::<Address>()
            .unwrap();

        let actual_delegate_address = "0xc76dcf5ed10555c417d3c85af987f2b7d2f63415"
            .parse::<Address>()
            .unwrap();

        let current_delegate = voting::get_current_delegate(network, address)
            .await
            .unwrap();
        println!("current delegate: {:?}", current_delegate);
        assert!(current_delegate == actual_delegate_address);
    }

    #[tokio::test]
    async fn test_get_delegate() {
        dotenv().ok();
        let network = Network::Mainnet;

        let address = "0xc76dcf5ed10555c417d3c85af987f2b7d2f63415"
            .parse::<Address>()
            .unwrap();

        let actual_delegate_address = "0xc76dcf5ed10555c417d3c85af987f2b7d2f63415"
            .parse::<Address>()
            .unwrap();

        let block_number = std::env::var("BLOCK_NUMBER").expect("BLOCK_NUMBER not set");
        let block = U256::from_dec_str(&block_number).expect("Invalid block number format");

        let delegate_address = voting::get_delegate(network, address, block).await.unwrap();
        println!("delegate address: {:?}", delegate_address);
        assert!(delegate_address == actual_delegate_address);
    }

    #[tokio::test]
    async fn test_get_delegates_list() {
        dotenv().ok();
        let network = Network::Mainnet;

        let block_number = std::env::var("BLOCK_NUMBER").expect("BLOCK_NUMBER not set");
        let block = U256::from_dec_str(&block_number).expect("Invalid block number format");

        let delegates_list = voting::get_delegates_list(network, block).await.unwrap();
        println!("delegates list: {:?}", delegates_list);
        assert!(delegates_list.len() > 0);
    }
}
