use display_interface::WriteOnlyDataCommand;

use super::{mode_graphics::GraphicsMode, mode_raw::RawMode};
use crate::{DisplaySpecs, ST7565};

pub struct InitialMode;

impl<DI: WriteOnlyDataCommand, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize>
    ST7565<DI, InitialMode, WIDTH, HEIGHT, PAGES>
{
    /// Creates an ST7565 driver.
    pub fn new(interface: DI, display_specs: DisplaySpecs<WIDTH, HEIGHT, PAGES>) -> Self {
        Self {
            interface,
            display_specs,
            mode: InitialMode,
        }
    }

    fn into_mode<MODE>(self, mode: MODE) -> ST7565<DI, MODE, WIDTH, HEIGHT, PAGES> {
        ST7565 {
            interface: self.interface,
            display_specs: self.display_specs,
            mode,
        }
    }

    pub fn into_raw_mode(self) -> ST7565<DI, RawMode, WIDTH, HEIGHT, PAGES> {
        self.into_mode(RawMode)
    }

    pub fn into_graphics_mode(
        self,
    ) -> ST7565<DI, GraphicsMode<WIDTH, HEIGHT, PAGES>, WIDTH, HEIGHT, PAGES> {
        self.into_mode(GraphicsMode::new())
    }
}
