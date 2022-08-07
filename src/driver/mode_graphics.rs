use core::ops::Range;

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

use crate::{
    command::{Command, SendSt7565Command},
    ST7565,
};

pub struct GraphicsMode<const WIDTH: usize, const PAGES: usize> {
    page_buffers: [([u8; WIDTH], Option<Range<usize>>); PAGES],
}

impl<const WIDTH: usize, const PAGES: usize> GraphicsMode<WIDTH, PAGES> {
    pub fn new() -> Self {
        Self {
            page_buffers: [(); PAGES].map(|()| ([0; WIDTH], Some(0..WIDTH))),
        }
    }
}

impl<DI: WriteOnlyDataCommand, const WIDTH: usize, const PAGES: usize>
    ST7565<DI, GraphicsMode<WIDTH, PAGES>>
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
