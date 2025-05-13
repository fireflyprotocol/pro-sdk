#[derive(Clone, Copy)]
pub enum Environment {
    Dev,
    Staging,
    Production,

    // Deprecated aliases
    #[deprecated(note = "Use Dev instead")]
    Devnet,
    #[deprecated(note = "Use Staging instead")]
    Testnet,
    #[deprecated(note = "Use Production instead")]
    Mainnet,
}

#[deprecated(note = "Will not be updated anymore.")]
pub mod symbols {
    #[deprecated(note = "Use exchange::info::markets() function instead")]
    pub mod perps {
        pub const ETH: &str = "ETH-PERP";
        pub const BTC: &str = "BTC-PERP";
        pub const SUI: &str = "SUI-PERP";
    }

    #[deprecated(note = "Use exchange::info::assets() function instead")]
    pub mod assets {
        pub const USDC: &str = "USDC";
    }
}

pub mod test {
    pub mod account {
        use crate::env::Environment;

        pub fn address<'a>(environment: Environment) -> &'a str {
            match environment {
                Environment::Dev | Environment::Devnet => dev::ADDRESS,
                Environment::Staging | Environment::Testnet => staging::ADDRESS,
                Environment::Production | Environment::Mainnet => {
                    unimplemented!("test address are not available in production")
                }
            }
        }

        pub fn public_key<'a>(environment: Environment) -> &'a str {
            match environment {
                Environment::Dev | Environment::Devnet => dev::PUBLIC_KEY,
                Environment::Staging | Environment::Testnet => staging::PUBLIC_KEY,
                Environment::Production | Environment::Mainnet => {
                    unimplemented!("test public key are not available in production")
                }
            }
        }

        pub fn private_key<'a>(environment: Environment) -> &'a str {
            match environment {
                Environment::Dev | Environment::Devnet => dev::PRIVATE_KEY,
                Environment::Staging | Environment::Testnet => staging::PRIVATE_KEY,
                Environment::Production | Environment::Mainnet => {
                    unimplemented!("test private key are not available in production")
                }
            }
        }

        #[deprecated(note = "use address(), public_key(), private_key() instead")]
        pub mod dev {
            pub const ADDRESS: &str =
                "0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786";
            pub const PUBLIC_KEY: &str =
                "989e35904229360dfbf53a9277db36d401f597c4c0a693acc286bf439269a5cb";
            pub const PRIVATE_KEY: &str =
                "3427d19dcf5781f0874c36c78aec22c03acda435d69efcbf249e8821793567a1";
        }

        #[deprecated(note = "use address(), public_key(), private_key() instead")]
        pub mod staging {
            pub const ADDRESS: &str =
                "0x9b11fafc580f23932f379d99ab6cc4c638e85ba4c252fc909296f3f9e6cea786";
            pub const PUBLIC_KEY: &str =
                "989e35904229360dfbf53a9277db36d401f597c4c0a693acc286bf439269a5cb";
            pub const PRIVATE_KEY: &str =
                "3427d19dcf5781f0874c36c78aec22c03acda435d69efcbf249e8821793567a1";
        }

        #[deprecated(note = "Use dev instead")]
        pub mod devnet {
            pub use super::dev::*;
        }
        #[deprecated(note = "Use staging instead")]
        pub mod testnet {
            pub use super::staging::*;
        }
    }
}

pub mod auth {
    use super::Environment;

    pub mod dev {
        pub const AUDIENCE: &str = "api";
        pub const URL: &str = "https://auth.api.sui-dev.bluefin.io";
    }

    pub mod staging {
        pub const AUDIENCE: &str = "api";
        pub const URL: &str = "https://auth.api.sui-staging.bluefin.io";
    }

    pub mod production {
        pub const AUDIENCE: &str = "api";
        pub const URL: &str = "https://auth.api.sui-prod.bluefin.io";
    }

    #[deprecated(note = "Use dev instead")]
    pub mod devnet {
        pub use super::dev::*;
    }
    #[deprecated(note = "Use staging instead")]
    pub mod testnet {
        pub use super::staging::*;
    }
    #[deprecated(note = "Use production instead")]
    pub mod mainnet {
        pub use super::production::*;
    }

