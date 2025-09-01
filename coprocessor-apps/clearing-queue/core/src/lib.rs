#![no_std]

extern crate alloc;

pub const VAULT_ADDRESS: &str = "0xbee15e5a0c3f2fbd8f4d39c3563eedd5564e9fe7";

mod proof;
mod types;

pub use proof::*;
pub use types::*;
