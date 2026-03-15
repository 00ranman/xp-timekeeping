/// Hydrogen-1 ground-state hyperfine transition frequency in Hz.
pub const HYDROGEN_HYPERFINE_FREQ: f64 = 1_420_405_751.768;

/// Duration of one quant (one hydrogen hyperfine period) in seconds.
pub const ONE_QUANT_SECONDS: f64 = 1.0 / HYDROGEN_HYPERFINE_FREQ;

/// Earth solar day in legacy seconds.
pub const EARTH_DAY_SECONDS: f64 = 86_400.0;

/// Earth solar day precise rotational period in seconds (includes secular deceleration).
pub const EARTH_DAY_SECONDS_PRECISE: f64 = 86_400.002;

/// Mars solar day (sol) in legacy seconds.
pub const MARS_SOL_SECONDS: f64 = 88_775.244;

/// Total ticks per local solar day (universal for all planets).
pub const TICKS_PER_DAY: u32 = 100_000;

/// Loops per day.
pub const LOOPS_PER_DAY: u32 = 10;

/// Arcs per loop.
pub const ARCS_PER_LOOP: u32 = 100;

/// Ticks per arc.
pub const TICKS_PER_ARC: u32 = 100;

/// Earth: seconds per tick.
pub const EARTH_SECONDS_PER_TICK: f64 = EARTH_DAY_SECONDS / TICKS_PER_DAY as f64;

/// Mars: seconds per tick.
pub const MARS_SECONDS_PER_TICK: f64 = MARS_SOL_SECONDS / TICKS_PER_DAY as f64;

/// Earth: approximate quants per tick.
pub const EARTH_QUANTS_PER_TICK: u64 = 1_228_000_000;

/// Mars: approximate quants per tick.
pub const MARS_QUANTS_PER_TICK: u64 = 1_261_000_000;

/// Earth: approximate quants per day.
pub const EARTH_QUANTS_PER_DAY: u128 = 122_800_000_000_000;

/// Mars: approximate quants per sol.
pub const MARS_QUANTS_PER_SOL: u128 = 126_100_000_000_000;

/// Earth orbital period in days.
pub const EARTH_ORBITAL_PERIOD_DAYS: f64 = 365.2422;

/// Mars orbital period in Earth days.
pub const MARS_ORBITAL_PERIOD_EARTH_DAYS: f64 = 686.97;

/// Mars orbital period in sols.
pub const MARS_ORBITAL_PERIOD_SOLS: f64 = 668.6;

/// Duration unit exponents: index 0 = Pulse (10^11), through index 6 = Epoch (10^17).
pub const DURATION_UNIT_BASE_EXPONENT: u32 = 11;

/// Number of duration unit levels (Pulse through Epoch).
pub const DURATION_UNIT_COUNT: usize = 7;

/// Duration unit names in ascending order.
pub const DURATION_UNIT_NAMES: [&str; 7] = [
    "Pulse", "Wave", "Tide", "Spin", "Current", "Season", "Epoch",
];

/// Compute the number of hydrogen periods for a given duration unit index (0=Pulse, 6=Epoch).
pub fn duration_h_periods(unit_index: usize) -> u128 {
    10u128.pow((DURATION_UNIT_BASE_EXPONENT + unit_index as u32) as u32)
}

/// Compute the duration in seconds for a given unit index.
pub fn duration_seconds(unit_index: usize) -> f64 {
    duration_h_periods(unit_index) as f64 * ONE_QUANT_SECONDS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_quant_seconds() {
        let expected = 1.0 / 1_420_405_751.768;
        assert!((ONE_QUANT_SECONDS - expected).abs() < 1e-20);
    }

    #[test]
    fn test_earth_seconds_per_tick() {
        assert!((EARTH_SECONDS_PER_TICK - 0.864).abs() < 1e-10);
    }

    #[test]
    fn test_mars_seconds_per_tick() {
        let expected = 88_775.244 / 100_000.0;
        assert!((MARS_SECONDS_PER_TICK - expected).abs() < 1e-10);
    }

    #[test]
    fn test_pulse_h_periods() {
        assert_eq!(duration_h_periods(0), 100_000_000_000); // 10^11
    }

    #[test]
    fn test_epoch_h_periods() {
        assert_eq!(duration_h_periods(6), 100_000_000_000_000_000); // 10^17
    }

    #[test]
    fn test_pulse_seconds() {
        let s = duration_seconds(0);
        assert!((s - 70.4).abs() < 0.1);
    }

    #[test]
    fn test_wave_seconds() {
        let s = duration_seconds(1);
        assert!((s - 704.0).abs() < 1.0);
    }

    #[test]
    fn test_tide_seconds() {
        let s = duration_seconds(2);
        assert!((s - 7040.0).abs() < 10.0);
    }

    #[test]
    fn test_spin_seconds() {
        let s = duration_seconds(3);
        assert!((s - 70400.0).abs() < 100.0);
    }

    #[test]
    fn test_duration_unit_names() {
        assert_eq!(DURATION_UNIT_NAMES[0], "Pulse");
        assert_eq!(DURATION_UNIT_NAMES[6], "Epoch");
    }
}
