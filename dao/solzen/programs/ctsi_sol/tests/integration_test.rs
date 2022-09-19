use std::str::FromStr;
use std::env;

use anchor_lang::{prelude::Pubkey, solana_program::pubkey};
use ctsi_sol::AccountManager;


fn create_account_manager(read_from_fixtures: bool) -> AccountManager {
    let dir = env::temp_dir();
    println!("Temporary directory: {}", dir.as_os_str().to_str().unwrap());
    let mut account_manager = ctsi_sol::AccountManager::new().unwrap();
    if !read_from_fixtures {
        account_manager.set_base_path(dir.as_os_str().to_str().unwrap().to_owned());
    }
    return account_manager;
}

#[test]
fn it_should_read_an_account_by_public_key() {
    let account_manager = create_account_manager(true);
    let pubkey = Pubkey::default();
    println!("key = {}", pubkey.to_string());
    let account_data = account_manager.read_account(&pubkey).unwrap();
    assert_eq!(account_data.lamports, 12345u64);
}

#[test]
fn it_should_write_an_account_by_public_key() {
    let pubkey = Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap();
    println!("key = {}", pubkey.to_string());
    let mut data = Vec::new();
    data.push(1);
    let account_file_data = ctsi_sol::AccountFileData {
        owner: Pubkey::from_str("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv").unwrap(),
        data: data,
        lamports: 123,
    };
    let account_manager = create_account_manager(false);
    account_manager.write_account(&pubkey, &account_file_data).unwrap();
}