    pub fn url<'a>(environment: Environment) -> &'a str {
        match environment {
            Environment::Dev | Environment::Devnet => dev::URL,
            Environment::Staging | Environment::Testnet => staging::URL,
            Environment::Production | Environment::Mainnet => production::URL,
        }
    }

    pub fn audience<'a>(environment: Environment) -> &'a str {
        match environment {
            Environment::Dev | Environment::Devnet => dev::AUDIENCE,
            Environment::Staging | Environment::Testnet => staging::AUDIENCE,
            Environment::Production | Environment::Mainnet => production::AUDIENCE,
        }
    }
}

pub mod trade {
    use super::Environment;

    pub const AUTH_TOKEN_PREFIX: &str = "Bearer";

    pub mod dev {
        pub const URL: &str = "https://trade.api.sui-dev.bluefin.io";
        pub const COLOCATED_URL: &str = "http://api.coloc.sui-dev.int.bluefin.io:9090";
    }

    pub mod staging {
        pub const URL: &str = "https://trade.api.sui-staging.bluefin.io";
        pub const COLOCATED_URL: &str = "http://api.coloc.sui-staging.int.bluefin.io:9090";
    }

    pub mod production {
        pub const URL: &str = "https://trade.api.sui-prod.bluefin.io";
        pub const COLOCATED_URL: &str = "http://api.coloc.sui-prod.int.bluefin.io:9090";
    }

    #[deprecated(note = "Use dev instead")]
    pub mod devnet {
        pub use super::dev::*;
    }
    #[deprecated(note = "Use staging instead")]
    pub mod testnet {
        pub use super::staging::*;
    }
    #[deprecated(note = "Use production instead")]
    pub mod mainnet {
        pub use super::production::*;
    }

    pub fn url<'a>(environment: Environment) -> &'a str {
        match environment {
            Environment::Dev | Environment::Devnet => dev::URL,
            Environment::Staging | Environment::Testnet => staging::URL,
            Environment::Production | Environment::Mainnet => production::URL,
        }
    }
}

pub mod exchange {
    use super::Environment;

    pub mod dev {
        pub const URL: &str = "https://api.sui-dev.bluefin.io";
    }

    pub mod staging {
        pub const URL: &str = "https://api.sui-staging.bluefin.io";
    }

    pub mod production {
        pub const URL: &str = "https://api.sui-prod.bluefin.io";
    }

    #[deprecated(note = "Use dev instead")]
    pub mod devnet {
        pub use super::dev::*;
    }
    #[deprecated(note = "Use staging instead")]
    pub mod testnet {
        pub use super::staging::*;
    }
    #[deprecated(note = "Use production instead")]
    pub mod mainnet {
        pub use super::production::*;
    }

    pub mod info {
        use super::*;
        use bluefin_api::apis::configuration::Configuration;
        use bluefin_api::models::{AssetConfig, ContractsConfig, Market};

        type Error = Box<dyn std::error::Error>;
        type Result<T> = std::result::Result<T, Error>;

        /// Returns the contracts config.
        ///
        /// # Errors
        ///
        /// Will return [`Err`] if client/server communication fails.
        pub async fn contracts_config(environment: Environment) -> Result<ContractsConfig> {
            bluefin_api::apis::exchange_api::get_exchange_info(&Configuration {
                base_path: super::url(environment).to_string(),
                ..Configuration::default()
            })
            .await?
            .contracts_config
            .ok_or("No Contracts Config found".into())
        }

        /// Returns a list of all markets.
        ///
        /// # Errors
        ///
        /// Will return [`Err`] if client/server communication fails.
        pub async fn markets(environment: Environment) -> Result<Vec<Market>> {
            bluefin_api::apis::exchange_api::get_exchange_info(&Configuration {
                base_path: super::url(environment).to_string(),
                ..Configuration::default()
            })
            .await
            .map(|response| response.markets)
            .map_err(Error::from)
        }

        /// Returns a list of all assets.
        ///
        /// # Errors
        ///
        /// Will return [`Err`] if client/server communication fails.
        pub async fn assets(environment: Environment) -> Result<Vec<AssetConfig>> {
            bluefin_api::apis::exchange_api::get_exchange_info(&Configuration {
                base_path: super::url(environment).to_string(),
                ..Configuration::default()
            })
            .await
            .map(|response| response.assets)
            .map_err(Error::from)
        }
    }

