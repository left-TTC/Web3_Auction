

use web3nameservice;

pub mod Cpi{
    use anchor_lang::{prelude::{Context, CpiContext}, solana_program::entrypoint::ProgramResult, ToAccountInfo};
    use left_utils::{get_PDA_key, get_hashed_name};
    use web3nameservice::{base_data, cpi::accounts::create_name_service};
    use anchor_lang::prelude::*;

    use crate::{constant::Constant::WEB3_NAME_SERVICE, crowded_service};

    use super::*;


    pub fn create_root_domain(
        ctx: &Context<crowded_service>,
        name: String,
    ) -> ProgramResult {
        //name_account:
        let root_name_account = &ctx.accounts.will_create_root;
        //record_account
        let (auction_record_account, _) = get_PDA_key(
            ctx.accounts.web3_name_service.key, get_hashed_name(&ctx.program_id.to_string()), None);
        
        if auction_record_account != *ctx.accounts.auction_record_account.key {
            msg!("coming record account key is error");
            return Err(ProgramError::InvalidArgument);
        }

        let Cpi_ctx = CpiContext::new(
            ctx.accounts.web3_name_service.to_account_info(), 
            create_name_service{
                name_account: root_name_account.to_account_info(),
                record_account: ctx.accounts.auction_record_account.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                payer: ctx.accounts.caller.to_account_info(),
                root_domain_opt: None,
            });

        let data = base_data {
            lamports:10000000,
            name: name,
            space: 0,
            owner: *ctx.program_id,
            ipfs: None,
        };

        web3nameservice::cpi::create(Cpi_ctx, data)?;

        Ok(())
    }
}