pub mod consts;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("GyC8iyGUyVxM9ovGw6DBpPnXLWXw6aAeXB2A8SEVqnN3");

pub type Timestamp = u64;
