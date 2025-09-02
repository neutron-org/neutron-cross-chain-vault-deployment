#![no_std]

extern crate alloc;

pub const VAULT_ADDRESS: &str = "0xac2f425e902cd25ba286b14ce4a7f9162a859294";

mod proof;
mod types;

pub use proof::*;
pub use types::*;
