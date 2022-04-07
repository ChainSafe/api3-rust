use api3_common::{derive_beacon_id, Address, Bytes, Bytes32, U256};

/// @notice Updates a Beacon using data signed by the respective Airnode,
/// without requiring a request or subscription
/// @param airnode Airnode address
/// @param templateId Template ID
/// @param timestamp Timestamp used in the signature
/// @param data Response data (an `int256` encoded in contract ABI)
/// @param signature Template ID, a timestamp and the response data signed
/// by the Airnode address
pub fn update_beacon_with_signed_data(
    airnode: &Address,
    template_id: &Bytes32,
    timestamp: U256,
    data: Vec<u8>,
    signature: Vec<u8>,
) {
    let airnode = Bytes::from(airnode.to_fixed_bytes());
    let beacon_id = derive_beacon_id(airnode, *template_id);
    dbg!(beacon_id);
    todo!();
}

/// @notice Updates the dAPI that is specified by the beacon IDs
/// @param beaconIds Beacon IDs
/// @return dapiId dAPI ID
pub fn update_dapi_with_beacons(beacond_ids: Vec<Bytes32>) -> Bytes32 {
    todo!();
}

/// @notice Updates a dAPI using data signed by the respective Airnodes
/// without requiring a request or subscription. The beacons for which the
/// signature is omitted will be read from the storage.
/// @param airnodes Airnode addresses
/// @param templateIds Template IDs
/// @param timestamps Timestamps used in the signatures
/// @param data Response data (an `int256` encoded in contract ABI per
/// Beacon)
/// @param signatures Template ID, a timestamp and the response data signed
/// by the respective Airnode address per Beacon
/// @return dapiId dAPI ID
pub fn update_dapi_with_signed_data(
    airnodes: Vec<Address>,
    template_ids: Vec<Bytes32>,
    timestamps: Vec<U256>,
    data: Vec<Bytes>,
    signatures: Vec<Bytes>,
) -> Bytes32 {
    todo!();
}

/// @notice Sets the data point ID the name points to
/// @dev While a data point ID refers to a specific Beacon or dAPI, names
/// provide a more abstract interface for convenience. This means a name
/// that was pointing at a Beacon can be pointed to a dAPI, then another
/// dAPI, etc.
/// @param name Human-readable name
/// @param dataPointId Data point ID the name will point to
pub fn set_name(name: Bytes32, data_point_id: Bytes32) {}

/// @notice Returns the data point ID the name is set to
/// @param name Name
/// @return Data point ID
pub fn name_to_data_point_id(name: Bytes32) -> Bytes32 {
    todo!()
}

/// @notice Reads the data point with ID
/// @param dataPointId Data point ID
/// @return value Data point value
/// @return timestamp Data point timestamp
pub fn read_with_data_point_id(data_point_id: Bytes32) -> (U256, u32) {
    todo!();
}

/// @notice Reads the data point with name
/// @dev The read data point may belong to a Beacon or dAPI. The reader
/// must be whitelisted for the hash of the data point name.
/// @param name Data point name
/// @return value Data point value
/// @return timestamp Data point timestamp
pub fn read_with_name(name: Bytes32) -> (U256, u32) {
    todo!();
}

/// @notice Returns the data point ID the name is set to
/// @param name Name
/// @return Data point ID
pub fn reader_can_read_data_point(data_point_id: Bytes32, reader: Address) -> bool {
    todo!();
}

/// @notice Returns the detailed whitelist status of the reader for the
/// data point
/// @param dataPointId Data point ID (or data point name hash)
/// @param reader Reader address
/// @return expirationTimestamp Timestamp at which the whitelisting of the
/// reader will expire
/// @return indefiniteWhitelistCount Number of times `reader` was
/// whitelisted indefinitely for `dataPointId`
pub fn data_point_id_to_reader_to_whitelist_status(
    data_point_id: Bytes32,
    reader: Address,
) -> (u64, U256) {
    todo!();
}

/// @notice Returns if an account has indefinitely whitelisted the reader
/// for the data point
/// @param dataPointId Data point ID (or data point name hash)
/// @param reader Reader address
/// @param setter Address of the account that has potentially whitelisted
/// the reader for the data point indefinitely
/// @return indefiniteWhitelistStatus If `setter` has indefinitely
/// whitelisted reader for the data point
pub fn data_point_id_to_reader_to_setter_to_indefinite_whitelist_status(
    data_point_id: Bytes32,
    reader: Address,
    setter: Address,
) -> bool {
    todo!();
}

#[test]
fn test_beacon() {
    let airnode = Address::from([0; 20]);
    let template_id = Bytes32::from([0; 32]);
    let timestamp = U256::from(0);
    let data = vec![0; 10];
    let signature = vec![0; 100];
    update_beacon_with_signed_data(&airnode, &template_id, timestamp, data, signature);
}
