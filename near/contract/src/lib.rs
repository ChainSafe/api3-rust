#![allow(unused)]

mod types;
mod utils;

use crate::types::{Address, NearDataPoint};
use crate::utils::{DatapointHashMap, msg_sender, NearAccessControlRegistry, NearClock, SignatureVerify};
use api3_common::abi::{
    decode, encode, encode_packed, keccak256, to_eth_signed_message_hash, ParamType, Token, Uint,
    U256,
};
use api3_common::util::median_wrapped_u256;
use api3_common::{derive_beacon_id, keccak_packed, process_beacon_update, Bytes, Bytes32, Error, SignatureManger, TimestampChecker, AccessControlRegistry};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, init, near_bindgen};

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
    initialized: bool,
    /// Data point related storage
    data_points: LookupMap<Bytes32, NearDataPoint>,
    name_hash_to_data_point_id: LookupMap<Bytes32, NearDataPoint>,

    /// Access control related storage
    unlimited_reader_role: Bytes32,
    name_setter_role: Bytes32,
    manager: Address,
    admin_role_description: String,
    role_membership: LookupMap<Bytes32, bool>,
    role_admin: LookupMap<Bytes32, Address>,

    // TODO: Whitelist related storage
}

impl Default for DapiServer {
    fn default() -> Self {
        let data_points = LookupMap::new(b'd');
        let name_hash_to_data_point_id = LookupMap::new(b'n');

        let manager = Address(Bytes32::default());
        let mut role_membership = LookupMap::new(b'm');
        let mut role_admin = LookupMap::new(b'a');

        Self {
            initialized: false,
            data_points,
            name_hash_to_data_point_id,
            unlimited_reader_role: Bytes32::default(),
            name_setter_role: Bytes32::default(),
            manager: Address(Bytes32::default()),
            admin_role_description: String::from("admin role"),
            role_membership,
            role_admin
        }
    }
}

#[near_bindgen]
impl DapiServer {
    /// The initializer of the contract
    pub fn initialize(&mut self) {
        ensure!(!self.initialized, Error::AlreadyInitialized);

        let manager = msg_sender();
        let mut access = NearAccessControlRegistry::new(
            manager.clone(),
            self.admin_role_description.clone(),
            &mut self.role_membership,
            &mut self.role_admin,
        );
        access.grant_role(
            &NearAccessControlRegistry::DEFAULT_ADMIN_ROLE,
            &msg_sender()
        ).unwrap();

        self.unlimited_reader_role = access.derive_role(
            access.derive_admin_role(&manager),
            hex::encode(keccak_packed(&[Token::String(UNLIMITED_READER_ROLE_DESCRIPTION.parse().unwrap())]))
        );
        self.name_setter_role = access.derive_role(
            access.derive_admin_role(&manager),
            hex::encode(keccak_packed(&[Token::String(NAME_SETTER_ROLE_DESCRIPTION.parse().unwrap())]))
        );

        self.manager = manager;
        self.initialized = true;
    }

    // ================== Access Control ====================
    /// Grants `role` to `who`
    pub fn grant_role(&mut self, role: Bytes32, who: Bytes32) {
        let mut access = NearAccessControlRegistry::new(
            self.manager.clone(),
            self.admin_role_description.clone(),
            &mut self.role_membership,
            &mut self.role_admin,
        );

        ensure!(
            access.only_role(&NearAccessControlRegistry::DEFAULT_ADMIN_ROLE, &msg_sender()).is_ok(),
            Error::NotAuthorized
        );

        access.grant_role(
            &role,
            &Address(who)
        ).unwrap();
    }

    // ================== Datapoint ====================
    /// Updates a Beacon using data signed by the respective Airnode,
    /// without requiring a request or subscription
    /// `airnode` Airnode address
    /// `template_id` Template ID
    /// `timestamp` Timestamp used in the signature
    /// `data` Response data (an `int256` encoded in contract ABI)
    /// `signature` Template ID, a timestamp and the response data signed by the Airnode address
    pub fn update_beacon_with_signed_data(
        &mut self,
        airnode: Bytes,
        template_id: Bytes32,
        timestamp: Bytes32,
        data: Vec<u8>,
        signature: Vec<u8>,
    ) {
        // create the utility structs
        let mut storage = DatapointHashMap::new(&mut self.data_points);

        // perform signature verification
        let message = keccak_packed(&[
            Token::FixedBytes(template_id.to_vec()),
            Token::Uint(Uint::from_big_endian(&timestamp)),
            Token::Bytes(data.clone()),
        ]);

        if !SignatureVerify::verify(&airnode, &message, &signature) {
            panic!("Signature verification wrong");
        }

        let beacon_id = derive_beacon_id(airnode.to_vec(), template_id);
        process_beacon_update(
            &mut storage,
            beacon_id,
            Uint::from_big_endian(&timestamp),
            data,
        )
        .unwrap();
    }

