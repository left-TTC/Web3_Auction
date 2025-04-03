use anchor_lang::prelude::*;
use constant::Constant::{VAULT, WEB3_NAME_SERVICE};

declare_id!("HBA191fKk9gWLk3TkVZoUMYnUuhhpiAdXkL43tXY5EJ8");

pub mod constant;
pub mod processor;
pub mod cpi;

#[program]
pub mod auction {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn check_funding_account(
        ctx: Context<crowded_service>,
        data: CrowdInfo) -> ProgramResult {
        crate::processor::Processor::add_amount_or_create_funding_account(ctx, data)
    }
}

#[derive(Accounts)]
pub struct crowded_service<'info> {
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    #[account(
        address = WEB3_NAME_SERVICE
    )]
    pub web3_name_service: UncheckedAccount<'info>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    pub auction_record_account: UncheckedAccount<'info>,

    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    pub will_create_root: UncheckedAccount<'info>,
    
    #[account(
        address = VAULT,
        mut,
    )]
    /// CHECK: This account is verified in the instruction logic to ensure its safety.
    pub vault: UncheckedAccount<'info>,

    //init's premise is PDA
    #[account(
        init_if_needed,
        payer = caller,
        space = 8 + 8 + 32 + 8,
        seeds = [
            b"web3 Auction",
            will_create_root.key.as_ref(),
        ],
        bump
    )]
    pub funding_record_account: Account<'info, CrowdfundingAccount>,

    #[account(mut)]
    caller: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        init_if_needed,
        payer = caller,
        space = 8 + 33,
        seeds = [
            b"unique web3 auction account list",
        ],
        bump
    )]
    pub crowding_account_lists: Account<'info, FundingAccountRecord>,
}

#[account]
pub struct CrowdInfo {
    root_name: String,

    paid_fees: u64,
}

#[account]
pub struct CrowdfundingAccount {
    //Calculate the current amount raised
    raise_amount: u64,

    funding_root: Pubkey,

    funding_target: u64,
}

#[account]
pub struct FundingAccountRecord {
    account_lists: Vec<u8>,
} 
