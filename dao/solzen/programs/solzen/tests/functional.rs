use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar::{self},
    },
    solana_program_test::*,
    solana_sdk::{signature::Signer, transaction::Transaction},
    std::str::FromStr,
};
use std::io::{Cursor, SeekFrom, Seek, Read};

use anchor_lang::AnchorSerialize;
use sha2::{Sha256, Digest};
use solzen::entry;

#[tokio::test]
async fn test_sysvar() {
    let program_id = solzen::ID;
    let program_test = ProgramTest::new(
        "spl_example_sysvar",
        program_id,
        processor!(entry),
    );
    // program_test.add_account(address, account);
    let (mut banks_client, payer, recent_blockhash) = 
    program_test
    .start()
    .await;

    let mut hasher = Sha256::new();
    hasher.update(b"global:initialize");
    let result = hasher.finalize();
    println!("result = {:?}", &result[..8]);
    let data: &[u8] = &result[..8];

    // parametros que vao na assinatura do initialize
    let init = solzen::instruction::Initialize {
        token: Pubkey::default(),
        min_balance: 10,
        dao_slug: String::from("slug"),
    };
    let mut writer = Cursor::new(Vec::new());
    init.serialize(&mut writer).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    writer.seek(SeekFrom::Start(0)).unwrap();
    writer.read_to_end(&mut buf).unwrap();
    let mut final_data = Vec::from(data);
    println!("raw final_data = {:?}", final_data);
    final_data.append(&mut buf);

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id,
            &final_data,
            vec![
                AccountMeta::new(sysvar::clock::id(), false),
                AccountMeta::new(sysvar::rent::id(), false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}
