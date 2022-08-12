#![no_main]
#![no_std]
//#![deny(missing_docs)]
#![forbid(unsafe_code)]

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