    pub fn url<'a>(environment: Environment) -> &'a str {
        match environment {
            Environment::Dev | Environment::Devnet => dev::URL,
            Environment::Staging | Environment::Testnet => staging::URL,
            Environment::Production | Environment::Mainnet => production::URL,
        }
    }
}

pub mod account {
    use super::Environment;

    pub const AUTH_TOKEN_PREFIX: &str = "Bearer";

    pub mod dev {
        pub const URL: &str = "https://api.sui-dev.bluefin.io";
    }

    pub mod staging {
        pub const URL: &str = "https://api.sui-staging.bluefin.io";
    }

    pub mod production {
        pub const URL: &str = "https://api.sui-prod.bluefin.io";
    }

    #[deprecated(note = "Use dev instead")]
    pub mod devnet {
        pub use super::dev::*;
    }
    #[deprecated(note = "Use staging instead")]
    pub mod testnet {
        pub use super::staging::*;
    }
    #[deprecated(note = "Use production instead")]
    pub mod mainnet {
        pub use super::production::*;
    }

    pub fn url<'a>(environment: Environment) -> &'a str {
        match environment {
            Environment::Dev | Environment::Devnet => dev::URL,
            Environment::Staging | Environment::Testnet => staging::URL,
            Environment::Production | Environment::Mainnet => production::URL,
        }
    }
}

pub mod websocket {
    use super::Environment;

    pub mod account {
        use super::Environment;

        pub mod dev {
            pub const URL: &str = "wss://stream.api.sui-dev.bluefin.io/ws/account";
            pub const COLOCATED_URL: &str = "ws://api.coloc.sui-dev.int.bluefin.io:9091/ws/account";
        }

        pub mod staging {
            pub const URL: &str = "wss://stream.api.sui-staging.bluefin.io/ws/account";
            pub const COLOCATED_URL: &str =
                "ws://api.coloc.sui-staging.int.bluefin.io:9091/ws/account";
        }

        pub mod production {
            pub const URL: &str = "wss://stream.api.sui-prod.bluefin.io/ws/account";
            pub const COLOCATED_URL: &str =
                "ws://api.coloc.sui-prod.int.bluefin.io:9091/ws/account";
        }

        #[deprecated(note = "Use dev instead")]
        pub mod devnet {
            pub use super::dev::*;
        }
        #[deprecated(note = "Use staging instead")]
        pub mod testnet {
            pub use super::staging::*;
        }
        #[deprecated(note = "Use production instead")]
        pub mod mainnet {
            pub use super::production::*;
        }

        pub fn url<'a>(environment: Environment) -> &'a str {
            match environment {
                Environment::Dev | Environment::Devnet => dev::URL,
                Environment::Staging | Environment::Testnet => staging::URL,
                Environment::Production | Environment::Mainnet => production::URL,
            }
        }
    }

    pub mod market {
        use super::Environment;

        pub mod dev {
            pub const URL: &str = "wss://stream.api.sui-dev.bluefin.io/ws/market";
            pub const COLOCATED_URL: &str = "ws://api.coloc.sui-dev.int.bluefin.io:9091/ws/market";
        }

        pub mod staging {
            pub const URL: &str = "wss://stream.api.sui-staging.bluefin.io/ws/market";
            pub const COLOCATED_URL: &str =
                "ws://api.coloc.sui-staging.int.bluefin.io:9091/ws/market";
        }

        pub mod production {
            pub const URL: &str = "wss://stream.api.sui-prod.bluefin.io/ws/market";
            pub const COLOCATED_URL: &str = "ws://api.coloc.sui-prod.int.bluefin.io:9091/ws/market";
        }

        #[deprecated(note = "Use dev instead")]
        pub mod devnet {
            pub use super::dev::*;
        }
        #[deprecated(note = "Use staging instead")]
        pub mod testnet {
            pub use super::staging::*;
        }
        #[deprecated(note = "Use production instead")]
        pub mod mainnet {
            pub use super::production::*;
        }

        pub fn url<'a>(environment: Environment) -> &'a str {
            match environment {
                Environment::Dev | Environment::Devnet => dev::URL,
                Environment::Staging | Environment::Testnet => staging::URL,
                Environment::Production | Environment::Mainnet => production::URL,
            }
        }
    }
}
