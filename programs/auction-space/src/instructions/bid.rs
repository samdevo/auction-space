use anchor_lang::{prelude::*, solana_program};
use solana_program::system_instruction;
use crate::state::*;

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(
        seeds = [b"auction", auction.publisher_wallet.as_ref(), &auction.id.to_le_bytes()],
        bump = auction.bump,
        mut
    )]
    pub auction: Account<'info, Auction>,
    #[account(
        seeds = [b"advertiser", advertiser.advertiser_wallet.as_ref()],
        bump = advertiser.bump,
        has_one = advertiser_wallet,
    )]
    pub advertiser: Account<'info, Advertiser>,
    #[account(mut)]
    pub advertiser_wallet: Signer<'info>,
    /// CHECK: No need to deserialize. Program will verify that this user is the same as the one in the auction struct
    pub prev_bidder_wallet: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_bid(
    ctx: Context<Bid>,
    bid_amount: u64
) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let advertiser = &mut ctx.accounts.advertiser;
    let advertiser_wallet = &ctx.accounts.advertiser_wallet;
    let prev_bidder_wallet = &ctx.accounts.prev_bidder_wallet;
    Ok(())
}