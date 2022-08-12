//! A collection of types used in this crate

/// The possible options for the internal booster ratio
#[derive(Debug, Copy, Clone)]
pub enum BoosterRatio {
    /// Use this if booster ratio is 2x, 3x or 4x
    StepUp2x3x4x,
    /// Use this if booster ratio is 5x
    StepUp5x,
    /// Use this if booster ratio is 6x
    StepUp6x,
}

/// The configuration of the power control circuit
#[derive(Debug, Copy, Clone)]
pub struct PowerControlMode {
    /// Activate booster circuit
    pub booster_circuit: bool,
    /// Activate voltage regulator circuit
    pub voltage_regulator_circuit: bool,
    /// Activate voltage follower circuit
    pub voltage_follower_circuit: bool,
}

/// The possible options for the static indicator
#[derive(Debug, Copy, Clone)]
pub enum StaticIndicatorMode {
    /// Off
    Off,
    /// Blinking at approximately one second intervals
    BlinkSlow,
    /// Blinking at approximately 0.5 second intervals
    BlinkFast,
    /// Constantly on
    On,
}
