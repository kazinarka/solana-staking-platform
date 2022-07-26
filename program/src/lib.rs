pub mod consts;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("");

pub type Timestamp = u64;
