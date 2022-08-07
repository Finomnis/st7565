mod common_functionality;

mod mode_graphics;
mod mode_initial;
mod mode_raw;

use display_interface::WriteOnlyDataCommand;

/// ST7565 driver.
pub struct ST7565<DI: WriteOnlyDataCommand, MODE> {
    interface: DI,
    display_specs: crate::DisplaySpecs,
    mode: MODE,
}
