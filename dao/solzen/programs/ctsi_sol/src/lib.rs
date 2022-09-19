use anchor_lang::prelude::ProgramError;
use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::Result;
use serde::{Deserialize, Serialize};
use std::fs;

pub struct Clock {
    pub unix_timestamp: i64,
}

impl Clock {
    pub fn get() -> Result<Clock> {
        Ok(Clock {
            unix_timestamp: 123,
        })
    }
}

pub struct Rent {}
impl Rent {
    pub fn get() -> core::result::Result<anchor_lang::prelude::Rent, ProgramError> {
        Ok(anchor_lang::prelude::Rent {
            lamports_per_byte_year: 1,
            exemption_threshold: 0.1,
            burn_percent: 1,
        })
    }
}

pub struct AccountManager {
    base_path: String,
}

impl AccountManager {
    pub fn new() -> std::result::Result<AccountManager, Box<dyn std::error::Error>> {
        Ok(Self {
            base_path: "tests/fixtures".to_string(),
        })
    }

    pub fn set_base_path(&mut self, base_path: String) {
        self.base_path = base_path;
    }

    pub fn write_account(
        &self,
        pubkey: &Pubkey,
        account_file_data: &AccountFileData,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/{}.json", &self.base_path, pubkey.to_string());
        let contents = serde_json::to_string(account_file_data)?;
        fs::write(file_path, contents)?;
        Ok(())
    }

    pub fn read_account(
        &self,
        pubkey: &Pubkey,
    ) -> std::result::Result<AccountFileData, Box<dyn std::error::Error>> {
        let file_path = format!("{}/{}.json", &self.base_path, pubkey.to_string());
        let contents = fs::read_to_string(file_path)?;
        let account = serde_json::from_str::<AccountFileData>(&contents)?;
        Ok(account)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccountFileData {
    pub owner: Pubkey,
    pub data: Vec<u8>,
    pub lamports: u64,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rent = Rent::get().unwrap();
        assert_eq!(rent.burn_percent, 1);
    }
}
