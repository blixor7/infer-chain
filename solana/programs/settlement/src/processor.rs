use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::error::SettlementError;
use crate::instruction::SettlementInstruction;
use crate::state::Counter;

// program state handler
pub struct Processor {}
impl<'a> Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo<'a>],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // Process Accounts
        let accounts_info_iter = &mut accounts.iter();
        let owner_account: &AccountInfo<'a> = next_account_info(accounts_info_iter)?;
        let counter_account: &AccountInfo<'a> = next_account_info(accounts_info_iter)?;

        // validate accounts
        Self::validate(program_id, owner_account, counter_account)?;

        // get instruction
        // let instruction = SettlementInstruction::try_from_slice(instruction_data)?;
        // match instruction {
        //     SettlementInstruction::InitializeCounter(bump) => {
        //         Self::process_initialize_account(program_id, owner_account, counter_account, bump)?
        //     }
        // }

        Ok(())
    }

    /// This validate function validates the following:
    /// - owner is [SIGNER]
    /// - counter is [SIGNER, WRITE]
    /// - counter is owned by the passed owner
    /// - passed counter PDA == computed PDA
    fn validate(program_id: &Pubkey, owner: &AccountInfo, counter: &AccountInfo) -> ProgramResult {
        Ok(())
    }
}
