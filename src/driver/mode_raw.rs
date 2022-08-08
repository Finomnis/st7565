use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

use crate::command::{Command, SendSt7565Command};
use crate::{DisplaySpecs, ST7565};

pub struct RawMode;

impl<DI, SPECS, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize>
    ST7565<DI, SPECS, RawMode, WIDTH, HEIGHT, PAGES>
where
    DI: WriteOnlyDataCommand,
{
    pub fn set_page(&mut self, page: u8) -> Result<(), DisplayError> {
        self.interface
            .send_command(Command::PageAddressSet { address: page })
    }
    pub fn set_column(&mut self, address: u8) -> Result<(), DisplayError> {
        self.interface
            .send_command(Command::ColumnAddressSet { address })
    }
    pub fn set_line_offset(&mut self, offset: u8) -> Result<(), DisplayError> {
        self.interface
            .send_command(Command::DisplayStartLineSet { address: offset })
    }
    pub fn set_inverted(&mut self, inverted: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(Command::DisplayNormalReverse { reverse: inverted })
    }
    pub fn write_pixel_data(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        self.interface.send_data(U8(data))
    }
}
