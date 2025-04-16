use anchor_lang::prelude::*;
use constant::constant::{VAULT, WEB3_NAME_SERVICE};
use web3nameservice::program::Web3NameService;

declare_id!("GFYGJHyekQawB4aTRerjAGighr64BYCwTaMGe3C11dCS");

pub mod constant;
pub mod processor;
pub mod cpi;
pub mod utils;

#[program]
pub mod auction {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use crate::utils::realloc_list_account_space;

    use super::*;

    pub fn create_funding(
        ctx: Context<CreateCrowdedService>,
        root_name: String) -> ProgramResult {
        processor::Processor::create_funding_account(ctx, root_name)
    }

    pub fn add_funding(
        ctx: Context<AddFundingService>,
        add: u64,
        funding_name: String) -> ProgramResult {
        processor::Processor::add_funding_amount(ctx, add, funding_name)
    }

    pub fn test_realloc(
        ctx: Context<FunctionTest>,
    ) -> ProgramResult {
        realloc_list_account_space(&mut ctx.accounts.crowding_account_lists, &ctx.accounts.system_program, &ctx.accounts.payer)
    }

}

#[derive(Accounts)]
pub struct CreateCrowdedService<'info> {
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    // will create root account this time
    // calculate by the web3 name service
    pub will_create_root: UncheckedAccount<'info>,

    #[account(mut)]
    caller: Signer<'info>,

    //record the state of fundrasing
    //in create we only init
    #[account(
        init,
        payer = caller,
        space = 8 + 8 + 32 + 8 + 1,
        seeds = [
            b"web3 Auction",
            will_create_root.key.to_bytes().as_ref(),
        ],
        bump
    )]
    pub fundraising_state_account: Account<'info, CrowdfundingAccount>,

    pub system_program: Program<'info, System>,

    //record all crowding_account
    //When enough money is collected
    //create desinated domain and delete it from the lists
    #[account(
        init_if_needed,
        payer = caller,
        space = 8 + 4 + 32,
        seeds = [
            b"web3 auction account list",
        ],
        bump
    )]
    pub crowding_account_lists: Account<'info, FundingAccountRecord>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    #[account(
        mut,
        address = VAULT,
    )]
    pub vault: UncheckedAccount<'info>,
}

#[account]
pub struct CrowdfundingAccount {
    //Calculate the current amount raised
    raised_amount: u64,

    funding_root: Pubkey,

    funding_target: u64,
}

#[account]
pub struct FundingAccountRecord {
    account_lists: Vec<u8>,
} 

#[derive(Accounts)]
pub struct AddFundingService<'info> {
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    #[account(
        mut,
        address = VAULT,
    )]
    pub vault: UncheckedAccount<'info>,

    #[account(
        address = WEB3_NAME_SERVICE
    )]
    pub web3_name_service: Program<'info, Web3NameService>,

    #[account(
        mut,
        seeds = [
            b"web3 auction account list",
        ],
        bump
    )]
    pub crowding_account_lists: Account<'info, FundingAccountRecord>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    //This is the record account derived from the public key of the auction account
    //will record all the root domain
    //calculate by the auction pubkey
    #[account(mut)]
    pub all_root_record_account: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [
            b"web3 Auction",
            will_create_root.key.to_bytes().as_ref(),
        ],
        bump
    )]
    pub fundraising_state_account: Account<'info, CrowdfundingAccount>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    // will create root account this time
    // calculate by the web3 name service
    #[account(
        address = fundraising_state_account.funding_root,
        mut
    )]
    pub will_create_root: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub payer: Signer<'info>,
}


#[derive(Accounts)]
pub struct FunctionTest<'info> {
    #[account(
        mut,
        seeds = [
            b"web3 auction account list",
        ],
        bump
    )]
    pub crowding_account_lists: Account<'info, FundingAccountRecord>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

















#[cfg(test)]
mod test {
    use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult, Discriminator};

    use crate::{constant::constant::{ADD, DELETE}, utils::{check_record_lists, find_subsequence}, FundingAccountRecord};


    fn test_fn(
        list: &mut Vec<u8>,
        check_name: String,
        operation: u8,
    ) -> ProgramResult {
        const BLOCK_SIZE: usize = 32;

        match operation {
            ADD => {
                const SEPARATION: &[u8] = b".";
    
                let check_bytes = check_name.as_bytes();
                let require_space = check_bytes.len() + SEPARATION.len();
    
                let list_len = list.len();
    
                //First, get the space that can be covered in the account
                let mut could_fill_zero: usize = 0;
                for i in 0..list_len {
                    if list[i] == 0 {
                        could_fill_zero += 1;
                    }
                }
    
                let useful_space = BLOCK_SIZE - list_len % BLOCK_SIZE;
    
                if ((useful_space + could_fill_zero) < require_space) || ((useful_space == 0) && (list_len > 0)){
                    msg!("space is not enough");
                    //Reallocate space
                    //realloc_list_account_space(funding_list_account, &system_account, &payer)?;
                }
    
                list.truncate(list_len - could_fill_zero);
    
                let will_add_zero = if could_fill_zero > require_space {
                    could_fill_zero - require_space
                } else {
                    0
                };
    
                list.extend_from_slice(check_bytes);
                list.extend_from_slice(SEPARATION);
    
                if will_add_zero > 0 {
                    list.extend(std::iter::repeat(0).take(will_add_zero));
                }
            }
            DELETE => {
                const SEPARATION: u8 = b'.';
    
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

    #[test]
    fn test_check_record_lists<'a>() {
        //construct a new vec by our structure
        let mut record_lists: Vec<u8> = Vec::new();

        println!("[1] lists start: {:?}", record_lists);
        println!("[ ] start length: {}", record_lists.len());

        let frist_add_name = String::from("000");

        test_fn(
            &mut record_lists, frist_add_name.clone(), ADD).unwrap();

        println!("[2] add one: {:?}", record_lists);

        let second_add_name = String::from("xyasasdaz");

        test_fn(
            &mut record_lists, second_add_name.clone(), ADD).unwrap();

        println!("[3] add two: {:?}", record_lists);

        test_fn(
            &mut record_lists, second_add_name.clone(), DELETE).unwrap();

        println!("[4] delete the two: {:?}", record_lists);

        let thrid_add_name = String::from("xy");

        test_fn(
            &mut record_lists, thrid_add_name, ADD).unwrap();

        println!("[5] three add: {:?}", record_lists);
        println!("[ ] now length: {}", record_lists.len());

        let thrid_add_name = String::from("abc");

        test_fn(
            &mut record_lists, thrid_add_name, ADD).unwrap();

        println!("[6] four add: {:?}", record_lists);
        println!("[ ] now length: {}", record_lists.len());
        
    }


    #[test]
    fn decode_funding(){
        let decode_aray: [u8; 36] = [32, 0, 0, 0, 108, 101, 111, 46, 108, 117, 111, 115, 97, 46, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 49, 46,];

        let result = FundingAccountRecord::deserialize(&mut decode_aray.as_ref()).unwrap();

        println!("decode list: {:?}", result.account_lists);
        println!("decoded list length: {}", result.account_lists.len());
        println!("des: {:?}", FundingAccountRecord::DISCRIMINATOR);
    }
}


