use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    clock::UnixTimestamp,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum VotingInstruction {
    CreatePoll {
        title: String,
        options: Vec<String>,
        start_time: UnixTimestamp,
        end_time: UnixTimestamp,
    },

    CastVote {
        option_index: u32,
    },
    
    ClosePoll {},
}

pub fn create_poll(
    program_id: &Pubkey,
    payer: &Pubkey,
    poll_account: &Pubkey,
    title: String,
    options: Vec<String>,
    start_time: UnixTimestamp,
    end_time: UnixTimestamp,
) -> Instruction {
    let data = VotingInstruction::CreatePoll {
        title,
        options,
        start_time,
        end_time,
    }
    .try_to_vec()
    .unwrap();

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*payer, true),
            AccountMeta::new(*poll_account, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data,
    }
}

pub fn cast_vote(
    program_id: &Pubkey,
    payer: &Pubkey,
    poll_account: &Pubkey,
    vote_account: &Pubkey,
    option_index: u32,
) -> Instruction {
    let data = VotingInstruction::CastVote { option_index }.try_to_vec().unwrap();

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*payer, true),
            AccountMeta::new(*poll_account, false),
            AccountMeta::new(*vote_account, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data,
    }
}

pub fn close_poll(
    program_id: &Pubkey,
    payer: &Pubkey,
    poll_account: &Pubkey,
) -> Instruction {
    let data = VotingInstruction::ClosePoll {}.try_to_vec().unwrap();

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*payer, true),
            AccountMeta::new(*poll_account, false),
        ],
        data,
    }
} 