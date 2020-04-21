#[macro_use]
extern crate num_derive;

use std::fmt::{self, Formatter};

pub mod commands;
pub mod enums;
pub mod events;

mod client;
mod error;
mod manager;

pub use client::Client;
pub use error::FlicError;
pub use manager::Manager;

pub type Result<T> = std::result::Result<T, error::FlicError>;

/// Flic's representation of a Bluetooth address, stored as 6 little endian-encoded bytes.
#[derive(PartialEq)]
pub struct BdAddr([u8; 6]);

impl BdAddr {
    fn to_vec(&self) -> Vec<u8> {
        self.0.clone().to_vec()
    }
}

impl fmt::Debug for BdAddr {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), fmt::Error> {
        <BdAddr as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for BdAddr {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), fmt::Error> {
        for i in (0..6).rev() {
            write!(f, "{}", hex::encode(&self.0[i..i + 1]))?;
            if i != 0 {
                write!(f, "{}", ":")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bd_addr_display() {
        let bd_addr = BdAddr([0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08]);
        assert_eq!(format!("{}", bd_addr), "08:09:0a:0b:0c:0d");
    }
}
