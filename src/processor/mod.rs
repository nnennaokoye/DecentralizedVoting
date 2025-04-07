use crate::{
    error::VotingError,
    instruction::VotingInstruction,
    state::{Poll, Vote, MAX_POLL_OPTIONS, MAX_POLL_OPTION_LENGTH, MAX_POLL_TITLE_LENGTH, POLL_ACCOUNT_SIZE, VOTE_ACCOUNT_SIZE},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VotingInstruction::try_from_slice(instruction_data)
        .map_err(|_| VotingError::InvalidInstruction)?;

    match instruction {
        VotingInstruction::CreatePoll {
            title,
            options,
            start_time,
            end_time,
        } => process_create_poll(accounts, program_id, title, options, start_time, end_time),
        VotingInstruction::CastVote { option_index } => {
            process_cast_vote(accounts, program_id, option_index)
        }
        VotingInstruction::ClosePoll {} => process_close_poll(accounts, program_id),
    }
}

fn process_create_poll(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    title: String,
    options: Vec<String>,
    start_time: i64,
    end_time: i64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let creator_info = next_account_info(account_info_iter)?;
    let poll_account_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    // Validate accounts
    if !creator_info.is_signer {
        return Err(VotingError::UnauthorizedAccess.into());
    }

    if !system_program_info.key.eq(&solana_program::system_program::id()) {
        return Err(VotingError::InvalidInstruction.into());
    }

    // Validate inputs
    if title.len() > MAX_POLL_TITLE_LENGTH {
        return Err(VotingError::InvalidPollTitleLength.into());
    }

    if options.len() > MAX_POLL_OPTIONS {
        return Err(VotingError::TooManyPollOptions.into());
    }

    for option in &options {
        if option.len() > MAX_POLL_OPTION_LENGTH {
            return Err(VotingError::InvalidPollOptionLength.into());
        }
    }

    if start_time >= end_time {
        return Err(VotingError::PollTimeConstraintError.into());
    }

    // Get clock to check if start_time is in the future
    let clock = Clock::get()?;
    if start_time < clock.unix_timestamp {
        return Err(VotingError::PollTimeConstraintError.into());
    }

    // Check if the poll account already exists
    if poll_account_info.lamports() != 0 {
        return Err(VotingError::PollAlreadyExists.into());
    }

    // Create the poll account (PDA)
    let (poll_pda, bump_seed) = Pubkey::find_program_address(
        &[b"poll", creator_info.key.as_ref(), title.as_bytes()],
        program_id,
    );

    if poll_pda != *poll_account_info.key {
        return Err(VotingError::InvalidInstruction.into());
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(POLL_ACCOUNT_SIZE);

    // Create new account
    invoke_signed(
        &system_instruction::create_account(
            creator_info.key,
            &poll_pda,
            rent_lamports,
            POLL_ACCOUNT_SIZE as u64,
            program_id,
        ),
        &[
            creator_info.clone(),
            poll_account_info.clone(),
            system_program_info.clone(),
        ],
        &[&[b"poll", creator_info.key.as_ref(), title.as_bytes(), &[bump_seed]]],
    )?;

    // Initialize poll data
    let poll_data = Poll::new(
        title,
        options,
        *creator_info.key,
        start_time,
        end_time,
    );

    poll_data.serialize(&mut *poll_account_info.data.borrow_mut())?;

    msg!("Poll created successfully!");
    Ok(())
}

fn process_cast_vote(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    option_index: u32,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let voter_info = next_account_info(account_info_iter)?;
    let poll_account_info = next_account_info(account_info_iter)?;
    let vote_account_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;

    // Check if signer
    if !voter_info.is_signer {
        return Err(VotingError::UnauthorizedAccess.into());
    }

    // Validate poll account
    if poll_account_info.owner != program_id {
        return Err(VotingError::InvalidAccountOwner.into());
    }

    // Deserialize poll data
    let mut poll_data = try_from_slice_unchecked::<Poll>(&poll_account_info.data.borrow())?;
    
    if !poll_data.is_initialized() {
        return Err(VotingError::PollDoesNotExist.into());
    }

    // Check if poll is active
    let clock = Clock::get()?;
    if !poll_data.is_active(clock.unix_timestamp) {
        return Err(VotingError::PollClosed.into());
    }

    // Check if option index is valid
    if option_index as usize >= poll_data.options.len() {
        return Err(VotingError::InvalidVoteOption.into());
    }

    // Check if vote account already exists (user already voted)
    if vote_account_info.lamports() != 0 {
        let vote_data = try_from_slice_unchecked::<Vote>(&vote_account_info.data.borrow())?;
        if vote_data.is_initialized() && vote_data.voter == *voter_info.key && vote_data.poll == *poll_account_info.key {
            return Err(VotingError::UserAlreadyVoted.into());
        }
    }

    // Create vote account (PDA)
    let (vote_pda, bump_seed) = Pubkey::find_program_address(
        &[b"vote", voter_info.key.as_ref(), poll_account_info.key.as_ref()],
        program_id,
    );

    if vote_pda != *vote_account_info.key {
        return Err(VotingError::InvalidInstruction.into());
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(VOTE_ACCOUNT_SIZE);

    // Create vote account
    invoke_signed(
        &system_instruction::create_account(
            voter_info.key,
            &vote_pda,
            rent_lamports,
            VOTE_ACCOUNT_SIZE as u64,
            program_id,
        ),
        &[
            voter_info.clone(),
            vote_account_info.clone(),
            system_program_info.clone(),
        ],
        &[&[b"vote", voter_info.key.as_ref(), poll_account_info.key.as_ref(), &[bump_seed]]],
    )?;

    // Record the vote
    poll_data.vote_counts[option_index as usize] += 1;
    poll_data.serialize(&mut *poll_account_info.data.borrow_mut())?;

    // Initialize vote data
    let vote_data = Vote::new(
        *voter_info.key,
        *poll_account_info.key,
        option_index,
    );

    vote_data.serialize(&mut *vote_account_info.data.borrow_mut())?;

    msg!("Vote cast successfully!");
    Ok(())
}

fn process_close_poll(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_info_iter)?;
    let poll_account_info = next_account_info(account_info_iter)?;

    // Check if signer
    if !authority_info.is_signer {
        return Err(VotingError::UnauthorizedAccess.into());
    }

    // Validate poll account
    if poll_account_info.owner != program_id {
        return Err(VotingError::InvalidAccountOwner.into());
    }

    // Deserialize poll data
    let poll_data = try_from_slice_unchecked::<Poll>(&poll_account_info.data.borrow())?;
    
    if !poll_data.is_initialized() {
        return Err(VotingError::PollDoesNotExist.into());
    }

    // Check if caller is the poll creator
    if poll_data.authority != *authority_info.key {
        return Err(VotingError::UnauthorizedAccess.into());
    }

    // Transfer lamports to the poll creator (closing the account)
    let dest_starting_lamports = authority_info.lamports();
    **authority_info.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(poll_account_info.lamports())
        .ok_or(ProgramError::ArithmeticOverflow)?;
    **poll_account_info.lamports.borrow_mut() = 0;

    msg!("Poll closed successfully!");
    Ok(())
} 