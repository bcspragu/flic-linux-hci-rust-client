#[macro_use]
extern crate num_derive;

pub mod commands;
pub mod enums;
pub mod error;
pub mod events;

mod client;

pub use client::Client;

pub type Result<T> = std::result::Result<T, error::FlicError>;
