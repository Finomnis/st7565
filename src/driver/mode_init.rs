use display_interface::WriteOnlyDataCommand;

use super::{mode_graphics::GraphicsMode, mode_raw::RawMode};
use crate::ST7565;

pub struct InitialMode;

impl<DI: WriteOnlyDataCommand> ST7565<DI, InitialMode> {
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

    pub fn into_graphics_mode<const WIDTH: u8, const HEIGHT: u8>(
        self,
    ) -> ST7565<DI, GraphicsMode<WIDTH, HEIGHT>> {
        self.into_mode(GraphicsMode::new())
    }
}
