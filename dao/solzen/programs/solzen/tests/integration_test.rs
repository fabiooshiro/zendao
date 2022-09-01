use std::{collections::BTreeMap, str::FromStr};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use anchor_lang::prelude::*;
use anchor_lang::solana_program::stake_history::Epoch;
use solzen::{
    self,
    models::{Validation, Zendao},
    solzen::initialize,
    InitDAO,
};

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

struct Resp<'a, T: AccountSerialize + AccountDeserialize + Owner + Clone> {
    account: Account<'a, T>,
    info_key: Pubkey
}

// fn create_account<'a, T: AccountSerialize + AccountDeserialize + Owner + Clone>() -> Resp<'a, T> {
//     let info_key<'a> = Pubkey::default();
//     let info_owner = Pubkey::default();
//     let mut lamports = 1000;
//     let mut buf: Vec<u8> = Vec::new();
//     let zendao_info = create_account_info(&info_key, &info_owner, &mut lamports, &mut buf);
//     Resp {
//         account: Account::<'a, T>::try_from_unchecked(&zendao_info).unwrap(),
//         info_key
//     }
// }

#[test]
fn it_just_runs() {
    let mut bumps = BTreeMap::new();
    bumps.insert(String::from("k"), 1);
    let info_key = Pubkey::default();
    let info_owner = Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap();
    
    

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
    let zendao = Account::<Zendao>::try_from_unchecked(&zendao_info).unwrap();

    assert_eq!(zendao.min_balance, 1);

    let validation_struct = Validation {
        parent: Pubkey::default(),
        child: Pubkey::default(),
        timestamp: 1,
    };

    let mut buffer = Cursor::new(Vec::new());
    validation_struct.try_serialize(&mut buffer).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    buffer.seek(SeekFrom::Start(0)).unwrap();
    buffer.read_to_end(&mut buf).unwrap();
    let mut lamports = 1000;
    let validation_info = create_account_info(&info_key, &info_owner, &mut lamports, &mut buf);
    let validation = Account::<Validation>::try_from_unchecked(&validation_info).unwrap();
    
    
    let mut buf: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let founder_info = create_signer_account_info(&info_key, &info_owner, &mut lamports, &mut buf);
    let founder = Signer::try_from(&founder_info).unwrap();

    let mut buf: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let program_info = create_program_account_info(&info_key, &info_owner, &mut lamports, &mut buf);
    let system_program = Program::try_from(&program_info).unwrap();
    let mut init_dao = InitDAO {
        zendao,
        validation,
        founder,
        system_program,
    };

    let ctx: Context<InitDAO> = Context {
        program_id: &Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap(),
        accounts: &mut init_dao,
        remaining_accounts: &[],
        bumps: bumps,
    };

    assert_eq!(ctx.accounts.zendao.min_balance, 1);

    let token = Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap();
    let min_balance = 10;
    let dao_slug = String::from("slug");
    initialize(ctx, token, min_balance, dao_slug);
}
