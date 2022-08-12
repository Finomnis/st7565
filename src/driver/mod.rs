mod common_functionality;

mod mode_graphics;
mod mode_initial;
mod mode_raw;

use core::marker::PhantomData;

/// The actual driver
pub struct ST7565<DI, SPECS, MODE, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize> {
    interface: DI,
    display_specs: PhantomData<SPECS>,
    mode: MODE,
}
