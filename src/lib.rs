#![no_std]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/Finomnis/st7565/issues")]
// Building tests on target with defmt_tests requires `no_main`
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

#[cfg(test)]
mod tests;

mod command;
mod display_specs;
mod driver;
mod error;

pub mod displays;
pub mod types;

pub use display_specs::DisplaySpecs;
pub use driver::GraphicsPageBuffer;
pub use driver::ST7565;
pub use error::Error;

/// Operating modes of the driver
pub mod modes {
    pub use crate::driver::mode_graphics::GraphicsMode;
    pub use crate::driver::mode_initial::InitialMode;
    pub use crate::driver::mode_raw::RawMode;
}
