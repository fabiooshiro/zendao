use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{CloseAccount, Mint, Token, TokenAccount, Transfer},
};

declare_id!("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv");

#[error_code]
pub enum MyError {
    #[msg("Insuficient amount")]
    InsuficientAmount,

    #[msg("Wrong token")]
    WrongToken,

    #[msg("Wrong owner")]
    WrongOwner,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const U64_LENGTH: usize = 8;
#[account]
pub struct Zendao {
    pub token: Pubkey,
    pub min_balance: u64,
}
const DAO_LEN: usize = DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH + U64_LENGTH;

#[account]
pub struct Validation {
    pub parent: Pubkey,
    pub child: Pubkey,
    pub timestamp: i64,
}
const VALIDATION_LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // parent.
        + PUBLIC_KEY_LENGTH // child.
        + TIMESTAMP_LENGTH; // timestamp.

#[program]
pub mod solzen {
    use super::*;

    pub fn initialize(ctx: Context<InitDAO>, token: Pubkey, min_balance: u64) -> Result<()> {
        let dao = &mut ctx.accounts.zendao;
        dao.token = token;
        dao.min_balance = min_balance;
        Ok(())
    }

    pub fn validate_human(ctx: Context<ValidateHuman>, child: Pubkey) -> Result<()> {
        let token_account = &ctx.accounts.token_account;
        let zendao = &ctx.accounts.zendao;
        if token_account.mint.key().to_string() != zendao.token.key().to_string() {
            return Err(error!(MyError::WrongToken));
        }
        msg!("amount = {:?} min_balance={:?}", &token_account.amount, zendao.min_balance);
        if token_account.amount < zendao.min_balance {
            return Err(error!(MyError::InsuficientAmount));
        }

        if token_account.owner != child {
            return Err(error!(MyError::WrongOwner));
        }
        msg!("owner = {:?}", token_account.owner);
        let validation: &mut Account<Validation> = &mut ctx.accounts.validation;
        let parent: &Signer = &ctx.accounts.parent;
        validation.parent = *parent.key;
        validation.child = child;
        let clock: Clock = Clock::get().unwrap();
        validation.timestamp = clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(token: Pubkey)]
pub struct InitDAO<'info> {
    #[account(init, payer = founder, space = DAO_LEN,
        seeds = [b"dao".as_ref(), token.as_ref()], bump)]
    pub zendao: Account<'info, Zendao>,

    #[account(mut)]
    pub founder: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(child: Pubkey)]
pub struct ValidateHuman<'info> {
    #[account(init, payer = parent, space = VALIDATION_LEN,
        seeds = [b"child".as_ref(), child.as_ref()], bump)]
    pub validation: Account<'info, Validation>,

    #[account()]
    pub token_account: Account<'info, TokenAccount>,

    #[account()]
    pub zendao: Account<'info, Zendao>,

    #[account(mut)]
    pub parent: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
