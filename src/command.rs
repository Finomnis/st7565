//! Display commands.

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

#[derive(Debug, Copy, Clone)]
pub enum BoosterRatio {
    StepUp_2x_3x_4x,
    StepUp_5x,
    StepUp_6x,
}

/// Commands
#[derive(Debug, Copy, Clone)]
pub enum Command {
    DisplayOnOff { on: bool },
    DisplayStartLineSet { address: u8 },
    PageAddressSet { address: u8 },
    ColumnAddressSetUpper { address: u8 },
    ColumnAddresSetLower { address: u8 },
    AdcSelect { reverse: bool },
    DisplayNormalReverse { reverse: bool },
    DisplayAllPoints { on: bool },
    LcdBiasSet { bias_1_7: bool },
    Reset,
    CommonOutputModeSelect { reverse_direction: bool },
    PowerControlSet { operating_mode: u8 },
    V0VoltageRegulatorInternalResistorSet { resistor_ratio: u8 },
    ElectronicVolumeSet { volume_value: u8 },
    StaticIndicatorSet { on: bool, flash: bool },
    BoosterRatioSet { stepup_value: BoosterRatio },
    NOP,
}

pub trait SendSt7565Command {
    fn send_command(&mut self, command: Command) -> Result<(), DisplayError>;
}

impl<T> SendSt7565Command for T
where
    T: WriteOnlyDataCommand,
{
    fn send_command(&mut self, command: Command) -> Result<(), DisplayError> {
        enum Code {
            Single(u8),
            Double(u8, u8),
        }

        use Code::*;
        use Command::*;

        let data = match command {
            DisplayOnOff { on } => Single(0b10101110 | on as u8),
            DisplayStartLineSet { address } => Single(0b01000000 | (address & 0b00111111)),
            _ => Single(0),
        };

        match data {
            Single(val) => self.send_commands(U8(&[val])),
            Double(val1, val2) => self.send_commands(U8(&[val1, val2])),
        }
    }
}
