use display_interface::WriteOnlyDataCommand;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use crate::{
    command::{Command, SendSt7565Command},
    BoosterRatio, Error, PowerControlMode,
};

/// ST7565 driver.
pub struct ST7565<DI> {
    pub(crate) interface: DI,
    pub(crate) lcd_bias_mode: bool,
    pub(crate) power_control_mode: PowerControlMode,
    pub(crate) voltage_regulator_resistor_ratio: u8,
    pub(crate) electric_volume: u8,
    pub(crate) booster_ratio: BoosterRatio,
}

impl<DI> ST7565<DI>
where
    DI: WriteOnlyDataCommand,
{
    /// Set the static indicator
    // /pub fn set_static_indicator(

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
        // Reset display
        rst.set_low().map_err(Error::Pin)?;
        delay.delay_ms(1);
        rst.set_high().map_err(Error::Pin)?;

        // Initialize display

        // LCD Bias
        self.interface
            .send_command(Command::LcdBiasSet {
                bias_1_7: self.lcd_bias_mode,
            })
            .map_err(Error::Comm)?;

        // ADC Selection - TODO
        // Common output mode selection - TODO

        // Booster ratio
        self.interface
            .send_command(Command::BoosterRatioSet {
                stepup_value: self.booster_ratio,
            })
            .map_err(Error::Comm)?;

        // v0 regulator resistor ratio
        self.interface
            .send_command(Command::VoltageRegulatorInternalResistorSet {
                resistor_ratio: self.voltage_regulator_resistor_ratio,
            })
            .map_err(Error::Comm)?;

        // electric volume
        self.interface
            .send_command(Command::ElectronicVolumeSet {
                volume_value: self.electric_volume,
            })
            .map_err(Error::Comm)?;

        // power control
        self.interface
            .send_command(Command::PowerControlSet {
                mode: self.power_control_mode,
            })
            .map_err(Error::Comm)?;

        // initialize dram - TODO

        Ok(())
    }
}
