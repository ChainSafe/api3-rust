#[derive(Debug)]
pub enum Error {
    CannotDeserializeDataPoint,
    InvalidData,
    InvalidName(String),
    #[cfg(feature = "recovery")]
    Libsecp256k1Error(libsecp256k1::Error),
}

#[cfg(feature = "recovery")]
impl From<libsecp256k1::Error> for Error {
    fn from(e: libsecp256k1::Error) -> Self {
        Error::Libsecp256k1Error(e)
    }
}

#[cfg(feature = "solana")]
impl From<Error> for anchor_lang::error::Error {
    fn from(_: Error) -> Self {
        anchor_lang::error::Error::from(anchor_lang::solana_program::program_error::ProgramError::Custom(0))
    }
}