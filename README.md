# `st7565`

[![DOGM132W-5 display showing the graphics example](https://github.com/Finomnis/st7565/blob/main/readme_banner.jpg?raw=true)](examples/nrf52840_dogm132w5_graphics.rs)

[![Crates.io](https://img.shields.io/crates/v/st7565)](https://crates.io/crates/st7565)
[![Crates.io](https://img.shields.io/crates/d/st7565)](https://crates.io/crates/st7565)
[![License](https://img.shields.io/crates/l/st7565)](https://github.com/Finomnis/st7565/blob/main/LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Finomnis/st7565/ci.yml)](https://github.com/Finomnis/st7565/actions/workflows/ci.yml?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/st7565)](https://docs.rs/st7565)

This crate aims to provide an
[embedded-graphics](https://crates.io/crates/embedded-graphics)
compatible driver for displays based on the ST7565 chipset.

## Example

The following example is intended for the [DOGM132W-5](https://www.displayvisions.us/products/dog.html) display connected to an [nRF52840](https://www.nordicsemi.com/products/nrf52840)
microcontroller.

Note the `MODE_3` of the SPI. The DOGM132W-5 display expects the clock to be configured
with `CPOL=1` and `CPHA=1`, which is also called `SPI Mode 3`.

```rust
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

// Create DOGM132W-5 display driver
let mut page_buffer = GraphicsPageBuffer::new();
let mut disp = ST7565::new(disp_spi, DOGM132W5).into_graphics_mode(&mut page_buffer);
disp.reset(&mut disp_rst, &mut timer).unwrap();
disp.flush().unwrap();
disp.set_display_on(true).unwrap();

// Draw on it using the embedded_graphics library
Circle::new(Point::new(10, 6), 20)
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
    .draw(&mut disp)
    .unwrap();
disp.flush().unwrap();
```

Note the [`DOGM132W5`](displays::DOGM132W5) object. This is the display specification that contains all the display specific configuration options that need to be applied to the ST7565 chip.

Further note the [`into_graphics_mode()`](ST7565::into_graphics_mode()) call, which switches the driver from its initial
mode to the [`embedded-graphics`](https://crates.io/crates/embedded-graphics) driver mode.
The `disp` object can then be used as a `DrawTarget` in `embedded-graphics` calls.

After drawing something, a [`flush()`](ST7565::flush()) call has to be issued to actually
send the modified data to the display.


## Adding support for new ST7565 based displays

The example above uses the [`DOGM132W5`](displays::DOGM132W5) struct in the [`ST7565::new()`] call.

To initialize the `ST7565` driver struct with a different display, a new display
specification has to be created. This can be done by creating an empty struct that
implements the [`DisplaySpecs`] trait.

For example, the definition of the [`DOGM132W5`](displays::DOGM132W5) struct looks like this:
```rust
pub struct DOGM132W5;
impl DisplaySpecs<132, 32, 4> for DOGM132W5 {
    const FLIP_ROWS: bool = false;
    const FLIP_COLUMNS: bool = true;
    const INVERTED: bool = false;
    const BIAS_MODE_1: bool = false;
    const POWER_CONTROL: PowerControlMode = PowerControlMode {
        booster_circuit: true,
        voltage_regulator_circuit: true,
        voltage_follower_circuit: true,
    };
    const VOLTAGE_REGULATOR_RESISTOR_RATIO: u8 = 0b011;
    const ELECTRONIC_VOLUME: u8 = 0b011111;
    const BOOSTER_RATIO: BoosterRatio = BoosterRatio::StepUp2x3x4x;
}
```

The exact values for the respective display have to be taken from the display's manual.

If you created a specification for a new display, please open a pull request on <https://github.com/Finomnis/st7565/pulls> to make it available to the public.
