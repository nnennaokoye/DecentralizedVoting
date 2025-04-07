pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(test)]
mod test;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

solana_program::declare_id!("VoteyXbxtVh4TbNu4KxGQQXyYgkEzQ9EJThJC4NdKH5");

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, instruction_data)
} 