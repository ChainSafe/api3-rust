//use api3_common::Zero;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;
use serde::{Deserialize, Serialize};
use std::io;
use uint::construct_uint;

pub const BYTES32_ZERO: Bytes32 = [0u8; 32];

/// Checks if the address is zero
pub trait Zero {
    fn is_zero(&self) -> bool;
}

impl Zero for Bytes32 {
    fn is_zero(&self) -> bool {
        (*self) == BYTES32_ZERO
    }
}

/// u256 is 4 u64
construct_uint! {
    #[derive(BorshDeserialize, BorshSerialize, Serialize)]
    pub struct U256(4);
}

pub type Bytes32 = [u8; 32];

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Address(AccountId);

impl Address {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Zero for Address {
    fn is_zero(&self) -> bool {
        todo!();
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<&str> for Address {
    fn from(value: &str) -> Self {
        Address(value.to_string())
    }
}
