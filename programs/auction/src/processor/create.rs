use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use left_utils::{get_hashed_name, get_PDA_key};
use anchor_lang::system_program::{Transfer, transfer};

use crate::constant::constant::{ADD, ISSUE_PRICE, REGISTER_FUND_FEE, WEB3_NAME_SERVICE};

use crate::utils::check_record_lists;
use crate::CreateCrowdedService;


pub fn create(
    ctx: Context<CreateCrowdedService>,
    root: String,
) -> ProgramResult {
    msg!("[1] start");

    msg!("[2] check will create root");
    //create is root, so root_opt is None
    let (cal_root_key, _) = get_PDA_key(
        &WEB3_NAME_SERVICE, get_hashed_name(&root), None
    );

    if cal_root_key != *ctx.accounts.will_create_root.key {
        msg!("coming:{}", ctx.accounts.will_create_root.key);
        msg!("should be {}", cal_root_key);
        return Err(ProgramError::InvalidArgument);
    }
    
    let raise_state_account = &mut ctx.accounts.fundraising_state_account;

    raise_state_account.funding_root = cal_root_key;
    raise_state_account.funding_target = ISSUE_PRICE;

    let funding_lists = &mut ctx.accounts.crowding_account_lists;
    
    check_record_lists(&mut funding_lists.account_lists, root, ADD)?;

    let combined_str = String::from_utf8(ctx.accounts.crowding_account_lists.account_lists.clone()).unwrap_or_else(|_| "Invalid UTF-8".to_string());
    msg!("saved string: {}", combined_str);
    msg!("[3] init the funding state acocount ok");

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.caller.to_account_info(), 
                to: ctx.accounts.vault.to_account_info() 
            }), 
            REGISTER_FUND_FEE
    )?;
    msg!("[4] transfer fee to vault ok");

    msg!("create over");
    Ok(())
}





