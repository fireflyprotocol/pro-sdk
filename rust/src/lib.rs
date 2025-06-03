mod account_authorization;
mod adjust_isolated_margin;
mod arithmetic;
mod authenticate;
mod create_order;
mod env;
mod signature;
mod update_leverage;
mod withdraw;

mod core {
    pub use crate::arithmetic::HasE9;
    #[allow(deprecated)]
    pub use crate::env::symbols;
    pub use crate::env::{account, auth, exchange, test, trade, websocket as ws, Environment};
    pub use crate::signature::PrivateKey;
}

pub mod prelude {
    pub use crate::account_authorization::AccountAuthorizationRequestExt;
    pub use crate::authenticate::{Authenticate as _, Refresh as _, RequestExt as _};
    pub use crate::core::*;
    pub use crate::signature::RequestExt as _;
}
