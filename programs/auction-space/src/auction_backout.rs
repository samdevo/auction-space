use std::mem::size_of;

use anchor_lang::{prelude::*, solana_program};

use crate::publisher::Publisher;
use crate::advertiser::Advertiser;
use solana_program::system_instruction;

pub fn publisher_backout(ctx: Context<PublisherBackout>) -> Result<()> {
    Ok(())
}

pub fn advertiser_backout(ctx: Context<AdvertiserBackout>) -> Result<()> {
    Ok(())
}


#[derive(Accounts)]
pub struct PublisherBackout<'info> {
    #[account(
        seeds = [b"publisher".as_ref(), authority.key().as_ref()],
        bump = publisher.bump,
        has_one = authority,
        mut
    )]
    pub publisher: Account<'info, Publisher>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdvertiserBackout<'info> {
    #[account(
        seeds = [b"advertiser".as_ref(), authority.key().as_ref()],
        bump = advertiser.bump,
        has_one = authority,
        mut
    )]
    pub advertiser: Account<'info, Advertiser>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}