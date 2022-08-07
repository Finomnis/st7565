use display_interface::WriteOnlyDataCommand;

use crate::ST7565;

pub struct GraphicsMode<const WIDTH: u8, const HEIGHT: u8>;

impl<const WIDTH: u8, const HEIGHT: u8> GraphicsMode<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Self
    }
}

impl<DI: WriteOnlyDataCommand, const WIDTH: u8, const HEIGHT: u8>
    ST7565<DI, GraphicsMode<WIDTH, HEIGHT>>
{
}
