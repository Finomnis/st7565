//! Display commands.

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

use crate::types::{BoosterRatio, PowerControlMode, StaticIndicatorMode};

/// Commands
#[derive(Debug, Copy, Clone)]
pub(crate) enum Command {
    DisplayOnOff { on: bool },
    DisplayStartLineSet { address: u8 },
    PageAddressSet { address: u8 },
    ColumnAddressSet { address: u8 },
    AdcSelect { reverse: bool },
    DisplayNormalReverse { reverse: bool },
    DisplayAllPoints { on: bool },
    LcdBiasSet { bias_mode_1: bool },
    //Reset,
    CommonOutputModeSelect { reverse: bool },
    PowerControlSet { mode: PowerControlMode },
    VoltageRegulatorInternalResistorSet { resistor_ratio: u8 },
    ElectronicVolumeSet { volume_value: u8 },
    StaticIndicatorSet { mode: Option<StaticIndicatorMode> },
    BoosterRatioSet { stepup_value: BoosterRatio },
    //NOP,
}

pub(crate) trait SendSt7565Command {
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

        let data = match command {
            Command::DisplayOnOff { on } => Single(0b10101110 | on as u8),
            Command::DisplayStartLineSet { address } => Single(0b01000000 | (address & 0b00111111)),
            Command::PageAddressSet { address } => Single(0b10110000 | (address & 0b00001111)),
            Command::ColumnAddressSet { address } => Double(
                0b00010000 | ((address >> 4) & 0b00001111),
                address & 0b00001111,
            ),
            Command::AdcSelect { reverse } => Single(0b10100000 | reverse as u8),
            Command::DisplayNormalReverse { reverse } => Single(0b10100110 | reverse as u8),
            Command::DisplayAllPoints { on } => Single(0b10100100 | on as u8),
            Command::LcdBiasSet { bias_mode_1 } => Single(0b10100010 | bias_mode_1 as u8),
            //Command::Reset => Single(0b11100010),
            Command::CommonOutputModeSelect { reverse } => {
                Single(0b11000000 | ((reverse as u8) << 3))
            }
            Command::PowerControlSet { mode } => Single(
                0b00101000
                    | ((mode.booster_circuit as u8) << 2)
                    | ((mode.voltage_regulator_circuit as u8) << 1)
                    | (mode.voltage_follower_circuit as u8),
            ),
            Command::VoltageRegulatorInternalResistorSet { resistor_ratio } => {
                Single(0b00100000 | (resistor_ratio & 0b00000111))
            }
            Command::ElectronicVolumeSet { volume_value } => {
                Double(0b10000001, volume_value & 0b00111111)
            }
            Command::StaticIndicatorSet { mode: None } => Single(0b10101100),
            Command::StaticIndicatorSet { mode: Some(mode) } => Double(
                0b10101101,
                match mode {
                    StaticIndicatorMode::Off => 0b00,
                    StaticIndicatorMode::BlinkSlow => 0b01,
                    StaticIndicatorMode::BlinkFast => 0b10,
                    StaticIndicatorMode::On => 0b11,
                },
            ),
            Command::BoosterRatioSet { stepup_value } => Double(
                0b11111000,
                match stepup_value {
                    BoosterRatio::StepUp2x3x4x => 0b00000000,
                    BoosterRatio::StepUp5x => 0b00000001,
                    BoosterRatio::StepUp6x => 0b00000011,
                },
            ),
            //Command::NOP => Single(0b11100011),
        };

        match data {
            Single(val) => self.send_commands(U8(&[val])),
            Double(val1, val2) => self.send_commands(U8(&[val1, val2])),
        }
    }
}
