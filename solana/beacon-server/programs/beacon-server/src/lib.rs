use anchor_lang::prelude::*;
use borsh::{ BorshSerialize, BorshDeserialize };
use api3_common::{DataPoint, Bytes, Bytes32, Uint256};

declare_id!("FRoo7m8Sf6ZAirGgnn3KopQymDtujWx818kcnRxzi23b");

#[program]
pub mod beacon_server {
    use super::*;

    /// Creates a new beacon data point. The beacon id is used as
    /// the seed to generate pda for the Beacon data account.
    pub fn new_beacon(
        ctx: Context<NewBeacon>,
        beacon_id: [u8; 32],
        template_id: [u8; 32],
        timestamp: [u8; 32],
        data: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<()> {
        // TOOD: perform signature check
        msg!("beacon_id: {:?}", beacon_id);
        msg!("template_id: {:?}", template_id);
        msg!("timestamp: {:?}", timestamp);
        msg!("data: {:?}", data);
        msg!("signature: {:?}", signature);
        Ok(())
    }

    // fn derive_beacon_id
}

#[derive(Accounts)]
// #[instruction(beacon_id: Bytes32)]
#[instruction(beacon_id: [u8; 32])]
pub struct NewBeacon<'info> {
    #[account(
    init,
    payer = user,
    space = 8 + 37,
    seeds = [b"beacon-id", beacon_id.as_ref()],
    bump
    )]
    pub beacon: Account<'info, WrappedDataPoint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// TODO: to use custom types, need to #[derive(AnchorSerialize, AnchorDeserialize, Clone)]

#[account]
pub struct WrappedDataPoint {
    raw_datapoint: Vec<u8>,
    bump: u8,
}
