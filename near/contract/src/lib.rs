#![allow(unused)]

mod types;
mod utils;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, init};
// use crate::ensure;
// use crate::error_panic;
use crate::types::Address;
// use api3_common::{decode, derive_beacon_id, SignatureManger, Uint};
// use api3_common::encode;
// use api3_common::encode_packed;
// use api3_common::keccak256;
// use api3_common::to_eth_signed_message_hash;
// use api3_common::types::U256;
// use api3_common::util::median_wrapped_u256;
// use api3_common::Bytes;
// use api3_common::Bytes32;
// use api3_common::Error;
// use api3_common::ParamType;
// use api3_common::Token;
// use api3_common::process_beacon_update;

use crate::types::NearDataPoint;
// use crate::utils::{DatapointHashMap }; //, SignatureVerify};

near_sdk::setup_alloc!();

/// @notice Unlimited reader role description
const UNLIMITED_READER_ROLE_DESCRIPTION: &str = "Unlimited reader";

/// @notice Name setter role description
const NAME_SETTER_ROLE_DESCRIPTION: &str = "Name setter";

const ONE_HOUR_IN_MS: u32 = 3_600_000;
const FIFTEEN_MINUTES_IN_MS: u32 = 900_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DapiServer {
    value: u32,
    // /// @notice Unlimited reader role
    // unlimited_reader_role: Bytes32,
    // /// @notice Name setter role
    // name_setter_role: Bytes32,
    // data_points: LookupMap<Bytes32, NearDataPoint>,
    // name_hash_to_data_point_id: LookupMap<Bytes32, NearDataPoint>,
}

impl Default for DapiServer {
    fn default() -> Self {
        near_sdk::env::log("in constructor".as_ref());
        // let data_points = LookupMap::new(b'd');
        // near_sdk::env::log("in datapoints".as_ref());
        // let name_hash_to_data_point_id = LookupMap::new(b'n');
        // near_sdk::env::log("in name_hash_to_data_point_id".as_ref());
        Self {
            value: 0
            // unlimited_reader_role: Bytes32::default(), //keccak256
            // name_setter_role: Bytes32::default(),      // keccac
            // data_points,
            // name_hash_to_data_point_id
        }
    }
}

#[near_bindgen]
impl DapiServer {
    // /// @dev Reverts if the timestamp is not valid
    // /// @param timestamp Timestamp used in the signature
    // fn only_valid_timestamp(timestamp: U256) {
    //     ensure!(Self::timestamp_is_valid(timestamp), Error::InvalidTimestamp)
    // }

    /// @param _accessControlRegistry AccessControlRegistry contract address
    /// @param _adminRoleDescription Admin role description
    /// @param _manager Manager address
    pub fn constructor() {
        near_sdk::env::log("in constructor".as_ref());
    }

