use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

use crate::command::{Command, SendSt7565Command};
use crate::ST7565;

/// Raw mode provides functionality to control the ST7565 chip directly
/// through low-level commands.
pub struct RawMode;

/// ---- Functionality of the raw mode ----
/// =======================================
///
/// This mode exists for interacting with the ST7565 chip via direct low level commands.
impl<DI, SPECS, const WIDTH: usize, const HEIGHT: usize, const PAGES: usize>
    ST7565<DI, SPECS, RawMode, WIDTH, HEIGHT, PAGES>
where
    DI: WriteOnlyDataCommand,
{
    /// Sets the page to write into
    pub fn set_page(&mut self, page: u8) -> Result<(), DisplayError> {
        self.interface
            .send_command(Command::PageAddressSet { address: page })
    }

    /// Sets the column to write into
    pub fn set_column(&mut self, address: u8) -> Result<(), DisplayError> {
        self.interface
            .send_command(Command::ColumnAddressSet { address })
    }

    /// Sets the line offset, effectively scrolling the display through memory.
    pub fn set_line_offset(&mut self, offset: u8) -> Result<(), DisplayError> {
        self.interface
            .send_command(Command::DisplayStartLineSet { address: offset })
    }

    /// Sets whether the pixels should be inverted.
    pub fn set_inverted(&mut self, inverted: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(Command::DisplayNormalReverse { reverse: inverted })
    }

    /// Writes raw pixel data.
    ///
    /// For more information how data is processed by the display, read the
    /// ST7565 reference manual.
    pub fn write_pixel_data(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        self.interface.send_data(U8(data))
    }

    /// Sets the adc direction.
    ///
    /// Effectively flips the display horizontally.
    pub fn adc_select(&mut self, reverse: bool) -> Result<(), DisplayError> {
        self.interface.send_command(Command::AdcSelect { reverse })
    }

    /// Sets the common mode direction.
    ///
    /// Effectively flips the display vertically.
    pub fn common_output_mode_select(&mut self, reverse: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(Command::CommonOutputModeSelect { reverse })
    }

    /// Displays all points of the display
    pub fn display_all_points(&mut self, enable: bool) -> Result<(), DisplayError> {
        self.interface
            .send_command(Command::DisplayAllPoints { on: enable })
    }
}
