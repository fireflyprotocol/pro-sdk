#[derive(Clone, Copy)]
pub enum Environment {
    Testnet,
    Mainnet,
}

pub mod test {
    pub mod market {
        pub const ETH_SYMBOL: &str = "ETH-PERP";
        pub const BTC_SYMBOL: &str = "BTC-PERP";
    }

    pub mod account {
        pub const ADDRESS: &str =
            "0x7fdaa9cd7b4088a312306c03b0d2ef5766a6cc721c13e90ded406df5e1b18cf8";
        pub const PUBLIC_KEY: &str =
            "871bffd9203d138e2dbbb5a57402e2014d0ba23ede6767dcf57884f78a67e5a4";
        pub const PRIVATE_KEY: &str =
            "cf1a072429be1443c5da59789b58fe8c8432eec5c277b3dc20f59934c1b1cce7";
    }
}

pub mod auth {
    pub mod testnet {
        pub const AUDIENCE: &str = "api";
        pub const URL: &str = "https://auth.api.sui-staging.bluefin.io";
    }

    pub mod mainnet {
        pub const AUDIENCE: &str = "api";
        pub const URL: &str = "https://auth.api.sui-prod.bluefin.io";
    }
}

pub mod trade {
    pub const AUTH_TOKEN_PREFIX: &str = "Bearer";

    pub mod testnet {
        pub const URL: &str = "https://trade.api.sui-staging.bluefin.io";
    }

    pub mod mainnet {
        pub const URL: &str = "https://trade.api.sui-prod.bluefin.io";
    }
}

pub mod exchange {
    pub mod testnet {
        pub const URL: &str = "https://api.sui-staging.bluefin.io";
    }

    pub mod mainnet {
        pub const URL: &str = "https://api.sui-prod.bluefin.io";
    }

    pub mod info {
        use crate::{exchange, Environment};
        use bluefin_api::apis::configuration::Configuration;
        use bluefin_api::models::ContractsConfig;

        type Error = Box<dyn std::error::Error>;
        type Result<T> = std::result::Result<T, Error>;

        pub async fn get_contracts_config(environment: Environment) -> Result<ContractsConfig> {
            bluefin_api::apis::exchange_api::get_exchange_info(&Configuration {
                base_path: match environment {
                    Environment::Testnet => exchange::testnet::URL.into(),
                    Environment::Mainnet => exchange::mainnet::URL.into(),
                },
                ..Configuration::default()
            })
            .await
            .map_err(|error| error.to_string())?
            .contracts_config
            .ok_or("No Contracts Config found".into())
        }
    }
}

pub mod account {
    pub const AUTH_TOKEN_PREFIX: &str = "Bearer";
    pub mod testnet {
        pub const URL: &str = "https://api.sui-staging.bluefin.io";
    }

    pub mod mainnet {
        pub const URL: &str = "https://api.sui-prod.bluefin.io";
    }
}

pub mod websocket {
    pub mod account {
        pub mod testnet {
            pub const URL: &str = "wss://stream.api.sui-staging.bluefin.io/ws/account";
        }

        pub mod mainnet {
            pub const URL: &str = "wss://stream.api.sui-prod.bluefin.io/ws/account";
        }
    }

    pub mod market {
        pub mod testnet {
            pub const URL: &str = "wss://stream.api.sui-staging.bluefin.io/ws/market";
        }

        pub mod mainnet {
            pub const URL: &str = "wss://stream.api.sui-prod.bluefin.io/ws/market";
        }
    }
}
