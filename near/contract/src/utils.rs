use crate::types::{Address, NearDataPoint};
use api3_common::{AccessControlRegistry, Bytes32, DataPoint, Error, keccak_packed, SignatureManger, Storage, TimestampChecker};
use ed25519_dalek::Verifier;
use near_sdk::collections::LookupMap;
use api3_common::abi::Token;

/// The utility struct for handling Near storage so that
/// we can use the code in `api3_common` for all the processing
pub(crate) struct DatapointHashMap<'account> {
    map: &'account mut LookupMap<Bytes32, NearDataPoint>,
}

impl<'account> DatapointHashMap<'account> {
    pub fn new(map: &'account mut LookupMap<Bytes32, NearDataPoint>) -> Self {
        Self { map }
    }
}

impl<'account> Storage<DataPoint> for DatapointHashMap<'account> {
    fn get(&self, k: &Bytes32) -> Option<DataPoint> {
        match self.map.get(k) {
            Some(d) => Some(d.clone().into()),
            None => Some(DataPoint::default()),
        }
    }

    fn store(&mut self, k: Bytes32, datapoint: DataPoint) {
        if self.map.contains_key(&k) {
            self.map.remove(&k);
        }
        self.map.insert(&k, &NearDataPoint::from(datapoint));
    }
}

/// Utility function for signature verification for Near so that we can use
/// `api3_common` package for the functions
pub(crate) struct SignatureVerify;

impl SignatureManger for SignatureVerify {
    fn verify(key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        let signature = ed25519_dalek::Signature::try_from(signature)
            .expect("Signature should be a valid array of 64 bytes");

        let public_key =
            ed25519_dalek::PublicKey::from_bytes(key).expect("Invalid public key passed");

        public_key.verify(message, &signature).is_ok()
    }
}

pub(crate) struct NearClock {
    current_timestamp: u32,
}

impl NearClock {
    pub fn new(current_timestamp: u32) -> Self {
        Self { current_timestamp }
    }
}

impl TimestampChecker for NearClock {
    fn current_timestamp(&self) -> u32 {
        self.current_timestamp
    }
}

pub(crate) fn msg_sender() -> Address {
    let sender_bytes = near_sdk::env::signer_account_pk().to_vec();
    let mut v = Bytes32::default();
    v.copy_from_slice(&sender_bytes[1..]);
    Address(v)
}

pub(crate) struct NearAccessControlRegistry<'account> {
    manager: Address,
    admin_role_description: String,
    role_membership: &'account mut LookupMap<Bytes32, bool>,
    role_admin: &'account mut LookupMap<Bytes32, Address>,
}

impl <'a> NearAccessControlRegistry<'a> {
    pub fn new(
        manager: Address,
        admin_role_description: String,
        role_membership: &'a mut LookupMap<Bytes32, bool>,
        role_admin: &'a mut LookupMap<Bytes32, Address>,
    ) -> Self {
        Self { manager, admin_role_description, role_membership, role_admin }
    }

    fn hash_membership(role: &Bytes32, who: &Address) -> Bytes32 {
        keccak_packed(&[
            Token::FixedBytes(role.to_vec()),
            Token::FixedBytes(who.as_ref().to_vec())
        ])
    }
}

impl <'a> AccessControlRegistry for NearAccessControlRegistry<'a> {
    type Address = Address;

    fn manager(&self) -> &Self::Address {
        &self.manager
    }

    fn admin_role_description(&self) -> String {
        self.admin_role_description.clone()
    }

    fn has_role(&self, role: &Bytes32, who: &Self::Address) -> bool {
        let hash = Self::hash_membership(role, who);
        self.role_membership.contains_key(&hash)
    }

    fn grant_role(&mut self, role: &Bytes32, who: &Self::Address) -> Result<(), Error> {
        let hash = Self::hash_membership(role, who);
        self.role_membership.remove(&hash);
        self.role_membership.insert(&hash, &true);
        Ok(())
    }

    fn get_role_admin(&self, role: &Bytes32) -> Option<Bytes32> {
        self.role_admin.get(role).map(|a| { Bytes32::from(a) })
    }

    fn set_role_admin(&mut self, role: &Bytes32, role_admin: Bytes32) -> Result<(), Error> {
        self.role_admin.remove(role);
        self.role_admin.insert(role, &Address(role_admin));
        Ok(())
    }

    fn renounce_role(&mut self, role: &Bytes32, account: &Self::Address) -> Result<(), Error> {
        let sender = msg_sender();
        api3_common::ensure!(*account == sender, Error::NotAuthorized)?;
        let hash = Self::hash_membership(role, account);
        self.role_membership.remove(&hash);
        Ok(())
    }
}

/// NEAR contract calls on the panic interface for errors
#[macro_export]
macro_rules! ensure {
    ( $x:expr, $y:expr ) => {{
        if !$x {
            near_sdk::env::panic(format!("{:?}", $y).as_bytes())
        }
    }};
}

/// a convenient way to call to the NEAR's blockchain panic
#[macro_export]
macro_rules! error_panic {
    ( $y:expr ) => {{
        near_sdk::env::panic(format!("{}", $y).as_bytes())
    }};
}
