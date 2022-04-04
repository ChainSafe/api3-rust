#[derive(Debug)]
pub enum Error {
    CannotDeserializeDataPoint,
    InvalidData,
    InvalidDataLength,
    InvalidDataType,
    BeaconDataNotFound,
    FulfillmentOlderThanBeacon,
    InvalidName(String),
    #[cfg(feature = "recovery")]
    Libsecp256k1Error(libsecp256k1::Error),

    #[cfg(feature = "ethabi")]
    EthAbiError(ethabi::Error),
}

#[cfg(feature = "recovery")]
impl From<libsecp256k1::Error> for Error {
    fn from(e: libsecp256k1::Error) -> Self {
        Error::Libsecp256k1Error(e)
    }
}
