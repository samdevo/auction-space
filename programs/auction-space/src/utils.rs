use anchor_lang::{prelude::*, solana_program::{self, system_instruction, entrypoint::ProgramResult}};

pub fn transfer_pda_to_user<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    amount: u64,
    seeds: &[&[u8]],
) -> ProgramResult{
    let transfer = system_instruction::transfer(
        from.key,
        to.key,
        amount,
    );
    msg!("transferring {} lamports from {} to {}", amount, from.key, to.key);
    return solana_program::program::invoke_signed(
        &transfer,
        &[
            from,
            to,
            system_program,
        ],
        &[seeds]
    );
}

pub fn transfer_user_to_pda<'info>(
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
    msg!("transferring {} lamports from {} to {}", amount, from.key(), to.key());
    return solana_program::program::invoke(
        &transfer,
        &[
            from.clone(),
            to.clone(),
            system_program.clone(),
        ],
    );
}

