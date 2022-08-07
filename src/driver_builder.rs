use display_interface::WriteOnlyDataCommand;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use crate::ST7565;

pub struct ST7565DriverBuilder<DI> {
    interface: DI,
    lcd_bias_mode: bool,
}

impl<DI> ST7565DriverBuilder<DI>
where
    DI: WriteOnlyDataCommand,
{
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            lcd_bias_mode: false,
        }
    }

    pub fn lcd_bias(mut self, mode: bool) -> Self {
        self.lcd_bias_mode = mode;
        self
    }

    pub fn build(self) -> ST7565<DI> {
        ST7565 {
            interface: self.interface,
            lcd_bias_mode: self.lcd_bias_mode,
        }
    }
}
