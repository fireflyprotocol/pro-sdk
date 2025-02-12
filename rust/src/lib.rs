mod address;
mod authenticate;
mod create_order;
mod env;
mod signature;
mod update_leverage;
mod withdraw;

pub use env::{account, auth, exchange, test, trade, websocket as ws, Environment};
pub use signature::{Ed25519PublicKey, PrivateKey, Secp256k1PublicKey, Type as SignatureType};

pub const E9: u64 = 1_000_000_000;

pub mod prelude {
    pub use crate::authenticate::{Authenticate as _, Refresh as _, RequestExt as _};
    pub use crate::signature::{IntoSuiAddress as _, RequestExt as _};
    pub use crate::E9;
}
