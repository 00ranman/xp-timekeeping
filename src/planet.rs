use serde::{Deserialize, Serialize};

use crate::constants::*;
use crate::solar_clock::SolarTime;
use crate::error::UtResult;

/// A planet profile fully defines how universal time maps to local experience.
///
/// The profile includes the planet's rotational period, orbital period,
/// and all derived clock parameters.
pub trait PlanetProfile {
    /// Planet name (e.g., "EARTH", "MARS").
    fn name(&self) -> &str;

    /// Star name (e.g., "Sol").
    fn star(&self) -> &str;

    /// Rotational period in legacy seconds (length of one solar day/sol).
    fn rotational_period_seconds(&self) -> f64;

    /// Orbital period in local days/sols.
    fn orbital_period_days(&self) -> f64;

    /// Ticks per local day (always 100,000).
    fn ticks_per_day(&self) -> u32 {
        TICKS_PER_DAY
    }

    /// Seconds per tick on this planet.
    fn seconds_per_tick(&self) -> f64 {
        self.rotational_period_seconds() / self.ticks_per_day() as f64
    }

    /// Approximate quants per tick on this planet.
    fn quants_per_tick(&self) -> u64;

    /// Approximate quants per day on this planet.
    fn quants_per_day(&self) -> u128;

    /// Convert a fractional number of seconds since start of day to SolarTime.
    fn seconds_to_solar_time(&self, seconds_since_midnight: f64) -> UtResult<SolarTime> {
        let fraction = seconds_since_midnight / self.rotational_period_seconds();
        SolarTime::from_day_fraction(fraction)
    }

    /// Convert a SolarTime back to seconds since start of day.
    fn solar_time_to_seconds(&self, time: SolarTime) -> f64 {
        time.day_fraction() * self.rotational_period_seconds()
    }

    /// Convert a quant offset within a day to SolarTime.
    fn quants_to_solar_time(&self, quants_since_day_start: u128) -> UtResult<SolarTime> {
        let total_ticks = (quants_since_day_start / self.quants_per_tick() as u128) as u32;
        let clamped = total_ticks.min(TICKS_PER_DAY - 1);
        SolarTime::from_total_ticks(clamped)
    }

    /// Convert SolarTime to quants offset within a day.
    fn solar_time_to_quants(&self, time: SolarTime) -> u128 {
        time.to_total_ticks() as u128 * self.quants_per_tick() as u128
    }
}

/// Canonical Earth planet profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Earth;

impl PlanetProfile for Earth {
    fn name(&self) -> &str {
        "EARTH"
    }

    fn star(&self) -> &str {
        "Sol"
    }

    fn rotational_period_seconds(&self) -> f64 {
        EARTH_DAY_SECONDS
    }

    fn orbital_period_days(&self) -> f64 {
        EARTH_ORBITAL_PERIOD_DAYS
    }

    fn quants_per_tick(&self) -> u64 {
        EARTH_QUANTS_PER_TICK
    }

    fn quants_per_day(&self) -> u128 {
        EARTH_QUANTS_PER_DAY
    }
}

/// Canonical Mars planet profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mars;

impl PlanetProfile for Mars {
    fn name(&self) -> &str {
        "MARS"
    }

    fn star(&self) -> &str {
        "Sol"
    }

    fn rotational_period_seconds(&self) -> f64 {
        MARS_SOL_SECONDS
    }

    fn orbital_period_days(&self) -> f64 {
        MARS_ORBITAL_PERIOD_SOLS
    }

    fn quants_per_tick(&self) -> u64 {
        MARS_QUANTS_PER_TICK
    }

    fn quants_per_day(&self) -> u128 {
        MARS_QUANTS_PER_SOL
    }
}

/// Get a boxed planet profile by name.
pub fn get_planet(name: &str) -> Option<Box<dyn PlanetProfile>> {
    match name.to_uppercase().as_str() {
        "EARTH" => Some(Box::new(Earth)),
        "MARS" => Some(Box::new(Mars)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_earth_basics() {
        let earth = Earth;
        assert_eq!(earth.name(), "EARTH");
        assert_eq!(earth.star(), "Sol");
        assert_eq!(earth.ticks_per_day(), 100_000);
        assert!((earth.rotational_period_seconds() - 86_400.0).abs() < 0.01);
    }

    #[test]
    fn test_earth_seconds_per_tick() {
        let earth = Earth;
        assert!((earth.seconds_per_tick() - 0.864).abs() < 1e-10);
    }

    #[test]
    fn test_mars_basics() {
        let mars = Mars;
        assert_eq!(mars.name(), "MARS");
        assert!((mars.rotational_period_seconds() - 88_775.244).abs() < 0.01);
    }

    #[test]
    fn test_mars_seconds_per_tick() {
        let mars = Mars;
        let expected = 88_775.244 / 100_000.0;
        assert!((mars.seconds_per_tick() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_earth_noon_conversion() {
        let earth = Earth;
        // Noon = 43200 seconds into the day
        let t = earth.seconds_to_solar_time(43200.0).unwrap();
        assert_eq!(t.loop_val(), 5);
        assert_eq!(t.arc(), 0);
        assert_eq!(t.tick(), 0);
    }

    #[test]
    fn test_earth_solar_time_to_seconds() {
        let earth = Earth;
        let noon = SolarTime::new(5, 0, 0).unwrap();
        let secs = earth.solar_time_to_seconds(noon);
        assert!((secs - 43200.0).abs() < 0.01);
    }

    #[test]
    fn test_mars_noon_conversion() {
        let mars = Mars;
        let half_sol = 88_775.244 / 2.0;
        let t = mars.seconds_to_solar_time(half_sol).unwrap();
        assert_eq!(t.loop_val(), 5);
        assert_eq!(t.arc(), 0);
        assert_eq!(t.tick(), 0);
    }

    #[test]
    fn test_earth_solar_time_round_trip() {
        let earth = Earth;
        let original = SolarTime::new(7, 42, 85).unwrap();
        let secs = earth.solar_time_to_seconds(original);
        let back = earth.seconds_to_solar_time(secs).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn test_mars_solar_time_round_trip() {
        let mars = Mars;
        let original = SolarTime::new(3, 25, 50).unwrap();
        let secs = mars.solar_time_to_seconds(original);
        let back = mars.seconds_to_solar_time(secs).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn test_get_planet_earth() {
        let p = get_planet("earth").unwrap();
        assert_eq!(p.name(), "EARTH");
    }

    #[test]
    fn test_get_planet_mars() {
        let p = get_planet("MARS").unwrap();
        assert_eq!(p.name(), "MARS");
    }

    #[test]
    fn test_get_planet_unknown() {
        assert!(get_planet("venus").is_none());
    }

    #[test]
    fn test_earth_quants_per_tick_approx() {
        let earth = Earth;
        // 0.864 seconds * freq ≈ 1.228e9
        let computed = (earth.seconds_per_tick() * HYDROGEN_HYPERFINE_FREQ) as u64;
        let diff = (computed as i64 - earth.quants_per_tick() as i64).unsigned_abs();
        // Allow some approximation tolerance
        assert!(diff < 1_000_000);
    }
}
