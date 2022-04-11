use crate::Error;
use borsh::{self, BorshDeserialize, BorshSerialize};
use derive_more::{Add, AddAssign, Display, Div, From, Into, Sub, SubAssign};
use ethabi::decode as eth_decode;
pub use ethabi::{encode, Address, FixedBytes, ParamType, Token, Uint, Uint as U256};
use std::io;

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Default,
    From,
    Into,
    Display,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Div,
)]
pub struct Int(pub ethabi::Int);

impl Int {
    pub fn checked_add(self, other: Int) -> Option<Self> {
        self.0.checked_add(other.0).map(|v| Int(v))
    }

    pub fn from_big_endian(raw: &[u8]) -> Self {
        Int(ethabi::Int::from_big_endian(raw))
    }

    pub fn to_big_endian(&self, buffer: &mut [u8]) {
        self.0.to_big_endian(buffer)
    }
}

impl BorshDeserialize for Int {
    fn deserialize(bytes: &mut &[u8]) -> Result<Self, io::Error> {
        let values: [u8; 32] = BorshDeserialize::deserialize(bytes)?;
        Ok(Int::from_big_endian(&values))
    }
}

impl BorshSerialize for Int {
    fn serialize<W>(&self, writer: &mut W) -> Result<(), io::Error>
    where
        W: io::Write,
    {
        let mut v = [0u8; 32];
        self.to_big_endian(&mut v);
        BorshSerialize::serialize(&v, writer)
    }
}

impl From<u128> for Int {
    fn from(v: u128) -> Self {
        Int(ethabi::Int::from(v))
    }
}

impl From<ethabi::Error> for Error {
    fn from(e: ethabi::Error) -> Self {
        Error::EthAbiError(e)
    }
}

pub fn decode(types: &[ParamType], data: &[u8]) -> Result<Vec<Token>, Error> {
    let v = eth_decode(types, data)?;
    Ok(v)
}
