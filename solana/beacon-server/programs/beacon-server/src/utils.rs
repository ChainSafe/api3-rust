use crate::{ERROR_INVALID_NAME_HASH, WrappedDataPoint, WrappedDataPointId};
use anchor_lang::accounts::account::Account;
use anchor_lang::prelude::borsh::maybestd::collections::HashMap;
use anchor_lang::prelude::*;
use api3_common::{ensure, Bytes32, DataPoint, keccak_packed, SignatureManger, Storage, TimestampChecker, Token};

pub type NameHashAccountRef<'info> = Account<'info, WrappedDataPointId>;
pub(crate) struct NameHashHashMap<'info, 'account> {
    write: HashMap<Bytes32, &'account mut NameHashAccountRef<'info>>,
}

impl<'info, 'account> NameHashHashMap<'info, 'account> {
    pub fn new(accounts: Vec<(Bytes32, &'account mut NameHashAccountRef<'info>)>) -> Self {
        let mut write = HashMap::new();
        for (key, aref) in accounts {
            write.insert(key, aref);
        }
        Self { write }
    }
}

impl<'info, 'account> Storage<Bytes32> for NameHashHashMap<'info, 'account> {
    fn get(&self, k: &Bytes32) -> Option<Bytes32> {
        self.write.get(k).map(|a| a.datapoint_id.clone())
    }

    fn store(&mut self, k: Bytes32, datapoint_id: Bytes32) {
        let a = self.write.get_mut(&k).expect("cannot load from datapoint");
        (*a).datapoint_id = datapoint_id;
        // panic!("datapoint id: {:?}", a.datapoint_id);
    }
}

pub type DatapointAccountRef<'info> = Account<'info, WrappedDataPoint>;
pub(crate) struct DatapointHashMap<'info, 'account> {
    write: HashMap<Bytes32, &'account mut DatapointAccountRef<'info>>,
    read: HashMap<Bytes32, DataPoint>,
}

impl<'info, 'account> DatapointHashMap<'info, 'account> {
    pub fn new(
        accounts: Vec<(Bytes32, &'account mut DatapointAccountRef<'info>)>,
        read: HashMap<Bytes32, DataPoint>,
    ) -> Self {
        let mut write = HashMap::new();
        for (key, aref) in accounts {
            write.insert(key, aref);
        }
        Self { write, read }
    }
}

impl<'info, 'account> Storage<DataPoint> for DatapointHashMap<'info, 'account> {
    fn get(&self, k: &Bytes32) -> Option<DataPoint> {
        match self.read.get(k) {
            Some(d) => Some(d.clone()),
            None => self.write.get(k).map(|a| {
                if a.raw_datapoint.is_empty() {
                    DataPoint::default()
                } else {
                    DataPoint::from(a.raw_datapoint.clone()).expect("cannot load datapoint")
                }
            }),
        }
    }

    fn store(&mut self, k: Bytes32, datapoint: DataPoint) {
        let a = self.write.get_mut(&k).expect("cannot load from datapoint");
        (*a).raw_datapoint = Vec::from(datapoint);
    }
}

pub(crate) struct SolanaClock {
    current_timestamp: u32,
}

impl SolanaClock {
    pub fn new(current_timestamp: u32) -> Self {
        Self { current_timestamp }
    }
}

impl TimestampChecker for SolanaClock {
    fn current_timestamp(&self) -> u32 {
        self.current_timestamp
    }
}

/// The dummy signature checker for solana. Reason we don't really need signature validation
/// here is because solana has done this for us. The implementation should have asked solana
/// to validate the signatures before hand. All this needs to do is just tracking the number
/// of signatures
pub(crate) struct DummySignatureManger {
    sig_count: usize,
}

impl DummySignatureManger {
    pub fn new(sig_count: usize) -> Self {
        Self { sig_count }
    }
}

impl SignatureManger for DummySignatureManger {
    /// For solana, we are assuming the signatures are arranged such as non-empty ones
    /// are placed before the `sig_count` and empty ones are placed after.
    fn is_empty(&self, index: usize) -> bool {
        index >= self.sig_count
    }

    /// Solana should have already verified the signature for us, return true by default
    fn verify(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        true
    }
}

pub(crate) fn derive_dapi_id(_beacon_ids: &[Bytes32]) -> Bytes32 {
    Bytes32::default()
}

pub(crate) fn check_beacon_ids(
    _beacon_ids: &[Bytes32],
    _beacon_id_tuples: &[(Pubkey, Account<WrappedDataPoint>)],
) -> Result<()> {
    Ok(())
}

pub(crate) fn check_beacon_ids_with_templates(
    _beacon_ids: &[Bytes32],
    _beacon_id_tuples: &[(Pubkey, Bytes32)],
) -> Result<()> {
    Ok(())
}

/// Checks the dapis_id passed as parameter is actually derived from beacon_ids.
pub(crate) fn check_dapi_id(_dapi_id: &Bytes32, _beacon_ids: &[Bytes32]) -> Result<()> {
    derive_dapi_id(_beacon_ids);
    Ok(())
}

/// Checks the dapis_id passed as parameter is actually derived from beacon_ids.
pub(crate) fn check_name_hash(name: &Bytes32, name_hash: &Bytes32) -> Result<()> {
    let derived_hash = keccak_packed(&[Token::FixedBytes(name.to_vec())]);
    ensure!(
        *name_hash == derived_hash,
        Error::from(ProgramError::from(ERROR_INVALID_NAME_HASH))
    )
}
