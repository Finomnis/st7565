mod common_functionality;

mod mode_init;
pub struct ModeInit;

mod mode_raw;
pub struct ModeRaw;

mod mode_graphical;
pub struct ModeGraphical;

use display_interface::WriteOnlyDataCommand;

/// ST7565 driver.
pub struct ST7565<DI: WriteOnlyDataCommand, MODE> {
    interface: DI,
    display_specs: crate::DisplaySpecs,
    _mode: core::marker::PhantomData<MODE>,
}

/// Creates an ST7565 driver.
pub fn st7565_driver<DI>(interface: DI, display_specs: crate::DisplaySpecs) -> ST7565<DI, ModeInit>
where
    DI: WriteOnlyDataCommand,
{
    ST7565::<DI, ModeInit> {
        interface,
        display_specs,
        _mode: core::marker::PhantomData,
    }
}
