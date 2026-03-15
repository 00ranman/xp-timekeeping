/**
 * Compute XP using the philosophical formula: XP = S × c_L².
 *
 * @param entropyReduction - measured entropy reduction in the relevant domain (S)
 * @param propagationSpeed - domain-specific propagation speed constant (c_L)
 */
export function xpPhilosophical(entropyReduction: number, propagationSpeed: number): number {
  return entropyReduction * propagationSpeed * propagationSpeed;
}

/** Parameters for the operational XP formula. */
export interface XpOperationalParams {
  base: number;
  domain: number;
  temporalDecay: number;
  validation: number;
  scarcity: number;
}

/** Compute XP using the operational formula: XP = B × D × T × V × S. */
export function xpOperational(params: XpOperationalParams): number {
  return params.base * params.domain * params.temporalDecay * params.validation * params.scarcity;
}

/**
 * Compute temporal decay factor based on elapsed duration in seconds.
 *
 * Uses exponential decay: T = e^(-λ × t)
 */
export function temporalDecay(elapsedSeconds: number, decayRate: number): number {
  return Math.exp(-decayRate * elapsedSeconds);
}

/**
 * Compute temporal decay using Universal Duration units.
 *
 * @param elapsedHPeriods - elapsed hydrogen hyperfine periods
 * @param decayRatePerPeriod - decay rate per hydrogen period
 */
export function temporalDecayQuants(elapsedHPeriods: bigint, decayRatePerPeriod: number): number {
  return Math.exp(-decayRatePerPeriod * Number(elapsedHPeriods));
}
