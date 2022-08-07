//! Display commands.

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

#[derive(Debug, Copy, Clone)]
pub enum BoosterRatio {
    StepUp2x3x4x,
    StepUp5x,
    StepUp6x,
}

/// Commands
#[derive(Debug, Copy, Clone)]
pub enum Command {
    DisplayOnOff { on: bool },
    DisplayStartLineSet { address: u8 },
    PageAddressSet { address: u8 },
    ColumnAddressSetUpper { address: u8 },
    ColumnAddressSetLower { address: u8 },
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
            PageAddressSet { address } => Single(0b10110000 | (address & 0b00001111)),
            ColumnAddressSetUpper { address } => Single(0b00010000 | ((address >> 4) & 0b00001111)),
            ColumnAddressSetLower { address } => Single(0b00000000 | (address & 0b00001111)),
            AdcSelect { reverse } => Single(0b10100000 | reverse as u8),
            DisplayNormalReverse { reverse } => Single(0b10100110 | reverse as u8),
            DisplayAllPoints { on } => Single(0b10100100 | on as u8),
            LcdBiasSet { bias_1_7 } => Single(0b10100010 | bias_1_7 as u8),
            Reset => Single(0b11100010),
            CommonOutputModeSelect { reverse_direction } => {
                Single(0b11000000 | ((reverse_direction as u8) << 3))
            }
            PowerControlSet { operating_mode } => {
                Single(0b00101000 | (operating_mode & 0b00000111))
            }
            V0VoltageRegulatorInternalResistorSet { resistor_ratio } => {
                Single(0b00100000 | (resistor_ratio & 0b00000111))
            }
            ElectronicVolumeSet { volume_value } => Double(0b10000001, volume_value & 0b00111111),
            StaticIndicatorSet { on, flash } => Double(0b10101100 | on as u8, flash as u8),
            BoosterRatioSet { stepup_value } => Double(
                0b11111000,
                match stepup_value {
                    BoosterRatio::StepUp2x3x4x => 0b00000000,
                    BoosterRatio::StepUp5x => 0b00000001,
                    BoosterRatio::StepUp6x => 0b00000011,
                },
            ),
            NOP => Single(0b11100011),
        };

        match data {
            Single(val) => self.send_commands(U8(&[val])),
            Double(val1, val2) => self.send_commands(U8(&[val1, val2])),
        }
    }
}
