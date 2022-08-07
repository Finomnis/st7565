use display_interface::WriteOnlyDataCommand;

use super::{mode_graphics::GraphicsMode, mode_raw::RawMode};
use crate::{DisplaySpecs, ST7565};

pub struct InitialMode;

impl<DI: WriteOnlyDataCommand> ST7565<DI, InitialMode> {
    /// Creates an ST7565 driver.
    pub fn new(interface: DI, display_specs: DisplaySpecs) -> Self {
        Self {
            interface,
            display_specs,
            mode: InitialMode,
        }
    }

    fn into_mode<MODE>(self, mode: MODE) -> ST7565<DI, MODE> {
        ST7565 {
            interface: self.interface,
            display_specs: self.display_specs,
            mode,
        }
    }

    pub fn into_raw_mode(self) -> ST7565<DI, RawMode> {
        self.into_mode(RawMode)
    }

    pub fn into_graphics_mode<const WIDTH: usize, const PAGES: usize>(
        self,
    ) -> ST7565<DI, GraphicsMode<WIDTH, PAGES>> {
        self.into_mode(GraphicsMode::new())
    }
}
