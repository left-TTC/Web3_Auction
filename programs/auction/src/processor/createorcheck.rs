use anchor_lang::solana_program::program::invoke;
use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use left_utils::{get_hashed_name, get_PDA_key};
use anchor_lang::solana_program::system_instruction;

use crate::constant::Constant::{ISSUE_PRICE, WEB3_NAME_SERVICE};

use crate::cpi::Cpi::{self, create_root_domain};
use crate::{crowded_service, CrowdInfo};


pub fn check(
    ctx: Context<crowded_service>,
    data: CrowdInfo,
) -> ProgramResult {
    msg!("start add amount to crowd funding");

    let (cal_root_key, _) = get_PDA_key(
        &WEB3_NAME_SERVICE, get_hashed_name(&data.root_name), None
    );

    if cal_root_key != *ctx.accounts.will_create_root.key {
        msg!("coming root key is error");
        return Err(ProgramError::InvalidArgument);
    }

    let if_init_flag = ctx.accounts.funding_record_account.to_account_info();
    if if_init_flag.to_account_info().data_is_empty() {
        msg!("the account is not exsist");
        msg!("create a a record account");
        let raised_target = ISSUE_PRICE;

        let record_account = &mut ctx.accounts.funding_record_account;
        record_account.funding_root = cal_root_key;
        record_account.funding_target = raised_target;

        msg!("add this funding root into funding account lists");

        let funding_lists = &mut ctx.accounts.crowding_account_lists;
        
        if funding_lists.account_lists.len() == 0{
            msg!("this is the frist root funding");
            
            let mut new_list: Vec<u8> = Vec::new();
            new_list.extend_from_slice(data.root_name.as_bytes());
            new_list.extend_from_slice(".".as_bytes());
            funding_lists.account_lists = new_list;
        }else {
            let new_add = data.root_name.as_bytes();
            if (funding_lists.account_lists.len() + new_add.len() + 1) < 33 {
                funding_lists.account_lists.extend_from_slice(new_add);
                funding_lists.account_lists.extend_from_slice(".".as_bytes());
            }else {
                msg!("space is too small");
                //re
            }
        }
    }else {
        if cal_root_key != ctx.accounts.funding_record_account.funding_root {
            msg!("coming root key is error");
            return Err(ProgramError::InvalidArgument);
        }
    }

    if data.paid_fees >0 {
        let ix = system_instruction::transfer(
            ctx.accounts.caller.key,
            ctx.accounts.vault.key,
            data.paid_fees,    
        );
    
        invoke(&ix, 
            &[
                ctx.accounts.caller.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ]
        )?;
    }

    msg!("transfer ok");

    let record_account = &mut ctx.accounts.funding_record_account;

    record_account.raise_amount += data.paid_fees;

    if record_account.raise_amount >= record_account.funding_target{
        msg!("CPI create a root domain account");
        create_root_domain(&ctx, data.root_name.clone())?;

        msg!("remove the root name from funding account lists");
        let will_remove = format!("{}.", &data.root_name);
        let will_remove_bytes = will_remove.as_bytes();

        let funding_lists = &mut ctx.accounts.crowding_account_lists;

        if let Some(pos) = 
            funding_lists.account_lists.windows(will_remove_bytes.len()).position(|w| w == will_remove_bytes){
            funding_lists.account_lists.drain(pos..pos + will_remove.len());
        }else {
            msg!("can't find the root domain");
            return Err(ProgramError::InvalidArgument);
        }

    }

    Ok(())
}





