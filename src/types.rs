/// The possible options for the internal booster ratio
#[derive(Debug, Copy, Clone)]
pub enum BoosterRatio {
    StepUp2x3x4x,
    StepUp5x,
    StepUp6x,
}

/// The configuration of the power control circuit
#[derive(Debug, Copy, Clone)]
pub struct PowerControlMode {
    pub booster_circuit: bool,
    pub voltage_regulator_circuit: bool,
    pub voltage_follower_circuit: bool,
}

/// The possible options for the internal booster ratio
#[derive(Debug, Copy, Clone)]
pub enum StaticIndicatorMode {
    Off,
    BlinkSlow,
    BlinkFast,
    On,
}
