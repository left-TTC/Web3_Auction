use std::cmp::Ordering;

use anchor_lang::{accounts::system_account, prelude::*, solana_program::{entrypoint::ProgramResult, lamports}};
use crate::{constant::constant::{ADD, DELETE}, FundingAccountRecord};

use anchor_lang::system_program::{Transfer, transfer};



pub fn check_record_lists<'info>(
    funding_list_account: &mut Account<'info, FundingAccountRecord>,
    system_account: &Program<'info, System>,
    payer: &Signer<'info>,
    check_name: String,
    operation: u8) -> ProgramResult {

    const BLOCK_SIZE: usize = 32;

    match operation {
        ADD => {
            const SEPARATION: &[u8] = b".";

            let check_bytes = check_name.as_bytes();
            let require_space = check_bytes.len() + SEPARATION.len();

            let list_len = funding_list_account.account_lists.len();

            //First, get the space that can be covered in the account
            let mut could_fill_zero: usize = 0;
            for i in 0..list_len {
                if funding_list_account.account_lists[i] == 0 {
                    could_fill_zero += 1;
                }
            }

            let useful_space = BLOCK_SIZE - list_len % BLOCK_SIZE;

            if ((useful_space + could_fill_zero) < require_space) || ((useful_space == 0) && (list_len > 0)){
                msg!("space is not enough");
                //Reallocate space
                realloc_list_account_space(funding_list_account, &system_account, &payer)?;
            }

            funding_list_account.account_lists.truncate(list_len - could_fill_zero);

            let will_add_zero = if could_fill_zero > require_space {
                could_fill_zero - require_space
            } else {
                0
            };

            funding_list_account.account_lists.extend_from_slice(check_bytes);
            funding_list_account.account_lists.extend_from_slice(SEPARATION);

            if will_add_zero > 0 {
                funding_list_account.account_lists.extend(std::iter::repeat(0).take(will_add_zero));
            }
        }
        DELETE => {
            const SEPARATION: u8 = b'.';
            let list = &mut funding_list_account.account_lists;

            let delete_bytes = check_name.as_bytes();
            let mut delete_block = Vec::with_capacity(delete_bytes.len() + 1);
            delete_block.extend_from_slice(delete_bytes);
            delete_block.push(SEPARATION);

            if let Some(pos) = find_subsequence(list, &delete_block){
                let end_pos = pos + delete_block.len();
                list.splice(pos..end_pos, std::iter::empty());

                let zeros_to_fill = delete_block.len();
                list.extend(std::iter::repeat(0).take(zeros_to_fill));
            }else {
                msg!("can't find the root domain");
                return Err(ProgramError::InvalidArgument);
            }
        }
        _ => {
            msg!("unkonw instruction");
            return Err(ProgramError::InvalidArgument);
        }
    }

    Ok(())
}


pub fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

pub fn realloc_list_account_space<'a>(
    funding_list_account: &mut Account<'a, FundingAccountRecord>,
    system_account: &Program<'a, System>,
    payer: &Signer<'a, >,
) -> ProgramResult {

    let new_space = funding_list_account.to_account_info().data_len() + 32;
    let require_lamports = Rent::get()?.minimum_balance(new_space);

    let lamports = funding_list_account.get_lamports();
    msg!("lamport is :{}", lamports);

    match lamports.cmp(&require_lamports) {
        Ordering::Less => {
            transfer(
                CpiContext::new(
                    system_account.to_account_info(),
                    Transfer { 
                        from: payer.to_account_info(), 
                        to: funding_list_account.to_account_info() 
                    }),
                    require_lamports - lamports
                )?;
        }
        _ =>{
            msg!("balance is enough")
        }
    }

    funding_list_account.to_account_info().realloc(new_space, false)?;

    Ok(())
}

