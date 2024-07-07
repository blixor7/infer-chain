use borsh::{BorshDeserialize, BorshSerialize};

// Instruction List
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SettlementInstruction {
    InitializeSettlement,
    ConfigureSettlement,
    RegisterNode,
    VerifyOutput,
}
