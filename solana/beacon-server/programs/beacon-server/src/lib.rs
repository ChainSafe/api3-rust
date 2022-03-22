use anchor_lang::prelude::*;
use api3_common::{DataPoint, Bytes, Bytes32, Uint256};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod beacon_server {
    use super::*;

    /// Creates a new beacon data point. The beacon id is used as
    /// the seed to generate pda for the Beacon data account.
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

#[derive(Accounts)]
#[instruction(beacon_id: Bytes32)]
pub struct NewBeacon<'info> {
    #[account(
    init,
    payer = user,
    space = 8 + 36,
    seeds = [b"beacon-id", beacon_id.as_ref()],
    bump
    )]
    pub beacon: Account<'info, WrappedDataPoint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WrappedDataPoint {
    data_point: DataPoint,
    bump: u8,
}
