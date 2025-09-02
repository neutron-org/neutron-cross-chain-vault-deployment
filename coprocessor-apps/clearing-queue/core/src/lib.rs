#![no_std]

extern crate alloc;

pub const VAULT_ADDRESS: &str = "0x9eeb468751a293d584d93bfaffa9e853c540cfff";

mod proof;
mod types;

pub use proof::*;
pub use types::*;
