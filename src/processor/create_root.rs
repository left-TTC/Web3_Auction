
use bonfida_utils::checks::check_account_owner;
use solana_program::{
    msg,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_name_service::state::NameRecordHeader;

use crate::{
    central_state, state::{constants::CREATE_FEE, record_header::{write_data, RecordHeader}}, utils::{get_hashed_name, get_seeds_and_key}
};

use {
    bonfida_utils::{
        checks::{check_account_key, check_signer},
        BorshSize, InstructionsAccount,
    },
    borsh::{BorshDeserialize, BorshSerialize},
    // sns_sdk::record::Record,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        system_program,
    },
};

use crate::cpi;




#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {
    pub root_name: String,
    pub add: u64,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The system program account
    pub system_program: &'a T,

    /// The account te recieve fund
    #[cons(writable)]
    pub vault: &'a T,

    /// The accoount to save fund state
    #[cons(writable)]
    pub root_cord_account: &'a T,

    #[cons(writable, signer)]
    /// The fee payer account
    pub fee_payer: &'a T,

    pub web3_name_service: &'a T,

    pub register_service_central: &'a T,

    #[cons(writable)]
    pub root_name_account: &'a T,

    #[cons(writable)]
    pub reverse_lookup: &'a T,

    pub central_state: &'a T,

    pub rent_sysvar: &'a T,

    #[cons(writable)]
    pub create_fee_saver: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(accounts: &'a [AccountInfo<'b>]) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Accounts {
            system_program: next_account_info(accounts_iter)?,
            vault: next_account_info(accounts_iter)?,
            root_cord_account: next_account_info(accounts_iter)?,
            fee_payer: next_account_info(accounts_iter)?,
            web3_name_service: next_account_info(accounts_iter)?,
            register_service_central: next_account_info(accounts_iter)?,
            root_name_account: next_account_info(accounts_iter)?,
            reverse_lookup: next_account_info(accounts_iter)?,
            central_state: next_account_info(accounts_iter)?,
            rent_sysvar: next_account_info(accounts_iter)?,
            create_fee_saver: next_account_info(accounts_iter)?,
        };

        // Check keys
        check_account_key(accounts.system_program, &system_program::ID)?;
        // check_account_key(accounts.web3_name_service, &spl_name_service::ID)?;

        // Check owners
        check_account_owner(accounts.root_cord_account, &crate::ID)?;

        // Check signer
        check_signer(accounts.fee_payer)?;

        Ok(accounts)
    }
}


pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], params: Params) -> ProgramResult {
    if params.add < 1000 {
        msg!("add amount is too small");
        return Err(ProgramError::InvalidArgument);
    }

    let accounts = Accounts::parse(accounts)?;

    let hashed_name_account = get_hashed_name(&params.root_name);
    
    let (root_record_key, _) = get_seeds_and_key(
        &crate::ID, 
        hashed_name_account.clone(), 
        None, 
        None);

    let (root_name_key, _) = get_seeds_and_key(
        accounts.web3_name_service.key,
        hashed_name_account.clone(), 
        None, 
        None
    );

    let hashed_reverse_lookup = get_hashed_name(&root_name_key.to_string());

    let (reserse_look_up, _) = get_seeds_and_key(
        accounts.web3_name_service.key, 
        hashed_reverse_lookup.clone(), 
        Some(&central_state::KEY), 
        None
    );

    let (fee_saver_key, seeds) = get_seeds_and_key(
        &crate::ID, 
        get_hashed_name(&params.root_name), 
        Some(&central_state::KEY), 
        Some(&central_state::KEY)
    );

    let root_record_account = accounts.root_cord_account;

    check_account_key(accounts.root_cord_account, &root_record_key)?;
    check_account_key(accounts.root_name_account, &root_name_key)?;

    msg!("check reverse, central is {}", central_state::KEY);
    check_account_key(accounts.reverse_lookup, &reserse_look_up)?;

    check_account_key(accounts.create_fee_saver, &fee_saver_key)?;

    let root_record_header = 
        RecordHeader::unpack_from_slice(&root_record_account.data.borrow())?;

    let new_amount = root_record_header.amount + params.add;

    let rent = Rent::get()?;

    let central_state_signer_seeds: &[&[u8]] = &[&crate::ID.to_bytes(), &[central_state::NONCE]];

    if new_amount >= CREATE_FEE {
        msg!("create root account");
        cpi::create_name_account(
            accounts.web3_name_service,
            accounts.system_program,
            accounts.root_name_account,
            accounts.create_fee_saver,
            accounts.register_service_central,
            hashed_name_account,
            rent.minimum_balance(NameRecordHeader::LEN),
            0,
            &seeds,
        )?;

        msg!("create root reverse account");
        if accounts.reverse_lookup.data_len() == 0 {
            cpi::create_reverse_lookup_account(
                accounts.web3_name_service, 
                accounts.system_program, 
                accounts.reverse_lookup, 
                accounts.create_fee_saver, 
                params.root_name, 
                hashed_reverse_lookup, 
                accounts.central_state, 
                accounts.rent_sysvar, 
                central_state_signer_seeds, 
                &seeds,
                None, 
                None
            )?;
        }
    }

    let bytes = new_amount.to_le_bytes();
    write_data(root_record_account, &bytes, 32);

    Ok(())
}
