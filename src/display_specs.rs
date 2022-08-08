use crate::{BoosterRatio, PowerControlMode};

#[derive(Debug, Clone)]
/// Represents a display specification
pub struct DisplaySpecs<const WIDTH: usize, const HEIGHT: usize, const PAGES: usize> {
    /// Mirrors vertically
    pub flip_rows: bool,

    /// Mirrors horizontally
    pub flip_columns: bool,

    /// Inverts the pixels
    pub inverted: bool,

    /// Whether the LCD bias mode needs to be "1".
    ///
    /// For more information, read the st7565 reference manual.
    pub bias_mode_1: bool,

    /// Which parts of the internal power circuits need to be enabled
    pub power_control: PowerControlMode,

    /// The required ratio of the internal voltage regulator resistors
    pub voltage_regulator_resistor_ratio: u8,

    /// The electronic volume of the driver stage
    pub electronic_volume: u8,

    /// The internal booster ratio
    pub booster_ratio: BoosterRatio,
}
