use serde::{Deserialize, Serialize};
use std::fmt;

use crate::constants::{HYDROGEN_HYPERFINE_FREQ, ONE_QUANT_SECONDS};
use crate::error::{UtError, UtResult};

/// A single quant value — one hydrogen-1 hyperfine period count.
///
/// Quants are the fundamental unit of Universal Times. Each quant represents
/// one period of the hydrogen-1 ground-state hyperfine transition at
/// 1,420,405,751.768 Hz.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Quant(pub u128);

impl Quant {
    /// Create a new Quant from a raw count.
    pub fn new(count: u128) -> Self {
        Self(count)
    }

    /// The zero quant (epoch origin).
    pub fn zero() -> Self {
        Self(0)
    }

    /// Convert a duration in seconds to quant count.
    pub fn from_seconds(seconds: f64) -> Self {
        Self((seconds * HYDROGEN_HYPERFINE_FREQ) as u128)
    }

    /// Convert this quant count to seconds.
    pub fn to_seconds(self) -> f64 {
        self.0 as f64 * ONE_QUANT_SECONDS
    }

    /// Get the raw count.
    pub fn count(self) -> u128 {
        self.0
    }

    /// Checked addition.
    pub fn checked_add(self, other: Quant) -> UtResult<Quant> {
        self.0
            .checked_add(other.0)
            .map(Quant)
            .ok_or(UtError::QuantOverflow)
    }

    /// Checked subtraction.
    pub fn checked_sub(self, other: Quant) -> UtResult<Quant> {
        self.0
            .checked_sub(other.0)
            .map(Quant)
            .ok_or(UtError::QuantOverflow)
    }

    /// Difference between two quant values (absolute).
    pub fn abs_diff(self, other: Quant) -> Quant {
        Quant(if self.0 >= other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        })
    }
}

impl fmt::Display for Quant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Q:{}", self.0)
    }
}

impl std::ops::Add for Quant {
    type Output = Quant;
    fn add(self, rhs: Quant) -> Quant {
        Quant(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Quant {
    type Output = Quant;
    fn sub(self, rhs: Quant) -> Quant {
        Quant(self.0 - rhs.0)
    }
}

/// The Quant Accumulator — a monotonically increasing counter of hydrogen
/// hyperfine periods since the epoch origin (t:0).
///
/// All timestamps across all planets share the same quant value at the same
/// physical instant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantAccumulator {
    /// Current quant count since epoch.
    current: Quant,
    /// The epoch origin quant (always 0 for the canonical epoch).
    epoch_origin: Quant,
}

impl QuantAccumulator {
    /// Create a new accumulator starting at the epoch origin.
    pub fn new() -> Self {
        Self {
            current: Quant::zero(),
            epoch_origin: Quant::zero(),
        }
    }

    /// Create an accumulator with a specific starting value.
    pub fn from_quant(q: Quant) -> Self {
        Self {
            current: q,
            epoch_origin: Quant::zero(),
        }
    }

    /// Create an accumulator from a Unix timestamp (seconds since 1970-01-01 UTC).
    /// The epoch mapping must be provided externally; this uses a configurable
    /// quant-at-unix-epoch offset.
    pub fn from_unix_timestamp(unix_secs: f64, quant_at_unix_epoch: Quant) -> Self {
        let elapsed_quants = Quant::from_seconds(unix_secs);
        Self {
            current: Quant(quant_at_unix_epoch.0 + elapsed_quants.0),
            epoch_origin: Quant::zero(),
        }
    }

    /// Get the current quant value.
    pub fn current(&self) -> Quant {
        self.current
    }

    /// Get the epoch origin.
    pub fn epoch_origin(&self) -> Quant {
        self.epoch_origin
    }

    /// Advance the accumulator by a given number of quants.
    pub fn advance(&mut self, delta: Quant) -> UtResult<()> {
        self.current = self.current.checked_add(delta)?;
        Ok(())
    }

    /// Advance by a number of seconds.
    pub fn advance_seconds(&mut self, seconds: f64) -> UtResult<()> {
        self.advance(Quant::from_seconds(seconds))
    }

    /// Get elapsed quants since epoch origin.
    pub fn elapsed(&self) -> Quant {
        Quant(self.current.0 - self.epoch_origin.0)
    }

    /// Get elapsed time in seconds since epoch.
    pub fn elapsed_seconds(&self) -> f64 {
        self.elapsed().to_seconds()
    }
}

impl Default for QuantAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quant_zero() {
        assert_eq!(Quant::zero().count(), 0);
    }

    #[test]
    fn test_quant_from_seconds() {
        let q = Quant::from_seconds(1.0);
        // Should be approximately HYDROGEN_HYPERFINE_FREQ
        let expected = HYDROGEN_HYPERFINE_FREQ as u128;
        assert!((q.count() as i128 - expected as i128).unsigned_abs() < 1000);
    }

    #[test]
    fn test_quant_to_seconds() {
        let q = Quant::new(1_420_405_751);
        let secs = q.to_seconds();
        assert!((secs - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_quant_round_trip() {
        let original_secs = 86400.0; // one Earth day
        let q = Quant::from_seconds(original_secs);
        let result_secs = q.to_seconds();
        assert!((result_secs - original_secs).abs() < 0.001);
    }

    #[test]
    fn test_quant_add() {
        let a = Quant::new(100);
        let b = Quant::new(200);
        assert_eq!((a + b).count(), 300);
    }

    #[test]
    fn test_quant_checked_add_overflow() {
        let a = Quant::new(u128::MAX);
        let b = Quant::new(1);
        assert!(a.checked_add(b).is_err());
    }

    #[test]
    fn test_quant_abs_diff() {
        let a = Quant::new(100);
        let b = Quant::new(300);
        assert_eq!(a.abs_diff(b).count(), 200);
        assert_eq!(b.abs_diff(a).count(), 200);
    }

    #[test]
    fn test_quant_display() {
        let q = Quant::new(42);
        assert_eq!(format!("{q}"), "Q:42");
    }

    #[test]
    fn test_accumulator_advance() {
        let mut acc = QuantAccumulator::new();
        acc.advance(Quant::new(1000)).unwrap();
        assert_eq!(acc.current().count(), 1000);
        acc.advance(Quant::new(500)).unwrap();
        assert_eq!(acc.current().count(), 1500);
    }

    #[test]
    fn test_accumulator_elapsed() {
        let mut acc = QuantAccumulator::new();
        acc.advance(Quant::new(5000)).unwrap();
        assert_eq!(acc.elapsed().count(), 5000);
    }

    #[test]
    fn test_accumulator_from_unix() {
        let epoch_offset = Quant::new(0);
        let acc = QuantAccumulator::from_unix_timestamp(1.0, epoch_offset);
        let expected = HYDROGEN_HYPERFINE_FREQ as u128;
        assert!((acc.current().count() as i128 - expected as i128).unsigned_abs() < 1000);
    }
}
