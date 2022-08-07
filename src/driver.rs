use display_interface::WriteOnlyDataCommand;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use crate::{
    command::{Command, SendSt7565Command},
    Error,
};

/// ST7565 driver.
pub struct ST7565<DI> {
    pub(crate) interface: DI,
    pub(crate) lcd_bias_mode: bool,
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
    ) -> Result<(), Error<PinE>>
    where
        RST: OutputPin<Error = PinE>,
        DELAY: DelayMs<u8>,
    {
        rst.set_low().map_err(Error::Pin)?;
        delay.delay_ms(1);
        rst.set_high().map_err(Error::Pin)?;

        // Initialize

        // LCD Bias
        self.interface
            .send_command(Command::LcdBiasSet {
                bias_1_7: self.lcd_bias_mode,
            })
            .map_err(Error::Comm)?;

        // ADC Selection - TODO
        // Common output mode selection - TODO

        // v0 regulator resistor ratio
        self.interface
            .send_command(Command::V0VoltageRegulatorInternalResistorSet { resistor_ratio: 0 })
            .map_err(Error::Comm)?;
        // electric volume

        // power control
        // self.interface
        //     .send_command(Command::PowerControlSet { mode: 0 })
        //     .map_err(Error::Comm)?;

        // initialize dram

        Ok(())
    }
}