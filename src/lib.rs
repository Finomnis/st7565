#![no_main]
#![no_std]

#[cfg(test)]
mod tests;

mod command;
mod display_specs;
mod driver;
mod error;

pub use command::BoosterRatio;
pub use command::PowerControlMode;
pub use command::StaticIndicatorMode;
pub use display_specs::DisplaySpecs;
pub use driver::st7565_driver;
pub use driver::ST7565;
pub use error::Error;
