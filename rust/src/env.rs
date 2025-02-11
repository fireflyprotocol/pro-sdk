#[derive(Clone, Copy)]
pub enum Environment {
    Devnet,
    Testnet,
    Mainnet,
}

pub mod symbols {
    pub mod perps {
        pub const ETH: &str = "ETH-PERP";
        pub const BTC: &str = "BTC-PERP";
        pub const SUI: &str = "SUI-PERP";
    }

    pub mod assets {
        pub const USDC: &str = "USDC";
    }
}

pub mod test {
    pub mod account {
        pub mod devnet {
            pub const ADDRESS: &str =
                "0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786";
            pub const PUBLIC_KEY: &str =
                "989e35904229360dfbf53a9277db36d401f597c4c0a693acc286bf439269a5cb";
            pub const PRIVATE_KEY: &str =
                "3427d19dcf5781f0874c36c78aec22c03acda435d69efcbf249e8821793567a1";
        }
        pub mod testnet {
            pub const ADDRESS: &str =
                "0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786";
            pub const PUBLIC_KEY: &str =
                "989e35904229360dfbf53a9277db36d401f597c4c0a693acc286bf439269a5cb";
            pub const PRIVATE_KEY: &str =
                "3427d19dcf5781f0874c36c78aec22c03acda435d69efcbf249e8821793567a1";
        }
    }
}

pub mod auth {
    use super::Environment;

    pub mod devnet {
        pub const AUDIENCE: &str = "api";
        pub const URL: &str = "https://auth.api.sui-dev.bluefin.io";
    }

    pub mod testnet {
        pub const AUDIENCE: &str = "api";
        pub const URL: &str = "https://auth.api.sui-staging.bluefin.io";
    }

    pub mod mainnet {
        pub const AUDIENCE: &str = "api";
        pub const URL: &str = "https://auth.api.sui-prod.bluefin.io";
    }

    pub fn get_url<'a>(environment: Environment) -> &'a str {
        match environment {
            Environment::Devnet => devnet::URL,
            Environment::Testnet => testnet::URL,
            Environment::Mainnet => mainnet::URL,
        }
    }
}

pub mod trade {
    use super::Environment;

    pub const AUTH_TOKEN_PREFIX: &str = "Bearer";

    pub mod devnet {
        pub const URL: &str = "https://trade.api.sui-dev.bluefin.io";
    }

    pub mod testnet {
        pub const URL: &str = "https://trade.api.sui-staging.bluefin.io";
    }

    pub mod mainnet {
        pub const URL: &str = "https://trade.api.sui-prod.bluefin.io";
    }

    pub fn get_url<'a>(environment: Environment) -> &'a str {
        match environment {
            Environment::Devnet => devnet::URL,
            Environment::Testnet => testnet::URL,
            Environment::Mainnet => mainnet::URL,
        }
    }
}

pub mod exchange {
    use super::Environment;

    pub mod devnet {
        pub const URL: &str = "https://api.sui-dev.bluefin.io";
    }

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

        /// # Errors
        ///
        /// Will return [`Err`] if client/server communication fails.
        pub async fn get_contracts_config(environment: Environment) -> Result<ContractsConfig> {
            bluefin_api::apis::exchange_api::get_exchange_info(&Configuration {
                base_path: match environment {
                    Environment::Devnet => exchange::devnet::URL.into(),
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

    pub fn get_url<'a>(environment: Environment) -> &'a str {
        match environment {
            Environment::Devnet => devnet::URL,
            Environment::Testnet => testnet::URL,
            Environment::Mainnet => mainnet::URL,
        }
    }
}

pub mod account {
    use super::Environment;

    pub const AUTH_TOKEN_PREFIX: &str = "Bearer";
    pub mod devnet {
        pub const URL: &str = "https://api.sui-dev.bluefin.io";
    }

    pub mod testnet {
        pub const URL: &str = "https://api.sui-staging.bluefin.io";
    }

    pub mod mainnet {
        pub const URL: &str = "https://api.sui-prod.bluefin.io";
    }

    pub fn get_url<'a>(environment: Environment) -> &'a str {
        match environment {
            Environment::Devnet => devnet::URL,
            Environment::Testnet => testnet::URL,
            Environment::Mainnet => mainnet::URL,
        }
    }
}

pub mod websocket {
    pub mod account {
        use crate::Environment;

        pub mod devnet {
            pub const URL: &str = "wss://stream.api.sui-dev.bluefin.io/ws/account";
        }

        pub mod testnet {
            pub const URL: &str = "wss://stream.api.sui-staging.bluefin.io/ws/account";
        }

        pub mod mainnet {
            pub const URL: &str = "wss://stream.api.sui-prod.bluefin.io/ws/account";
        }

        pub fn get_url<'a>(environment: Environment) -> &'a str {
            match environment {
                Environment::Devnet => devnet::URL,
                Environment::Testnet => testnet::URL,
                Environment::Mainnet => mainnet::URL,
            }
        }
    }

    pub mod market {
        use crate::Environment;

        pub mod devnet {
            pub const URL: &str = "wss://stream.api.sui-dev.bluefin.io/ws/market";
        }

        pub mod testnet {
            pub const URL: &str = "wss://stream.api.sui-staging.bluefin.io/ws/market";
        }

        pub mod mainnet {
            pub const URL: &str = "wss://stream.api.sui-prod.bluefin.io/ws/market";
        }

        pub fn get_url<'a>(environment: Environment) -> &'a str {
            match environment {
                Environment::Devnet => devnet::URL,
                Environment::Testnet => testnet::URL,
                Environment::Mainnet => mainnet::URL,
            }
        }
    }
}
