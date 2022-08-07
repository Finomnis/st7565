use display_interface::WriteOnlyDataCommand;

use super::{ModeGraphical, ModeInit, ModeRaw};
use crate::ST7565;

impl<DI: WriteOnlyDataCommand> ST7565<DI, ModeInit> {
    fn into_mode<MODE>(self) -> ST7565<DI, MODE> {
        ST7565::<DI, MODE> {
            interface: self.interface,
            display_specs: self.display_specs,
            _mode: core::marker::PhantomData,
        }
    }

    pub fn into_raw_mode(self) -> ST7565<DI, ModeRaw> {
        self.into_mode()
    }

    pub fn into_graphical_mode(self) -> ST7565<DI, ModeGraphical> {
        self.into_mode()
    }
}
