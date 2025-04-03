use anchor_lang::prelude::*;


pub mod Constant {
    use anchor_lang::{prelude::Pubkey, pubkey};

    pub const VAULT: Pubkey = pubkey!("2NFji3XWVs2tb8btmGgkunjA9AFTr5x3DaTbsrZ7abGh");
    //only for test
    pub const ISSUE_PRICE: u64 = 100;

    pub const WEB3_NAME_SERVICE: Pubkey = pubkey!("EWVnJDmu8CRLPyuHQqxgR1oFB8WhXBXRENRr1skQZxA9");
}

