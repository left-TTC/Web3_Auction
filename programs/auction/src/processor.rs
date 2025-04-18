use add::add;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::prelude::*;
use create::create;
use realloc::realloc;

use crate::{AddFundingService, CreateCrowdedService, ReallocListService};

pub mod create;
pub mod add;
pub mod realloc;

pub struct Processor {}

impl Processor {
    
    pub fn create_funding_account(
        ctx: Context<CreateCrowdedService>,
        root_name: String,
    ) -> ProgramResult {
        create(ctx, root_name)
    }

    pub fn add_funding_amount(
        ctx: Context<AddFundingService>,
        add_amount: u64,
        funding_name: String,
    ) -> ProgramResult {
        add(ctx, add_amount, funding_name)
    }

    pub fn realloc_list_account_space(
        ctx: Context<ReallocListService>,
        add: u8,
    ) -> ProgramResult {
        realloc(ctx, add)
    }
}


