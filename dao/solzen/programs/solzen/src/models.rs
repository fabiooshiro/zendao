use anchor_lang::prelude::*;

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const U64_LENGTH: usize = 8;

#[account]
#[derive(Default)]
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

impl Zendao {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH + U64_LENGTH;
    pub fn space(name: &str) -> usize {
        // discriminator + owner pubkey + bump + capacity
        Zendao::LEN +
            // name string
            4 + name.len()
    }

}

#[account]
pub struct Validation {
    pub parent: Pubkey,
    pub child: Pubkey,
    pub timestamp: i64,
}

impl Validation {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // parent.
        + PUBLIC_KEY_LENGTH // child.
        + TIMESTAMP_LENGTH; // timestamp.
}

#[account]
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

impl TelegramUser {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH + PUBLIC_KEY_LENGTH + U64_LENGTH;
}
