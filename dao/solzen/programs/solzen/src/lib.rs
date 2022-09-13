use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{CloseAccount, Mint, Token, TokenAccount, Transfer},
};
// use ctsi_sol::Clock;
pub mod models;

declare_id!("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv");

#[error_code]
pub enum MyError {
    #[msg("Insuficient amount")]
    InsuficientAmount,

    #[msg("Wrong token")]
    WrongToken,

    #[msg("Wrong owner")]
    WrongOwner,

    #[msg("Wrong parent validation")]
    WrongParentValidation,
}

#[program]
pub mod solzen {
    use super::*;
    
    pub fn initialize(ctx: Context<InitDAO>, token: Pubkey, min_balance: u64, dao_slug: String) -> Result<()> {
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

    pub fn validate_telegram_user(ctx: Context<ValidateTelegramUser>, id: u64) -> Result<()> {
        let token_account = &ctx.accounts.token_account;
        let zendao = &ctx.accounts.zendao;
        msg!("telegram id = {:?}", id);
        if token_account.mint.key().to_string() != zendao.token.key().to_string() {
            msg!(
                "wrong mint {:?} should be {:?}",
                token_account.mint.key(),
                zendao.token.key()
            );
            return Err(error!(MyError::WrongToken));
        }
        msg!("token owner {:?}", token_account.owner);
        if token_account.owner != ctx.accounts.signer.key() {
            return Err(error!(MyError::WrongOwner));
        }
        let telegram_user = &mut ctx.accounts.telegram_user;
        if ctx.accounts.token_account.amount < ctx.accounts.zendao.min_balance {
            msg!("token owner = {:?} amount = {:?} min = {:?}", token_account.owner, ctx.accounts.token_account.amount, ctx.accounts.zendao.min_balance);
            return Err(error!(MyError::InsuficientAmount));
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
            msg!(
                "wrong mint {:?} should be {:?}",
                token_account.mint.key(),
                zendao.token.key()
            );
            return Err(error!(MyError::WrongToken));
        }
        let parent: &Signer = &ctx.accounts.parent;
        msg!(
            "amount = {:?} min_balance = {:?}",
            &token_account.amount,
            zendao.min_balance
        );
        msg!("parent = {:?}", parent.key);
        let parent_validation = &ctx.accounts.parent_validation;
        msg!("parent as child = {:?}", parent_validation.child);
        if parent_validation.child.to_string() != *parent.key.to_string() {
            return Err(error!(MyError::WrongParentValidation));
        }

        if token_account.amount < zendao.min_balance {
            return Err(error!(MyError::InsuficientAmount));
        }
        msg!("owner = {:?} child = {:?}", token_account.owner, child);
        if token_account.owner != child {
            return Err(error!(MyError::WrongOwner));
        }

        let validation: &mut Account<models::Validation> = &mut ctx.accounts.validation;

        validation.parent = *parent.key;
        validation.child = child;
        let clock: Clock = Clock::get().unwrap();
        validation.timestamp = clock.unix_timestamp;
        Ok(())
    }
}

fn name_seed(name: &str) -> &[u8] {
    let b = name.as_bytes();
    if b.len() > 32 { &b[0..32] } else { b }
}
#[derive(Accounts)]
// Atencao isso eh posicional
#[instruction(token: Pubkey, min_balance: u64, dao_slug: String)]
pub struct InitDAO<'info> {
    #[account(init, payer = founder, space = models::Zendao::space(&dao_slug),
        seeds = [b"dao".as_ref(), 
            name_seed(&dao_slug).as_ref(),
        ], bump)]
    pub zendao: Account<'info, models::Zendao>,

    #[account(init, payer = founder, space = models::Validation::LEN,
        seeds = [b"child".as_ref(), founder.key.as_ref(), zendao.key().as_ref()], bump)]
    pub validation: Account<'info, models::Validation>,

    #[account(mut)]
    pub founder: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseDAO<'info> {
    #[account(mut, close = signer)]
    pub zendao: Account<'info, models::Zendao>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(child: Pubkey)]
pub struct ValidateHuman<'info> {
    #[account(init, payer = parent, space = models::Validation::LEN,
        seeds = [b"child".as_ref(), child.as_ref(), zendao.key().as_ref()], bump)]
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

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct ValidateTelegramUser<'info> {
    #[account(init, payer = signer, space = models::TelegramUser::LEN,
        seeds = [b"telegram_user".as_ref(), zendao.key().as_ref(), &id.to_le_bytes()], bump)]
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
