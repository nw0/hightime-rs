//! A high-level time library for Rust.
//!
//! `hightime` aims to be:
//!
//! - Simple and API-stable
//! - Well-documented
//! - Minimal in implementation and dependencies
mod date;
mod error;

pub use date::NaiveDate;
pub use error::Error;