    // /// @notice Updates a Beacon using data signed by the respective Airnode,
    // /// without requiring a request or subscription
    // /// @param airnode Airnode address
    // /// @param templateId Template ID
    // /// @param timestamp Timestamp used in the signature
    // /// @param data Response data (an `int256` encoded in contract ABI)
    // /// @param signature Template ID, a timestamp and the response data signed
    // /// by the Airnode address
    // pub fn update_beacon_with_signed_data(
    //     &mut self,
    //     airnode: Address,
    //     template_id: Bytes32,
    //     timestamp: Bytes32,
    //     data: Vec<u8>,
    //     signature: Vec<u8>,
    // ) {
    //     // create the utility structs
    //     let mut storage = DatapointHashMap::new(&mut self.data_points);
    //     let mut sig_verify = SignatureVerify::new(vec![signature.is_empty()]);
    //
    //     // perform signature verification
    //     let message = Self::encode_signed_message_hash(&template_id, &timestamp, &data);
    //     if !sig_verify.verify(airnode.as_bytes(), &message, &signature) {
    //         panic!("Signature verification wrong");
    //     }
    //
    //     let beacon_id = derive_beacon_id(airnode.as_bytes().to_vec(), template_id);
    //     process_beacon_update(&mut storage, beacon_id, Uint::from_big_endian(&timestamp), data).unwrap();
    // }
    //
    // #[cfg(test)]
    // fn get_data_point(
    //     &self,
    //     template_id: &Bytes32,
    // ) -> NearDataPoint {
    //     self.data_points.get(template_id).unwrap_or(NearDataPoint::new(U256::from(0u32), 0))
    // }
    //
    // /// @notice Updates the dAPI that is specified by the beacon IDs
    // /// @param beaconIds Beacon IDs
    // /// @return dapiId dAPI ID
    // fn update_dapi_with_beacons(&mut self, beacon_ids: &[Bytes32]) -> Bytes32 {
    //     let beacon_count = beacon_ids.len();
    //     ensure!(beacon_count > 1, Error::LessThanTwoBeacons);
    //
    //     // TODO: this is originally int256, find out if this deals with negative values
    //     // if not then U256 is fine
    //     let mut values: Vec<U256> = Vec::with_capacity(beacon_count);
    //     let mut accumulated_timestamp: U256 = U256::from(0_u32);
    //
    //     for beacon_id in beacon_ids.iter() {
    //         if let Some(data_point) = self.data_points.get(beacon_id) {
    //             values.push(data_point.value);
    //             accumulated_timestamp += U256::from(data_point.timestamp);
    //         }
    //     }
    //     let updated_timestamp: u32 = (accumulated_timestamp / U256::from(beacon_count)).as_u32();
    //     //TODO: use the function from common by willes
    //     let dapi_id = Self::derive_dapi_id(beacon_ids);
    //     if let Some(data_point_for_dapi_id) = self.data_points.get(&dapi_id) {
    //         ensure!(
    //             updated_timestamp >= data_point_for_dapi_id.timestamp,
    //             Error::UpdatedValueOutdated
    //         );
    //     } else {
    //         env::panic(b"data point has no entry")
    //     }
    //     let updated_value: U256 = median_wrapped_u256(&values);
    //
    //     let data_point = NearDataPoint::new(updated_value, updated_timestamp);
    //
    //     self.data_points.insert(&dapi_id, &data_point);
    //     dapi_id
    // }
    //
    // /// @notice Updates a dAPI using data signed by the respective Airnodes
    // /// without requiring a request or subscription. The beacons for which the
    // /// signature is omitted will be read from the storage.
    // /// @param airnodes Airnode addresses
    // /// @param templateIds Template IDs
    // /// @param timestamps Timestamps used in the signatures
    // /// @param data Response data (an `int256` encoded in contract ABI per
    // /// Beacon)
    // /// @param signatures Template ID, a timestamp and the response data signed
    // /// by the respective Airnode address per Beacon
    // /// @return dapiId dAPI ID
    // fn update_dapi_with_signed_data(
    //     &mut self,
    //     _airnodes: &[Bytes],
    //     _template_ids: &[Bytes32],
    //     _timestamps: &[U256],
    //     _data: Vec<Bytes>,
    //     _signatures: Vec<Bytes>,
    // ) -> Bytes32 {
    //     Bytes32::default()
    // }
    //
    // fn encode_signed_message_hash(
    //     template_id: &[u8],
    //     timestamp: &[u8],
    //     data: &[u8],
    // ) -> [u8; 32] {
    //     let (encoded, _) = encode_packed(&[
    //         Token::FixedBytes(template_id.to_vec()),
    //         Token::Uint(Uint::from_big_endian(timestamp)),
    //         Token::Bytes(data.to_vec()),
    //     ]);
    //     let message = to_eth_signed_message_hash(&keccak256(&encoded));
    //     message
    // }
    //
    // fn decode_fulfillment_data(data: &Bytes) -> U256 {
    //     ensure!(data.len() == 32, Error::InvalidDataLength);
    //
    //     let decoded_data = decode(&[ParamType::Int(0)], data).unwrap();
    //     ensure!(decoded_data.len() == 1, Error::InvalidDataLength);
    //
    //     if let Token::Int(i) = decoded_data[0] {
    //         U256::from(i)
    //     } else {
    //         error_panic!(Error::InvalidDataType);
    //     }
    // }
    //
    // /// TODO: implement signature verification in NEAR
    // fn verify(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
    //     true
    // }
    //
    // /// TODO: this copied from common code, call it from there directly
    // ///
    // /// @notice Derives the dAPI ID from the beacon IDs
    // /// @dev Notice that `abi.encode()` is used over `abi.encodePacked()`
    // /// @param beaconIds Beacon IDs
    // /// @return dapiId dAPI ID
    // fn derive_dapi_id(beacon_ids: &[Bytes32]) -> Bytes32 {
    //     let tokens: Vec<Token> = beacon_ids
    //         .iter()
    //         .map(|b| Token::FixedBytes(b.to_vec()))
    //         .collect();
    //     let encoded = encode(&tokens);
    //     keccak256(&encoded)
    // }
    //
    // /// @notice Returns if the timestamp used in the signature is valid
    // /// @dev Returns `false` if the timestamp is not at most 1 hour old to
    // /// prevent replays. Returns `false` if the timestamp is not from the past,
    // /// with some leeway to accomodate for some benign time drift. These values
    // /// are appropriate in most cases, but you can adjust them if you are aware
    // /// of the implications.
    // /// @param timestamp Timestamp used in the signature
    // fn timestamp_is_valid(timestamp: U256) -> bool {
    //     timestamp + U256::from(ONE_HOUR_IN_MS) > U256::from(env::block_timestamp())
    //         && timestamp < U256::from(env::block_timestamp()) + U256::from(FIFTEEN_MINUTES_IN_MS)
    // }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use near_sdk::test_utils::{get_logs, VMContextBuilder};
    // use near_sdk::{testing_env, VMContext};
    // use near_sdk::json_types::ValidAccountId;
    // use near_sdk::MockedBlockchain;
    //
    // fn get_context(is_view: bool) -> VMContext {
    //     VMContextBuilder::new()
    //         .signer_account_id(ValidAccountId::try_from("bob_near").unwrap())
    //         .is_view(is_view)
    //         .build()
    // }
    //
    // #[test]
    // fn test() {
    //     let context = get_context(false);
    //     testing_env!(context);
    //
    //     let mut server = DapiServer::constructor();
    //     let account = Address::from("sample.testnet");
    //
    //     let template_id = Bytes32::default();
    //     let data = vec![1;32];
    //     let mut timestamp = [0u8; 32];
    //     timestamp[31] = 123;
    //     server.update_beacon_with_signed_data(
    //         Address::from("sample.testnet"),
    //         template_id.clone(),
    //         timestamp,
    //         data.clone(),
    //         vec![]
    //     );
    //
    //     let beacon_id = derive_beacon_id(Address::from("sample.testnet").as_bytes().to_vec(), template_id);
    //     let datapoint = server.get_data_point(&beacon_id);
    //     assert_eq!(datapoint.timestamp, 123);
    //     assert_eq!(datapoint.value.to_u256(), Uint::from_big_endian(&data));
    // }
}