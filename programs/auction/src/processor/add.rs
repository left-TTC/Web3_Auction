use anchor_lang::{prelude::{msg, Context, CpiContext}, solana_program::entrypoint::ProgramResult, ToAccountInfo};
use anchor_lang::system_program::{Transfer, transfer};
use crate::{constant::Constant::DELETE, cpi::cpi::create_root_domain, utils::check_record_lists, AddFundingService};
use anchor_lang::prelude::*;


pub fn add(
    ctx:Context<AddFundingService>,
    add_amount: u64,
    funding_name: String,
) -> ProgramResult {
    msg!("[1] start");

    if ctx.accounts.fundraising_state_account.raised_amount >= ctx.accounts.fundraising_state_account.funding_target {
        if !ctx.accounts.will_create_root.data_is_empty(){
            msg!("have created");
            return Err(ProgramError::InvalidArgument);
        }

        create_root_domain(
            &ctx.accounts.web3_name_service, 
            &ctx.accounts.will_create_root, 
            &ctx.accounts.all_root_record_account, 
            &ctx.accounts.system_program, 
            &ctx.accounts.payer, 
            funding_name.clone())?;

        check_record_lists(
            &mut ctx.accounts.crowding_account_lists, 
            funding_name, 
            DELETE)?;

        return Ok(());
    }

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer.to_account_info(), 
                to: ctx.accounts.vault.to_account_info() 
            }), 
            add_amount
    )?;
    msg!("[2] transfer amount to vault");

    let funding_state_account = &mut ctx.accounts.fundraising_state_account;
    funding_state_account.raised_amount += add_amount;
    msg!("[3] add amount to target amount");

    if funding_state_account.raised_amount >= funding_state_account.funding_target{
        msg!("have enough coin, start create root domain");
        create_root_domain(
            &ctx.accounts.web3_name_service, 
            &ctx.accounts.will_create_root, 
            &ctx.accounts.all_root_record_account, 
            &ctx.accounts.system_program, 
            &ctx.accounts.payer, 
            funding_name.clone())?;

        check_record_lists(
            &mut ctx.accounts.crowding_account_lists, 
            funding_name, 
            DELETE)?;
    }
    msg!("[4] check raise money ok");

    Ok(())
}