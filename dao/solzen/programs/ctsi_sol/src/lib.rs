use anchor_lang::prelude::Result;
use anchor_lang::prelude::ProgramError;

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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Rent {}
impl Rent {
    pub fn get() -> core::result::Result<anchor_lang::prelude::Rent, ProgramError> {
        Ok(anchor_lang::prelude::Rent { 
            lamports_per_byte_year: 1, 
            exemption_threshold: 0.1, 
            burn_percent: 1
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
