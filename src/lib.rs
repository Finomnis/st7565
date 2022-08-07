#![no_main]
#![no_std]

#[cfg(test)]
mod tests;

mod command;
mod error;

use display_interface::WriteOnlyDataCommand;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use error::Error;

/// ST7565 driver.
pub struct ST7565<DI, SIZE> {
    interface: DI,
    size: SIZE,
}

impl<DI, SIZE> ST7565<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    //SIZE: DisplaySize,
{
    /// Create a basic SSD1306 interface.
    pub fn new(interface: DI, size: SIZE) -> Self {
        Self { interface, size }
    }

    /// Reset the display.
    pub fn reset<RST, DELAY, PinE>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), Error<(), PinE>>
    where
        RST: OutputPin<Error = PinE>,
        DELAY: DelayMs<u8>,
    {
        rst.set_high().map_err(Error::Pin)?;
        delay.delay_ms(1);
        rst.set_low().map_err(Error::Pin)?;
        delay.delay_ms(1);
        rst.set_high().map_err(Error::Pin)
    }
}
