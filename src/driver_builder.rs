use display_interface::WriteOnlyDataCommand;

use crate::{BoosterRatio, PowerControlMode, ST7565};

pub struct ST7565DriverBuilder<DI> {
    interface: DI,
    lcd_bias_mode: bool,
    power_control_mode: PowerControlMode,
    voltage_regulator_resistor_ratio_value: u8,
    electric_volume_value: u8,
    booster_ratio_value: BoosterRatio,
}

impl<DI> ST7565DriverBuilder<DI>
where
    DI: WriteOnlyDataCommand,
{
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            lcd_bias_mode: false,
            power_control_mode: PowerControlMode {
                booster_circuit: false,
                voltage_regulator_circuit: false,
                voltage_follower_circuit: false,
            },
            voltage_regulator_resistor_ratio_value: 0,
            electric_volume_value: 0,
            booster_ratio_value: BoosterRatio::StepUp2x3x4x,
        }
    }

    pub fn lcd_bias(mut self, mode: bool) -> Self {
        self.lcd_bias_mode = mode;
        self
    }

    pub fn power_control(mut self, mode: PowerControlMode) -> Self {
        self.power_control_mode = mode;
        self
    }

    pub fn voltage_regulator_resistor_ratio(mut self, value: u8) -> Self {
        self.voltage_regulator_resistor_ratio_value = value;
        self
    }

    pub fn electric_volume(mut self, value: u8) -> Self {
        self.electric_volume_value = value;
        self
    }

    pub fn booster_ratio(mut self, value: BoosterRatio) -> Self {
        self.booster_ratio_value = value;
        self
    }

    pub fn build(self) -> ST7565<DI> {
        ST7565 {
            interface: self.interface,
            lcd_bias_mode: self.lcd_bias_mode,
            power_control_mode: self.power_control_mode,
            voltage_regulator_resistor_ratio: self.voltage_regulator_resistor_ratio_value,
            electric_volume: self.electric_volume_value,
            booster_ratio: self.booster_ratio_value,
        }
    }
}
