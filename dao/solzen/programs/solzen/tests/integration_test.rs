use std::error::Error;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::{collections::BTreeMap, str::FromStr};

use anchor_lang::prelude::*;
use anchor_lang::solana_program::stake_history::Epoch;
use sha2::{Digest, Sha256};
use solzen::entry;
use solzen::{
    self,
    models::{Validation, Zendao},
    solzen::initialize,
    InitDAO,
};
pub mod factory;


fn create_account_info<'a>(
    key: &'a Pubkey,
    owner: &'a Pubkey,
    lamports: &'a mut u64,
    info_data: &'a mut [u8],
) -> AccountInfo<'a> {
    return AccountInfo::new(
        key,
        false,
        false,
        lamports,
        info_data,
        owner,
        false,
        Epoch::default(),
    );
}

fn create_signer_account_info<'a>(
    key: &'a Pubkey,
    owner: &'a Pubkey,
    lamports: &'a mut u64,
    info_data: &'a mut [u8],
) -> AccountInfo<'a> {
    return AccountInfo::new(
        key,
        true,
        false,
        lamports,
        info_data,
        owner,
        false,
        Epoch::default(),
    );
}

fn create_program_account_info<'a>(
    key: &'a Pubkey,
    owner: &'a Pubkey,
    lamports: &'a mut u64,
    info_data: &'a mut [u8],
) -> AccountInfo<'a> {
    return AccountInfo::new(
        key,
        true,
        false,
        lamports,
        info_data,
        owner,
        true,
        Epoch::default(),
    );
}

#[test]
fn it_should_call_entry_generated_by_anchors_macro() {
    let program_id = solzen::ID;

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
        
        create_account_info(&info_key, &info_owner, &mut validation_lamports, &mut validation_buf)
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
    let zendao_info = create_account_info(&info_key, &info_owner, &mut lamports, &mut buf);
    

    let mut buf: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let founder_info = create_signer_account_info(&info_key, &info_owner, &mut lamports, &mut buf);

    let mut buf: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let program_info = create_program_account_info(&info_key, &info_owner, &mut lamports, &mut buf);
    let accounts = &[zendao_info, validation_info, founder_info, program_info];
    entry(&program_id, accounts, &final_data).unwrap();
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

    let mut buf: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let founder_info = create_signer_account_info(&test_ctx.info_key, &test_ctx.info_owner, &mut lamports, &mut buf);
    let founder = Signer::try_from(&founder_info).unwrap();

    let mut buf: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let program_info = create_program_account_info(&test_ctx.info_key, &test_ctx.info_owner, &mut lamports, &mut buf);
    let system_program = Program::try_from(&program_info).unwrap();
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
    let dao_slug = String::from("slug");
    let result = initialize(ctx, token, min_balance, dao_slug);
    // ctx foi retirado da memoria nesse escopo
    match result {
        Ok(_) => println!("Ok"),
        Err(_) => panic!("Wowww"),
    }

    assert_eq!(init_dao.validation.timestamp, 123);
}
