use display_interface::{DisplayError, WriteOnlyDataCommand};
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use crate::{
    command::{Command, SendSt7565Command},
    DisplaySpecs, Error, StaticIndicatorMode,
};

use super::{ModeInit, ST7565};

impl<DI: WriteOnlyDataCommand, MODE> ST7565<DI, MODE> {
    /// Set the static indicator
    pub fn set_static_indicator(
        &mut self,
        mode: Option<StaticIndicatorMode>,
    ) -> Result<(), DisplayError> {
        self.interface
            .send_command(Command::StaticIndicatorSet { mode })
    }

    /// Enable/Disable the display output
    pub fn set_display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.interface.send_command(Command::DisplayOnOff { on })
    }

    /// Reset the display and restore all settings
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
        delay.delay_ms(1);

        // Initialize display

        // LCD Bias
        self.interface
            .send_command(Command::LcdBiasSet {
                bias_mode_1: self.display_specs.bias_mode_1,
            })
            .map_err(Error::Comm)?;

        // ADC Selection
        self.interface
            .send_command(Command::AdcSelect {
                reverse: self.display_specs.flip_columns,
            })
            .map_err(Error::Comm)?;

        // Common output mode selection
        self.interface
            .send_command(Command::CommonOutputModeSelect {
                reverse_direction: self.display_specs.flip_rows,
            })
            .map_err(Error::Comm)?;

        // Display invertion
        self.interface
            .send_command(Command::DisplayNormalReverse {
                reverse: self.display_specs.inverted,
            })
            .map_err(Error::Comm)?;

        // Booster ratio
        self.interface
            .send_command(Command::BoosterRatioSet {
                stepup_value: self.display_specs.booster_ratio,
            })
            .map_err(Error::Comm)?;

        // voltage regulator resistor ratio
        self.interface
            .send_command(Command::VoltageRegulatorInternalResistorSet {
                resistor_ratio: self.display_specs.voltage_regulator_resistor_ratio,
            })
            .map_err(Error::Comm)?;

        // electric volume
        self.interface
            .send_command(Command::ElectronicVolumeSet {
                volume_value: self.display_specs.electronic_volume,
            })
            .map_err(Error::Comm)?;

        // power control
        self.interface
            .send_command(Command::PowerControlSet {
                mode: self.display_specs.power_control,
            })
            .map_err(Error::Comm)?;

        Ok(())
    }
}
