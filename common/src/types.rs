use crate::Empty;
use borsh::{self, BorshDeserialize, BorshSerialize};
use derive_more::{
    Add, AddAssign, Display, Div, DivAssign, From, Into, Mul, MulAssign, Sub, SubAssign,
};
use serde::{Deserialize, Serialize};
use std::io;

/// This needs to be wrapped here, otherwise
/// There is no way for use to implement Borsh serde for U256 due to the fact
/// that both type and trait are foreign
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Default,
    From,
    Into,
    Display,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
)]
pub struct U256(pub crate::abi::U256);

impl BorshDeserialize for U256 {
    fn deserialize(bytes: &mut &[u8]) -> Result<Self, io::Error> {
        let values: [u8; 32] = BorshDeserialize::deserialize(bytes)?;
        Ok(U256(crate::abi::U256::from_big_endian(&values)))
    }
}

impl BorshSerialize for U256 {
    fn serialize<W>(&self, writer: &mut W) -> Result<(), io::Error>
    where
        W: io::Write,
    {
        let mut v = [0u8; 32];
        self.0.to_big_endian(&mut v);
        BorshSerialize::serialize(&v, writer)
    }
}

macro_rules! impl_u256 {
    ($($t: ty;)*) => {
        $(
            impl From<$t> for U256 {
                fn from(v: $t) -> Self {
                    U256(crate::abi::U256::from(v))
                }
            }
        )*
    };
}

impl_u256!(i32; i64; isize; i128; u32; u64; usize; u128;);

impl U256 {
    pub fn as_u32(&self) -> u32 {
        self.0.as_u32()
    }
}

/// Address is an alias to H160, which is [u8;20]
#[derive(Serialize, Deserialize)]
pub struct Address(crate::Address);

impl Address {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Empty for Address {
    fn is_empty(&self) -> bool {
        todo!();
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl BorshDeserialize for Address {
    fn deserialize(bytes: &mut &[u8]) -> Result<Self, io::Error> {
        let values: [u8; 20] = BorshDeserialize::deserialize(bytes)?;
        Ok(Address(crate::Address::from(values)))
    }
}

impl BorshSerialize for Address {
    fn serialize<W>(&self, writer: &mut W) -> Result<(), io::Error>
    where
        W: io::Write,
    {
        BorshSerialize::serialize(&self.0 .0, writer)
    }
}

impl From<[u8; 20]> for Address {
    fn from(bytes: [u8; 20]) -> Self {
        Address(crate::Address::from(bytes))
    }
}

#[test]
fn serialization() {
    let mut buffer = vec![];
    let v = U256::from(u128::MAX);
    dbg!(&v);
    v.serialize(&mut buffer).unwrap();
    dbg!(&buffer);

    let uv = U256::try_from_slice(&mut buffer);
    dbg!(&uv);
    assert_eq!(v, uv.unwrap());
}
