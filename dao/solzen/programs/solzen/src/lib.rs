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

#[program]
pub mod solzen {
    use super::*;

    pub fn validate_human(ctx: Context<ValidateHuman>, child: Pubkey) -> Result<()> {
        let token_account = &ctx.accounts.token_account;
        // if token_account.mint.key().to_string() != "" {
        //     return Err(error!(MyError::WrongToken));
        // }
        if token_account.amount <= 0 {
            return Err(error!(MyError::InsuficientAmount));
        }

        if token_account.owner != child {
            return Err(error!(MyError::WrongOwner));
        }
        msg!("owner = {:?}", token_account.owner);

        msg!("amount = {:?}", &token_account.amount);
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
#[instruction(child: Pubkey)]
pub struct ValidateHuman<'info> {
    #[account(init, payer = parent, space = Validation::LEN,
        seeds = [b"child", child.as_ref()], bump)]
    pub validation: Account<'info, Validation>,

    #[account()]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub parent: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Validation {
    pub parent: Pubkey,
    pub child: Pubkey,
    pub timestamp: i64,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;

impl Validation {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // parent.
        + PUBLIC_KEY_LENGTH // child.
        + TIMESTAMP_LENGTH; // timestamp.
}
