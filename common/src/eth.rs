use ethabi::Token;
use tiny_keccak::{Hasher, Keccak};
use crate::Bytes32;

pub fn keccak(x: &[u8]) -> Bytes32 {
    let mut keccak = Keccak::v256();
    keccak.update(x);
    let mut out = [0u8; 32];
    keccak.finalize(&mut out);
    out
}

/// Rust implementation of solidity abi.encodePacked(...)
pub fn encode_packed(items: &[Token]) -> (Vec<u8>, String) {
    let res = items.iter().fold(Vec::new(), |mut acc, i| {
        let pack = pack(i);
        acc.push(pack);
        acc
    });
    let res = res.join(&[][..]);
    let hexed = hex::encode(&res);
    (res, hexed)
}

/// Pack a single `Token` into bytes
fn pack(data_type: &Token) -> Vec<u8> {
    let mut res = Vec::new();
    match data_type {
        Token::String(s) => res.extend(s.as_bytes()),
        _ => panic!("not supported yet"),
    };
    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode_works() {

    }
}