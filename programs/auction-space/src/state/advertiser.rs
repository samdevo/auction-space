use anchor_lang::prelude::*;

#[account]
pub struct Advertiser {
    pub advertiser_wallet: Pubkey,
    pub num_bids: u64,
    pub num_auctions_won: u64,
    pub publisher_backouts: u64,
    pub advertiser_backouts: u64,
    pub bump: u8,
}

pub const ADVERTISER_SIZE: usize = 
    8 + // authority
    8 + // num_bids
    8 + // num_auctions_won
    8 + // publisher_backouts
    8 + // advertiser_backouts
    1 + // bump
    32;  // padding

