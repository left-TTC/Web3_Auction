use crate::{error::AuctionError, processor::Processor};

use {
    num_traits::FromPrimitive,
    solana_program::{
        account_info::AccountInfo, decode_error::DecodeError, entrypoint::ProgramResult, msg,
        program_error::PrintProgramError, pubkey::Pubkey,
    },
};

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

/// The entrypoint to the program
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Entrypoint");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<AuctionError>();
        return Err(error);
    }
    Ok(())
}

impl PrintProgramError for AuctionError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            AuctionError::AlreadyInitialized => {
                msg!("Error: This account is already initialized")
            }
            AuctionError::DataTypeMismatch => msg!("Error: Data type mismatch"),
            AuctionError::WrongOwner => msg!("Error: Wrong account owner"),
            AuctionError::Uninitialized => msg!("Error: Account is uninitialized"),
            AuctionError::UnsupportedValidation => msg!("Error: Unsupported validation"),
            AuctionError::Secp256k1Recover => msg!("Error: Could not recover public key"),
            AuctionError::EthPubkeyMismatch => msg!("Error: ETH public key mismatch"),
            AuctionError::WrongDomainOwner => msg!("Error: Wrong domain owner"),
            AuctionError::NumericalOverflow => msg!("Error: Numerical overflow"),
            AuctionError::OutOfBound => msg!("Error: Array out of bound"),
            AuctionError::InvalidVerifier => msg!("Error: Invalid verifier"),
            AuctionError::WrongParent => msg!("Error: Wrong parent owner"),
            AuctionError::WrongClass => msg!("Error: Wrong class"),
        }
    }
}
