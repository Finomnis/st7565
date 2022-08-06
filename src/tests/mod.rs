use defmt_rtt as _; // global logger
use nrf52840_hal as _; // memory layout
use panic_probe as _;

mod display_mock;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
//#[cfg(test)]
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
#[defmt_test::tests]
mod unit_tests {
    use defmt::assert;

    use super::display_mock::DisplayMock;

    #[test]
    fn commands() {
        use crate::command::{
            BoosterRatio,
            Command::{self, *},
            SendSt7565Command,
        };
        fn check_command(cmd: Command, result: &[u8]) {
            DisplayMock::expect_command(result)
                .send_command(cmd)
                .unwrap();
        }

        check_command(DisplayOnOff { on: true }, &[0b10101111]);
        check_command(DisplayOnOff { on: false }, &[0b10101110]);
        check_command(DisplayStartLineSet { address: 0b101010 }, &[0b01101010]);
        check_command(DisplayStartLineSet { address: 0b010101 }, &[0b01010101]);
        check_command(
            DisplayStartLineSet {
                address: 0b11000000,
            },
            &[0b01000000],
        );
        check_command(PageAddressSet { address: 0b1010 }, &[0b10111010]);
        check_command(PageAddressSet { address: 0b0101 }, &[0b10110101]);
        check_command(
            PageAddressSet {
                address: 0b11110000,
            },
            &[0b10110000],
        );
        check_command(ColumnAddressSetUpper { address: 0b1010 }, &[0b00011010]);
        check_command(ColumnAddressSetUpper { address: 0b0101 }, &[0b00010101]);
        check_command(
            ColumnAddressSetUpper {
                address: 0b11110000,
            },
            &[0b00010000],
        );
        check_command(ColumnAddresSetLower { address: 0b1010 }, &[0b00001010]);
        check_command(ColumnAddresSetLower { address: 0b0101 }, &[0b00000101]);
        check_command(
            ColumnAddresSetLower {
                address: 0b11110000,
            },
            &[0b00000000],
        );
        check_command(AdcSelect { reverse: true }, &[0b10100001]);
        check_command(AdcSelect { reverse: false }, &[0b10100000]);
        check_command(DisplayNormalReverse { reverse: true }, &[0b10100111]);
        check_command(DisplayNormalReverse { reverse: false }, &[0b10100110]);
        check_command(DisplayAllPoints { on: true }, &[0b10100101]);
        check_command(DisplayAllPoints { on: false }, &[0b10100100]);
        check_command(LcdBiasSet { bias_1_7: true }, &[0b10100011]);
        check_command(LcdBiasSet { bias_1_7: false }, &[0b10100010]);
        check_command(Reset, &[0b11100010]);
        check_command(
            CommonOutputModeSelect {
                reverse_direction: true,
            },
            &[0b11001000],
        );
        check_command(
            CommonOutputModeSelect {
                reverse_direction: false,
            },
            &[0b11000000],
        );
        check_command(
            PowerControlSet {
                operating_mode: 0b101,
            },
            &[0b00101101],
        );
        check_command(
            PowerControlSet {
                operating_mode: 0b010,
            },
            &[0b00101010],
        );
        check_command(
            PowerControlSet {
                operating_mode: 0b11111000,
            },
            &[0b00101000],
        );
        check_command(
            V0VoltageRegulatorInternalResistorSet {
                resistor_ratio: 0b101,
            },
            &[0b00100101],
        );
        check_command(
            V0VoltageRegulatorInternalResistorSet {
                resistor_ratio: 0b010,
            },
            &[0b00100010],
        );
        check_command(
            V0VoltageRegulatorInternalResistorSet {
                resistor_ratio: 0b11111000,
            },
            &[0b00100000],
        );
        check_command(
            ElectronicVolumeSet {
                volume_value: 0b101010,
            },
            &[0b10000001, 0b00101010],
        );
        check_command(
            ElectronicVolumeSet {
                volume_value: 0b010101,
            },
            &[0b10000001, 0b00010101],
        );
        check_command(
            ElectronicVolumeSet {
                volume_value: 0b11000000,
            },
            &[0b10000001, 0b00000000],
        );
        check_command(
            StaticIndicatorSet {
                on: true,
                flash: false,
            },
            &[0b10101101, 0b00000000],
        );
        check_command(
            StaticIndicatorSet {
                on: false,
                flash: true,
            },
            &[0b10101100, 0b00000001],
        );
        check_command(
            StaticIndicatorSet {
                on: false,
                flash: false,
            },
            &[0b10101100, 0b00000000],
        );
        check_command(
            BoosterRatioSet {
                stepup_value: BoosterRatio::StepUp_2x_3x_4x,
            },
            &[0b11111000, 0b00000000],
        );
        check_command(
            BoosterRatioSet {
                stepup_value: BoosterRatio::StepUp_5x,
            },
            &[0b11111000, 0b00000001],
        );
        check_command(
            BoosterRatioSet {
                stepup_value: BoosterRatio::StepUp_6x,
            },
            &[0b11111000, 0b00000011],
        );
        check_command(NOP, &[0b11100011]);
    }
}
