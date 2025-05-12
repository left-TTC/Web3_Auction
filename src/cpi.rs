use crate::state::ReverseLookup;
use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

use spl_name_service::{instruction::NameRegistryInstruction, state::NameRecordHeader};

// use crate::utils::TEST_NAME_ID;

#[allow(clippy::too_many_arguments)]
pub fn create_name_account<'a>(
    name_service_program: &AccountInfo<'a>,
    system_program_account: &AccountInfo<'a>,
    name_account: &AccountInfo<'a>,
    fee_payer: &AccountInfo<'a>,
    new_owner_account: &AccountInfo<'a>,
    hashed_name: Vec<u8>,
    lamports: u64,
    space: u32,
    signer_seeds: &Vec<u8>,
) -> ProgramResult {
    let create_name_instruction = spl_name_service::instruction::create(
        *name_service_program.key,
        NameRegistryInstruction::Create {
            hashed_name,
            lamports,
            space,
        },
        *name_account.key,
        *fee_payer.key,
        *new_owner_account.key,
        None,
        None,
        None,
    )?;

    invoke_signed(
        &create_name_instruction,
        &[
            name_service_program.clone(),
            fee_payer.clone(),
            name_account.clone(),
            new_owner_account.clone(),
            system_program_account.clone(),
        ],
        &[&signer_seeds.chunks(32).collect::<Vec<&[u8]>>()],
    )
}

#[allow(clippy::too_many_arguments)]
    pub fn create_reverse_lookup_account<'a>(
        name_service_program: &AccountInfo<'a>,
        system_program_account: &AccountInfo<'a>,
        reverse_lookup_account: &AccountInfo<'a>,
        fee_payer: &AccountInfo<'a>,
        name: String,
        hashed_reverse_lookup: Vec<u8>,
        authority: &AccountInfo<'a>,
        rent_sysvar_account: &AccountInfo<'a>,
        signer_seeds: &[&[u8]],
        record_seeds: &Vec<u8>,
        parent_name_opt: Option<&AccountInfo<'a>>,
        parent_name_owner_opt: Option<&AccountInfo<'a>>,
    ) -> ProgramResult {
        let name_bytes = ReverseLookup { name }.try_to_vec().unwrap();
        let rent = Rent::from_account_info(rent_sysvar_account)?;
        let lamports = rent.minimum_balance(name_bytes.len() + NameRecordHeader::LEN);

        let create_name_instruction = spl_name_service::instruction::create(
            *name_service_program.key,
            NameRegistryInstruction::Create {
                hashed_name: hashed_reverse_lookup,
                lamports,
                space: name_bytes.len() as u32,
            },
            *reverse_lookup_account.key,
            *fee_payer.key,
            *authority.key,
            Some(*authority.key),
            parent_name_opt.map(|a| *a.key),
            parent_name_owner_opt.map(|a| *a.key),
        )?;

        let mut accounts_create = vec![
            name_service_program.clone(),
            fee_payer.clone(),
            authority.clone(),
            reverse_lookup_account.clone(),
            system_program_account.clone(),
        ];

        let mut accounts_update = vec![
            name_service_program.clone(),
            reverse_lookup_account.clone(),
            authority.clone(),
        ];

        if let Some(parent_name) = parent_name_opt {
            accounts_create.push(parent_name.clone());
            accounts_create.push(parent_name_owner_opt.unwrap().clone());
            accounts_update.push(parent_name.clone());
        }

        invoke_signed(
            &create_name_instruction, 
            &accounts_create, 
            &[
                &record_seeds.chunks(32).collect::<Vec<&[u8]>>(),
                signer_seeds
                ])?;

        let write_name_instruction = spl_name_service::instruction::update(
            *name_service_program.key,
            0,
            name_bytes,
            *reverse_lookup_account.key,
            *authority.key,
            parent_name_opt.map(|a| *a.key),
        )?;

        invoke_signed(&write_name_instruction, &accounts_update, &[signer_seeds])?;
        Ok(())
    }
