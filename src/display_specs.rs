use crate::types::{BoosterRatio, PowerControlMode};

/// Represents a display specification
///
/// Const generic parameters:
///    - *WIDTH*: the horizontal resolution of the display
///    - *HEIGHT*: the vertical resolution of the display
///    - *PAGES*: the number of pages used by the display; usually *HEIGHT* / 8 (rounded up)
///
/// To create a custom display specification, create an empty struct that
/// implements this trait.
pub trait DisplaySpecs<const WIDTH: usize, const HEIGHT: usize, const PAGES: usize> {
    /// Mirrors vertically
    const FLIP_ROWS: bool;

    /// Mirrors horizontally
    const FLIP_COLUMNS: bool;

    /// Inverts the pixels
    const INVERTED: bool;

    /// Whether the LCD bias mode needs to be "1".
    ///
    /// For more information, read the ST7565 reference manual.
    const BIAS_MODE_1: bool;

    /// Which parts of the internal power circuits need to be enabled
    const POWER_CONTROL: PowerControlMode;

    /// The required ratio of the internal voltage regulator resistors
    const VOLTAGE_REGULATOR_RESISTOR_RATIO: u8;

    /// The electronic volume of the driver stage
    const ELECTRONIC_VOLUME: u8;

    /// The internal booster ratio
    const BOOSTER_RATIO: BoosterRatio;

    /// The offset of the first column on the display
    const COLUMN_OFFSET: u8;
}
