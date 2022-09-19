use anchor_lang::solana_program::example_mocks::solana_sdk::transaction::Transaction;
use anchor_lang::solana_program::instruction::Instruction;
use bincode::deserialize;
use solana_perf::packet::{Meta, Packet, PacketBatch, PacketFlags};
use solana_perf::recycler::Recycler;
use solana_perf::sigverify::{self, TxOffset};
use solana_program::sysvar::{self};
use solana_sdk::bs58;
use std::cell::RefCell;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::net::{IpAddr, Ipv6Addr};
use std::rc::Rc;
use std::{collections::BTreeMap, str::FromStr};

use anchor_lang::{prelude::*, solana_program};
use sha2::{Digest, Sha256};
use solzen::{
    self,
    models::{Validation, Zendao},
    solzen::initialize,
    InitDAO,
};
pub mod factory;

fn load_account_info(_pubkey: &Pubkey) -> (Vec<u8>, u64, Pubkey) {
    let lamports = 1000;
    let info_data = Vec::new();
    let owner = Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap();
    (info_data, lamports, owner)
}

#[test]
fn it_should_deserialize_transaction_from_base64_encoded_sent_through_rpc_from_frontend() {
    let encoded64 = "AT5FiVtGESwkZI4CSYS3rB1BUKhO/SsuWkdI7U0a+EfOYhWoUFcpPgFDhCa9n6lZP4j/JurMY90/6/PY/XoErA8BAAIFaLXcC6Cywbwm74mPOjeCatSweRxlWr35eTLpIEf+WOE9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW788smY3PSJVY8mgIeGmx7C+RnzWnx1yuebvR7LVvAwu3AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUy4Hb7mSWFQTbYEIchzRyZVRLl4KLQEuaiG+hVkKvUa4D7xZK8QCXLSfAn7UqVg66AIgQ0PFcKgpkbDEnLEQFAQQEAQIAAzivr20fDZib7awePxMEdMwUsv7P4+23SFy4GOfkeXuwWPj1MMZHa6Xu6AMAAAAAAAAEAAAAc2x1Zw==";
    let decoded = base64::decode(encoded64).unwrap();
    let tx: solana_sdk::transaction::Transaction = deserialize(&decoded).unwrap();
    println!("tx = {:?}", tx);
    assert_eq!(tx.signatures.len(), 1);
    assert_eq!(tx.message.account_keys.len(), 5);
}

#[test]
fn it_should_deserialize_transaction_from_base64_and_call_anchors_entry() {
    let encoded64 = "AT5FiVtGESwkZI4CSYS3rB1BUKhO/SsuWkdI7U0a+EfOYhWoUFcpPgFDhCa9n6lZP4j/JurMY90/6/PY/XoErA8BAAIFaLXcC6Cywbwm74mPOjeCatSweRxlWr35eTLpIEf+WOE9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW788smY3PSJVY8mgIeGmx7C+RnzWnx1yuebvR7LVvAwu3AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUy4Hb7mSWFQTbYEIchzRyZVRLl4KLQEuaiG+hVkKvUa4D7xZK8QCXLSfAn7UqVg66AIgQ0PFcKgpkbDEnLEQFAQQEAQIAAzivr20fDZib7awePxMEdMwUsv7P4+23SFy4GOfkeXuwWPj1MMZHa6Xu6AMAAAAAAAAEAAAAc2x1Zw==";
    let decoded = base64::decode(encoded64).unwrap();
    let tx: solana_sdk::transaction::Transaction = deserialize(&decoded).unwrap();

    let program_id = solzen::ID;
    let first = &tx.message.instructions[0];

    let mut accounts = Vec::new();
    let mut params = Vec::new();
    for pubkey in tx.message.account_keys.iter() {
        let (a, b, c) = load_account_info(&pubkey);
        params.push((a, b, c, pubkey));
    }
    for param in params.iter_mut() {
        accounts.push(AccountInfo {
            key: &param.3,
            is_signer: true,
            is_writable: true,
            lamports: Rc::new(RefCell::new(&mut param.1)),
            data: Rc::new(RefCell::new(&mut param.0)),
            owner: &param.2,
            executable: true,
            rent_epoch: 1,
        });
    }
    println!("accounts indexes {:?}", first.accounts);
    println!("method dispatch's sighash = {:?}", &first.data[..8]);

    solzen::entry(&program_id, &accounts, &first.data).unwrap();
}

