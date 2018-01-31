//! Help Scout API Bindings in Rust Documentation
//!
//! Help Scout References:
//! * [Developer Docs](https://developer.helpscout.com/)
//! * [Help Desk API](https://developer.helpscout.com/help-desk-api/)
//!
//! Note: the api module implements the Help Desk API.
//!
//! ## Usage
//!
//! Add the dependency to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! helpscout = "*"
//! ```
//!
//! Create a client and then use any of the endpoints.
//!
//! ```rust
//! use helpscout::Client;
//!
//! fn main() {
//!     let api_key = env::var("HELPSCOUT_API_KEY").expect("to have HELPSCOUT_API_KEY set");
//!     let client = Client::new(&api_key);
//!
//!     // Used the mailbox list endpoint for test. Use whatever you need here.
//!     let mailboxes = mailboxes::list(&client);
//!
//!     assert!(mailboxes.items.len() > 0);
//! }
//! ```
//!
//! Additional client setup documentation can be found here:
//! [Client](struct.Client.html#method.new)
//!
//! ## Endpoints
//!
//! Drill down into endpoint documentation by following the links below.
//!
//! * [conversations](api/conversations/index.html)
//! * [customers](api/customers/index.html)
//! * [mailboxes](api/mailboxes/index.html)
//! * [reports](api/reports/index.html)
//! * [users](api/users/index.html)
extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_url_params;

extern crate chrono;

#[macro_use]
extern crate log;

// TODO: Possibly make conditionally compiled?
extern crate dotenv;

// Make this enabled by webhook feature
extern crate ring;

mod error;
pub use error::HelpScoutError;

mod client;
pub use client::{Client, Status};

mod envelope;
pub use envelope::{Collection, Item};

mod date_format;

pub mod api;
pub mod webhook;
