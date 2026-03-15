use serde::{Deserialize, Serialize};
use std::fmt;

use crate::constants::{ARCS_PER_LOOP, LOOPS_PER_DAY, TICKS_PER_ARC, TICKS_PER_DAY};
use crate::error::{UtError, UtResult};

/// Solar Clock time — a position in the local solar day.
///
/// Display format: `t:L:AA:TT`
/// - L = loop (0-9)
/// - AA = arc (00-99)
/// - TT = tick (00-99)
///
/// Loop 0 = local solar midnight, Loop 5 = local solar noon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SolarTime {
    loop_val: u32,
    arc: u32,
    tick: u32,
}

impl SolarTime {
    /// Create a new SolarTime, validating ranges.
    pub fn new(loop_val: u32, arc: u32, tick: u32) -> UtResult<Self> {
        if loop_val >= LOOPS_PER_DAY {
            return Err(UtError::InvalidSolarTime {
                field: "loop",
                value: loop_val,
                max: LOOPS_PER_DAY - 1,
            });
        }
        if arc >= ARCS_PER_LOOP {
            return Err(UtError::InvalidSolarTime {
                field: "arc",
                value: arc,
                max: ARCS_PER_LOOP - 1,
            });
        }
        if tick >= TICKS_PER_ARC {
            return Err(UtError::InvalidSolarTime {
                field: "tick",
                value: tick,
                max: TICKS_PER_ARC - 1,
            });
        }
        Ok(Self {
            loop_val,
            arc,
            tick,
        })
    }

    /// Create from a total tick count within the day (0-99999).
    pub fn from_total_ticks(total: u32) -> UtResult<Self> {
        if total >= TICKS_PER_DAY {
            return Err(UtError::InvalidSolarTime {
                field: "total_ticks",
                value: total,
                max: TICKS_PER_DAY - 1,
            });
        }
        let loop_val = total / (ARCS_PER_LOOP * TICKS_PER_ARC);
        let remainder = total % (ARCS_PER_LOOP * TICKS_PER_ARC);
        let arc = remainder / TICKS_PER_ARC;
        let tick = remainder % TICKS_PER_ARC;
        Ok(Self {
            loop_val,
            arc,
            tick,
        })
    }

    /// Convert to total ticks within the day.
    pub fn to_total_ticks(self) -> u32 {
        self.loop_val * ARCS_PER_LOOP * TICKS_PER_ARC + self.arc * TICKS_PER_ARC + self.tick
    }

    /// Get the loop value (0-9).
    pub fn loop_val(self) -> u32 {
        self.loop_val
    }

    /// Get the arc value (0-99).
    pub fn arc(self) -> u32 {
        self.arc
    }

    /// Get the tick value (0-99).
    pub fn tick(self) -> u32 {
        self.tick
    }

    /// Check if this is solar midnight (t:0:00:00).
    pub fn is_midnight(self) -> bool {
        self.to_total_ticks() == 0
    }

    /// Check if this is solar noon (t:5:00:00).
    pub fn is_noon(self) -> bool {
        self.loop_val == 5 && self.arc == 0 && self.tick == 0
    }

    /// Fraction of day elapsed (0.0 to 1.0).
    pub fn day_fraction(self) -> f64 {
        self.to_total_ticks() as f64 / TICKS_PER_DAY as f64
    }

    /// Create from a fraction of the day (0.0 to 1.0).
    pub fn from_day_fraction(fraction: f64) -> UtResult<Self> {
        let total = (fraction * TICKS_PER_DAY as f64).round() as u32;
        let clamped = total.min(TICKS_PER_DAY - 1);
        Self::from_total_ticks(clamped)
    }

