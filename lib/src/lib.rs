//! `active911` is an unofficial library with serde definitions of the API along
//! with a function to request an agency's recent alarms.
//!
//! ```no_run
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let agency_key = "example key";
//! use active911::request;
//!
//! let response = request::alarms(agency_key).await?;
//!
//! for alarm in response.alarms {
//!    println!("alarm #{}: {}", alarm.id, alarm.description);
//! }
//! # Ok(()) }
//! ```
//!
//! Active911 is not on crates.io at this time. You can install the library by
//! depending on the Git repository in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! active911 = { default-features = false, git = "https://github.com/zeylahellyer/active911" }
//! ```
//!
//! # License
//!
//! This library is licensed under the ISC.

#![deny(
    clippy::all,
    clippy::pedantic,
    broken_intra_doc_links,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code,
    unused,
    warnings
)]

pub mod model;

#[cfg(feature = "http")]
pub mod request;
