#![no_std]

extern crate alloc;

pub const VAULT_ADDRESS: &str = "0x9d6900be765ef542d984eabda75f65a7609713d7";

mod proof;
mod types;

pub use proof::*;
pub use types::*;
