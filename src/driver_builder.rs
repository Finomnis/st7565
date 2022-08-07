use display_interface::WriteOnlyDataCommand;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use crate::Error;
use crate::ST7565;

pub struct ST7565DriverBuilder<DI> {
    interface: DI,
}

impl<DI> ST7565DriverBuilder<DI>
where
    DI: WriteOnlyDataCommand,
{
    pub fn new(interface: DI) -> Self {
        Self { interface }
    }

    pub fn build(self) -> ST7565<DI> {
        ST7565 {
            interface: self.interface,
        }
    }
}
