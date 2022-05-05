/**
 * This line is create PrepareError in Near testnet.
 * Following the below commands:
 *   cargo build --target wasm32-unknown-unknown --release
 *   near deploy --wasmFile <path to>/wasm32-unknown-unknown/release/dapi_server.wasm --contractName=<You Account Id>.testnet --accountId <You Account Id>
 *   near call <You Account Id>.testnet constructor --accountId <You Account Id>.testnet
 * The last command would give the `Error happened during instantiation`
 */
use ed25519_dalek::Verifier;
// use near_crypto::PublicKey;
use near_sdk::collections::LookupMap;
// use api3_common::{Bytes32, DataPoint, SignatureManger, Storage};
use crate::types::NearDataPoint;

// /// The utility struct for handling Near storage so that
// /// we can use the code in `api3_common` for all the processing
// pub(crate) struct DatapointHashMap<'account> {
//     map: &'account mut LookupMap<Bytes32, NearDataPoint>,
// }
//
// impl<'account> DatapointHashMap<'account> {
//     pub fn new(map: &'account mut LookupMap<Bytes32, NearDataPoint>) -> Self {
//         Self { map }
//     }
// }
//
// impl<'account> Storage<DataPoint> for DatapointHashMap<'account> {
//     fn get(&self, k: &Bytes32) -> Option<DataPoint> {
//         match self.map.get(k) {
//             Some(d) => Some(d.clone().into()),
//             None => Some(DataPoint::default()),
//         }
//     }
//
//     fn store(&mut self, k: Bytes32, datapoint: DataPoint) {
//         if self.map.contains_key(&k) {
//             self.map.remove(&k);
//         }
//         self.map.insert(&k, &NearDataPoint::from(datapoint));
//     }
// }

// /// Utility function for signature verification for Near so that we can use
// /// `api3_common` package for the functions
// pub(crate) struct SignatureVerify {
//     sig_empty: Vec<bool>
// }
//
// impl SignatureVerify {
//     pub fn new(sig_empty: Vec<bool>) -> Self {
//         Self { sig_empty }
//     }
// }
//
// impl SignatureManger for SignatureVerify {
//     fn is_empty(&self, index: usize) -> bool {
//         *self.sig_empty.get(index).unwrap_or(&false)
//     }
//
//     fn verify(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
//         let signature = ed25519_dalek::Signature::try_from(signature)
//             .expect("Signature should be a valid array of 64 bytes [13, 254, 123, ...]");
//
//         let public_key = ed25519_dalek::PublicKey::from_bytes(key)
//             .expect("Invalid public key passed");
//
//         public_key.verify(message, &signature).is_ok()
//     }
// }

// /// NEAR contract calls on the panic interface for errors
// #[macro_export]
// macro_rules! ensure {
//     ( $x:expr, $y:expr ) => {{
//         if !$x {
//             near_sdk::env::panic(format!("{}", $y).as_bytes())
//         }
//     }};
// }

// /// a convenient way to call to the NEAR's blockchain panic
// #[macro_export]
// macro_rules! error_panic {
//     ( $y:expr ) => {{
//         near_sdk::env::panic(format!("{}", $y).as_bytes())
//     }};
// }