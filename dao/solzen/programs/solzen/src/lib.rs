use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv");

#[program]
pub mod solzen {
    use super::*;

    pub fn validate_human(ctx: Context<ValidateHuman>, child: Pubkey) -> Result<()> {
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