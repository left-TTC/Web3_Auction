use std::cmp::Ordering;

use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_lang::system_program::{Transfer, transfer};

use crate::ReallocListService;


pub fn realloc(
    ctx: Context<ReallocListService>,
    num: u8,
) -> ProgramResult {

    let magnification = num as usize;

    let funding_list_account = &mut ctx.accounts.crowding_account_lists;

    let origin_space = funding_list_account.account_lists.len() / 32 + 1;

    if magnification < origin_space {
        msg!("now: {}", origin_space);
        msg!("can't be smaller than before");
        return Err(ProgramError::InvalidArgument);
    }

    Ok(())
}