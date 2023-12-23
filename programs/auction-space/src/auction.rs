use std::mem::size_of;

use anchor_lang::{prelude::*, solana_program};

use crate::publisher::Publisher;
use crate::advertiser::Advertiser;
use solana_program::system_instruction;



pub fn create_auction(ctx: Context<CreateAuction>, title: String) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let publisher = &mut ctx.accounts.publisher;
    auction.publisher = publisher.key();
    // if title is more than 32 bytes, throw an error
    if title.len() > crate::MAX_STRING_LENGTH {
        return err!(AuctionErrors::TitleTooLong);
    }
    auction.title = title;
    auction.active = false; // is this default?
    auction.bump = ctx.bumps.auction;
    auction.id = publisher.num_auctions;
    // increment the number of auctions for publisher
    publisher.num_auctions += 1;
    Ok(())
}


#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 32 + size_of::<Auction>(),
        seeds = [
            b"auction".as_ref(), 
            authority.key().as_ref(), 
            &publisher.num_auctions.to_le_bytes()
        ],
        bump
    )]
    pub auction: Account<'info, Auction>,
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


#[account]
pub struct Auction {
    pub publisher: Pubkey,
    pub winner: Pubkey,
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
    pub title: String,
    // timestamp frequency in seconds
    pub start_time: u64,
    pub end_time: u64,
    pub active: bool,
    pub rounds_left: u64,
    pub id: u64,
    bump: u8,
}

pub fn activate_auction(ctx: Context<ActivateAuction>, duration: u64, num_rounds: u64) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    if auction.active {
        return err!(AuctionErrors::AuctionAlreadyActive);
    }
    auction.active = true;
    if num_rounds == 0 {
        auction.rounds_left = u64::MAX;
    } else {
        auction.rounds_left = num_rounds;
    }
    let clock = Clock::get()?;
    auction.start_time = clock.unix_timestamp.unsigned_abs();
    auction.end_time = auction.start_time + duration;
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

// check for the end of the auction, and set the winner
fn check_status(auction: &mut Auction) {
    let clock = Clock::get().unwrap();
    if clock.unix_timestamp.unsigned_abs() > auction.end_time {
        auction.active = false;
        // handle end of auction
        auction.winner = auction.highest_bidder;
        return;
    }
}


pub fn bid(ctx: Context<Bid>, amount: u64) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let advertiser = &mut ctx.accounts.advertiser;
    let user = &ctx.accounts.user;
    check_status(auction);
    if amount <= auction.highest_bid {
        return err!(AuctionErrors::NotHighestBid);
    }
    if !auction.active {
        return err!(AuctionErrors::AuctionNotActive);
    }

    // transfer amount from user to auction in case they win
    let transfer = system_instruction::transfer(
        &user.key(),
        &auction.key(),
        amount,
    );

   solana_program::program::invoke_signed(
        &transfer,
        &[
            user.to_account_info(),
            auction.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;
    msg!("bid successful. transferred {} lamports from {} to {}", amount, user.key(), auction.key());

    // refund previous highest bidder
    if auction.highest_bidder != Pubkey::default() {
        let refund = system_instruction::transfer(
            &auction.key(),
            &auction.highest_bidder,
            auction.highest_bid,
        );
        solana_program::program::invoke_signed(
            &refund,
            &[
                auction.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
        msg!("refunded {} lamports to {}", auction.highest_bid, auction.highest_bidder);
    }
    auction.highest_bid = amount;
    auction.highest_bidder = user.key();
    advertiser.num_bids += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub advertiser: Account<'info, Advertiser>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum AuctionErrors {
    #[msg("Title too long")]
    TitleTooLong,
    #[msg("Auction is already active")]
    AuctionAlreadyActive,
    #[msg("Auction is not active")]
    AuctionNotActive,
    #[msg("Not highest bid")]
    NotHighestBid,
}