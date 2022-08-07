#![no_main]
#![no_std]

#[cfg(test)]
mod tests;

mod command;
mod driver;
mod driver_builder;
mod error;

pub use command::BoosterRatio;
pub use command::PowerControlMode;
pub use driver::ST7565;
pub use driver_builder::ST7565DriverBuilder;
pub use error::Error;
