use near_sdk::collections::LookupMap;
use api3_common::{Bytes32, DataPoint, Storage};
use crate::types::NearDataPoint;

pub(crate) struct DatapointHashMap<'account> {
    map: &'account mut LookupMap<Bytes32, NearDataPoint>,
}

impl<'account> DatapointHashMap<'account> {
    pub fn new(map: &'account mut LookupMap<Bytes32, NearDataPoint>) -> Self {
        Self { map }
    }
}

impl<'account> Storage<DataPoint> for DatapointHashMap<'account> {
    fn get(&self, k: &Bytes32) -> Option<DataPoint> {
        match self.map.get(k) {
            Some(d) => Some(d.clone().into()),
            None => Some(DataPoint::default()),
        }
    }

    fn store(&mut self, k: Bytes32, datapoint: DataPoint) {
        if self.map.contains_key(&k) {
            self.map.remove(&k);
        }
        self.map.insert(&k, &NearDataPoint::from(datapoint));
    }
}