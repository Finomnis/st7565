#![no_main]
#![no_std]

mod command;

//#[cfg(test)]
mod tests;

/// ST7565 driver.
pub struct ST7565<DI, SIZE> {
    interface: DI,
    size: SIZE,
}
