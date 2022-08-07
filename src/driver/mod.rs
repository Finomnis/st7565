mod common_functionality;

mod mode_graphics;
mod mode_init;
mod mode_raw;

use display_interface::WriteOnlyDataCommand;

/// ST7565 driver.
pub struct ST7565<DI: WriteOnlyDataCommand, MODE> {
    interface: DI,
    display_specs: crate::DisplaySpecs,
    mode: MODE,
}

/// Creates an ST7565 driver.
pub fn st7565_driver<DI>(
    interface: DI,
    display_specs: crate::DisplaySpecs,
) -> ST7565<DI, mode_init::InitialMode>
where
    DI: WriteOnlyDataCommand,
{
    ST7565 {
        interface,
        display_specs,
        mode: mode_init::InitialMode,
    }
}
