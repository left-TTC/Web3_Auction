use anchor_lang::prelude::*;
use constant::constant::{VAULT, WEB3_NAME_SERVICE};
use web3nameservice::program::Web3NameService;

declare_id!("2UFgq1J3PJFNMbFGiDF7BvX7HdBVwuu16nVz68gk1ZAW");

pub mod constant;
pub mod processor;
pub mod cpi;
pub mod utils;

#[program]
pub mod auction {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

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
        space = 8 + 8 + 32 + 8,
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

#[cfg(test)]
mod test {
    use crate::{constant::constant::{ADD, DELETE}, utils::check_record_lists};



    #[test]
    fn test_check_record_lists() {
        //construct a new vec by our structure
        let mut record_lists: Vec<u8> = Vec::new();

        println!("[1] lists start: {:?}", record_lists);
        println!("[ ] start length: {}", record_lists.len());

        let frist_add_name = String::from("000");

        check_record_lists(
            &mut record_lists, frist_add_name.clone(), ADD).unwrap();

        println!("[2] add one: {:?}", record_lists);

        let second_add_name = String::from("xyasasdaz");

        check_record_lists(
            &mut record_lists, second_add_name.clone(), ADD).unwrap();

        println!("[3] add two: {:?}", record_lists);

        check_record_lists(
            &mut record_lists, second_add_name.clone(), DELETE).unwrap();

        println!("[4] delete the two: {:?}", record_lists);

        let thrid_add_name = String::from("xy");

        check_record_lists(
            &mut record_lists, thrid_add_name, ADD).unwrap();

        println!("[5] three add: {:?}", record_lists);
        println!("[ ] now length: {}", record_lists.len());

        let thrid_add_name = String::from("abc");

        check_record_lists(
            &mut record_lists, thrid_add_name, ADD).unwrap();

        println!("[6] four add: {:?}", record_lists);
        println!("[ ] now length: {}", record_lists.len());
        
    }
}


