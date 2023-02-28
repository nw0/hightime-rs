//! A high-level time library for Rust.
//!
//! `hightime` aims to be:
//!
//! - Simple and API-stable
//! - Well-documented
//! - Minimal in implementation and dependencies
//!
//! This is a work in progress.
//! Currently I am working out the API definitions.
//! See the git repository for more information.
mod date;
mod error;

pub use date::NaiveDate;
pub use error::Error;
