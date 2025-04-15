use anchor_lang::{accounts::system_account, prelude::*, solana_program::entrypoint::ProgramResult};
use crate::{constant::constant::{ADD, DELETE}, FundingAccountRecord};



pub fn check_record_lists(
    funding_list_account_lists: &mut Vec<u8>,
    check_name: String,
    operation: u8) -> ProgramResult {

    const BLOCK_SIZE: usize = 32;

    match operation {
        ADD => {
            const SEPARATION: &[u8] = b".";

            let check_bytes = check_name.as_bytes();
            let require_space = check_bytes.len() + SEPARATION.len();

            //First, get the space that can be covered in the account
            let mut could_fill_zero: usize = 0;
            for i in 0..funding_list_account_lists.len() {
                if funding_list_account_lists[i] == 0 {
                    could_fill_zero += 1;
                }
            }

            let useful_space = BLOCK_SIZE - funding_list_account_lists.len() % BLOCK_SIZE;

            if ((useful_space + could_fill_zero) < require_space) || ((useful_space == 0) && (funding_list_account_lists.len() > 0)){
                msg!("space is not enough");
                //Reallocate space

                return Err(ProgramError::InvalidArgument);
            }

            funding_list_account_lists.truncate(funding_list_account_lists.len() - could_fill_zero);

            let will_add_zero = if could_fill_zero > require_space {
                could_fill_zero - require_space
            } else {
                0
            };

            funding_list_account_lists.extend_from_slice(check_bytes);
            funding_list_account_lists.extend_from_slice(SEPARATION);

            if will_add_zero > 0 {
                funding_list_account_lists.extend(std::iter::repeat(0).take(will_add_zero));
            }
        }
        DELETE => {
            const SEPARATION: u8 = b'.';

            let delete_bytes = check_name.as_bytes();
            let mut delete_block = Vec::with_capacity(delete_bytes.len() + 1);
            delete_block.extend_from_slice(delete_bytes);
            delete_block.push(SEPARATION);

            if let Some(pos) = find_subsequence(&funding_list_account_lists, &delete_block){
                let end_pos = pos + delete_block.len();
                funding_list_account_lists.splice(pos..end_pos, std::iter::empty());

                let zeros_to_fill = delete_block.len();
                funding_list_account_lists.extend(std::iter::repeat(0).take(zeros_to_fill));
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


fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

pub fn realloc_account_space(
    system_account: Program<System>,
    payer: Signer,
    account_to_realloc: Pubkey,
) -> ProgramResult {


    Ok(())
}