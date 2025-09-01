#![no_std]

extern crate alloc;

pub const VAULT_ADDRESS: &str = "0x4dc0058ad13293a468a724660f4be0bbc46c9383";

mod proof;
mod types;

pub use proof::*;
pub use types::*;
