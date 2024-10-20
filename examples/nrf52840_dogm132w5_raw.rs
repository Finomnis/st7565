#![no_main]
#![no_std]

use defmt_rtt as _; // global logger
use nrf52840_hal as hal; // memory layout
use panic_probe as _;

use display_interface_spi::SPIInterface;
use embedded_hal::delay::DelayNs;
use embedded_hal_bus::spi::ExclusiveDevice;
use hal::gpio::Level;
use st7565::{displays::DOGM132W5, ST7565};

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
    let spi_bus = hal::Spim::new(
        peripherals.SPIM0,
        hal::spim::Pins {
            sck: Some(disp_scl),
            mosi: Some(disp_si),
            miso: None,
        },
        hal::spim::Frequency::M8,
        hal::spim::MODE_3,
        0,
    );
    let disp_spidevice = ExclusiveDevice::new_no_delay(spi_bus, disp_cs).unwrap();
    let disp_interface = SPIInterface::new(disp_spidevice, disp_a0);

    // Create DOGM132W-5 display driver
    let mut disp = ST7565::new(disp_interface, DOGM132W5).into_raw_mode();
    disp.reset(&mut disp_rst, &mut timer).unwrap();
    disp.set_display_on(true).unwrap();

    disp.set_page(2).unwrap();
    disp.set_column(10).unwrap();
    let mut data = [0u8; 100];
    for (pos, val) in data.iter_mut().enumerate() {
        *val = pos as u8;
    }
    disp.write_pixel_data(&data).unwrap();

    let mut scroll = 0;
    loop {
        disp.set_line_offset(scroll).unwrap();
        scroll = (scroll + 1) % 32;
        timer.delay_ms(100);
    }
}
