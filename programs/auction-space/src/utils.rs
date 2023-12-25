use anchor_lang::{prelude::*, solana_program::{self, system_instruction, entrypoint::ProgramResult}};

pub fn transfer_pda_to_user<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    amount: u64,
) -> ProgramResult{
    let transfer = system_instruction::transfer(
        from.key,
        to.key,
        amount,
    );
    msg!("transferring {} lamports from {} to {}", amount, from.key, to.key);
    return solana_program::program::invoke(
        &transfer,
        &[
            from,
            to,
            system_program,
        ],
    );
}