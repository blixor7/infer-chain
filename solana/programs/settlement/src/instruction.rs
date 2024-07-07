use borsh::{BorshDeserialize, BorshSerialize};

/// Settlement Program Instructions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SettlementInstruction {
    /// FOR PARTNERS
    ///
    /// - Paying and acquiring network license
    /// - Creating initial configuration for specific partner
    /// - Create criterion for verification
    PartnerInitialize,

    /// FOR PARTNERS
    ///
    /// - Reconfigure configuration set for specific partner
    PartnerReconfigure,

    /// FOR NODE OPERATORS
    ///
    /// - Reconfigure configuration set
    NodeRegister,

    /// FOR ROUTER GATEWAY
    ///
    /// - Reconfigure configuration set
    RouterVerify,
}
