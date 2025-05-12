
use bonfida_utils::checks::check_account_owner;
use solana_program::{
    msg,
    program::{invoke, invoke_signed},
    rent::Rent,
    sysvar::Sysvar,
};
use spl_name_service::state::{NameRecordHeader};

use crate::{central_state, state::record_header::RecordHeader, utils::{get_hashed_name, get_seeds_and_key}};

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
        system_instruction,
        system_program,
    },
};



#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {
    pub root_name: String,
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
            create_fee_saver: next_account_info(accounts_iter)?,
        };

        // Check keys
        check_account_key(accounts.system_program, &system_program::ID)?;

        // Check owners
        check_account_owner(accounts.root_cord_account, &system_program::ID)?;

        // Check signer
        check_signer(accounts.fee_payer)?;

        Ok(accounts)
    }
}

pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], params: Params) -> ProgramResult {
    let accounts = Accounts::parse(accounts)?;

    let (root_record_key, seeds) = get_seeds_and_key(
        &crate::ID, 
        get_hashed_name(&params.root_name), 
        None, 
        None
    );

    let (fee_saver_key, _) = get_seeds_and_key(
        &crate::ID, 
        get_hashed_name(&params.root_name), 
        Some(&central_state::KEY), 
        Some(&central_state::KEY)
    );

    let root_record_account = accounts.root_cord_account;

    if root_record_key != *root_record_account.key {
        msg!("The given root account is incorrect.");
        return Err(ProgramError::InvalidArgument);
    }

    check_account_key(accounts.create_fee_saver, &fee_saver_key)?;

    if root_record_account.data.borrow().len() > 0 {
        let root_record_header = 
            RecordHeader::unpack_from_slice(&root_record_account.data.borrow())?;
        if root_record_header.root_name_key != Pubkey::default() {
            msg!("The given root account already exists.");
            return Err(ProgramError::InvalidArgument);
        }
    }

    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(RecordHeader::LEN);
    let create_fee = rent.minimum_balance(NameRecordHeader::LEN * 6);

    if root_record_account.data.borrow().len() == 0 {
        invoke(
            &system_instruction::transfer(
                accounts.fee_payer.key, &root_record_key, lamports), 
                &[
                    accounts.fee_payer.clone(),
                    accounts.root_cord_account.clone(),
                    accounts.system_program.clone(),
                ],
            )?;

        invoke_signed(
            &system_instruction::allocate(
                &root_record_key, 
                RecordHeader::LEN as u64
            ), 
            &[accounts.root_cord_account.clone(), accounts.system_program.clone()], 
            &[&seeds.chunks(32).collect::<Vec<&[u8]>>()],
        )?;

        invoke_signed(
            &system_instruction::assign(&root_record_key, &crate::ID),
            &[accounts.root_cord_account.clone(), accounts.system_program.clone()],
            &[&seeds.chunks(32).collect::<Vec<&[u8]>>()],
        )?;

        invoke(
            &system_instruction::transfer(
                accounts.fee_payer.key, accounts.create_fee_saver.key, create_fee), 
                &[
                    accounts.fee_payer.clone(),
                    accounts.create_fee_saver.clone(),
                    accounts.system_program.clone(),
                ]
            )?;
    }
    

    let init_state = RecordHeader {
        root_name_key: root_record_key,
        amount: 0,
        name: params.root_name,
    };

    init_state.pack_into_slice(&mut accounts.root_cord_account.data.borrow_mut());

    Ok(())
}