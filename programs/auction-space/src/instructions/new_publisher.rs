use anchor_lang::prelude::*;
use crate::state::*;

pub fn handle_new_publisher(ctx: Context<NewPublisher>) -> Result<()> {
    msg!("new publisher");
    let publisher = &mut ctx.accounts.publisher;
    publisher.publisher_wallet = ctx.accounts.publisher_wallet.key();
    publisher.bump = ctx.bumps.publisher;
    Ok(())
}

#[derive(Accounts)]
pub struct NewPublisher<'info> {
    #[account(
        init, 
        payer = publisher_wallet, 
        space = PUBLISHER_SIZE,
        seeds = [b"publisher", publisher_wallet.key().as_ref()],
        bump
    )]
    pub publisher: Account<'info, Publisher>,
    #[account(mut)]
    pub publisher_wallet: Signer<'info>,
    pub system_program: Program<'info, System>
}