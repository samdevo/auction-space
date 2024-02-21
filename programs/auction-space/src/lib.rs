use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

pub use instructions::*;
pub use state::*;
pub use errors::*;

declare_id!("41vpQQjubJn25HVHTLTt6YZViRM9hefnd9DJ1PXvt5NE");

const MAX_STRING_LENGTH: usize = 32;
const PUBLISHER_DEPOSIT: u64 = 10000;

#[program]
pub mod auction_space {
    use super::*;

    pub fn new_publisher(ctx: Context<NewPublisher>) -> Result<()> {
        instructions::handle_new_publisher(ctx)
    }

    pub fn new_advertiser(ctx: Context<NewAdvertiser>) -> Result<()> {
        instructions::handle_new_advertiser(ctx)
    }

    // pub fn new_auction(ctx: Context<NewAuction>, title: String) -> Result<()> {
    //     new_auction(
    //         ctx, 
    //         title
    //     )
    // }

    pub fn new_auction(ctx: Context<NewAuction>, title: String, min_bid: u64, auction_start: u64, auction_end: u64, effect_start: u64, effect_end: u64) -> Result<()> {
        instructions::handle_new_auction(
            ctx,
            title,
            min_bid, 
            auction_start, 
            auction_end, 
            effect_start, 
            effect_end
        )
    }

    // pub fn upload_ad(ctx: Context<UploadAd>, url: String) -> Result<()> {
    //     auction::upload_ad(ctx, url)
    // }

    pub fn bid(ctx: Context<Bid>, bid_amount: u64) -> Result<()> {
        instructions::handle_bid(ctx, bid_amount)
    }

    // pub fn publisher_backout(ctx: Context<PublisherBackout>) -> Result<()> {
    //     auction_backout::publisher_backout(ctx)
    // }

    // pub fn advertiser_backout(ctx: Context<AdvertiserBackout>) -> Result<()> {
    //     auction_backout::advertiser_backout(ctx)
    // }

    // pub fn transfer_to_from(ctx: Context<TestTransfer>) -> Result<()> {
    //     Ok(())
    // }

}
