use crate::DataPoint;
use crate::U256;

/// The Manager for handling multiple datapoints
pub struct Manager;

impl Manager {
    pub fn agg(_datapoints: &[DataPoint]) -> DataPoint {
        DataPoint::new(U256::default(), u32::default())
    }
}
