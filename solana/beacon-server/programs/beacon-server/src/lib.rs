mod utils;

use anchor_lang::prelude::*;

declare_id!("FRoo7m8Sf6ZAirGgnn3KopQymDtujWx818kcnRxzi23b");

#[program]
pub mod beacon_server {
    use super::*;

    /// Update a new beacon data point with signed data. The beacon id is used as
    /// the seed to generate pda for the Beacon data account.
    pub fn update_beacon_with_signed_data(
        ctx: Context<UpdateBeaconWithSignedData>,
        _beacon_id: [u8; 32],
        _template_id: [u8; 32],
        _timestamp: [u8; 32],
        data: Vec<u8>,
        _signature: Vec<u8>,
    ) -> Result<()> {
        // TOOD: perform signature check

        utils::update_beacon_data(&mut ctx.accounts.beacon, data)?;

        Ok(())
    }

    /// Update a new beacon data point with signed data. The beacon id is used as
    /// the seed to generate pda for the Beacon data account.
    pub fn update_dapi_with_beacons(
        ctx: Context<UpdateDapiWithBeacons>,
        dapi_id: [u8; 32],
        beacon_ids: Vec<[u8; 32]>,
    ) -> Result<()> {
        // TOOD: perform signature check

        assert!(ctx.remaining_accounts.len() > 0);

        let beacon_id_tuples = ctx
            .remaining_accounts
            .iter()
            .map(|item| -> Result<(Pubkey, Account<WrappedDataPoint>)> {
                Account::try_from_unchecked(item).map(|i| (item.key.clone(), i))
            })
            .collect::<Result<Vec<(Pubkey, Account<WrappedDataPoint>)>>>()?;

        for t in beacon_id_tuples.iter().clone() {
            panic!("{:?}, {:?}", t.0, t.1.raw_datapoint);
        }

        utils::check_beacon_ids(&beacon_ids, &beacon_id_tuples)?;
        utils::check_dapi_id(&dapi_id, &beacon_ids)?;

        let account = &mut ctx.accounts.dapi;
        account.raw_datapoint = vec![1];

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(beacon_id: [u8; 32])]
pub struct UpdateBeaconWithSignedData<'info> {
    #[account(
    init_if_needed,
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

#[derive(Accounts)]
#[instruction(dapi_id: [u8; 32])]
pub struct UpdateDapiWithBeacons<'info> {
    #[account(
    init_if_needed,
    payer = user,
    space = 8 + 37,
    seeds = [b"dapi-id", dapi_id.as_ref()],
    bump
    )]
    pub dapi: Account<'info, WrappedDataPoint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct WrappedDataPoint {
    raw_datapoint: Vec<u8>,
    bump: u8,
}
