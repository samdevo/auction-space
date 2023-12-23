use anchor_lang::prelude::*;

mod publisher;
use publisher::*;

mod advertiser;
use advertiser::*;

mod auction;
use auction::*;

declare_id!("APkuWosZy33sLc5mbsPfWzi3VRbC9RTAqyZbspkECd5j");

const MAX_STRING_LENGTH: usize = 16;


#[program]
pub mod auction_space {
    use super::*;

    // initialize a new publisher 
    pub fn new_publisher(ctx: Context<NewPublisher>) -> Result<()> {
        msg!("creating new publisher");
        publisher::new_publisher(ctx)
    }

    pub fn new_advertiser(ctx: Context<NewAdvertiser>) -> Result<()> {
        advertiser::new_advertiser(ctx)
    }

    pub fn create_auction(ctx: Context<CreateAuction>, title: String) -> Result<()> {
        auction::create_auction(ctx, title)
    }

    pub fn activate_auction(ctx: Context<ActivateAuction>, auction_end: u64, effect_start: u64, effect_end: u64) -> Result<()> {
        auction::activate_auction(ctx, auction_end, effect_start, effect_end)
    }

    // pub fn deactivate_auction(ctx: Context<DeactivateAuction>) -> Result<()> {
    //     auction::deactivate_auction(ctx)
    // }

    pub fn foo(ctx: Context<HelloWorld>) -> Result<()> {
        msg!("Hello world hello {}!", ctx.accounts.my_account.key());
        Ok(())
    }

    pub fn bid(ctx: Context<Bid>, bid_amount: u64) -> Result<()> {
        auction::bid(ctx, bid_amount)
    }

}

#[derive(Accounts)]
pub struct HelloWorld<'info> {
    pub my_account: Signer<'info>,
}
