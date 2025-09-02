#![no_std]

extern crate alloc;

pub const VAULT_ADDRESS: &str = "0x03397fec419ea8461aa63b6f293c4ea1e6ea5c86";

mod proof;
mod types;

pub use proof::*;
pub use types::*;
