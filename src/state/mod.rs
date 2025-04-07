use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    clock::UnixTimestamp,
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

pub const MAX_POLL_TITLE_LENGTH: usize = 100;
pub const MAX_POLL_OPTION_LENGTH: usize = 50;
pub const MAX_POLL_OPTIONS: usize = 10;
pub const POLL_ACCOUNT_SIZE: usize = 1 + 4 + MAX_POLL_TITLE_LENGTH + 4 + (MAX_POLL_OPTIONS * MAX_POLL_OPTION_LENGTH) + 32 + 8 + 8 + 4 + (MAX_POLL_OPTIONS * 4);
pub const VOTE_ACCOUNT_SIZE: usize = 1 + 32 + 32 + 4;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Poll {
    pub is_initialized: bool,
    pub title: String,
    pub options: Vec<String>,
    pub authority: Pubkey,
    pub start_time: UnixTimestamp,
    pub end_time: UnixTimestamp,
    pub vote_counts: Vec<u32>,
}

impl Sealed for Poll {}

impl IsInitialized for Poll {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Poll {
    pub fn new(
        title: String,
        options: Vec<String>,
        authority: Pubkey,
        start_time: UnixTimestamp,
        end_time: UnixTimestamp,
    ) -> Self {
        let mut vote_counts = Vec::with_capacity(options.len());
        vote_counts.resize(options.len(), 0);
        
        Self {
            is_initialized: true,
            title,
            options,
            authority,
            start_time,
            end_time,
            vote_counts,
        }
    }
    
    pub fn is_active(&self, current_time: UnixTimestamp) -> bool {
        current_time >= self.start_time && current_time <= self.end_time
    }
    
    pub fn cast_vote(&mut self, option_index: usize) -> Result<(), &'static str> {
        if option_index >= self.options.len() {
            return Err("Invalid option index");
        }
        
        self.vote_counts[option_index] += 1;
        Ok(())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Vote {
    pub is_initialized: bool,
    pub voter: Pubkey,
    pub poll: Pubkey,
    pub option_index: u32,
}

impl Sealed for Vote {}

impl IsInitialized for Vote {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Vote {
    pub fn new(voter: Pubkey, poll: Pubkey, option_index: u32) -> Self {
        Self {
            is_initialized: true,
            voter,
            poll,
            option_index,
        }
    }
} 