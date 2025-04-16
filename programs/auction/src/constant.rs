use anchor_lang::prelude::*;


pub mod constant {
    use anchor_lang::{prelude::Pubkey, pubkey};

    pub const VAULT: Pubkey = pubkey!("2NFji3XWVs2tb8btmGgkunjA9AFTr5x3DaTbsrZ7abGh");
    //only for test
    pub const ISSUE_PRICE: u64 = 1000000;

    pub const ADD: u8 = 0;

    pub const DELETE: u8 = 1;

    pub const REGISTER_FUND_FEE: u64 = 100000;

    pub const WEB3_NAME_SERVICE: Pubkey = pubkey!("77tWhvBTKvZVHudKKLV9EpzwFoTrGAJL9gwuNUA9MaRY");
}

