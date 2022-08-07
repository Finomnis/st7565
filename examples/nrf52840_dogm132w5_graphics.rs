#![no_main]
#![no_std]

use defmt_rtt as _; // global logger
use nrf52840_hal as hal; // memory layout
use panic_probe as _;

use display_interface_spi::SPIInterface;
use embedded_hal::blocking::delay::DelayMs;
use hal::gpio::Level;
use st7565::{BoosterRatio, DisplaySpecs, PowerControlMode, ST7565};

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
    let port1 = hal::gpio::p1::Parts::new(peripherals.P1);

    // Create timer
    let mut timer = hal::timer::Timer::new(peripherals.TIMER0);

    // Get DOGM132W-5 pins
    let mut disp_rst = port0.p0_12.into_push_pull_output(Level::High);
    let disp_cs = port1.p1_09.into_push_pull_output(Level::High).degrade();
    let disp_a0 = port0.p0_23.into_push_pull_output(Level::High).degrade();
    let disp_scl = port0.p0_21.into_push_pull_output(Level::High).degrade();
    let disp_si = port0.p0_19.into_push_pull_output(Level::Low).degrade();

    // Create DOGM132W-5 spi bus
    let disp_spi = SPIInterface::new(
        hal::Spim::new(
            peripherals.SPIM0,
            hal::spim::Pins {
                sck: disp_scl,
                mosi: Some(disp_si),
                miso: None,
            },
            hal::spim::Frequency::M8,
            hal::spim::MODE_3,
            0,
        ),
        disp_a0,
        disp_cs,
    );

    // Build DOGM132W-5 display driver
    let display_specs = DisplaySpecs {
        power_control: PowerControlMode {
            booster_circuit: true,
            voltage_regulator_circuit: true,
            voltage_follower_circuit: true,
        },
        voltage_regulator_resistor_ratio: 0b011,
        electronic_volume: 0b011111,
        flip_rows: false,
        flip_columns: true,
        inverted: false,
        bias_mode_1: false,
        booster_ratio: BoosterRatio::StepUp2x3x4x,
    };
    let mut disp = ST7565::new(disp_spi, display_specs).into_graphics_mode::<132, 4>();

    disp.reset(&mut disp_rst, &mut timer).unwrap();
    disp.flush().unwrap();
    disp.set_display_on(true).unwrap();

    loop {
        timer.delay_ms(100u8);
    }
}
