use crate::{Bytes, Bytes32, encode_packed, FixedBytes, keccak256, Token};

pub fn derive_beacon_id(airnode: Bytes, template_id: Bytes32) -> Bytes32 {
    let (encoded, _) = encode_packed(&[
        Token::Bytes(airnode),
        Token::FixedBytes(template_id.to_vec())
    ]);
    keccak256(&encoded)
}