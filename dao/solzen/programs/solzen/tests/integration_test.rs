use anchor_lang::solana_program::example_mocks::solana_sdk::transaction::Transaction;
use anchor_lang::solana_program::instruction::Instruction;
use solana_program::sysvar::{self};
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::{collections::BTreeMap, str::FromStr};

use anchor_lang::{prelude::*, solana_program};
use sha2::{Digest, Sha256};
use solzen::entry;
use solzen::{
    self,
    models::{Validation, Zendao},
    solzen::initialize,
    InitDAO,
};
pub mod factory;

#[test]
fn it_should_call_sysvar() {
    let program_id = Pubkey::from_str("Sysvar1111111111111111111111111111111111111").unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &(),
            vec![
                AccountMeta::new(sysvar::clock::id(), false),
                AccountMeta::new(sysvar::rent::id(), false),
            ],
        )],
        Some(&Pubkey::default()),
    );
    if transaction.message.is_signer(0) {
        println!("ok");
    } else {
        println!("nok {:?}", sysvar::clock::id());
    }
}

#[test]
fn it_should_call_entry_generated_by_anchors_macro() {

    let program_id = solzen::ID;
    let solana_program_id = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    let dao_pubkey = Pubkey::from_str("58tPH4GsdSn5ehKszkcWK12S2rTBcG9GaMfKtkEZDBKt").unwrap();
    let mut hasher = Sha256::new();
    hasher.update(b"global:initialize");
    let result = hasher.finalize();
    println!("result = {:?}", &result[..8]);

    let data: &[u8] = &result[..8];
    println!("data = {:?}", data);

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
    let info_key = Pubkey::default();
    let info_owner = Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap();

    let mut validation_lamports = 1000;
    let mut validation_buf: Vec<u8> = Vec::new();
    let validation_info = {
        let validation_struct = Validation {
            parent: Pubkey::default(),
            child: Pubkey::default(),
            timestamp: 1,
        };
        let mut buffer = Cursor::new(Vec::new());
        validation_struct.try_serialize(&mut buffer).unwrap();

        buffer.seek(SeekFrom::Start(0)).unwrap();
        buffer.read_to_end(&mut validation_buf).unwrap();

        factory::create_account_info(
            &info_key,
            &info_owner,
            &mut validation_lamports,
            &mut validation_buf,
        )
    };

    let zd = Zendao {
        token: Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap(),
        min_balance: 1,
        slug: String::from("slug"),
    };

    let mut buffer = Cursor::new(Vec::new());
    zd.try_serialize(&mut buffer).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    buffer.seek(SeekFrom::Start(0)).unwrap();
    buffer.read_to_end(&mut buf).unwrap();
    let mut lamports = 1000;
    let zendao_info = factory::create_account_info(&dao_pubkey, &info_owner, &mut lamports, &mut buf);

    let mut buf: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let founder_info =
        factory::create_signer_account_info(&info_key, &info_owner, &mut lamports, &mut buf);

    
    let mut buf: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let program_info =
        factory::create_program_account_info(&solana_program_id, &solana_program_id, &mut lamports, &mut buf);
    let accounts = &[zendao_info, validation_info, founder_info, program_info];
    solzen::entry(&program_id, accounts, &final_data).unwrap();
}

#[test]
fn it_just_runs() {
    let test_ctx = factory::create_test_context();
    let mut data: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let zendao = factory::create_zendao_account(&test_ctx, &mut lamports, &mut data);
    assert_eq!(zendao.min_balance, 1);

    let mut data: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let validation = factory::create_validation_account(&test_ctx, &mut lamports, &mut data);

    let mut data: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let founder = factory::create_signer(&test_ctx, &mut lamports, &mut data);

    let mut data: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let system_program = factory::create_system_program(&test_ctx, &mut lamports, &mut data);
    let mut init_dao = InitDAO {
        zendao,
        validation,
        founder,
        system_program,
    };
    // init_dao.to_account_infos()
    let ctx: Context<InitDAO> = Context {
        program_id: &Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap(),
        accounts: &mut init_dao,
        remaining_accounts: &[],
        bumps: BTreeMap::new(),
    };

    assert_eq!(ctx.accounts.zendao.min_balance, 1);

    let token = Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap();
    let min_balance = 10;
    let dao_slug = String::from("new_slug");
    let result = initialize(ctx, token, min_balance, dao_slug);
    // ctx foi retirado da memoria nesse escopo
    // assert_eq!(ctx.accounts.zendao.slug, "new_slug");
    match result {
        Ok(_) => println!("Ok"),
        Err(_) => panic!("Wowww"),
    }

    assert_eq!(init_dao.validation.timestamp, 123);
    assert_eq!(init_dao.zendao.slug, "new_slug");
    assert_eq!(init_dao.zendao.min_balance, 10);
}
