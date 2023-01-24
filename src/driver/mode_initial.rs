use core::marker::PhantomData;

use display_interface::WriteOnlyDataCommand;

use super::{mode_graphics::GraphicsMode, mode_raw::RawMode};
use crate::{DisplaySpecs, GraphicsPageBuffer, ST7565};

pub struct InitialMode;

/// ---- Functionality of the initial mode ----
/// ===========================================
///
/// This mode is purely to transition into other modes.
impl<DI, SPECS, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize>
    ST7565<DI, SPECS, InitialMode, WIDTH, HEIGHT, PAGES>
where
    DI: WriteOnlyDataCommand,
    SPECS: DisplaySpecs<WIDTH, HEIGHT, PAGES>,
{
    /// Creates an ST7565 driver.
    pub fn new(interface: DI, _display_specs: SPECS) -> Self {
        Self {
            interface,
            display_specs: PhantomData,
            mode: InitialMode,
        }
    }

    fn into_mode<MODE>(self, mode: MODE) -> ST7565<DI, SPECS, MODE, WIDTH, HEIGHT, PAGES> {
        ST7565 {
            interface: self.interface,
            display_specs: self.display_specs,
            mode,
        }
    }

    /// Transitions the driver into raw mode.
    ///
    /// Raw mode provides functionality to control the ST7565 chip directly
    /// through low-level commands.
    pub fn into_raw_mode(self) -> ST7565<DI, SPECS, RawMode, WIDTH, HEIGHT, PAGES> {
        self.into_mode(RawMode)
    }

    /// Transitions the driver into graphics mode.
    ///
    /// Graphics mode enables the driver to be used as a [DrawTarget](embedded_graphics_core::draw_target::DrawTarget) for the [embedded_graphics](embedded_graphics_core) crate.
    pub fn into_graphics_mode(
        self,
        buffer: &mut GraphicsPageBuffer<WIDTH, PAGES>,
    ) -> ST7565<DI, SPECS, GraphicsMode<'_, WIDTH, PAGES>, WIDTH, HEIGHT, PAGES> {
        self.into_mode(GraphicsMode::new(buffer))
    }
}
