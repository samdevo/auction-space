use std::mem::size_of;

use anchor_lang::prelude::*;

use crate::publisher::Publisher;


pub fn create_auction(ctx: Context<CreateAuction>, title: String) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let publisher = &mut ctx.accounts.publisher;
    auction.publisher = publisher.key();
    // if title is more than 32 bytes, throw an error
    if title.len() > 32 {
        return err!(AuctionErrors::TitleTooLong);
    }
    auction.title = title;
    auction.active = false; // is this default?
    auction.bump = ctx.bumps.auction;
    auction.id = publisher.num_auctions;
    publisher.num_auctions += 1;
    Ok(())
}


#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 32 + size_of::<Auction>(),
        seeds = [b"auction", authority.key().as_ref(), &publisher.num_auctions.to_le_bytes()],
        bump
    )]
    pub auction: Account<'info, Auction>,
    #[account(
        seeds = [b"publisher", publisher.key().as_ref()],
        bump = publisher.bump,
        has_one = authority,
        mut
    )]
    pub publisher: Account<'info, Publisher>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct Auction {
    pub publisher: Pubkey,
    pub cur_owner: Pubkey,
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
    pub title: String,
    // timestamp frequency in seconds
    pub duration: u64,
    pub active: bool,
    pub rounds_left: u64,
    pub id: u64,
    bump: u8,
}

pub fn activate_auction(ctx: Context<ActivateAuction>, min_price: u64, duration: u64, num_rounds: u64) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    if auction.active {
        return err!(AuctionErrors::AuctionAlreadyActive);
    }
    auction.active = true;
    auction.duration = duration;
    if num_rounds == 0 {
        auction.rounds_left = u64::MAX;
    } else {
        auction.rounds_left = num_rounds;
    }
    Ok(())
}

pub fn deactivate_auction(ctx: Context<DeactivateAuction>) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    if !auction.active {
        return err!(AuctionErrors::AuctionNotActive);
    }
    auction.rounds_left = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct ActivateAuction<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub auction: Account<'info, Auction>,
}

#[derive(Accounts)]
pub struct DeactivateAuction<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub auction: Account<'info, Auction>,
}

#[error_code]
pub enum AuctionErrors {
    #[msg("Title too long")]
    TitleTooLong,
    #[msg("Auction is already active")]
    AuctionAlreadyActive,
    #[msg("Auction is not active")]
    AuctionNotActive,
}