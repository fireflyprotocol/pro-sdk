mod arithmetic;
mod authenticate;
mod create_order;
mod env;
mod signature;
mod update_leverage;
mod withdraw;

mod core {
    pub use crate::arithmetic::HasE9;
    pub use crate::env::{
        account, auth, exchange, symbols, test, trade, websocket as ws, Environment,
    };
    pub use crate::signature::PrivateKey;
}

pub mod prelude {
    pub use crate::authenticate::{Authenticate as _, Refresh as _, RequestExt as _};
    pub use crate::core::*;
    pub use crate::signature::RequestExt as _;
}
