#![no_std]

extern crate alloc;

pub const VAULT_ADDRESS: &str = "0x65bce996fee6b43c899fb6a0a621bdc4bd49fde4";

mod proof;
mod types;

pub use proof::*;
pub use types::*;
