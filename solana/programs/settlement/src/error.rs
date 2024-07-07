use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Debug, Error, FromPrimitive)]
pub enum SettlementError {
    #[error("account already initialized")]
    AlreadyInitialized,
}

impl From<SettlementError> for ProgramError {
    fn from(e: SettlementError) -> Self {
        // https://docs.rs/solana-program/latest/solana_program/program_error/enum.ProgramError.html#variant.Custom
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for SettlementError {
    fn type_of() -> &'static str {
        "SettlementError"
    }
}

impl PrintProgramError for SettlementError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            SettlementError::AlreadyInitialized => msg!("Error: account already initialized"),
        }
    }
}
