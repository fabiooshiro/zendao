use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{CloseAccount, Mint, Token, TokenAccount, Transfer},
};
mod models;

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
        msg!("amount = {:?} min_balance = {:?}", &token_account.amount, zendao.min_balance);
        if token_account.amount < zendao.min_balance {
            return Err(error!(MyError::InsuficientAmount));
        }
        msg!("owner = {:?} child = {:?}", token_account.owner, child);
        if token_account.owner != child {
            return Err(error!(MyError::WrongOwner));
        }
        
        let validation: &mut Account<models::Validation> = &mut ctx.accounts.validation;
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
    #[account(init, payer = founder, space = models::Zendao::LEN,
        seeds = [b"dao".as_ref(), token.as_ref()], bump)]
    pub zendao: Account<'info, models::Zendao>,

    #[account(mut)]
    pub founder: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(child: Pubkey)]
pub struct ValidateHuman<'info> {
    #[account(init, payer = parent, space = models::Validation::LEN,
        seeds = [b"child".as_ref(), child.as_ref()], bump)]
    pub validation: Account<'info, models::Validation>,

    #[account()]
    pub token_account: Account<'info, TokenAccount>,

    #[account()]
    pub zendao: Account<'info, models::Zendao>,

    #[account(mut)]
    pub parent: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
