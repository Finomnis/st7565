use core::marker::PhantomData;

use display_interface::WriteOnlyDataCommand;

use super::{mode_graphics::GraphicsMode, mode_raw::RawMode};
use crate::{DisplaySpecs, ST7565};

pub struct InitialMode;

impl<DI, SPECS> ST7565<DI, SPECS, InitialMode>
where
    DI: WriteOnlyDataCommand,
    SPECS: DisplaySpecs,
{
    /// Creates an ST7565 driver.
    pub fn new(interface: DI, _display_specs: SPECS) -> Self {
        Self {
            interface,
            display_specs: PhantomData,
            mode: InitialMode,
        }
    }

    fn into_mode<MODE>(self, mode: MODE) -> ST7565<DI, SPECS, MODE> {
        ST7565 {
            interface: self.interface,
            display_specs: self.display_specs,
            mode,
        }
    }

    pub fn into_raw_mode(self) -> ST7565<DI, SPECS, RawMode> {
        self.into_mode(RawMode)
    }

    pub fn into_graphics_mode(self) -> ST7565<DI, SPECS, GraphicsMode<SPECS>> {
        self.into_mode(GraphicsMode::new())
    }
}
