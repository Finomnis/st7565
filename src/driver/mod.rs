mod common_functionality;

pub mod mode_graphics;
pub mod mode_initial;
mod mode_raw;

mod page_buffer;

use core::marker::PhantomData;

pub use page_buffer::GraphicsPageBuffer;

/// The actual driver
pub struct ST7565<DI, SPECS, MODE, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize> {
    interface: DI,
    display_specs: PhantomData<SPECS>,
    mode: MODE,
}
