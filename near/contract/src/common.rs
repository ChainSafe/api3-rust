//! This are code that should have been in common, but due to certain issues
//! such as reliance on ethabi/getrandom, we copy some types to here.

pub use error::Error;

mod error;
