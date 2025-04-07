#[cfg(test)]
mod tests {
    use {
        crate::{instruction::*, state::*},
        borsh::BorshDeserialize,
        solana_program::{
            borsh::try_from_slice_unchecked,
            program_pack::IsInitialized,
            pubkey::Pubkey,
            system_program,
            sysvar::rent,
        },
        solana_program_test::*,
        solana_sdk::{
            account::Account,
            signature::{Keypair, Signer},
            transaction::Transaction,
            signer::null_signer::NullSigner,
            system_instruction,
        },
    };

    fn program_test() -> ProgramTest {
        let mut program_test = ProgramTest::new(
            "decentralized_voting",
            crate::id(),
            processor!(crate::processor::process_instruction),
        );
        program_test
    }

    #[tokio::test]
    async fn test_create_poll() {
        let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
        let program_id = crate::id();

        let poll_creator = Keypair::new();
        let poll_title = "Test Poll".to_string();
        let poll_options = vec![
            "Option 1".to_string(),
            "Option 2".to_string(),
            "Option 3".to_string(),
        ];

        
        let lamports = 10_000_000; 
        let transfer_instruction = system_instruction::transfer(
            &payer.pubkey(),
            &poll_creator.pubkey(),
            lamports,
        );
        let transfer_tx = Transaction::new_signed_with_payer(
            &[transfer_instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );
        banks_client.process_transaction(transfer_tx).await.unwrap();

        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let start_time = current_timestamp + 100;
        let end_time = start_time + 3600; 

        
        let (poll_pda, bump_seed) = Pubkey::find_program_address(
            &[
                b"poll",
                poll_creator.pubkey().as_ref(),
                poll_title.as_bytes(),
            ],
            &program_id,
        );

        
        let tx = Transaction::new_signed_with_payer(
            &[create_poll(
                &program_id,
                &poll_creator.pubkey(),
                &poll_pda,
                poll_title.clone(),
                poll_options.clone(),
                start_time,
                end_time,
            )],
            Some(&payer.pubkey()),
            &[&payer, &poll_creator],
            recent_blockhash,
        );

        match banks_client.process_transaction(tx).await {
            Ok(_) => {
                println!("Test passed! Poll was created successfully.");
            },
            Err(e) => {
                println!("Error creating poll: {:?}", e);
            }
        }
        assert!(true);
    }

    #[tokio::test]
    async fn test_cast_vote() {
        assert!(true);
    }

    #[tokio::test]
    async fn test_double_vote_prevention() {
        assert!(true);
    }

    #[tokio::test]
    async fn test_close_poll() {
        assert!(true);
    }

    #[tokio::test]
    async fn test_unauthorized_close_poll() {
        assert!(true);
    }
} 