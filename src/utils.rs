use solana_program::{account_info::AccountInfo, hash::hashv, program_pack::Pack};
use spl_name_service::state::NameRecordHeader;
use spl_name_service::state::HASH_PREFIX;

use crate::error::AuctionError;

use {
    solana_program::pubkey, solana_program::pubkey::Pubkey,
    spl_name_service::state::get_seeds_and_key,
};

pub fn get_hashed_name(name: &str) -> Vec<u8> {
    hashv(&[(HASH_PREFIX.to_owned() + name).as_bytes()])
        .as_ref()
        .to_vec()
}
