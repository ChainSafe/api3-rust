use anchor_lang::accounts::account::Account;
use anchor_lang::accounts::program::Program;
use anchor_lang::accounts::signer::Signer;
use anchor_lang::system_program::System;
use anchor_lang::prelude::*;
use borsh::{ BorshSerialize, BorshDeserialize };

pub(crate) type Bytes = Vec<u8>;
pub(crate) type Bytes32 = [u8; 32];
pub(crate) type Uint256 = [u8; 32];

#[derive(Accounts)]
// #[instruction(beacon_id: Bytes32)]
pub struct NewBeacon<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 36,
        // seeds = [b"beacon-id", beacon_id.as_ref()],
        // bump
    )]
    pub beacon: Account<'info, WrappedDataPoint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// TODO: make it common
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct DataPoint {
    value: [u8; 32],
    timestamp: u32
}

#[account]
pub struct WrappedDataPoint {
    data_point: DataPoint,
    bump: u8,
}