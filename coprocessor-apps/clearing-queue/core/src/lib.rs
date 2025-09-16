#![no_std]

extern crate alloc;

pub const VAULT_ADDRESS: &str = "0xa1e5a311d2dc7bfe4ae770e23225839b688a2456";

mod proof;
mod types;

pub use proof::*;
pub use types::*;
