use borsh::BorshDeserialize;
use cryptonstudio::{process_instruction, DonationAccount};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use std::mem;

#[tokio::test]
async fn test_cryptonstudio() {
    let program_id = Pubkey::new_unique();
    let donated_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "cryptonstudio", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );
    program_test.add_account(
        donated_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Verify account has zero donatings
    let donated_account = banks_client
        .get_account(donated_pubkey)
        .await
        .expect("get_account")
        .expect("donated_account not found");
    assert_eq!(
        DonationAccount::try_from_slice(&donated_account.data)
            .unwrap()
            .counter,
        0
    );

    // donate once
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(donated_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has one donating
    let donated_account = banks_client
        .get_account(donated_pubkey)
        .await
        .expect("get_account")
        .expect("donated_account not found");
    assert_eq!(
        DonationAccount::try_from_slice(&donated_account.data)
            .unwrap()
            .counter,
        1
    );

    // donat again
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[1], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(donated_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has two donatings
    let donated_account = banks_client
        .get_account(donated_pubkey)
        .await
        .expect("get_account")
        .expect("donated_account not found");
    assert_eq!(
        DonationAccount::try_from_slice(&donated_account.data)
            .unwrap()
            .counter,
        2
    );
}
