#[macro_use]
extern crate num_derive;

pub mod commands;
pub mod enums;
pub mod error;
pub mod events;

mod client;
mod manager;

pub use client::Client;
pub use manager::Manager;

pub type Result<T> = std::result::Result<T, error::FlicError>;
