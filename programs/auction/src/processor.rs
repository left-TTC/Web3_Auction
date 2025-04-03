use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::prelude::*;
use createorcheck::check;

use crate::crowded_service;
use crate::CrowdInfo;

pub mod createorcheck;

pub struct Processor {}

impl Processor {
    
    pub fn add_amount_or_create_funding_account(
        ctx: Context<crowded_service>,
        data: CrowdInfo
    ) -> ProgramResult {
        msg!("start check");
        check(ctx, data)
    }
}


