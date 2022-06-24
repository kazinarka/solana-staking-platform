pub mod consts;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("C2t7aRk2LUWQjafXrSGwkYVoTgt6viRo7xUcShC5qCR6");

pub type Timestamp = u64;