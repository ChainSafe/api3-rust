use api3_common::Bytes32;
use api3_common::DataPoint;
use api3_common::DataPointStorage;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::near_bindgen;
use near_sdk::AccountId;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearDataPoint {
    map: LookupMap<Bytes32, DataPoint>,
}

impl DataPointStorage for NearDataPoint {
    fn get(&self, key: &Bytes32) -> Option<DataPoint> {
        self.map.get(key)
    }

    fn store(&mut self, key: Bytes32, datapoint: DataPoint) {}
}
