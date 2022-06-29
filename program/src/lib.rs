pub mod consts;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("98VZJGjpUc7QfQi9KJQsjDe1abDLDiDMD7ndT98YZY9S");

pub type Timestamp = u64;
