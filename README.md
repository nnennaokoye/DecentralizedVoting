# Decentralized Voting System

A decentralized voting system built on Solana using Rust. This smart contract enables users to create polls, vote on options, and view results transparently.

## Features

- Creation of voting polls with customizable options
- Secure vote casting mechanism (one vote per wallet)
- Real-time vote tallying and results display
- Time-bound voting periods with automatic closure
- Access control for poll creation and administration
- Prevention of double-voting through on-chain verification
- Gas-efficient implementation

## Technical Implementation

The system is composed of several key components:

### State Management

- `Poll`: Data structure representing a voting poll with title, options, and vote counts
- `Vote`: Data structure representing a user's vote on a specific poll

### Instructions

1. `CreatePoll`: Creates a new poll with customizable options and time constraints
2. `CastVote`: Allows a user to vote on a specific option in a poll
3. `ClosePoll`: Closes a poll and returns rent to the creator

### Security Features

- Prevention of double voting by creating a unique account for each voter-poll combination
- Time-bound voting periods with automatic validation
- Access control (only poll creator can close the poll)
- Proper validation of all inputs and state transitions

## Project Structure

```
src/
├── error/          # Custom error definitions
├── instruction/    # Instruction definitions and handling
├── processor/      # Core instruction processing logic
├── state/          # Program state definitions
├── lib.rs          # Program entry point
└── test.rs         # Comprehensive test suite
```

## Testing

The project includes a comprehensive test suite covering:

- Poll creation
- Vote casting
- Prevention of double voting
- Poll closure
- Access control validation

Run tests with:

```bash
cargo test
```

## Building and Deployment

### Prerequisites

- Rust and Cargo
- Solana CLI

### Build

```bash
cargo build-sbf
```

### Deploy to Devnet

```bash
solana program deploy --program-id <KEYPAIR_PATH> target/deploy/decentralized_voting.so
```

## Usage

### Creating a Poll

```rust
// Example code for creating a poll
let title = "Favorite Color";
let options = vec!["Red", "Blue", "Green"];
let start_time = current_timestamp + 3600; // Start in 1 hour
let end_time = start_time + 86400; // End in 24 hours after start

// Create the poll using the provided instruction
let instruction = create_poll(
    &program_id,
    &payer.pubkey(),
    &poll_pda,
    title,
    options,
    start_time,
    end_time,
);
```

### Casting a Vote

```rust
// Example code for casting a vote
let option_index = 1; // Vote for the second option (Blue)

// Cast the vote
let instruction = cast_vote(
    &program_id,
    &voter.pubkey(),
    &poll_pda,
    &vote_pda,
    option_index,
);
```

### Closing a Poll

```rust
// Example code for closing a poll
let instruction = close_poll(
    &program_id,
    &poll_creator.pubkey(),
    &poll_pda,
);
```

## Contributors

- [Your Name](https://github.com/yourusername)

## License

This project is licensed under the MIT License - see the LICENSE file for details. 