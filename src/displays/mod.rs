use crate::{
    types::{BoosterRatio, PowerControlMode},
    DisplaySpecs,
};

pub struct DOGM132W5;
impl DisplaySpecs<132, 32, 4> for DOGM132W5 {
    const FLIP_ROWS: bool = false;
    const FLIP_COLUMNS: bool = true;
    const INVERTED: bool = false;
    const BIAS_MODE_1: bool = false;
    const POWER_CONTROL: PowerControlMode = PowerControlMode {
        booster_circuit: true,
        voltage_regulator_circuit: true,
        voltage_follower_circuit: true,
    };
    const VOLTAGE_REGULATOR_RESISTOR_RATIO: u8 = 0b011;
    const ELECTRONIC_VOLUME: u8 = 0b011111;
    const BOOSTER_RATIO: BoosterRatio = BoosterRatio::StepUp2x3x4x;
}
