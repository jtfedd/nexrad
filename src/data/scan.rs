use crate::data::Sweep;

/// A single radar scan composed of a series of sweeps. This represents a single volume scan which
/// is composed of multiple sweeps at different elevations. The pattern of sweeps, including
/// elevations and resolution, is determined by the scanning strategy of the radar. This is
/// referred to as the Volume Coverage Pattern.
pub struct Scan {
    coverage_pattern_number: u16,
    sweeps: Vec<Sweep>,
}

impl Scan {
    /// This scan's volume coverage pattern number.
    pub fn coverage_pattern_number(&self) -> u16 {
        self.coverage_pattern_number
    }

    /// The elevation sweeps comprising this scan.
    pub fn sweeps(&self) -> &Vec<Sweep> {
        self.sweeps.as_ref()
    }
}
