use display_interface::DisplayError;

/// Errors in this crate
#[derive(Debug)]
pub enum Error<PinE> {
    /// Communication error
    Comm(DisplayError),
    /// Pin setting error
    Pin(PinE),
}
