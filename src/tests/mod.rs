#[cfg(all(target_arch = "arm", target_os = "none"))]
use defmt_rtt as _; // global logger
#[cfg(all(target_arch = "arm", target_os = "none"))]
use defmt_test::tests;
#[cfg(all(target_arch = "arm", target_os = "none"))]
use nrf52840_hal as _; // memory layout
#[cfg(all(target_arch = "arm", target_os = "none"))]
use panic_probe as _;

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
extern crate std;

mod display_mock;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[cfg(all(target_arch = "arm", target_os = "none"))]
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
#[tests]
mod unit_tests {
    use super::display_mock::DisplayMock;

    #[test]
    fn commands() {
        use crate::{
            command::{
                Command::{self, *},
                SendSt7565Command,
            },
            types::{BoosterRatio, PowerControlMode, StaticIndicatorMode},
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
        check_command(
            ColumnAddressSet {
                address: 0b10100000,
            },
            &[0b00011010, 0b00000000],
        );
        check_command(
            ColumnAddressSet {
                address: 0b01010000,
            },
            &[0b00010101, 0b00000000],
        );
        check_command(
            ColumnAddressSet {
                address: 0b00001010,
            },
            &[0b00010000, 0b00001010],
        );
        check_command(
            ColumnAddressSet {
                address: 0b00000101,
            },
            &[0b00010000, 0b00000101],
        );
        check_command(AdcSelect { reverse: true }, &[0b10100001]);
        check_command(AdcSelect { reverse: false }, &[0b10100000]);
        check_command(DisplayNormalReverse { reverse: true }, &[0b10100111]);
        check_command(DisplayNormalReverse { reverse: false }, &[0b10100110]);
        check_command(DisplayAllPoints { on: true }, &[0b10100101]);
        check_command(DisplayAllPoints { on: false }, &[0b10100100]);
        check_command(LcdBiasSet { bias_mode_1: true }, &[0b10100011]);
        check_command(LcdBiasSet { bias_mode_1: false }, &[0b10100010]);
        //check_command(Reset, &[0b11100010]);
        check_command(CommonOutputModeSelect { reverse: true }, &[0b11001000]);
        check_command(CommonOutputModeSelect { reverse: false }, &[0b11000000]);
        check_command(
            PowerControlSet {
                mode: PowerControlMode {
                    booster_circuit: true,
                    voltage_regulator_circuit: false,
                    voltage_follower_circuit: false,
                },
            },
            &[0b00101100],
        );
        check_command(
            PowerControlSet {
                mode: PowerControlMode {
                    booster_circuit: false,
                    voltage_regulator_circuit: true,
                    voltage_follower_circuit: false,
                },
            },
            &[0b00101010],
        );
        check_command(
            PowerControlSet {
                mode: PowerControlMode {
                    booster_circuit: false,
                    voltage_regulator_circuit: false,
                    voltage_follower_circuit: true,
                },
            },
            &[0b00101001],
        );
        check_command(
            VoltageRegulatorInternalResistorSet {
                resistor_ratio: 0b101,
            },
            &[0b00100101],
        );
        check_command(
            VoltageRegulatorInternalResistorSet {
                resistor_ratio: 0b010,
            },
            &[0b00100010],
        );
        check_command(
            VoltageRegulatorInternalResistorSet {
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
                mode: Some(StaticIndicatorMode::Off),
            },
            &[0b10101101, 0b00000000],
        );
        check_command(
            StaticIndicatorSet {
                mode: Some(StaticIndicatorMode::On),
            },
            &[0b10101101, 0b00000011],
        );
        check_command(
            StaticIndicatorSet {
                mode: Some(StaticIndicatorMode::BlinkSlow),
            },
            &[0b10101101, 0b00000001],
        );
        check_command(
            StaticIndicatorSet {
                mode: Some(StaticIndicatorMode::BlinkFast),
            },
            &[0b10101101, 0b00000010],
        );
        check_command(StaticIndicatorSet { mode: None }, &[0b10101100]);
        check_command(
            BoosterRatioSet {
                stepup_value: BoosterRatio::StepUp2x3x4x,
            },
            &[0b11111000, 0b00000000],
        );
        check_command(
            BoosterRatioSet {
                stepup_value: BoosterRatio::StepUp5x,
            },
            &[0b11111000, 0b00000001],
        );
        check_command(
            BoosterRatioSet {
                stepup_value: BoosterRatio::StepUp6x,
            },
            &[0b11111000, 0b00000011],
        );
        //check_command(NOP, &[0b11100011]);
    }
}
