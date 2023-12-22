use anchor_lang::prelude::*;

mod publisher;
use publisher::*;

mod advertiser;
use advertiser::*;

mod auction;
use auction::*;

declare_id!("APkuWosZy33sLc5mbsPfWzi3VRbC9RTAqyZbspkECd5j");

#[program]
pub mod auction_space {
    use super::*;

    // initialize a new publisher 
    pub fn new_publisher(ctx: Context<NewPublisher>) -> Result<()> {
        publisher::new_publisher(ctx)
    }

    pub fn new_advertiser(ctx: Context<NewAdvertiser>) -> Result<()> {
        advertiser::new_advertiser(ctx)
    }

    pub fn create_auction(ctx: Context<CreateAuction>, title: String) -> Result<()> {
        auction::create_auction(ctx, title)
    }

    pub fn activate_auction(ctx: Context<ActivateAuction>, min_price: u64, duration: u64, num_rounds: u64) -> Result<()> {
        auction::activate_auction(ctx, min_price, duration, num_rounds)
    }

    pub fn deactivate_auction(ctx: Context<DeactivateAuction>) -> Result<()> {
        auction::deactivate_auction(ctx)
    }
}


