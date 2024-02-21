use anchor_lang::prelude::*;
use crate::state::*;
use crate::instructions::backout_utils::payout;

pub fn handle_pub_backout(
    ctx: Context<PubBackout>
) -> Result<()> {

    let auction = &mut ctx.accounts.auction;
    let advertiser_wallet = &ctx.accounts.advertiser_wallet;
    let advertiser = &mut ctx.accounts.advertiser;
    let publisher = &mut ctx.accounts.publisher;
    let publisher_wallet = &mut ctx.accounts.publisher_wallet;
    
    payout(
        auction,
        &publisher_wallet.to_account_info(),
        advertiser_wallet,
        publisher,
        advertiser,
        true
    )
}

#[derive(Accounts)]
pub struct PubBackout<'info> {
    #[account(
        seeds = [b"auction", publisher_wallet.key().as_ref(), &auction.id.to_le_bytes()],
        bump = auction.bump,
        mut
    )]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    /// CHECK: Getting paid that's it
    pub advertiser_wallet: AccountInfo<'info>,
    #[account(
        seeds = [b"advertiser", advertiser_wallet.key().as_ref()],
        bump = advertiser.bump,
        mut
    )]
    pub advertiser: Account<'info, Advertiser>,
    #[account(
        seeds = [b"publisher".as_ref(), publisher_wallet.key().as_ref()],
        bump = publisher.bump,
        mut
    )]
    pub publisher: Account<'info, Publisher>,
    #[account(mut)]
    pub publisher_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}