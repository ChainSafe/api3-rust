use std::fmt::Debug;
use thiserror::Error;

//TODO: copy the exact error messages from the original solidity code
#[derive(Error, Debug)]
pub enum Error {
    #[error("")]
    CannotDeserializeDataPoint,
    #[error("")]
    InvalidData,
    #[error("Data length not correct")]
    InvalidDataLength,
    #[error("Invalid data type")]
    InvalidDataType,
    #[error("Beacon data not found")]
    BeaconDataNotFound,
    #[error("Fulfillment older than Beacon")]
    FulfillmentOlderThanBeacon,
    #[error("Invalid name: {0}")]
    InvalidName(String),
    #[error("Eth Abi Error: {0}")]
    EthAbiError(ethabi::Error),
    #[cfg(feature = "recovery")]
    Libsecp256k1Error(libsecp256k1::Error),
    #[error("Parameter length mismatch")]
    ParameterLengthMismatch,
    #[error("Specified less than two Beacons")]
    LessThanTwoBeacons,
    #[error("Timestamp not valid")]
    InvalidTimestamp,
    #[error("Signature mismatch")]
    InvalidSignature,
    #[error("Updated value outdated")]
    UpdatedValueOutdated,
    #[error("Whitelist Error: {0}")]
    WhiteListError(WhitelistError),
}

#[derive(Error, Debug)]
pub enum WhitelistError {
    #[error("Does not extend expiration")]
    DoesNotExtendExpiration,
    AccessDenied,
    NameHashNotFound,
    RoleDescriptionEmpty,
}

#[cfg(feature = "recovery")]
impl From<libsecp256k1::Error> for Error {
    fn from(e: libsecp256k1::Error) -> Self {
        Error::Libsecp256k1Error(e)
    }
}

impl From<Error> for u32 {
    fn from(e: Error) -> Self {
        match e {
            Error::CannotDeserializeDataPoint => 0,
            Error::InvalidData => 1,
            Error::InvalidDataLength => 2,
            Error::InvalidDataType => 3,
            Error::BeaconDataNotFound => 4,
            Error::FulfillmentOlderThanBeacon => 5,
            Error::InvalidName(_) => 6,
            Error::EthAbiError(_) => 7,
            #[cfg(feature = "recovery")]
            Error::Libsecp256k1Error(_) => 8,
            Error::ParameterLengthMismatch => 9,
            Error::LessThanTwoBeacons => 10,
            Error::InvalidTimestamp => 11,
            Error::InvalidSignature => 12,
            Error::UpdatedValueOutdated => 13,
            Error::WhiteListError(_) => 14,
            Error::AccessDenied => 14,
            Error::NameHashNotFound => 15,
            Error::RoleDescriptionEmpty => 16,
        }
    }
}
