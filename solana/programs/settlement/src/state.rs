use borsh::{BorshDeserialize, BorshSerialize};

// NetworkConfig Data
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NetworkConfig {
    pub conversion: u64, // 8 bytes
}

impl NetworkConfig {
    pub const LEN: usize = 8; // bytes

    pub fn from(conversion: u64) -> Self {
        Self { conversion }
    }
}
