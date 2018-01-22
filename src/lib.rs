extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod error;
pub use error::HelpScoutError;

mod client;
pub use client::{Client, Status};

pub mod api;
