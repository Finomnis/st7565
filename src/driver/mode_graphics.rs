use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

use crate::{
    command::{Command, SendSt7565Command},
    DisplaySpecs, GraphicsPageBuffer, ST7565,
};

/// In this mode, the driver can be used as a [DrawTarget] for the [embedded_graphics](embedded_graphics_core) crate.
pub struct GraphicsMode<'a, const WIDTH: usize, const PAGES: usize> {
    page_buffers: &'a mut GraphicsPageBuffer<WIDTH, PAGES>,
}

/// Initialize GraphicsMode with a page buffer.
impl<'a, const WIDTH: usize, const PAGES: usize> GraphicsMode<'a, WIDTH, PAGES> {
    pub(crate) fn new(page_buffers: &'a mut GraphicsPageBuffer<WIDTH, PAGES>) -> Self {
        page_buffers.mark_dirty();
        Self { page_buffers }
    }
}

/// ---- Functionality of the graphics mode ----
/// ============================================
///
/// In this mode, the driver can be used as a [DrawTarget] for the [embedded_graphics](embedded_graphics_core) crate.
impl<
        'a,
        DI: WriteOnlyDataCommand,
        SPECS: DisplaySpecs<WIDTH, HEIGHT, PAGES>,
        const WIDTH: usize,
        const HEIGHT: usize,
        const PAGES: usize,
    > ST7565<DI, SPECS, GraphicsMode<'a, WIDTH, PAGES>, WIDTH, HEIGHT, PAGES>
{
    /// Flushes the internal buffer to the screen.
    ///
    /// Needs to be called after drawing to actually display the data on screen.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        for (address, page) in self.mode.page_buffers.pages.iter_mut().enumerate() {
            let address = address as u8;

            if let Some((start, end)) = page.dirty.take() {
                if start < end && start < WIDTH {
                    self.interface
                        .send_command(Command::PageAddressSet { address })?;
                    self.interface.send_command(Command::ColumnAddressSet {
                        address: SPECS::COLUMN_OFFSET + start as u8,
                    })?;
                    self.interface.send_data(U8(&page.data[start..end]))?;
                }
            }
        }

        Ok(())
    }

    /// Release the display interface object
    ///
    /// This is meant for situations where the display interface is shared between several devices.
    ///
    /// All functions that perform communication with the display are
    /// unavailable until the display interface is attached again.
    ///
    /// Note that the bus attached later does not need to be the same bus as returned by this function,
    /// so the returned SPI bus object may be ignored (for example if the display interface object
    /// is based on references instead of ownership).
    pub fn release_display_interface(
        self,
    ) -> (
        ST7565<(), SPECS, GraphicsMode<'a, WIDTH, PAGES>, WIDTH, HEIGHT, PAGES>,
        DI,
    ) {
        (
            ST7565 {
                interface: (),
                display_specs: self.display_specs,
                mode: self.mode,
            },
            self.interface,
        )
    }
}

/// ---- Functionality of the detached graphics mode ----
/// =====================================================
///
/// In this mode, the driver can be still used as a [DrawTarget] for the [embedded_graphics](embedded_graphics_core) crate,
/// but no display communication can happen until the display interface is attached again.
///
/// This makes it possible to share the SPI bus with multiple devices.
///
impl<'a, SPECS, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize>
    ST7565<(), SPECS, GraphicsMode<'a, WIDTH, PAGES>, WIDTH, HEIGHT, PAGES>
{
    /// Attach the display interface back to the driver
    pub fn attach_display_interface<DI: WriteOnlyDataCommand>(
        self,
        interface: DI,
    ) -> ST7565<DI, SPECS, GraphicsMode<'a, WIDTH, PAGES>, WIDTH, HEIGHT, PAGES> {
        ST7565 {
            interface,
            display_specs: self.display_specs,
            mode: self.mode,
        }
    }
}

impl<'a, DI, SPECS, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize> DrawTarget
    for ST7565<DI, SPECS, GraphicsMode<'a, WIDTH, PAGES>, WIDTH, HEIGHT, PAGES>
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
            if x < 0 || y < 0 || x as usize >= WIDTH || y as usize >= HEIGHT {
                continue;
            }

            let x = x as usize;
            let page = (y / 8) as usize;
            let y_offset = (y % 8) as u8;

            if let Some(page) = self.mode.page_buffers.pages.get_mut(page) {
                if let Some(buffer_line) = page.data.get_mut(x) {
                    let updated = match color {
                        BinaryColor::On => *buffer_line | (1u8 << y_offset),
                        BinaryColor::Off => *buffer_line & (!(1u8 << y_offset)),
                    };

                    if updated != *buffer_line {
                        match &mut page.dirty {
                            Some(dirty_range) => {
                                dirty_range.0 = dirty_range.0.min(x);
                                dirty_range.1 = dirty_range.1.max(x + 1);
                            }
                            None => page.dirty = Some((x, x + 1)),
                        };
                        *buffer_line = updated;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<'a, DI, SPECS, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize> OriginDimensions
    for ST7565<DI, SPECS, GraphicsMode<'a, WIDTH, PAGES>, WIDTH, HEIGHT, PAGES>
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
