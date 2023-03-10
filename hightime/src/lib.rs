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
#![deny(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_results
)]
#![cfg_attr(not(feature = "std"), no_std)]

mod date;
mod error;
#[cfg(feature = "unstable")]
mod time;

pub use date::{Date, Weekday};
pub use error::Error;
#[cfg(feature = "unstable")]
pub use time::{DateTime, FixedOffset, NaiveTime, Offset};
