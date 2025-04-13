use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use crate::{constant::Constant::{ADD, DELETE}, FundingAccountRecord};



pub fn check_record_lists(
    funding_list_account: &mut FundingAccountRecord,
    check_name: String,
    operation: u8) -> ProgramResult {

    let lists =  funding_list_account;

    if operation == ADD {
        let check_bytes = check_name.as_bytes();

        if (lists.account_lists.len() % 32 + check_bytes.len() + 1) > 32
        || (lists.account_lists.len() > 0 && lists.account_lists.len() % 32 == 0){
            msg!("space is not enough");
            //
        }else {
            lists.account_lists.extend_from_slice(check_bytes);
            lists.account_lists.extend_from_slice(".".as_bytes());
        } 
    }else{
        let will_remove = format!("{}.", &check_name);
        let will_remove_bytes = will_remove.as_bytes();

        if let Some(pos) = 
            lists.account_lists.windows(will_remove_bytes.len()).position(|w| w == will_remove_bytes){
            lists.account_lists.drain(pos..pos + will_remove.len());
        }else {
            msg!("can't find the root domain");
            return Err(ProgramError::InvalidArgument);
        }
    }

    Ok(())
}