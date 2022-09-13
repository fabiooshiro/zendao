use anchor_lang::prelude::*;

pub struct Clock {
    pub unix_timestamp: i64
}

impl Clock {
    pub fn get() -> Result<Clock>{
        Ok(Clock {
            unix_timestamp: 123
        })
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
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
