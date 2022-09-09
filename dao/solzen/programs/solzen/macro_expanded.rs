#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{CloseAccount, Mint, Token, TokenAccount, Transfer},
};
use ctsi_sol::Clock;
pub mod models {
    use anchor_lang::prelude::*;
    const DISCRIMINATOR_LENGTH: usize = 8;
    const PUBLIC_KEY_LENGTH: usize = 32;
    const TIMESTAMP_LENGTH: usize = 8;
    const U64_LENGTH: usize = 8;
    pub struct Zendao {
        /**
     * The token to be managed
     */
        pub token: Pubkey,
        /**
     * Minimum amount to be a member
     */
        pub min_balance: u64,
        /**
     * DAOs slug
     */
        pub slug: String,
    }
    #[automatically_derived]
    impl ::core::default::Default for Zendao {
        #[inline]
        fn default() -> Zendao {
            Zendao {
                token: ::core::default::Default::default(),
                min_balance: ::core::default::Default::default(),
                slug: ::core::default::Default::default(),
            }
        }
    }
    impl borsh::ser::BorshSerialize for Zendao
    where
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        String: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.token, writer)?;
            borsh::BorshSerialize::serialize(&self.min_balance, writer)?;
            borsh::BorshSerialize::serialize(&self.slug, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Zendao
    where
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                token: borsh::BorshDeserialize::deserialize(buf)?,
                min_balance: borsh::BorshDeserialize::deserialize(buf)?,
                slug: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Zendao {
        #[inline]
        fn clone(&self) -> Zendao {
            Zendao {
                token: ::core::clone::Clone::clone(&self.token),
                min_balance: ::core::clone::Clone::clone(&self.min_balance),
                slug: ::core::clone::Clone::clone(&self.slug),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Zendao {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[40, 218, 11, 136, 181, 213, 76, 238]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Zendao {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [40, 218, 11, 136, 181, 213, 76, 238].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[40, 218, 11, 136, 181, 213, 76, 238] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/solzen/src/models.rs",
                                    line: 8u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("Zendao"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Zendao {
        fn discriminator() -> [u8; 8] {
            [40, 218, 11, 136, 181, 213, 76, 238]
        }
    }
    #[automatically_derived]
    impl anchor_lang::Owner for Zendao {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    impl Zendao {
        pub const LEN: usize = DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH + U64_LENGTH;
        pub fn space(name: &str) -> usize {
            Zendao::LEN + 4 + name.len()
        }
    }
    pub struct Validation {
        pub parent: Pubkey,
        pub child: Pubkey,
        pub timestamp: i64,
    }
    impl borsh::ser::BorshSerialize for Validation
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.parent, writer)?;
            borsh::BorshSerialize::serialize(&self.child, writer)?;
            borsh::BorshSerialize::serialize(&self.timestamp, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Validation
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                parent: borsh::BorshDeserialize::deserialize(buf)?,
                child: borsh::BorshDeserialize::deserialize(buf)?,
                timestamp: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Validation {
        #[inline]
        fn clone(&self) -> Validation {
            Validation {
                parent: ::core::clone::Clone::clone(&self.parent),
                child: ::core::clone::Clone::clone(&self.child),
                timestamp: ::core::clone::Clone::clone(&self.timestamp),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Validation {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[130, 241, 151, 113, 169, 195, 219, 148]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Validation {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [130, 241, 151, 113, 169, 195, 219, 148].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[130, 241, 151, 113, 169, 195, 219, 148] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/solzen/src/models.rs",
                                    line: 38u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("Validation"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Validation {
        fn discriminator() -> [u8; 8] {
            [130, 241, 151, 113, 169, 195, 219, 148]
        }
    }
    #[automatically_derived]
    impl anchor_lang::Owner for Validation {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    impl Validation {
        pub const LEN: usize = DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH
            + PUBLIC_KEY_LENGTH + TIMESTAMP_LENGTH;
    }
    pub struct TelegramUser {
        /**
     * User public key
     */
        pub pubkey: Pubkey,
        /**
     * Its like an ID
     */
        pub dao: Pubkey,
        /**
     * Telegram user ID
     */
        pub id: u64,
    }
    impl borsh::ser::BorshSerialize for TelegramUser
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.pubkey, writer)?;
            borsh::BorshSerialize::serialize(&self.dao, writer)?;
            borsh::BorshSerialize::serialize(&self.id, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for TelegramUser
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                pubkey: borsh::BorshDeserialize::deserialize(buf)?,
                dao: borsh::BorshDeserialize::deserialize(buf)?,
                id: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TelegramUser {
        #[inline]
        fn clone(&self) -> TelegramUser {
            TelegramUser {
                pubkey: ::core::clone::Clone::clone(&self.pubkey),
                dao: ::core::clone::Clone::clone(&self.dao),
                id: ::core::clone::Clone::clone(&self.id),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for TelegramUser {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[170, 26, 59, 28, 82, 43, 166, 247]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for TelegramUser {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [170, 26, 59, 28, 82, 43, 166, 247].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[170, 26, 59, 28, 82, 43, 166, 247] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/solzen/src/models.rs",
                                    line: 52u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("TelegramUser"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TelegramUser {
        fn discriminator() -> [u8; 8] {
            [170, 26, 59, 28, 82, 43, 166, 247]
        }
    }
    #[automatically_derived]
    impl anchor_lang::Owner for TelegramUser {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    impl TelegramUser {
        pub const LEN: usize = DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH
            + PUBLIC_KEY_LENGTH + U64_LENGTH;
    }
}
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
    20u8,
    203u8,
    129u8,
    219u8,
    238u8,
    100u8,
    150u8,
    21u8,
    4u8,
    219u8,
    96u8,
    66u8,
    28u8,
    135u8,
    52u8,
    114u8,
    101u8,
    84u8,
    75u8,
    151u8,
    130u8,
    139u8,
    64u8,
    75u8,
    154u8,
    136u8,
    111u8,
    161u8,
    86u8,
    66u8,
    175u8,
    81u8,
]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
#[repr(u32)]
pub enum MyError {
    InsuficientAmount,
    WrongToken,
    WrongOwner,
    WrongParentValidation,
}
#[automatically_derived]
impl ::core::fmt::Debug for MyError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            MyError::InsuficientAmount => {
                ::core::fmt::Formatter::write_str(f, "InsuficientAmount")
            }
            MyError::WrongToken => ::core::fmt::Formatter::write_str(f, "WrongToken"),
            MyError::WrongOwner => ::core::fmt::Formatter::write_str(f, "WrongOwner"),
            MyError::WrongParentValidation => {
                ::core::fmt::Formatter::write_str(f, "WrongParentValidation")
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MyError {
    #[inline]
    fn clone(&self) -> MyError {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for MyError {}
impl MyError {
    /// Gets the name of this [#enum_name].
    pub fn name(&self) -> String {
        match self {
            MyError::InsuficientAmount => "InsuficientAmount".to_string(),
            MyError::WrongToken => "WrongToken".to_string(),
            MyError::WrongOwner => "WrongOwner".to_string(),
            MyError::WrongParentValidation => "WrongParentValidation".to_string(),
        }
    }
}
impl From<MyError> for u32 {
    fn from(e: MyError) -> u32 {
        e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
    }
}
impl From<MyError> for anchor_lang::error::Error {
    fn from(error_code: MyError) -> anchor_lang::error::Error {
        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
            error_name: error_code.name(),
            error_code_number: error_code.into(),
            error_msg: error_code.to_string(),
            error_origin: None,
            compared_values: None,
        })
    }
}
impl std::fmt::Display for MyError {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            MyError::InsuficientAmount => {
                fmt
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(&["Insuficient amount"], &[]),
                    )
            }
            MyError::WrongToken => {
                fmt.write_fmt(::core::fmt::Arguments::new_v1(&["Wrong token"], &[]))
            }
            MyError::WrongOwner => {
                fmt.write_fmt(::core::fmt::Arguments::new_v1(&["Wrong owner"], &[]))
            }
            MyError::WrongParentValidation => {
                fmt
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(&["Wrong parent validation"], &[]),
                    )
            }
        }
    }
}
use self::solzen::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) = unsafe {
        ::solana_program::entrypoint::deserialize(input)
    };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data)
        .map_err(|e| {
            e.log();
            e.into()
        })
}
fn try_entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    if data.len() < 8 {
        return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct Solzen;
    #[automatically_derived]
    impl ::core::clone::Clone for Solzen {
        #[inline]
        fn clone(&self) -> Solzen {
            Solzen
        }
    }
    impl anchor_lang::Id for Solzen {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>:<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [175, 175, 109, 31, 13, 152, 155, 237] => {
            __private::__global::initialize(program_id, accounts, ix_data)
        }
        [225, 31, 147, 9, 219, 183, 123, 107] => {
            __private::__global::close_dao(program_id, accounts, ix_data)
        }
        [225, 88, 123, 176, 217, 144, 52, 155] => {
            __private::__global::validate_telegram_user(program_id, accounts, ix_data)
        }
        [234, 55, 172, 111, 113, 18, 156, 156] => {
            __private::__global::validate_human(program_id, accounts, ix_data)
        }
        _ => Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(
                    anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into(),
                );
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(
                    &mut account_data_slice,
                )?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn initialize(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Initialize");
            let ix = instruction::Initialize::deserialize(&mut &ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Initialize { token, min_balance, dao_slug } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = InitDAO::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = solzen::initialize(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                token,
                min_balance,
                dao_slug,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn close_dao(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CloseDao");
            let ix = instruction::CloseDao::deserialize(&mut &ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::CloseDao = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = CloseDAO::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = solzen::close_dao(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn validate_telegram_user(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: ValidateTelegramUser");
            let ix = instruction::ValidateTelegramUser::deserialize(&mut &ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::ValidateTelegramUser { id } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = ValidateTelegramUser::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = solzen::validate_telegram_user(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                id,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn validate_human(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: ValidateHuman");
            let ix = instruction::ValidateHuman::deserialize(&mut &ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::ValidateHuman { child } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = ValidateHuman::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = solzen::validate_human(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                child,
            )?;
            accounts.exit(program_id)
        }
    }
}
pub mod solzen {
    use super::*;
    pub fn initialize(
        ctx: Context<InitDAO>,
        token: Pubkey,
        min_balance: u64,
        dao_slug: String,
    ) -> Result<()> {
        let dao = &mut ctx.accounts.zendao;
        let founder: &Signer = &ctx.accounts.founder;
        dao.token = token;
        dao.min_balance = min_balance;
        dao.slug = dao_slug;
        let validation = &mut ctx.accounts.validation;
        validation.child = *founder.key;
        let clock: Clock = Clock::get().unwrap();
        validation.timestamp = clock.unix_timestamp;
        Ok(())
    }
    pub fn close_dao(_ctx: Context<CloseDAO>) -> Result<()> {
        Ok(())
    }
    pub fn validate_telegram_user(
        ctx: Context<ValidateTelegramUser>,
        id: u64,
    ) -> Result<()> {
        let token_account = &ctx.accounts.token_account;
        let zendao = &ctx.accounts.zendao;
        ::solana_program::log::sol_log(
            &{
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["telegram id = "],
                        &[::core::fmt::ArgumentV1::new_debug(&id)],
                    ),
                );
                res
            },
        );
        if token_account.mint.key().to_string() != zendao.token.key().to_string() {
            ::solana_program::log::sol_log(
                &{
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["wrong mint ", " should be "],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(
                                    &token_account.mint.key(),
                                ),
                                ::core::fmt::ArgumentV1::new_debug(&zendao.token.key()),
                            ],
                        ),
                    );
                    res
                },
            );
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: MyError::WrongToken.name(),
                    error_code_number: MyError::WrongToken.into(),
                    error_msg: MyError::WrongToken.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/solzen/src/lib.rs",
                            line: 58u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        ::solana_program::log::sol_log(
            &{
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["token owner "],
                        &[::core::fmt::ArgumentV1::new_debug(&token_account.owner)],
                    ),
                );
                res
            },
        );
        if token_account.owner != ctx.accounts.signer.key() {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: MyError::WrongOwner.name(),
                    error_code_number: MyError::WrongOwner.into(),
                    error_msg: MyError::WrongOwner.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/solzen/src/lib.rs",
                            line: 62u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        let telegram_user = &mut ctx.accounts.telegram_user;
        if ctx.accounts.token_account.amount < ctx.accounts.zendao.min_balance {
            ::solana_program::log::sol_log(
                &{
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["token owner = ", " amount = ", " min = "],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(&token_account.owner),
                                ::core::fmt::ArgumentV1::new_debug(
                                    &ctx.accounts.token_account.amount,
                                ),
                                ::core::fmt::ArgumentV1::new_debug(
                                    &ctx.accounts.zendao.min_balance,
                                ),
                            ],
                        ),
                    );
                    res
                },
            );
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: MyError::InsuficientAmount.name(),
                    error_code_number: MyError::InsuficientAmount.into(),
                    error_msg: MyError::InsuficientAmount.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/solzen/src/lib.rs",
                            line: 67u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        telegram_user.pubkey = *ctx.accounts.signer.key;
        telegram_user.id = id;
        telegram_user.dao = ctx.accounts.zendao.key();
        Ok(())
    }
    pub fn validate_human(ctx: Context<ValidateHuman>, child: Pubkey) -> Result<()> {
        let token_account = &ctx.accounts.token_account;
        let zendao = &ctx.accounts.zendao;
        if token_account.mint.key().to_string() != zendao.token.key().to_string() {
            ::solana_program::log::sol_log(
                &{
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["wrong mint ", " should be "],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(
                                    &token_account.mint.key(),
                                ),
                                ::core::fmt::ArgumentV1::new_debug(&zendao.token.key()),
                            ],
                        ),
                    );
                    res
                },
            );
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: MyError::WrongToken.name(),
                    error_code_number: MyError::WrongToken.into(),
                    error_msg: MyError::WrongToken.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/solzen/src/lib.rs",
                            line: 84u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        let parent: &Signer = &ctx.accounts.parent;
        ::solana_program::log::sol_log(
            &{
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["amount = ", " min_balance = "],
                        &[
                            ::core::fmt::ArgumentV1::new_debug(&&token_account.amount),
                            ::core::fmt::ArgumentV1::new_debug(&zendao.min_balance),
                        ],
                    ),
                );
                res
            },
        );
        ::solana_program::log::sol_log(
            &{
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["parent = "],
                        &[::core::fmt::ArgumentV1::new_debug(&parent.key)],
                    ),
                );
                res
            },
        );
        let parent_validation = &ctx.accounts.parent_validation;
        ::solana_program::log::sol_log(
            &{
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["parent as child = "],
                        &[::core::fmt::ArgumentV1::new_debug(&parent_validation.child)],
                    ),
                );
                res
            },
        );
        if parent_validation.child.to_string() != *parent.key.to_string() {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: MyError::WrongParentValidation.name(),
                    error_code_number: MyError::WrongParentValidation.into(),
                    error_msg: MyError::WrongParentValidation.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/solzen/src/lib.rs",
                            line: 96u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        if token_account.amount < zendao.min_balance {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: MyError::InsuficientAmount.name(),
                    error_code_number: MyError::InsuficientAmount.into(),
                    error_msg: MyError::InsuficientAmount.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/solzen/src/lib.rs",
                            line: 100u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        ::solana_program::log::sol_log(
            &{
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["owner = ", " child = "],
                        &[
                            ::core::fmt::ArgumentV1::new_debug(&token_account.owner),
                            ::core::fmt::ArgumentV1::new_debug(&child),
                        ],
                    ),
                );
                res
            },
        );
        if token_account.owner != child {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: MyError::WrongOwner.name(),
                    error_code_number: MyError::WrongOwner.into(),
                    error_msg: MyError::WrongOwner.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/solzen/src/lib.rs",
                            line: 104u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        let validation: &mut Account<models::Validation> = &mut ctx.accounts.validation;
        validation.parent = *parent.key;
        validation.child = child;
        let clock: Clock = Clock::get().unwrap();
        validation.timestamp = clock.unix_timestamp;
        Ok(())
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct Initialize {
        pub token: Pubkey,
        pub min_balance: u64,
        pub dao_slug: String,
    }
    impl borsh::ser::BorshSerialize for Initialize
    where
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        String: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.token, writer)?;
            borsh::BorshSerialize::serialize(&self.min_balance, writer)?;
            borsh::BorshSerialize::serialize(&self.dao_slug, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Initialize
    where
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                token: borsh::BorshDeserialize::deserialize(buf)?,
                min_balance: borsh::BorshDeserialize::deserialize(buf)?,
                dao_slug: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Initialize {
        fn data(&self) -> Vec<u8> {
            let mut d = [175, 175, 109, 31, 13, 152, 155, 237].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct CloseDao;
    impl borsh::ser::BorshSerialize for CloseDao {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CloseDao {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::InstructionData for CloseDao {
        fn data(&self) -> Vec<u8> {
            let mut d = [225, 31, 147, 9, 219, 183, 123, 107].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct ValidateTelegramUser {
        pub id: u64,
    }
    impl borsh::ser::BorshSerialize for ValidateTelegramUser
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.id, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ValidateTelegramUser
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                id: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for ValidateTelegramUser {
        fn data(&self) -> Vec<u8> {
            let mut d = [225, 88, 123, 176, 217, 144, 52, 155].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct ValidateHuman {
        pub child: Pubkey,
    }
    impl borsh::ser::BorshSerialize for ValidateHuman
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.child, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ValidateHuman
    where
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                child: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for ValidateHuman {
        fn data(&self) -> Vec<u8> {
            let mut d = [234, 55, 172, 111, 113, 18, 156, 156].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_init_dao::*;
    pub use crate::__client_accounts_close_dao::*;
    pub use crate::__client_accounts_validate_telegram_user::*;
    pub use crate::__client_accounts_validate_human::*;
}
fn name_seed(name: &str) -> &[u8] {
    let b = name.as_bytes();
    if b.len() > 32 { &b[0..32] } else { b }
}
#[instruction(token:Pubkey, min_balance:u64, dao_slug:String)]
pub struct InitDAO<'info> {
    #[account(
        init,
        payer = founder,
        space = models::Zendao::space(&dao_slug),
        seeds = [b"dao".as_ref(),
        name_seed(&dao_slug).as_ref(),
        ],
        bump
    )]
    pub zendao: Account<'info, models::Zendao>,
    #[account(
        init,
        payer = founder,
        space = models::Validation::LEN,
        seeds = [b"child".as_ref(),
        founder.key.as_ref(),
        zendao.key().as_ref()],
        bump
    )]
    pub validation: Account<'info, models::Validation>,
    #[account(mut)]
    pub founder: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for InitDAO<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let mut ix_data = ix_data;
        struct __Args {
            token: Pubkey,
            min_balance: u64,
            dao_slug: String,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            Pubkey: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            String: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.token, writer)?;
                borsh::BorshSerialize::serialize(&self.min_balance, writer)?;
                borsh::BorshSerialize::serialize(&self.dao_slug, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            Pubkey: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            String: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    token: borsh::BorshDeserialize::deserialize(buf)?,
                    min_balance: borsh::BorshDeserialize::deserialize(buf)?,
                    dao_slug: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { token, min_balance, dao_slug } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
        if accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let zendao = &accounts[0];
        *accounts = &accounts[1..];
        if accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let validation = &accounts[0];
        *accounts = &accounts[1..];
        let founder: Signer = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("founder"))?;
        let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("system_program"))?;
        let __anchor_rent = Rent::get()?;
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"dao".as_ref(), name_seed(&dao_slug).as_ref()],
            program_id,
        );
        __bumps.insert("zendao".to_string(), __bump);
        let zendao = {
            let actual_field = zendao.to_account_info();
            let actual_owner = actual_field.owner;
            let space = models::Zendao::space(&dao_slug);
            let pa: anchor_lang::accounts::account::Account<models::Zendao> = if !false
                || actual_owner == &anchor_lang::solana_program::system_program::ID
            {
                let payer = founder.to_account_info();
                let __current_lamports = zendao.lamports();
                if __current_lamports == 0 {
                    let lamports = __anchor_rent.minimum_balance(space);
                    let cpi_accounts = anchor_lang::system_program::CreateAccount {
                        from: payer.to_account_info(),
                        to: zendao.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::create_account(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"dao".as_ref(),
                                        name_seed(&dao_slug).as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        lamports,
                        space as u64,
                        program_id,
                    )?;
                } else {
                    let required_lamports = __anchor_rent
                        .minimum_balance(space)
                        .max(1)
                        .saturating_sub(__current_lamports);
                    if required_lamports > 0 {
                        let cpi_accounts = anchor_lang::system_program::Transfer {
                            from: payer.to_account_info(),
                            to: zendao.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::transfer(
                            cpi_context,
                            required_lamports,
                        )?;
                    }
                    let cpi_accounts = anchor_lang::system_program::Allocate {
                        account_to_allocate: zendao.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::allocate(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"dao".as_ref(),
                                        name_seed(&dao_slug).as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        space as u64,
                    )?;
                    let cpi_accounts = anchor_lang::system_program::Assign {
                        account_to_assign: zendao.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::assign(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"dao".as_ref(),
                                        name_seed(&dao_slug).as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        program_id,
                    )?;
                }
                anchor_lang::accounts::account::Account::try_from_unchecked(&zendao)
                    .map_err(|e| e.with_account_name("zendao"))?
            } else {
                anchor_lang::accounts::account::Account::try_from(&zendao)
                    .map_err(|e| e.with_account_name("zendao"))?
            };
            if false {
                if space != actual_field.data_len() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("zendao")
                            .with_values((space, actual_field.data_len())),
                    );
                }
                if actual_owner != program_id {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("zendao")
                            .with_pubkeys((*actual_owner, *program_id)),
                    );
                }
                {
                    let required_lamports = __anchor_rent.minimum_balance(space);
                    if pa.to_account_info().lamports() < required_lamports {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("zendao"),
                        );
                    }
                }
            }
            pa
        };
        if zendao.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("zendao")
                    .with_pubkeys((zendao.key(), __pda_address)),
            );
        }
        if !zendao.to_account_info().is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("zendao"),
            );
        }
        if !__anchor_rent
            .is_exempt(
                zendao.to_account_info().lamports(),
                zendao.to_account_info().try_data_len()?,
            )
        {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("zendao"),
            );
        }
        let __anchor_rent = Rent::get()?;
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"child".as_ref(), founder.key.as_ref(), zendao.key().as_ref()],
            program_id,
        );
        __bumps.insert("validation".to_string(), __bump);
        let validation = {
            let actual_field = validation.to_account_info();
            let actual_owner = actual_field.owner;
            let space = models::Validation::LEN;
            let pa: anchor_lang::accounts::account::Account<models::Validation> = if !false
                || actual_owner == &anchor_lang::solana_program::system_program::ID
            {
                let payer = founder.to_account_info();
                let __current_lamports = validation.lamports();
                if __current_lamports == 0 {
                    let lamports = __anchor_rent.minimum_balance(space);
                    let cpi_accounts = anchor_lang::system_program::CreateAccount {
                        from: payer.to_account_info(),
                        to: validation.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::create_account(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"child".as_ref(),
                                        founder.key.as_ref(),
                                        zendao.key().as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        lamports,
                        space as u64,
                        program_id,
                    )?;
                } else {
                    let required_lamports = __anchor_rent
                        .minimum_balance(space)
                        .max(1)
                        .saturating_sub(__current_lamports);
                    if required_lamports > 0 {
                        let cpi_accounts = anchor_lang::system_program::Transfer {
                            from: payer.to_account_info(),
                            to: validation.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::transfer(
                            cpi_context,
                            required_lamports,
                        )?;
                    }
                    let cpi_accounts = anchor_lang::system_program::Allocate {
                        account_to_allocate: validation.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::allocate(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"child".as_ref(),
                                        founder.key.as_ref(),
                                        zendao.key().as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        space as u64,
                    )?;
                    let cpi_accounts = anchor_lang::system_program::Assign {
                        account_to_assign: validation.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::assign(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"child".as_ref(),
                                        founder.key.as_ref(),
                                        zendao.key().as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        program_id,
                    )?;
                }
                anchor_lang::accounts::account::Account::try_from_unchecked(&validation)
                    .map_err(|e| e.with_account_name("validation"))?
            } else {
                anchor_lang::accounts::account::Account::try_from(&validation)
                    .map_err(|e| e.with_account_name("validation"))?
            };
            if false {
                if space != actual_field.data_len() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("validation")
                            .with_values((space, actual_field.data_len())),
                    );
                }
                if actual_owner != program_id {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("validation")
                            .with_pubkeys((*actual_owner, *program_id)),
                    );
                }
                {
                    let required_lamports = __anchor_rent.minimum_balance(space);
                    if pa.to_account_info().lamports() < required_lamports {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("validation"),
                        );
                    }
                }
            }
            pa
        };
        if validation.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("validation")
                    .with_pubkeys((validation.key(), __pda_address)),
            );
        }
        if !validation.to_account_info().is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("validation"),
            );
        }
        if !__anchor_rent
            .is_exempt(
                validation.to_account_info().lamports(),
                validation.to_account_info().try_data_len()?,
            )
        {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("validation"),
            );
        }
        if !founder.to_account_info().is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("founder"),
            );
        }
        {
            let actual = system_program.key();
            let expected = system_program::ID;
            if actual != expected {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAddress,
                        )
                        .with_account_name("system_program")
                        .with_pubkeys((actual, expected)),
                );
            }
        }
        Ok(InitDAO {
            zendao,
            validation,
            founder,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for InitDAO<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.zendao.to_account_infos());
        account_infos.extend(self.validation.to_account_infos());
        account_infos.extend(self.founder.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for InitDAO<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.zendao.to_account_metas(None));
        account_metas.extend(self.validation.to_account_metas(None));
        account_metas.extend(self.founder.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for InitDAO<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.zendao, program_id)
            .map_err(|e| e.with_account_name("zendao"))?;
        anchor_lang::AccountsExit::exit(&self.validation, program_id)
            .map_err(|e| e.with_account_name("validation"))?;
        anchor_lang::AccountsExit::exit(&self.founder, program_id)
            .map_err(|e| e.with_account_name("founder"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_init_dao {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`InitDAO`].
    pub struct InitDAO {
        pub zendao: anchor_lang::solana_program::pubkey::Pubkey,
        pub validation: anchor_lang::solana_program::pubkey::Pubkey,
        pub founder: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for InitDAO
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.zendao, writer)?;
            borsh::BorshSerialize::serialize(&self.validation, writer)?;
            borsh::BorshSerialize::serialize(&self.founder, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for InitDAO {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.zendao,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.validation,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.founder,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_init_dao {
    use super::*;
    /// Generated CPI struct of the accounts for [`InitDAO`].
    pub struct InitDAO<'info> {
        pub zendao: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub validation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub founder: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitDAO<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.zendao),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.validation),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.founder),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitDAO<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.zendao));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.validation));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.founder));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.system_program));
            account_infos
        }
    }
}
pub struct CloseDAO<'info> {
    #[account(mut, close = signer)]
    pub zendao: Account<'info, models::Zendao>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for CloseDAO<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let zendao: anchor_lang::accounts::account::Account<models::Zendao> = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("zendao"))?;
        let signer: Signer = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("signer"))?;
        let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("system_program"))?;
        if !zendao.to_account_info().is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("zendao"),
            );
        }
        if zendao.key() == signer.key() {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintClose,
                    )
                    .with_account_name("zendao"),
            );
        }
        if !signer.to_account_info().is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("signer"),
            );
        }
        {
            let actual = system_program.key();
            let expected = system_program::ID;
            if actual != expected {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAddress,
                        )
                        .with_account_name("system_program")
                        .with_pubkeys((actual, expected)),
                );
            }
        }
        Ok(CloseDAO {
            zendao,
            signer,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for CloseDAO<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.zendao.to_account_infos());
        account_infos.extend(self.signer.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for CloseDAO<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.zendao.to_account_metas(None));
        account_metas.extend(self.signer.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for CloseDAO<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsClose::close(&self.zendao, self.signer.to_account_info())
            .map_err(|e| e.with_account_name("zendao"))?;
        anchor_lang::AccountsExit::exit(&self.signer, program_id)
            .map_err(|e| e.with_account_name("signer"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_close_dao {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`CloseDAO`].
    pub struct CloseDAO {
        pub zendao: anchor_lang::solana_program::pubkey::Pubkey,
        pub signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for CloseDAO
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.zendao, writer)?;
            borsh::BorshSerialize::serialize(&self.signer, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for CloseDAO {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.zendao,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.signer,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_close_dao {
    use super::*;
    /// Generated CPI struct of the accounts for [`CloseDAO`].
    pub struct CloseDAO<'info> {
        pub zendao: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CloseDAO<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.zendao),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.signer),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CloseDAO<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.zendao));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.signer));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.system_program));
            account_infos
        }
    }
}
#[instruction(child:Pubkey)]
pub struct ValidateHuman<'info> {
    #[account(
        init,
        payer = parent,
        space = models::Validation::LEN,
        seeds = [b"child".as_ref(),
        child.as_ref(),
        zendao.key().as_ref()],
        bump
    )]
    pub validation: Account<'info, models::Validation>,
    #[account()]
    pub token_account: Account<'info, TokenAccount>,
    #[account()]
    pub parent_validation: Account<'info, models::Validation>,
    #[account()]
    pub zendao: Account<'info, models::Zendao>,
    #[account(mut)]
    pub parent: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for ValidateHuman<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let mut ix_data = ix_data;
        struct __Args {
            child: Pubkey,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.child, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            Pubkey: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    child: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { child } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
        if accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let validation = &accounts[0];
        *accounts = &accounts[1..];
        let token_account: anchor_lang::accounts::account::Account<TokenAccount> = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("token_account"))?;
        let parent_validation: anchor_lang::accounts::account::Account<
            models::Validation,
        > = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("parent_validation"))?;
        let zendao: anchor_lang::accounts::account::Account<models::Zendao> = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("zendao"))?;
        let parent: Signer = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("parent"))?;
        let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("system_program"))?;
        let __anchor_rent = Rent::get()?;
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"child".as_ref(), child.as_ref(), zendao.key().as_ref()],
            program_id,
        );
        __bumps.insert("validation".to_string(), __bump);
        let validation = {
            let actual_field = validation.to_account_info();
            let actual_owner = actual_field.owner;
            let space = models::Validation::LEN;
            let pa: anchor_lang::accounts::account::Account<models::Validation> = if !false
                || actual_owner == &anchor_lang::solana_program::system_program::ID
            {
                let payer = parent.to_account_info();
                let __current_lamports = validation.lamports();
                if __current_lamports == 0 {
                    let lamports = __anchor_rent.minimum_balance(space);
                    let cpi_accounts = anchor_lang::system_program::CreateAccount {
                        from: payer.to_account_info(),
                        to: validation.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::create_account(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"child".as_ref(),
                                        child.as_ref(),
                                        zendao.key().as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        lamports,
                        space as u64,
                        program_id,
                    )?;
                } else {
                    let required_lamports = __anchor_rent
                        .minimum_balance(space)
                        .max(1)
                        .saturating_sub(__current_lamports);
                    if required_lamports > 0 {
                        let cpi_accounts = anchor_lang::system_program::Transfer {
                            from: payer.to_account_info(),
                            to: validation.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::transfer(
                            cpi_context,
                            required_lamports,
                        )?;
                    }
                    let cpi_accounts = anchor_lang::system_program::Allocate {
                        account_to_allocate: validation.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::allocate(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"child".as_ref(),
                                        child.as_ref(),
                                        zendao.key().as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        space as u64,
                    )?;
                    let cpi_accounts = anchor_lang::system_program::Assign {
                        account_to_assign: validation.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::assign(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"child".as_ref(),
                                        child.as_ref(),
                                        zendao.key().as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        program_id,
                    )?;
                }
                anchor_lang::accounts::account::Account::try_from_unchecked(&validation)
                    .map_err(|e| e.with_account_name("validation"))?
            } else {
                anchor_lang::accounts::account::Account::try_from(&validation)
                    .map_err(|e| e.with_account_name("validation"))?
            };
            if false {
                if space != actual_field.data_len() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("validation")
                            .with_values((space, actual_field.data_len())),
                    );
                }
                if actual_owner != program_id {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("validation")
                            .with_pubkeys((*actual_owner, *program_id)),
                    );
                }
                {
                    let required_lamports = __anchor_rent.minimum_balance(space);
                    if pa.to_account_info().lamports() < required_lamports {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("validation"),
                        );
                    }
                }
            }
            pa
        };
        if validation.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("validation")
                    .with_pubkeys((validation.key(), __pda_address)),
            );
        }
        if !validation.to_account_info().is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("validation"),
            );
        }
        if !__anchor_rent
            .is_exempt(
                validation.to_account_info().lamports(),
                validation.to_account_info().try_data_len()?,
            )
        {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("validation"),
            );
        }
        if !parent.to_account_info().is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("parent"),
            );
        }
        {
            let actual = system_program.key();
            let expected = system_program::ID;
            if actual != expected {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAddress,
                        )
                        .with_account_name("system_program")
                        .with_pubkeys((actual, expected)),
                );
            }
        }
        Ok(ValidateHuman {
            validation,
            token_account,
            parent_validation,
            zendao,
            parent,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ValidateHuman<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.validation.to_account_infos());
        account_infos.extend(self.token_account.to_account_infos());
        account_infos.extend(self.parent_validation.to_account_infos());
        account_infos.extend(self.zendao.to_account_infos());
        account_infos.extend(self.parent.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ValidateHuman<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.validation.to_account_metas(None));
        account_metas.extend(self.token_account.to_account_metas(None));
        account_metas.extend(self.parent_validation.to_account_metas(None));
        account_metas.extend(self.zendao.to_account_metas(None));
        account_metas.extend(self.parent.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ValidateHuman<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.validation, program_id)
            .map_err(|e| e.with_account_name("validation"))?;
        anchor_lang::AccountsExit::exit(&self.parent, program_id)
            .map_err(|e| e.with_account_name("parent"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_validate_human {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`ValidateHuman`].
    pub struct ValidateHuman {
        pub validation: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub parent_validation: anchor_lang::solana_program::pubkey::Pubkey,
        pub zendao: anchor_lang::solana_program::pubkey::Pubkey,
        pub parent: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for ValidateHuman
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.validation, writer)?;
            borsh::BorshSerialize::serialize(&self.token_account, writer)?;
            borsh::BorshSerialize::serialize(&self.parent_validation, writer)?;
            borsh::BorshSerialize::serialize(&self.zendao, writer)?;
            borsh::BorshSerialize::serialize(&self.parent, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ValidateHuman {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.validation,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.token_account,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.parent_validation,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zendao,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.parent,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_validate_human {
    use super::*;
    /// Generated CPI struct of the accounts for [`ValidateHuman`].
    pub struct ValidateHuman<'info> {
        pub validation: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub parent_validation: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
        pub zendao: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub parent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ValidateHuman<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.validation),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.token_account),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.parent_validation),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zendao),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.parent),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ValidateHuman<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.validation));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.token_account));
            account_infos
                .push(
                    anchor_lang::ToAccountInfo::to_account_info(&self.parent_validation),
                );
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.zendao));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.parent));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.system_program));
            account_infos
        }
    }
}
#[instruction(id:u64)]
pub struct ValidateTelegramUser<'info> {
    #[account(
        init,
        payer = signer,
        space = models::TelegramUser::LEN,
        seeds = [b"telegram_user".as_ref(),
        zendao.key().as_ref(),
        &id.to_le_bytes()],
        bump
    )]
    pub telegram_user: Account<'info, models::TelegramUser>,
    #[account()]
    pub zendao: Account<'info, models::Zendao>,
    #[account()]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info> for ValidateTelegramUser<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
        ix_data: &[u8],
        __bumps: &mut std::collections::BTreeMap<String, u8>,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let mut ix_data = ix_data;
        struct __Args {
            id: u64,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.id, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u64: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    id: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        let __Args { id } = __Args::deserialize(&mut ix_data)
            .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
        if accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let telegram_user = &accounts[0];
        *accounts = &accounts[1..];
        let zendao: anchor_lang::accounts::account::Account<models::Zendao> = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("zendao"))?;
        let token_account: anchor_lang::accounts::account::Account<TokenAccount> = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("token_account"))?;
        let signer: Signer = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("signer"))?;
        let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                program_id,
                accounts,
                ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("system_program"))?;
        let __anchor_rent = Rent::get()?;
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"telegram_user".as_ref(), zendao.key().as_ref(), &id.to_le_bytes()],
            program_id,
        );
        __bumps.insert("telegram_user".to_string(), __bump);
        let telegram_user = {
            let actual_field = telegram_user.to_account_info();
            let actual_owner = actual_field.owner;
            let space = models::TelegramUser::LEN;
            let pa: anchor_lang::accounts::account::Account<models::TelegramUser> = if !false
                || actual_owner == &anchor_lang::solana_program::system_program::ID
            {
                let payer = signer.to_account_info();
                let __current_lamports = telegram_user.lamports();
                if __current_lamports == 0 {
                    let lamports = __anchor_rent.minimum_balance(space);
                    let cpi_accounts = anchor_lang::system_program::CreateAccount {
                        from: payer.to_account_info(),
                        to: telegram_user.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::create_account(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"telegram_user".as_ref(),
                                        zendao.key().as_ref(),
                                        &id.to_le_bytes(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        lamports,
                        space as u64,
                        program_id,
                    )?;
                } else {
                    let required_lamports = __anchor_rent
                        .minimum_balance(space)
                        .max(1)
                        .saturating_sub(__current_lamports);
                    if required_lamports > 0 {
                        let cpi_accounts = anchor_lang::system_program::Transfer {
                            from: payer.to_account_info(),
                            to: telegram_user.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::transfer(
                            cpi_context,
                            required_lamports,
                        )?;
                    }
                    let cpi_accounts = anchor_lang::system_program::Allocate {
                        account_to_allocate: telegram_user.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::allocate(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"telegram_user".as_ref(),
                                        zendao.key().as_ref(),
                                        &id.to_le_bytes(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        space as u64,
                    )?;
                    let cpi_accounts = anchor_lang::system_program::Assign {
                        account_to_assign: telegram_user.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::assign(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"telegram_user".as_ref(),
                                        zendao.key().as_ref(),
                                        &id.to_le_bytes(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        program_id,
                    )?;
                }
                anchor_lang::accounts::account::Account::try_from_unchecked(
                        &telegram_user,
                    )
                    .map_err(|e| e.with_account_name("telegram_user"))?
            } else {
                anchor_lang::accounts::account::Account::try_from(&telegram_user)
                    .map_err(|e| e.with_account_name("telegram_user"))?
            };
            if false {
                if space != actual_field.data_len() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("telegram_user")
                            .with_values((space, actual_field.data_len())),
                    );
                }
                if actual_owner != program_id {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("telegram_user")
                            .with_pubkeys((*actual_owner, *program_id)),
                    );
                }
                {
                    let required_lamports = __anchor_rent.minimum_balance(space);
                    if pa.to_account_info().lamports() < required_lamports {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("telegram_user"),
                        );
                    }
                }
            }
            pa
        };
        if telegram_user.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("telegram_user")
                    .with_pubkeys((telegram_user.key(), __pda_address)),
            );
        }
        if !telegram_user.to_account_info().is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("telegram_user"),
            );
        }
        if !__anchor_rent
            .is_exempt(
                telegram_user.to_account_info().lamports(),
                telegram_user.to_account_info().try_data_len()?,
            )
        {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("telegram_user"),
            );
        }
        if !signer.to_account_info().is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("signer"),
            );
        }
        {
            let actual = system_program.key();
            let expected = system_program::ID;
            if actual != expected {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAddress,
                        )
                        .with_account_name("system_program")
                        .with_pubkeys((actual, expected)),
                );
            }
        }
        Ok(ValidateTelegramUser {
            telegram_user,
            zendao,
            token_account,
            signer,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ValidateTelegramUser<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.telegram_user.to_account_infos());
        account_infos.extend(self.zendao.to_account_infos());
        account_infos.extend(self.token_account.to_account_infos());
        account_infos.extend(self.signer.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ValidateTelegramUser<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.telegram_user.to_account_metas(None));
        account_metas.extend(self.zendao.to_account_metas(None));
        account_metas.extend(self.token_account.to_account_metas(None));
        account_metas.extend(self.signer.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ValidateTelegramUser<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.telegram_user, program_id)
            .map_err(|e| e.with_account_name("telegram_user"))?;
        anchor_lang::AccountsExit::exit(&self.signer, program_id)
            .map_err(|e| e.with_account_name("signer"))?;
        Ok(())
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_validate_telegram_user {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`ValidateTelegramUser`].
    pub struct ValidateTelegramUser {
        pub telegram_user: anchor_lang::solana_program::pubkey::Pubkey,
        pub zendao: anchor_lang::solana_program::pubkey::Pubkey,
        pub token_account: anchor_lang::solana_program::pubkey::Pubkey,
        pub signer: anchor_lang::solana_program::pubkey::Pubkey,
        pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    }
    impl borsh::ser::BorshSerialize for ValidateTelegramUser
    where
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
        anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.telegram_user, writer)?;
            borsh::BorshSerialize::serialize(&self.zendao, writer)?;
            borsh::BorshSerialize::serialize(&self.token_account, writer)?;
            borsh::BorshSerialize::serialize(&self.signer, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ValidateTelegramUser {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.telegram_user,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.zendao,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.token_account,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.signer,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_validate_telegram_user {
    use super::*;
    /// Generated CPI struct of the accounts for [`ValidateTelegramUser`].
    pub struct ValidateTelegramUser<'info> {
        pub telegram_user: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub zendao: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ValidateTelegramUser<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.telegram_user),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.zendao),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.token_account),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.signer),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ValidateTelegramUser<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.telegram_user));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.zendao));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.token_account));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.signer));
            account_infos
                .push(anchor_lang::ToAccountInfo::to_account_info(&self.system_program));
            account_infos
        }
    }
}
