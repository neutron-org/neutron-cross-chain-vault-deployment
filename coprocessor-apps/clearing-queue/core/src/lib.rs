#![no_std]

extern crate alloc;

pub const VAULT_ADDRESS: &str = "0x821b6a88eb6c2ccde24104760e8167afdc81ad50";

mod proof;
mod types;

pub use proof::*;
pub use types::*;
