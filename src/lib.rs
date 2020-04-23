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
        write!(
            f,
            "{}:{}:{}:{}:{}:{}",
            hex::encode(&self.0[5..6]),
            hex::encode(&self.0[4..5]),
            hex::encode(&self.0[3..4]),
            hex::encode(&self.0[2..3]),
            hex::encode(&self.0[1..2]),
            hex::encode(&self.0[0..1]),
        )
    }
}

#[derive(PartialEq)]
pub struct Uuid([u8; 16]);

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), fmt::Error> {
        <Uuid as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), fmt::Error> {
        write!(
            f,
            "{}-{}-{}-{}-{}",
            hex::encode(&self.0[..4]),
            hex::encode(&self.0[4..6]),
            hex::encode(&self.0[6..8]),
            hex::encode(&self.0[8..10]),
            hex::encode(&self.0[10..16]),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bd_addr_debug() {
        let bd_addr = BdAddr([0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08]);
        assert_eq!(format!("{:?}", bd_addr), "08:09:0a:0b:0c:0d");
    }

    #[test]
    fn uuid_debug() {
        let uuid = Uuid([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ]);
        assert_eq!(
            format!("{:?}", uuid),
            "00010203-0405-0607-0809-0a0b0c0d0e0f"
        );
    }
}
