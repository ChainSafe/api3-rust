mod types;

use anchor_lang::prelude::*;
use crate::types::{Bytes, Bytes32, NewBeacon, Uint256};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod beacon_server {
    use super::*;

    pub fn new_beacon(
        ctx: Context<NewBeacon>,
        beacon_id: Bytes32,
        template_id: Bytes32,
        timestamp: Uint256,
        data: Bytes,
        signature: Bytes,
    ) -> Result<()> {
        // TOOD: perform signature check
        Ok(())
    }

    // fn derive_beacon_id
}

