use borsh::{BorshDeserialize, BorshSerialize};


pub mod record_header;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ReverseLookup {
    pub name: String,
}


pub mod constants {



    pub const CREATE_FEE: u64 = 10000;

}
