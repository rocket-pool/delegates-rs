#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Network {
    Mainnet,
}

pub fn network_path(network: Network) -> String {
    match network {
        Network::Mainnet => "mainnet",
    }
    .to_string()
}

pub fn network_from_string(network: String) -> Network {
    match network.as_str() {
        "mainnet" => Network::Mainnet,
        _ => Network::Mainnet,
    }
}

pub fn display_banner() -> &'static str {
    r"
/**
   *       .
   *      / \
   *     |.'.|
   *     |'.'|
   *   ,'|   |'.
   *  |,-'-|-'-.|
   *   __|_| |         _        _      _____           _
   *  | ___ \|        | |      | |    | ___ \         | |
   *  | |_/ /|__   ___| | _____| |_   | |_/ /__   ___ | |
   *  |    // _ \ / __| |/ / _ \ __|  |  __/ _ \ / _ \| |
   *  | |\ \ (_) | (__|   <  __/ |_   | | | (_) | (_) | |
   *  \_| \_\___/ \___|_|\_\___|\__|  \_|  \___/ \___/|_|
   * +---------------------------------------------------+
   * |    DECENTRALISED STAKING PROTOCOL FOR ETHEREUM    |
   * +---------------------------------------------------+
   *
   *  Rocket Pool is a first-of-its-kind Ethereum staking pool protocol, designed to
   *  be community-owned, decentralised, permissionless, & trustless.
   *
   *  For more information about Rocket Pool, visit https://rocketpool.net
   *
   *  Authored by the Rocket Pool Core Team
   *  A special thanks to the Rocket Pool community for all their contributions.
   *
   */
    "
}
