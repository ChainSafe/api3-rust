// pub use ethabi::{decode, encode};

// pub use abi::eth::{encode_packed, keccak};
pub use datapoint::DataPoint;
pub use error::Error;
pub use manager::Manager;

mod error;
mod datapoint;
mod manager;
mod abi;

pub type Bytes = Vec<u8>;
pub type Bytes32 = [u8; 32];
pub type Uint256 = [u8; 32];