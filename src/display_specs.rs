use crate::{BoosterRatio, PowerControlMode};

#[derive(Debug, Clone)]
pub struct DisplaySpecs {
    pub flip_rows: bool,
    pub flip_columns: bool,
    pub inverted: bool,
    pub bias_mode_1: bool,
    pub power_control: PowerControlMode,
    pub voltage_regulator_resistor_ratio: u8,
    pub electronic_volume: u8,
    pub booster_ratio: BoosterRatio,
}
