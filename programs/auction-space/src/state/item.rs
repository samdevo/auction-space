use anchor_lang::prelude::*;

#[account]
pub struct Item {
    pub title: String,
    pub url: String,
    pub id: u64,
    pub num_auctions: u64,
    pub num_auctions_completed: u64,
    pub num_auctions_aborted: u64,
    pub publisher: Pubkey, 
    pub active_auction: Pubkey,
    // if active, the time the auction ends
    pub active_until: u64,
    pub bump: u8,
}

pub const ITEM_SIZE: usize = 
    32 + // title
    32 + // url
    8 + // id
    8 + // num_auctions
    8 + // num_auctions_completed
    8 + // num_auctions_aborted
    8 + // publisher
    8 + // auction
    8 + // active_until
    1 + // bump
    128;  // padding

