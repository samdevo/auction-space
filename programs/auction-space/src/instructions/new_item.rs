use anchor_lang::prelude::*;
use crate::state::*;

pub fn handle_new_item(
    ctx: Context<NewItem>,
    title: String,
    url: String,
) -> Result<()> {
    let publisher = &mut ctx.accounts.publisher;
    let item = &mut ctx.accounts.item;
    item.publisher = publisher.key();
    item.bump = ctx.bumps.item;
    item.id = publisher.num_items;
    publisher.num_items += 1;
    item.title = title;
    item.url = url;
    Ok(())
}

#[derive(Accounts)]
pub struct NewItem<'info> {
    #[account(
        init, 
        payer = publisher_wallet, 
        space = ITEM_SIZE,
        seeds = [b"item", publisher_wallet.key().as_ref(), &publisher.num_items.to_le_bytes()],
        bump
    )]
    pub item: Account<'info, Item>,
    #[account(mut)]
    pub publisher_wallet: Signer<'info>,
    #[account(
        seeds = [b"publisher", publisher_wallet.key().as_ref()],
        bump = publisher.bump,
        has_one = publisher_wallet,
        mut
    )]
    pub publisher: Account<'info, Publisher>,
    pub system_program: Program<'info, System>,
}