    #[cfg(test)]
    fn get_data_point(&self, template_id: &Bytes32) -> NearDataPoint {
        self.data_points
            .get(template_id)
            .unwrap_or(NearDataPoint::new(U256::from(0u32), 0))
    }

    /// Updates the dAPI that is specified by the beacon IDs
    /// `beacon_ids` Beacon IDs
    pub fn update_dapi_with_beacons(&mut self, beacon_ids: Vec<Bytes32>) -> Bytes32 {
        let mut storage = DatapointHashMap::new(&mut self.data_points);
        api3_common::update_dapi_with_beacons(&mut storage, &beacon_ids).unwrap()
    }

    /// Updates a dAPI using data signed by the respective Airnodes
    /// without requiring a request or subscription. The beacons for which the
    /// signature is omitted will be read from the storage.
    /// `airnodes` Airnode addresses
    /// `templateIds` Template IDs
    /// `timestamps` Timestamps used in the signatures
    /// `data` Response data (an `int256` encoded in contract ABI per Beacon)
    /// `signatures` Template ID, a timestamp and the response data signed by the respective Airnode address per Beacon
    pub fn update_dapi_with_signed_data(
        &mut self,
        airnodes: Vec<Bytes>,
        template_ids: Vec<Bytes32>,
        timestamps: Vec<Bytes32>,
        data: Vec<Bytes>,
        signatures: Vec<Bytes>,
    ) -> Bytes32 {
        let mut storage = DatapointHashMap::new(&mut self.data_points);
        let sig_verify = SignatureVerify{};
        let clock = NearClock::new(nanoseconds_to_seconds(near_sdk::env::block_timestamp()));

        api3_common::update_dapi_with_signed_data::<_, SignatureVerify, _>(
            &mut storage,
            &clock,
            airnodes,
            template_ids,
            timestamps,
            data,
            signatures,
        )
        .unwrap()
    }
}

fn nanoseconds_to_seconds(nano: u64) -> u32 {
    (nano / (1e9 as u64)) as u32
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use near_sdk::json_types::ValidAccountId;
//     use near_sdk::test_utils::{get_logs, VMContextBuilder};
//     use near_sdk::MockedBlockchain;
//     use near_sdk::{testing_env, VMContext};
//
//     fn get_context(is_view: bool) -> VMContext {
//         VMContextBuilder::new()
//             .signer_account_id(ValidAccountId::try_from("bob_near").unwrap())
//             .is_view(is_view)
//             .build()
//     }
//
//     #[test]
//     fn test_update_beacon_with_signed_data() {
//         let context = get_context(false);
//         testing_env!(context);
//
//         let mut server = DapiServer::constructor(
//             [1u8; 32],
//             String::from("admin role")
//         );
//
//         let timestamp = [
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98,
//             75, 201, 172,
//         ];
//         let data = vec![
//             0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 0, 121,
//         ];
//         let template_id = [
//             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//             0, 0, 1,
//         ];
//         let address = vec![
//             122u8, 24, 207, 215, 142, 241, 170, 222, 110, 132, 166, 81, 243, 188, 219, 223, 220,
//             244, 98, 172, 46, 85, 249, 115, 21, 242, 146, 143, 196, 204, 192, 131,
//         ];
//         let signature = vec![
//             42u8, 186, 218, 46, 220, 60, 51, 121, 176, 254, 154, 86, 164, 244, 66, 221, 225, 133,
//             96, 147, 202, 166, 221, 107, 139, 249, 63, 89, 119, 222, 248, 184, 155, 55, 51, 24, 96,
//             251, 206, 154, 52, 66, 247, 11, 64, 80, 246, 212, 175, 191, 82, 245, 219, 202, 152, 51,
//             116, 104, 189, 64, 127, 161, 151, 4,
//         ];
//         server.update_beacon_with_signed_data(
//             address.clone(),
//             template_id.clone(),
//             timestamp,
//             data.clone(),
//             signature,
//         );
//
//         let beacon_id = derive_beacon_id(address, template_id);
//         let datapoint = server.get_data_point(&beacon_id);
//         assert_eq!(
//             datapoint.timestamp,
//             Uint::from_big_endian(&timestamp).as_u32()
//         );
//         assert_eq!(datapoint.value, Uint::from_big_endian(&data));
//     }
// }
