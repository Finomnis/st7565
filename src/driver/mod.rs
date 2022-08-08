mod common_functionality;

mod mode_graphics;
mod mode_initial;
mod mode_raw;

use core::marker::PhantomData;

use display_interface::WriteOnlyDataCommand;

use crate::DisplaySpecs;

/// ST7565 driver.
pub struct ST7565<DI: WriteOnlyDataCommand, SPECS: DisplaySpecs, MODE> {
    interface: DI,
    display_specs: PhantomData<SPECS>,
    mode: MODE,
}
