#![no_main]
#![no_std]

use defmt_rtt as _; // global logger
use nrf52840_hal as hal; // memory layout
use panic_probe as _;

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    geometry::Size,
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};
use hal::gpio::Level;
use st7565::{displays::DOGM132W5, GraphicsPageBuffer, ST7565};

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
                sck: Some(disp_scl),
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

    // Create DOGM132W-5 display driver
    let mut page_buffer = GraphicsPageBuffer::new();
    let mut disp = ST7565::new(disp_spi, DOGM132W5).into_graphics_mode(&mut page_buffer);
    disp.reset(&mut disp_rst, &mut timer).unwrap();
    disp.flush().unwrap();
    disp.set_display_on(true).unwrap();

    // Release the SPI bus.
    // The SPI bus can now be used for communication with other devicesuntil we desire to call `flush()`,
    // where we have to attach it again.
    let (spi, mut disp) = disp.release_display_interface();

    // Draw content
    Circle::new(Point::new(6, 6), 20)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
        .draw(&mut disp)
        .unwrap();
    Rectangle::new(Point::new(106, 6), Size::new(20, 20))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
        .draw(&mut disp)
        .unwrap();
    let font = MonoTextStyle::new(&FONT_8X13, BinaryColor::On);
    Text::new("Hello,\nRust!", Point::new(43, 13), font)
        .draw(&mut disp)
        .unwrap();

    // Send content to display
    let mut disp = disp.attach_display_interface(spi);
    disp.flush().unwrap();

    // Done
    loop {}
}
