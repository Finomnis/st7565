use display_interface::WriteOnlyDataCommand;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use crate::Error;

/// ST7565 driver.
pub struct ST7565<DI> {
    pub(crate) interface: DI,
}

impl<DI> ST7565<DI>
where
    DI: WriteOnlyDataCommand,
{
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
