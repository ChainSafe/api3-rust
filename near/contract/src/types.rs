use api3_common::{DataPoint, Int, Zero};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;
use serde::{Deserialize, Serialize};
use std::io;
use api3_common::types::U256;

/// Address is an alias to H160, which is [u8;20]
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

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub(crate) struct NearDataPoint {
    pub value: U256,
    pub timestamp: u32,
}

impl NearDataPoint {
    pub fn new(value: U256, timestamp: u32) -> Self {
        NearDataPoint { value, timestamp }
    }
}

impl From<NearDataPoint> for DataPoint {
    fn from(t: NearDataPoint) -> Self {
        let mut v = [0u8; 32];
        t.value.to_big_endian(&mut v);
        DataPoint::new(Int::from_big_endian(&v), t.timestamp)
    }
}

impl From<DataPoint> for NearDataPoint {
    fn from(t: DataPoint) -> Self {
        let mut v = [0u8; 32];
        t.value.to_big_endian(&mut v);
        NearDataPoint::new(U256::from_big_endian(&v), t.timestamp)
    }
}