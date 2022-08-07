use core::ops::Range;

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use crate::{
    command::{Command, SendSt7565Command},
    ST7565,
};

pub struct GraphicsMode<const WIDTH: usize, const HEIGHT: usize, const PAGES: usize> {
    page_buffers: [([u8; WIDTH], Option<Range<usize>>); PAGES],
}

impl<const WIDTH: usize, const HEIGHT: usize, const PAGES: usize>
    GraphicsMode<WIDTH, HEIGHT, PAGES>
{
    pub fn new() -> Self {
        Self {
            page_buffers: [(); PAGES].map(|()| ([0; WIDTH], Some(0..WIDTH))),
        }
    }
}

impl<DI: WriteOnlyDataCommand, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize>
    ST7565<DI, GraphicsMode<WIDTH, HEIGHT, PAGES>>
{
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        for (page, (buffer, dirty)) in self.mode.page_buffers.iter_mut().enumerate() {
            let page = page as u8;

            if let Some(range) = dirty.take() {
                if range.start < range.end && range.start < WIDTH {
                    self.interface
                        .send_command(Command::PageAddressSet { address: page })?;
                    self.interface.send_command(Command::ColumnAddressSet {
                        address: range.start as u8,
                    })?;
                    self.interface.send_data(U8(buffer))?;
                }
            }
        }

        Ok(())
    }
}

impl<DI: WriteOnlyDataCommand, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize>
    DrawTarget for ST7565<DI, GraphicsMode<WIDTH, HEIGHT, PAGES>>
{
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(Point { x, y }, color) in pixels.into_iter() {
            todo!()
        }

        Ok(())
    }
}

impl<DI: WriteOnlyDataCommand, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize>
    OriginDimensions for ST7565<DI, GraphicsMode<WIDTH, HEIGHT, PAGES>>
{
    fn size(&self) -> Size {
        Size {
            width: WIDTH as u32,
            height: HEIGHT as u32,
        }
    }
}
