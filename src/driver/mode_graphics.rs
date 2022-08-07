use core::ops::Range;

use display_interface::WriteOnlyDataCommand;

use crate::ST7565;

pub struct GraphicsMode<const WIDTH: usize, const PAGES: usize> {
    buffer: [[u8; WIDTH]; PAGES],
    dirty: [Option<Range<usize>>; PAGES],
}

impl<const WIDTH: usize, const PAGES: usize> GraphicsMode<WIDTH, PAGES> {
    pub fn new() -> Self {
        Self {
            buffer: [[0; WIDTH]; PAGES],
            dirty: [(); PAGES].map(|()| Some(0..WIDTH)),
        }
    }
}

impl<DI: WriteOnlyDataCommand, const WIDTH: usize, const PAGES: usize>
    ST7565<DI, GraphicsMode<WIDTH, PAGES>>
{
}
