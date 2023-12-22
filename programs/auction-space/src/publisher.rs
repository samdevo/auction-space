use anchor_lang::prelude::*;
const MAX_URL_LENGTH: usize = 16;
use std::mem::size_of;

pub fn new_publisher(ctx: Context<NewPublisher>) -> Result<()> {
    let publisher = &mut ctx.accounts.publisher;
    publisher.authority = ctx.accounts.user.key();
    publisher.num_auctions = 0;
    publisher.num_successful_auctions = 0;
    // create an empty list of strings
    publisher.bump = ctx.bumps.publisher;
    Ok(())
}

#[account]
pub struct Publisher {
    pub authority: Pubkey,
    pub num_auctions: u64,
    pub num_successful_auctions: u64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct NewPublisher<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + size_of::<Publisher>() + MAX_URL_LENGTH as usize,
        seeds = [b"publisher", user.key().as_ref()],
        bump
    )]
    pub publisher: Account<'info, Publisher>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,

}