use borsh::{BorshDeserialize, BorshSerialize};

// Config Data
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Config {
    pub conversion: u64, // 8 bytes
}

impl Config {
    pub const LEN: usize = 8; // bytes

    pub fn from(conversion: u64) -> Self {
        Self { conversion }
    }
}
