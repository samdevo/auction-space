use anchor_lang::{prelude::*, solana_program};
use solana_program::system_instruction;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct NewAuction<'info> {
    #[account(
        init, 
        payer = publisher_wallet, 
        space = AUCTION_SIZE + 128,
        seeds = [
            b"auction", 
            publisher_wallet.key().as_ref(), 
            &publisher.num_auctions.to_le_bytes()
        ],
        bump
    )]
    pub auction: Account<'info, Auction>,
    #[account(
        seeds = [b"publisher", publisher_wallet.key().as_ref()],
        bump = publisher.bump,
        has_one = publisher_wallet,
        mut
    )]
    pub publisher: Account<'info, Publisher>,
    #[account(mut)]
    pub publisher_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_new_auction(
    ctx: Context<NewAuction>,
    item: Pubkey,
    min_bid: u64,
    auction_start: u64,
    auction_end: u64,
    effect_start: u64,
    effect_end: u64
) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let publisher = &mut ctx.accounts.publisher;
    let publisher_wallet = &ctx.accounts.publisher_wallet;

    auction.cur_winner_wallet = publisher_wallet.key();

    // if title.len() > crate::MAX_STRING_LENGTH {
    //     return err!(AuctionErrors::TitleTooLong);
    // }

    let clock = Clock::get()?;
    let timestamp = clock.unix_timestamp.unsigned_abs();
    if timestamp > auction_end {
        return err!(AuctionErrors::AuctionAlreadyEnded);
    }
    if auction_end > effect_start {
        return err!(AuctionErrors::AuctionEffectBeforeEnd);
    }
    if effect_start >= effect_end {
        return err!(AuctionErrors::AuctionEffectEndBeforeStart);
    }

    // transfer despo to the auction account

    let transfer = system_instruction::transfer(
        &publisher_wallet.key(),
        &auction.key(),
        crate::PUBLISHER_DEPOSIT,
    );

    solana_program::program::invoke(
        &transfer,
        &[
            publisher_wallet.to_account_info().clone(),
            auction.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;

    // set auction fields
    auction.item = item;
    auction.id = publisher.num_auctions;
    auction.bump = ctx.bumps.auction;
    auction.min_bid = min_bid;
    auction.publisher = publisher.key();
    auction.publisher_wallet = publisher_wallet.key();
    auction.start_time = auction_start;
    auction.end_time = auction_end;
    auction.effect_start_time = effect_start;
    auction.effect_end_time = effect_end;
    auction.bump = ctx.bumps.auction;

    // increment publisher num_auctions for unique auction id
    publisher.num_auctions += 1;

    // send an escrow to the auction account

    Ok(())
}
