use std::{
    io::{Cursor, Read, Seek, SeekFrom},
    str::FromStr,
};

use anchor_lang::{
    prelude::{Account, AccountInfo, Pubkey, Signer, Program},
    solana_program::stake_history::Epoch,
    AccountSerialize, system_program::System,
};
use solzen::models::{Validation, Zendao};

pub struct TestContext {
    pub info_key: Pubkey,
    pub info_owner: Pubkey,
}

pub fn create_account_info<'a>(
    key: &'a Pubkey,
    owner: &'a Pubkey,
    lamports: &'a mut u64,
    info_data: &'a mut [u8],
) -> AccountInfo<'a> {
    return AccountInfo::new(
        key,
        false,
        true,
        lamports,
        info_data,
        owner,
        false,
        Epoch::default(),
    );
}

pub fn create_zendao_account<'a>(
    test_ctx: &'a TestContext,
    lamports: &'a mut u64,
    data: &'a mut Vec<u8>,
) -> Account<'a, Zendao> {
    let zd = create_zendao();

    let mut buffer = Cursor::new(Vec::new());
    zd.try_serialize(&mut buffer).unwrap();
    buffer.seek(SeekFrom::Start(0)).unwrap();
    buffer.read_to_end(data).unwrap();
    let zendao_info = create_account_info(&test_ctx.info_key, &test_ctx.info_owner, lamports, data);
    let zendao = Account::<Zendao>::try_from_unchecked(&zendao_info).unwrap();
    return zendao;
}

pub fn create_validation_account<'a>(
    test_ctx: &'a TestContext,
    lamports: &'a mut u64,
    data: &'a mut Vec<u8>,
) -> Account<'a, Validation> {
    let validation_struct = Validation {
        parent: Pubkey::default(),
        child: Pubkey::default(),
        timestamp: 1,
    };

    let mut buffer = Cursor::new(Vec::new());
    validation_struct.try_serialize(&mut buffer).unwrap();
    buffer.seek(SeekFrom::Start(0)).unwrap();
    buffer.read_to_end(data).unwrap();
    let validation_info =
        create_account_info(&test_ctx.info_key, &test_ctx.info_owner, lamports, data);
    let validation = Account::<Validation>::try_from_unchecked(&validation_info).unwrap();
    return validation;
}

pub fn create_signer_account_info<'a>(
    key: &'a Pubkey,
    owner: &'a Pubkey,
    lamports: &'a mut u64,
    info_data: &'a mut [u8],
) -> AccountInfo<'a> {
    return AccountInfo::new(
        key,
        true,
        true,
        lamports,
        info_data,
        owner,
        false,
        Epoch::default(),
    );
}

pub fn create_signer<'a>(
    test_ctx: &'a TestContext,
    lamports: &'a mut u64,
    data: &'a mut Vec<u8>,
) -> Signer<'a> {
    let founder_info =
        create_signer_account_info(&test_ctx.info_key, &test_ctx.info_owner, lamports, data);
    let founder = Signer::try_from(&founder_info).unwrap();
    return founder;
}

pub fn create_program_account_info<'a>(
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

pub fn create_system_program<'a>(
    test_ctx: &'a TestContext,
    lamports: &'a mut u64,
    data: &'a mut Vec<u8>,
) -> Program<'a, System> {
    let program_info = create_program_account_info(
        &test_ctx.info_key,
        &test_ctx.info_owner,
        lamports,
        data,
    );
    let system_program = Program::try_from(&program_info).unwrap();
    return system_program;
}

pub fn create_zendao() -> Zendao {
    let zd = Zendao {
        token: Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap(),
        min_balance: 1,
        slug: String::from("slug"),
    };
    return zd;
}

pub fn create_test_context() -> TestContext {
    TestContext {
        info_key: Pubkey::default(),
        info_owner: Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap(),
    }
}
