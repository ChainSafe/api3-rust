#![allow(unused)]

pub use error::Error;
pub use near_whitelist::NearWhitelistContract;
pub use types::Address;
pub use whitelist::Whitelist;

//mod dapi_server;
mod error;
mod near_whitelist;
mod types;
mod whitelist;

/// NEAR contract calls on the panic interface for errors
#[macro_export]
macro_rules! ensure {
    ( $x:expr, $y:expr ) => {{
        if !$x {
            near_sdk::env::panic(format!("{}", $y).as_bytes())
        }
    }};
}

/// a convenient way to call to the NEAR's blockchain panic
#[macro_export]
macro_rules! error_panic {
    ( $y:expr ) => {{
        near_sdk::env::panic(format!("{}", $y).as_bytes())
    }};
}
