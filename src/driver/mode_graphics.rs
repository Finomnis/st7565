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
    DisplaySpecs, ST7565,
};

pub struct GraphicsMode<const WIDTH: usize, const PAGES: usize> {
    page_buffers: [([u8; WIDTH], Option<Range<usize>>); PAGES],
}

impl<const WIDTH: usize, const PAGES: usize> Default for GraphicsMode<WIDTH, PAGES> {
    fn default() -> Self {
        Self {
            // Fill with full dirty flags to force an initial synchronization
            page_buffers: [(); PAGES].map(|()| ([0; WIDTH], Some(0..WIDTH))),
        }
    }
}

/// ---- Functionality of the graphics mode ----
/// ============================================
///
/// In this mode, the driver can be used as a [DrawTarget] for the [embedded_graphics](embedded_graphics_core) crate.
impl<
        DI: WriteOnlyDataCommand,
        SPECS,
        const WIDTH: usize,
        const HEIGHT: usize,
        const PAGES: usize,
    > ST7565<DI, SPECS, GraphicsMode<WIDTH, PAGES>, WIDTH, HEIGHT, PAGES>
{
    /// Flushes the internal buffer to the screen.
    ///
    /// Needs to be called after drawing to actually display the data on screen.
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
                    self.interface.send_data(U8(&buffer[range]))?;
                }
            }
        }

        Ok(())
    }
}

impl<
        DI: WriteOnlyDataCommand,
        SPECS,
        const WIDTH: usize,
        const HEIGHT: usize,
        const PAGES: usize,
    > DrawTarget for ST7565<DI, SPECS, GraphicsMode<WIDTH, PAGES>, WIDTH, HEIGHT, PAGES>
where
    SPECS: DisplaySpecs<WIDTH, HEIGHT, PAGES>,
{
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(Point { x, y }, color) in pixels.into_iter() {
            let x = x as usize;
            let page = (y / 8) as usize;
            let y_offset = (y % 8) as u8;

            if let Some((buffer, dirty)) = self.mode.page_buffers.get_mut(page) {
                if let Some(buffer_line) = buffer.get_mut(x) {
                    let updated = match color {
                        BinaryColor::On => *buffer_line | (1u8 << y_offset),
                        BinaryColor::Off => *buffer_line & (!(1u8 << y_offset)),
                    };

                    if updated != *buffer_line {
                        match dirty {
                            Some(dirty_range) => {
                                dirty_range.start = dirty_range.start.min(x);
                                dirty_range.end = dirty_range.end.max(x + 1);
                            }
                            None => *dirty = Some(x..(x + 1)),
                        };
                        *buffer_line = updated;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<
        DI: WriteOnlyDataCommand,
        SPECS,
        const WIDTH: usize,
        const HEIGHT: usize,
        const PAGES: usize,
    > OriginDimensions for ST7565<DI, SPECS, GraphicsMode<WIDTH, PAGES>, WIDTH, HEIGHT, PAGES>
where
    SPECS: DisplaySpecs<WIDTH, HEIGHT, PAGES>,
{
    fn size(&self) -> Size {
        Size {
            width: WIDTH as u32,
            height: HEIGHT as u32,
        }
    }
}
