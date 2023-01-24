#[cfg(all(target_arch = "arm", target_os = "none"))]
mod arch_dependent_impl {
    use defmt_rtt as _; // global logger
    use nrf52840_hal as _; // memory layout
    use panic_probe as _;

    pub use defmt_test::tests;

    // same panicking *behavior* as `panic-probe` but doesn't print a panic message
    // this prevents the panic message being printed *twice* when `defmt::panic` is invoked
    #[defmt::panic_handler]
    fn panic() -> ! {
        cortex_m::asm::udf()
    }
}

#[cfg(not(all(target_arch = "arm", target_os = "none")))]
mod arch_dependent_impl {
    extern crate std;

    // Noop macro, just to mimik the API of defmt_test
    pub use noop_attr::noop as tests;
}

pub use arch_dependent_impl::tests;
