use serde::{Deserialize, Serialize};
use std::fmt;

use crate::constants::{
    duration_h_periods, duration_seconds, DURATION_UNIT_COUNT, DURATION_UNIT_NAMES,
};
use crate::error::{UtError, UtResult};
use crate::quant::Quant;

/// Universal Duration unit levels, from smallest to largest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DurationUnit {
    Pulse = 0,
    Wave = 1,
    Tide = 2,
    Spin = 3,
    Current = 4,
    Season = 5,
    Epoch = 6,
}

impl DurationUnit {
    /// Get the unit from an index (0=Pulse, 6=Epoch).
    pub fn from_index(index: usize) -> UtResult<Self> {
        match index {
            0 => Ok(Self::Pulse),
            1 => Ok(Self::Wave),
            2 => Ok(Self::Tide),
            3 => Ok(Self::Spin),
            4 => Ok(Self::Current),
            5 => Ok(Self::Season),
            6 => Ok(Self::Epoch),
            _ => Err(UtError::DurationOverflow),
        }
    }

    /// Get the index of this unit.
    pub fn index(self) -> usize {
        self as usize
    }

    /// Get the name of this unit.
    pub fn name(self) -> &'static str {
        DURATION_UNIT_NAMES[self.index()]
    }

    /// Number of hydrogen hyperfine periods in one of this unit.
    pub fn h_periods(self) -> u128 {
        duration_h_periods(self.index())
    }

    /// Duration in seconds of one of this unit.
    pub fn seconds(self) -> f64 {
        duration_seconds(self.index())
    }

    /// All duration units in ascending order.
    pub fn all() -> [DurationUnit; DURATION_UNIT_COUNT] {
        [
            Self::Pulse,
            Self::Wave,
            Self::Tide,
            Self::Spin,
            Self::Current,
            Self::Season,
            Self::Epoch,
        ]
    }
}

impl fmt::Display for DurationUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// A duration value: a count of a specific duration unit.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Duration {
    pub count: f64,
    pub unit: DurationUnit,
}

impl Duration {
    /// Create a new duration.
    pub fn new(count: f64, unit: DurationUnit) -> Self {
        Self { count, unit }
    }

    /// Convert to total seconds.
    pub fn to_seconds(self) -> f64 {
        self.count * self.unit.seconds()
    }

    /// Convert from seconds to the given unit.
    pub fn from_seconds(seconds: f64, unit: DurationUnit) -> Self {
        Self {
            count: seconds / unit.seconds(),
            unit,
        }
    }

    /// Convert to a quant count.
    pub fn to_quants(self) -> Quant {
        let h_periods = (self.count * self.unit.h_periods() as f64) as u128;
        Quant::new(h_periods)
    }

    /// Convert from quants to the given unit.
    pub fn from_quants(quants: Quant, unit: DurationUnit) -> Self {
        Self {
            count: quants.count() as f64 / unit.h_periods() as f64,
            unit,
        }
    }

    /// Convert this duration to a different unit.
    pub fn convert_to(self, target: DurationUnit) -> Self {
        let seconds = self.to_seconds();
        Self::from_seconds(seconds, target)
    }

    /// Get the total hydrogen periods represented by this duration.
    pub fn total_h_periods(self) -> f64 {
        self.count * self.unit.h_periods() as f64
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.count == 1.0 {
            write!(f, "1 {}", self.unit.name())
        } else {
            write!(f, "{:.2} {}s", self.count, self.unit.name())
        }
    }
}

/// Find the most human-readable duration unit for a given number of seconds.
pub fn best_unit_for_seconds(seconds: f64) -> DurationUnit {
    let units = DurationUnit::all();
    for i in (0..units.len()).rev() {
        if seconds >= units[i].seconds() * 0.5 {
            return units[i];
        }
    }
    DurationUnit::Pulse
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pulse_seconds() {
        let s = DurationUnit::Pulse.seconds();
        assert!((s - 70.4).abs() < 0.1);
    }

    #[test]
    fn test_wave_seconds() {
        let s = DurationUnit::Wave.seconds();
        assert!((s - 704.0).abs() < 1.0);
    }

    #[test]
    fn test_tide_seconds() {
        let s = DurationUnit::Tide.seconds();
        assert!((s - 7040.0).abs() < 10.0);
    }

    #[test]
    fn test_spin_seconds() {
        let s = DurationUnit::Spin.seconds();
        assert!((s - 70_400.0).abs() < 100.0);
    }

    #[test]
    fn test_current_seconds() {
        let s = DurationUnit::Current.seconds();
        assert!((s - 704_000.0).abs() < 1000.0);
    }

    #[test]
    fn test_season_seconds() {
        let s = DurationUnit::Season.seconds();
        assert!((s - 7_040_000.0).abs() < 10_000.0);
    }

    #[test]
    fn test_epoch_seconds() {
        let s = DurationUnit::Epoch.seconds();
        assert!((s - 70_400_000.0).abs() < 100_000.0);
    }

    #[test]
    fn test_each_unit_is_10x_previous() {
        let units = DurationUnit::all();
        for i in 1..units.len() {
            assert_eq!(units[i].h_periods(), units[i - 1].h_periods() * 10);
        }
    }

    #[test]
    fn test_duration_to_seconds() {
        let d = Duration::new(3.0, DurationUnit::Pulse);
        let secs = d.to_seconds();
        assert!((secs - 211.2).abs() < 1.0);
    }

    #[test]
    fn test_duration_convert() {
        let d = Duration::new(10.0, DurationUnit::Pulse);
        let w = d.convert_to(DurationUnit::Wave);
        assert!((w.count - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_duration_round_trip() {
        let original = Duration::new(2.5, DurationUnit::Tide);
        let secs = original.to_seconds();
        let back = Duration::from_seconds(secs, DurationUnit::Tide);
        assert!((back.count - original.count).abs() < 1e-10);
    }

    #[test]
    fn test_duration_to_quants() {
        let d = Duration::new(1.0, DurationUnit::Pulse);
        let q = d.to_quants();
        assert_eq!(q.count(), 100_000_000_000); // 10^11
    }

    #[test]
    fn test_best_unit() {
        assert_eq!(best_unit_for_seconds(60.0), DurationUnit::Pulse);
        assert_eq!(best_unit_for_seconds(600.0), DurationUnit::Wave);
        assert_eq!(best_unit_for_seconds(5000.0), DurationUnit::Tide);
    }

    #[test]
    fn test_unit_display() {
        assert_eq!(format!("{}", DurationUnit::Pulse), "Pulse");
        assert_eq!(format!("{}", DurationUnit::Epoch), "Epoch");
    }
}