#[test]
fn it_should_call_sysvar() {
    let program_id = Pubkey::from_str("Sysvar1111111111111111111111111111111111111").unwrap();

    let transaction = Transaction::new_with_payer(
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
fn it_should_read_offset() {
    // just learning, look at it_should_deserialize_transaction_from_base64_encoded_sent_through_rpc_from_frontend
    let mut packets = Vec::new();
    let encoded64 = "AT5FiVtGESwkZI4CSYS3rB1BUKhO/SsuWkdI7U0a+EfOYhWoUFcpPgFDhCa9n6lZP4j/JurMY90/6/PY/XoErA8BAAIFaLXcC6Cywbwm74mPOjeCatSweRxlWr35eTLpIEf+WOE9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW788smY3PSJVY8mgIeGmx7C+RnzWnx1yuebvR7LVvAwu3AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUy4Hb7mSWFQTbYEIchzRyZVRLl4KLQEuaiG+hVkKvUa4D7xZK8QCXLSfAn7UqVg66AIgQ0PFcKgpkbDEnLEQFAQQEAQIAAzivr20fDZib7awePxMEdMwUsv7P4+23SFy4GOfkeXuwWPj1MMZHa6Xu6AMAAAAAAAAEAAAAc2x1Zw==";
    let decoded = base64::decode(encoded64).unwrap();
    let meta = Meta {
        size: decoded.len(),
        addr: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
        port: 403,
        flags: PacketFlags::FORWARDED,
        sender_stake: 1,
    };
    let mut data: [u8; 1232] = [0; 1232];
    for n in 0..decoded.len() {
        data[n] = decoded[n];
    }
    let packet = Packet::new(data, meta);
    packets.push(packet);
    let packet_batch = PacketBatch::new(packets);
    let mut batches = [packet_batch];
    let recicler = Recycler::<TxOffset>::default();
    let (signature_offsets, pubkey_offsets, msg_start_offsets, msg_sizes, offsets) =
        sigverify::generate_offsets(&mut batches, &recicler, false);
    println!("signature_offsets {:?}", signature_offsets);
    println!("pubkey_offsets {:?}", pubkey_offsets);
    println!("msg_start_offsets {:?}", msg_start_offsets);
    println!("msg_sizes {:?}", msg_sizes);
    println!("offsets {:?}", offsets);
}

#[test]
fn it_should_deserialize_the_arguments() {
    // just learning, look at it_should_deserialize_transaction_from_base64_encoded_sent_through_rpc_from_frontend
    // fontend's sample data
    let encoded64 = "AT5FiVtGESwkZI4CSYS3rB1BUKhO/SsuWkdI7U0a+EfOYhWoUFcpPgFDhCa9n6lZP4j/JurMY90/6/PY/XoErA8BAAIFaLXcC6Cywbwm74mPOjeCatSweRxlWr35eTLpIEf+WOE9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW788smY3PSJVY8mgIeGmx7C+RnzWnx1yuebvR7LVvAwu3AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUy4Hb7mSWFQTbYEIchzRyZVRLl4KLQEuaiG+hVkKvUa4D7xZK8QCXLSfAn7UqVg66AIgQ0PFcKgpkbDEnLEQFAQQEAQIAAzivr20fDZib7awePxMEdMwUsv7P4+23SFy4GOfkeXuwWPj1MMZHa6Xu6AMAAAAAAAAEAAAAc2x1Zw==";
    let decoded = base64::decode(encoded64).unwrap();
    println!("decoded.len = {}", decoded.len());
    let (signatures_count, u16_size) =
        solana_program::short_vec::decode_shortu16_len(&decoded).unwrap();
    println!(
        "signatures_count = {}; u16_size = {}",
        signatures_count, u16_size
    );
    let end_signature = u16_size + 64;
    let first_signature: Vec<u8> = decoded[u16_size..end_signature].to_vec();
    println!("signature = {:?}", first_signature);
    println!(
        "signature = {:?}",
        bs58::encode(&first_signature).into_string()
    );
    assert_eq!(
        "2FDDFY8z5h1nz12vJGgQekjf8qqMXxKED4ANxhS2uqdDSRhGio4FvyreKc6eCQD5SUAhgrACEQiLkbqfox7sFFUi",
        bs58::encode(&first_signature).into_string()
    );
    // https://explorer.solana.com/tx/2FDDFY8z5h1nz12vJGgQekjf8qqMXxKED4ANxhS2uqdDSRhGio4FvyreKc6eCQD5SUAhgrACEQiLkbqfox7sFFUi?cluster=testnet

    // let end_header = end_signature + 3;
    // let header = decoded[end_signature..end_header].to_vec();
    // println!("message header = {:?}", header);
    // let required_signatures = header[0];
    // let read_only_addresses = header[1];
    // let read_only_addr_nsig = header[2];
}

#[test]
fn it_should_call_entry_generated_by_anchors_macro() {
    // working sample
    let program_id = solzen::ID;
    let solana_program_id = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    let dao_pubkey = Pubkey::from_str("58tPH4GsdSn5ehKszkcWK12S2rTBcG9GaMfKtkEZDBKt").unwrap();
    let validation_pubkey =
        Pubkey::from_str("86VPuJvVmpsr3GGGL2BhiUESvALS6xujKznBpNpBvYsj").unwrap();
    let mut hasher = Sha256::new();
    hasher.update(b"global:initialize");
    let result = hasher.finalize();
    println!("method dispatch's sighash = {:?}", &result[..8]);

    let data: &[u8] = &result[..8];
    println!("data = {:?}", data);

    let init = solzen::instruction::Initialize {
        token: Pubkey::from_str("CasshNb6PacBzSwbd5gw8uqoQEjcWxaQ9u9byFApShwT").unwrap(),
        min_balance: 123,
        dao_slug: String::from("slug"),
    };
    let mut writer = Cursor::new(Vec::new());
    init.serialize(&mut writer).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    writer.seek(SeekFrom::Start(0)).unwrap();
    writer.read_to_end(&mut buf).unwrap();
    let mut final_data = Vec::from(data);
    println!("token pubkey = {:?}", init.token);
    println!("Initialize params = {:?}", buf);
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
            &validation_pubkey,
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
    let zendao_info =
        factory::create_account_info(&dao_pubkey, &info_owner, &mut lamports, &mut buf);

    let mut buf: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let founder_info =
        factory::create_signer_account_info(&info_key, &info_owner, &mut lamports, &mut buf);

    let mut buf: Vec<u8> = Vec::new();
    let mut lamports = 1000;
    let program_info = factory::create_program_account_info(
        &solana_program_id,
        &solana_program_id,
        &mut lamports,
        &mut buf,
    );
    let accounts = &[zendao_info, validation_info, founder_info, program_info];
    solzen::entry(&program_id, accounts, &final_data).unwrap();
    let res = Account::<Zendao>::try_from_unchecked(&accounts[0]).unwrap();
    assert_eq!(res.min_balance, 123);
    assert_eq!(res.slug, "slug");
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
