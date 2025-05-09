use {
    borsh::BorshDeserialize,
    num_traits::FromPrimitive,
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
        pubkey::Pubkey,
    },
};

use crate::instruction::ProgramInstruction;

pub mod crowd_root;
pub mod create_root;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = FromPrimitive::from_u8(instruction_data[0])
            .ok_or(ProgramError::InvalidInstructionData)?;
        let instruction_data = &instruction_data[1..];
        msg!("Instruction unpacked");

        match instruction {
            ProgramInstruction::CreateRoot => {
                msg!("Instruction: create an root domain");
                let params = crowd_root::Params::try_from_slice(instruction_data)?;
                crowd_root::process(program_id, accounts, params)?;
            }
            ProgramInstruction::DonateRoot => {
                msg!("Instruction: try to create an root domain");
                let params = create_root::Params::try_from_slice(instruction_data)?;
                create_root::process(program_id, accounts, params)?;
            }
            ProgramInstruction::CreateAuction => {

            }
            ProgramInstruction::DeleteAuction => {

            }
        }

        Ok(())
    }
}
