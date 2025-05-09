use solana_program::program_pack::Pack;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};
// use spl_name_service::state::NameRecordHeader;

#[derive(Clone,Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct RecordHeader {
    pub root_name_key: Pubkey,
    pub amount: u64,
    pub name: String,
}

impl Sealed for RecordHeader {}

impl Pack for RecordHeader {
    const LEN: usize = 72;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap()
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mut p = src;
        RecordHeader::deserialize(&mut p).map_err(|_| {
            msg!("Failed to deserialize name record");
            ProgramError::InvalidAccountData
        })
    }
}


pub fn write_data(account: &AccountInfo, input: &[u8], offset: usize) {
    let mut account_data = account.data.borrow_mut();
    account_data[offset..offset.saturating_add(input.len())].copy_from_slice(input);
}
