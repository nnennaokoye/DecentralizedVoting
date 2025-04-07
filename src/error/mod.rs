use solana_program::{msg, program_error::ProgramError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum VotingError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    
    #[error("Not rent exempt")]
    NotRentExempt,
    
    #[error("Poll already exists")]
    PollAlreadyExists,
    
    #[error("Poll does not exist")]
    PollDoesNotExist,
    
    #[error("Poll is closed")]
    PollClosed,
    
    #[error("Unauthorized access")]
    UnauthorizedAccess,
    
    #[error("Invalid poll title length")]
    InvalidPollTitleLength,
    
    #[error("Invalid poll option length")]
    InvalidPollOptionLength,
    
    #[error("Too many poll options")]
    TooManyPollOptions,
    
    #[error("User has already voted")]
    UserAlreadyVoted,
    
    #[error("Invalid vote option")]
    InvalidVoteOption,
    
    #[error("Poll time constraints error")]
    PollTimeConstraintError,
    
    #[error("Invalid account owner")]
    InvalidAccountOwner,
}

impl From<VotingError> for ProgramError {
    fn from(e: VotingError) -> Self {
        msg!("{}", e);
        ProgramError::Custom(e as u32)
    }
} 