    /// Parse from display format `t:L:AA:TT`.
    pub fn parse(s: &str) -> UtResult<Self> {
        let s = s.trim();
        if !s.starts_with("t:") {
            return Err(UtError::ParseError(format!(
                "solar time must start with 't:', got '{s}'"
            )));
        }
        let rest = &s[2..];
        let parts: Vec<&str> = rest.split(':').collect();
        if parts.len() != 3 {
            return Err(UtError::ParseError(format!(
                "expected format t:L:AA:TT, got '{s}'"
            )));
        }
        let loop_val = parts[0]
            .parse::<u32>()
            .map_err(|_| UtError::ParseError(format!("invalid loop: '{}'", parts[0])))?;
        let arc = parts[1]
            .parse::<u32>()
            .map_err(|_| UtError::ParseError(format!("invalid arc: '{}'", parts[1])))?;
        let tick = parts[2]
            .parse::<u32>()
            .map_err(|_| UtError::ParseError(format!("invalid tick: '{}'", parts[2])))?;
        Self::new(loop_val, arc, tick)
    }
}

impl fmt::Display for SolarTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t:{}:{:02}:{:02}", self.loop_val, self.arc, self.tick)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_time_midnight() {
        let t = SolarTime::new(0, 0, 0).unwrap();
        assert!(t.is_midnight());
        assert_eq!(format!("{t}"), "t:0:00:00");
    }

    #[test]
    fn test_solar_time_noon() {
        let t = SolarTime::new(5, 0, 0).unwrap();
        assert!(t.is_noon());
        assert_eq!(format!("{t}"), "t:5:00:00");
    }

    #[test]
    fn test_solar_time_end_of_day() {
        let t = SolarTime::new(9, 99, 99).unwrap();
        assert_eq!(format!("{t}"), "t:9:99:99");
        assert_eq!(t.to_total_ticks(), 99999);
    }

    #[test]
    fn test_solar_time_invalid_loop() {
        assert!(SolarTime::new(10, 0, 0).is_err());
    }

    #[test]
    fn test_solar_time_invalid_arc() {
        assert!(SolarTime::new(0, 100, 0).is_err());
    }

    #[test]
    fn test_solar_time_invalid_tick() {
        assert!(SolarTime::new(0, 0, 100).is_err());
    }

    #[test]
    fn test_from_total_ticks() {
        let t = SolarTime::from_total_ticks(50000).unwrap();
        assert_eq!(t.loop_val(), 5);
        assert_eq!(t.arc(), 0);
        assert_eq!(t.tick(), 0);
    }

    #[test]
    fn test_total_ticks_round_trip() {
        let t = SolarTime::new(7, 42, 85).unwrap();
        let total = t.to_total_ticks();
        let t2 = SolarTime::from_total_ticks(total).unwrap();
        assert_eq!(t, t2);
    }

    #[test]
    fn test_parse() {
        let t = SolarTime::parse("t:7:42:85").unwrap();
        assert_eq!(t.loop_val(), 7);
        assert_eq!(t.arc(), 42);
        assert_eq!(t.tick(), 85);
    }

    #[test]
    fn test_parse_display_round_trip() {
        let original = SolarTime::new(3, 15, 7).unwrap();
        let s = format!("{original}");
        let parsed = SolarTime::parse(&s).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_day_fraction_noon() {
        let t = SolarTime::new(5, 0, 0).unwrap();
        assert!((t.day_fraction() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_from_day_fraction() {
        let t = SolarTime::from_day_fraction(0.5).unwrap();
        assert_eq!(t.loop_val(), 5);
        assert_eq!(t.arc(), 0);
        assert_eq!(t.tick(), 0);
    }

    #[test]
    fn test_parse_invalid_prefix() {
        assert!(SolarTime::parse("5:00:00").is_err());
    }

    #[test]
    fn test_parse_invalid_format() {
        assert!(SolarTime::parse("t:5:00").is_err());
    }

    #[test]
    fn test_quarter_day() {
        let t = SolarTime::new(2, 50, 0).unwrap();
        assert_eq!(t.to_total_ticks(), 25000);
        assert!((t.day_fraction() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn test_three_quarter_day() {
        let t = SolarTime::new(7, 50, 0).unwrap();
        assert_eq!(t.to_total_ticks(), 75000);
        assert!((t.day_fraction() - 0.75).abs() < 1e-10);
    }
}
