use crate::{Bytes, Bytes32, Uint256};

/// Corresponds to eth types, should be a
/// one to one mapping to common solidity types
pub enum Token {
    Bytes32(Bytes32),
    Bytes(Bytes),
    Uint256(Uint256)
}

impl From<Token> for ethabi::Token {
    fn from(token: Token) -> Self {
        match token {
            Token::Bytes32(v) => ethabi::Token::FixedBytes(Vec::from(v)),
            Token::Bytes(v) => ethabi::Token::Bytes(v),
            Token::Uint256(uint) => ethabi::Token::Uint(ethabi::Uint::from(uint))
        }
    }
}

pub fn encode(tokens: &[Token]) -> Bytes {
    let eth_tokens = tokens.iter().map(|t| t.into()).collect::<Vec<ethabi::Token>>();
    ethabi::encode(&eth_tokens)
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode_works() {

    }
}