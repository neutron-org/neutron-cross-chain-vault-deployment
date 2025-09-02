#![no_std]

extern crate alloc;

pub const VAULT_ADDRESS: &str = "0x63544ac5d49956db530baf37f7f7df9a7dce6cd1";

mod proof;
mod types;

pub use proof::*;
pub use types::*;
