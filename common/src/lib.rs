use borsh::{ BorshSerialize, BorshDeserialize };

pub type Bytes = Vec<u8>;
pub type Bytes32 = [u8; 32];
pub type Uint256 = [u8; 32];

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct DataPoint {
    value: Uint256,
    timestamp: u32
}
