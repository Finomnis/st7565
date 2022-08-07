use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

use super::ModeRaw;
use crate::command::{Command, SendSt7565Command};
use crate::ST7565;

impl<DI: WriteOnlyDataCommand> ST7565<DI, ModeRaw> {
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
