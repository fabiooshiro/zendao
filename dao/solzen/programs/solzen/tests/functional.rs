use std::io::{Cursor, Read, Seek, SeekFrom};
use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        sysvar::{self},
    },
    solana_program_test::*,
    solana_sdk::{signature::Signer, transaction::Transaction},
    std::str::FromStr,
};

use anchor_lang::{prelude::Pubkey, AnchorSerialize};
use sha2::{Digest, Sha256};
use solana_program::stake_history::Epoch;
use solana_sdk::account::Account;
use solzen::entry;

//+--------------------------------------------------
// Call a test code to run on Cartesi will be weirdo
//+--------------------------------------------------
// #[tokio::test]
// #[cfg(feature="test-bpf")]
pub async fn test_sysvar() {
    
    let program_id = solzen::ID;
    let program_test = ProgramTest::new("spl_example_sysvar", program_id, processor!(entry));

    // let payer_pubkey = payer.pubkey();
    // let vault_bump_seed = 0u8;
    // let vault_seeds = &[b"dao".as_ref(), solzen::name_seed(&init.dao_slug).as_ref(), &[vault_bump_seed]];
    // let dao_pda = Pubkey::create_program_address(vault_seeds, &program_id).unwrap();
    let dao_pubkey = Pubkey::from_str("58tPH4GsdSn5ehKszkcWK12S2rTBcG9GaMfKtkEZDBKt").unwrap();
    let dao_data = Vec::new();
    let _account = Account {
        lamports: 1234567,
        owner: program_id,
        data: dao_data,
        executable: false,
        rent_epoch: Epoch::default(),
    };
    // program_test.add_account(dao_pubkey, account);
    // assert_eq!(dao_pubkey, dao_pda);

    // program_test.add_account(address, account);
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

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
                AccountMeta::new(dao_pubkey, false),
                AccountMeta::new(sysvar::rent::id(), false),
                AccountMeta::new(payer.pubkey(), false),
                AccountMeta::new(
                    Pubkey::from_str("11111111111111111111111111111111").unwrap(),
                    false,
                ),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}
