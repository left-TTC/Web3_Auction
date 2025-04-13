


pub mod cpi{
    use anchor_lang::{prelude::{Context, CpiContext}, solana_program::entrypoint::ProgramResult, ToAccountInfo};
    use left_utils::{get_PDA_key, get_hashed_name};
    use web3nameservice::{cpi::accounts::CreateNameService, program::Web3NameService, BaseData};
    use anchor_lang::prelude::*;
    use web3nameservice::cpi::create;


    pub fn create_root_domain<'a>(
        web3_name_service: &Program<'a, Web3NameService>,
        will_create_root_account: &UncheckedAccount<'a>,
        all_root_record_account: &UncheckedAccount<'a>,
        system_program: &Program<'a, System>,
        payer: &Signer<'a>,
        name: String,
    ) -> ProgramResult {
        msg!("start CPI");

        let hased_name = get_hashed_name(&name);

        //calculate the record_account whick record all the root domain
        let (auction_record_account, _) = get_PDA_key(
            web3_name_service.key, get_hashed_name(&crate::ID.to_string()), None);

        let (cal_root_pda, _) = get_PDA_key(
            web3_name_service.key, hased_name.clone(), None);
        
        if auction_record_account != *all_root_record_account.key 
        || cal_root_pda != *will_create_root_account.key {
            msg!("coming record account key is error");
            msg!("coming:{}", all_root_record_account.key);
            msg!("should be {}", auction_record_account);
            return Err(ProgramError::InvalidArgument);
        }
        msg!("[1] check PDA ok");

        let cpi_ctx = CpiContext::new(
            web3_name_service.to_account_info(), 
            CreateNameService{
                name_account: web3_name_service.to_account_info(),
                record_account: all_root_record_account.to_account_info(),
                system_program: system_program.to_account_info(),
                payer: payer.to_account_info(),
                root_domain_opt: None
            });

        let auction_data = BaseData{
            name: name,
            root: Pubkey::default(),
            owner: crate::ID,
            hased_name: hased_name,
            ipfs: None,
        };

        create(cpi_ctx, auction_data)?;
        msg!("[2] cpi ok");

        msg!("cpi create ok");

        Ok(())
    }
}