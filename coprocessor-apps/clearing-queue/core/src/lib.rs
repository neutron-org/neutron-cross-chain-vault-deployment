#![no_std]

extern crate alloc;

pub const VAULT_ADDRESS: &str = "0x1d2042c6770d01f4c91cfe128cfc3aff1ec1561c";

mod proof;
mod types;

pub use proof::*;
pub use types::*;
