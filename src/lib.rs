//! This crate provides an [embedded-graphics](https://crates.io/crates/embedded-graphics)
//! compatible driver for displays based on the st7565 chipset.
//!
//! # Example
//!
//! ```
//! ```

#![no_main]
#![no_std]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![doc(issue_tracker_base_url = "https://github.com/Finomnis/st7565/issues")]

#[cfg(test)]
mod tests;

mod command;
mod display_specs;
mod driver;
mod error;

pub mod displays;
pub mod types;

pub use display_specs::DisplaySpecs;
pub use driver::ST7565;
pub use error::Error;
