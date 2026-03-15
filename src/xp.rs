use serde::{Deserialize, Serialize};

/// XP (Experience Points) — entropy-based value calculation.
///
/// Two formulations exist:
///
/// ## Philosophical (Section 2.2)
/// ```text
/// XP = S × c_L²
/// ```
/// Where S = entropy reduction, c_L = domain-specific propagation speed.
///
/// ## Operational (Section 11.2)
/// ```text
/// XP = B × D × T × V × S
/// ```
/// Where:
/// - B = Base contribution metric
/// - D = Domain multiplier
/// - T = Temporal decay factor (uses Universal Duration, planet-independent)
/// - V = Validation score
/// - S = Scarcity modifier

/// Compute XP using the philosophical formula: XP = S × c_L².
///
/// - `entropy_reduction`: measured entropy reduction in the relevant domain (S)
/// - `propagation_speed`: domain-specific propagation speed constant (c_L)
pub fn xp_philosophical(entropy_reduction: f64, propagation_speed: f64) -> f64 {
    entropy_reduction * propagation_speed * propagation_speed
}

/// Parameters for the operational XP formula.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct XpOperationalParams {
    /// Base contribution metric.
    pub base: f64,
    /// Domain multiplier.
    pub domain: f64,
    /// Temporal decay factor (planet-independent, uses Universal Duration).
    pub temporal_decay: f64,
    /// Validation score.
    pub validation: f64,
    /// Scarcity modifier.
    pub scarcity: f64,
}

impl XpOperationalParams {
    /// Create new operational XP parameters.
    pub fn new(base: f64, domain: f64, temporal_decay: f64, validation: f64, scarcity: f64) -> Self {
        Self {
            base,
            domain,
            temporal_decay,
            validation,
            scarcity,
        }
    }
}

/// Compute XP using the operational formula: XP = B × D × T × V × S.
pub fn xp_operational(params: &XpOperationalParams) -> f64 {
    params.base * params.domain * params.temporal_decay * params.validation * params.scarcity
}

/// Compute temporal decay factor based on elapsed duration in seconds.
///
/// Uses exponential decay: T = e^(-λ × t)
/// where λ is the decay rate and t is elapsed time in seconds.
pub fn temporal_decay(elapsed_seconds: f64, decay_rate: f64) -> f64 {
    (-decay_rate * elapsed_seconds).exp()
}

/// Compute temporal decay using Universal Duration units.
///
/// - `elapsed_h_periods`: elapsed hydrogen hyperfine periods
/// - `decay_rate_per_period`: decay rate per hydrogen period
pub fn temporal_decay_quants(elapsed_h_periods: u128, decay_rate_per_period: f64) -> f64 {
    (-decay_rate_per_period * elapsed_h_periods as f64).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_philosophical_xp() {
        let xp = xp_philosophical(10.0, 3.0);
        assert!((xp - 90.0).abs() < 1e-10); // 10 * 9
    }

    #[test]
    fn test_philosophical_xp_zero_entropy() {
        assert!((xp_philosophical(0.0, 100.0)).abs() < 1e-10);
    }

    #[test]
    fn test_operational_xp() {
        let params = XpOperationalParams::new(100.0, 1.5, 0.9, 0.8, 1.2);
        let xp = xp_operational(&params);
        let expected = 100.0 * 1.5 * 0.9 * 0.8 * 1.2;
        assert!((xp - expected).abs() < 1e-10);
    }

    #[test]
    fn test_operational_xp_identity() {
        let params = XpOperationalParams::new(1.0, 1.0, 1.0, 1.0, 1.0);
        assert!((xp_operational(&params) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_temporal_decay_zero_elapsed() {
        let t = temporal_decay(0.0, 0.001);
        assert!((t - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_temporal_decay_large_elapsed() {
        let t = temporal_decay(10000.0, 0.001);
        assert!(t < 0.001);
        assert!(t > 0.0);
    }

    #[test]
    fn test_temporal_decay_quants_zero() {
        let t = temporal_decay_quants(0, 1e-15);
        assert!((t - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_xp_params_construction() {
        let p = XpOperationalParams::new(50.0, 2.0, 0.95, 0.85, 1.1);
        assert!((p.base - 50.0).abs() < 1e-10);
        assert!((p.domain - 2.0).abs() < 1e-10);
    }
}
