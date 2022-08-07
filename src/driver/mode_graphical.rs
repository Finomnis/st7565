use display_interface::WriteOnlyDataCommand;

use super::ModeGraphical;
use crate::ST7565;

impl<DI: WriteOnlyDataCommand> ST7565<DI, ModeGraphical> {}
