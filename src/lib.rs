//!
//! This crate aims to provide an
//! [embedded-graphics](https://crates.io/crates/embedded-graphics)
//! compatible driver for displays based on the st7565 chipset.
//!
//! ## Example:
//!
//! The following code is intended for the [DOGM132W-5](https://www.displayvisions.us/products/dog.html) display connected to an [nRF52840](https://www.nordicsemi.com/products/nrf52840)
//! microcontroller.
//!
//! Note the `MODE_3` of the SPI. The DOGM132W-5 display expects the clock to be configured
//! with `CPOL=1` and `CPHA=1`, which is also called `SPI Mode 3`.
//!
//! ```Rust
//! // Create DOGM132W-5 spi bus
//! let disp_spi = SPIInterface::new(
//!     hal::Spim::new(
//!         peripherals.SPIM0,
//!         hal::spim::Pins {
//!             sck: disp_scl,
//!             mosi: Some(disp_si),
//!             miso: None,
//!         },
//!         hal::spim::Frequency::M8,
//!         hal::spim::MODE_3,
//!         0,
//!     ),
//!     disp_a0,
//!     disp_cs,
//! );
//!
//! // Create DOGM132W-5 display driver
//! let mut disp = ST7565::new(disp_spi, DOGM132W5).into_graphics_mode();
//! disp.reset(&mut disp_rst, &mut timer).unwrap();
//! disp.flush().unwrap();
//! disp.set_display_on(true).unwrap();
//!
//! // Draw on it using the embedded_graphics library
//! Circle::new(Point::new(10, 6), 20)
//!     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 2))
//!     .draw(&mut disp)
//!     .unwrap();
//! disp.flush().unwrap();
//! ```
//!
//! Note the `DOGM132W5` object. This is the display specification that contains all the display specific configuration options that need to be applied to the st7565 chip.
//!
//! Further note the `.into_graphics_mode()` call, which switches the driver from its initial
//! mode to the [embedded-graphics](https://crates.io/crates/embedded-graphics) driver mode.
//! The `disp` object can then be used as a `DrawTarget` in `embedded-graphics` calls.
//!
//! After drawing something, a `.flush()` call has to be issued to actually
//! send the modified data to the display.
//!
//!
//! ## Adding support for a new st7565 based display
//!
//! The example above uses the `DOGM132W5` struct in the `ST7565::new()` call.
//!
//! To initialize the `ST7565` driver struct with a different display, a new display
//! specification has to be created. This can be done by creating an empty struct that
//! implements the [DisplaySpecs] object.
//!
//! For example, the definition for the `DOGM132W5` struct looks like this:
//! ```Rust
//! pub struct DOGM132W5;
//! impl DisplaySpecs<132, 32, 4> for DOGM132W5 {
//!     const FLIP_ROWS: bool = false;
//!     const FLIP_COLUMNS: bool = true;
//!     const INVERTED: bool = false;
//!     const BIAS_MODE_1: bool = false;
//!     const POWER_CONTROL: PowerControlMode = PowerControlMode {
//!         booster_circuit: true,
//!         voltage_regulator_circuit: true,
//!         voltage_follower_circuit: true,
//!     };
//!     const VOLTAGE_REGULATOR_RESISTOR_RATIO: u8 = 0b011;
//!     const ELECTRONIC_VOLUME: u8 = 0b011111;
//!     const BOOSTER_RATIO: BoosterRatio = BoosterRatio::StepUp2x3x4x;
//! }
//! ```
//!
//! The exact values for the respective display have to be taken from the displays manual.
//!
//! If you created a specification for a new display, please open a pull request on <https://github.com/Finomnis/st7565/pulls> in order to make it available to the public.
//!

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
