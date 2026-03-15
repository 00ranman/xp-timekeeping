/** Hydrogen-1 ground-state hyperfine transition frequency in Hz. */
export const HYDROGEN_HYPERFINE_FREQ = 1_420_405_751.768;

/** Duration of one quant (one hydrogen hyperfine period) in seconds. */
export const ONE_QUANT_SECONDS = 1.0 / HYDROGEN_HYPERFINE_FREQ;

/** Earth solar day in legacy seconds. */
export const EARTH_DAY_SECONDS = 86_400.0;

/** Earth solar day precise rotational period in seconds (includes secular deceleration). */
export const EARTH_DAY_SECONDS_PRECISE = 86_400.002;

/** Mars solar day (sol) in legacy seconds. */
export const MARS_SOL_SECONDS = 88_775.244;

/** Total ticks per local solar day (universal for all planets). */
export const TICKS_PER_DAY = 100_000;

/** Loops per day. */
export const LOOPS_PER_DAY = 10;

/** Arcs per loop. */
export const ARCS_PER_LOOP = 100;

/** Ticks per arc. */
export const TICKS_PER_ARC = 100;

/** Earth: seconds per tick. */
export const EARTH_SECONDS_PER_TICK = EARTH_DAY_SECONDS / TICKS_PER_DAY;

/** Mars: seconds per tick. */
export const MARS_SECONDS_PER_TICK = MARS_SOL_SECONDS / TICKS_PER_DAY;

/** Earth: approximate quants per tick. */
export const EARTH_QUANTS_PER_TICK = 1_228_000_000n;

/** Mars: approximate quants per tick. */
export const MARS_QUANTS_PER_TICK = 1_261_000_000n;

/** Earth: approximate quants per day. */
export const EARTH_QUANTS_PER_DAY = 122_800_000_000_000n;

/** Mars: approximate quants per sol. */
export const MARS_QUANTS_PER_SOL = 126_100_000_000_000n;

/** Earth orbital period in days. */
export const EARTH_ORBITAL_PERIOD_DAYS = 365.2422;

/** Mars orbital period in Earth days. */
export const MARS_ORBITAL_PERIOD_EARTH_DAYS = 686.97;

/** Mars orbital period in sols. */
export const MARS_ORBITAL_PERIOD_SOLS = 668.6;

/** Duration unit exponents: index 0 = Pulse (10^11), through index 6 = Epoch (10^17). */
export const DURATION_UNIT_BASE_EXPONENT = 11;

/** Number of duration unit levels (Pulse through Epoch). */
export const DURATION_UNIT_COUNT = 7;

/** Duration unit names in ascending order. */
export const DURATION_UNIT_NAMES = [
  "Pulse", "Wave", "Tide", "Spin", "Current", "Season", "Epoch",
] as const;

/** Compute the number of hydrogen periods for a given duration unit index (0=Pulse, 6=Epoch). */
export function durationHPeriods(unitIndex: number): bigint {
  return BigInt(10) ** BigInt(DURATION_UNIT_BASE_EXPONENT + unitIndex);
}

/** Compute the duration in seconds for a given unit index. */
export function durationSeconds(unitIndex: number): number {
  return Number(durationHPeriods(unitIndex)) * ONE_QUANT_SECONDS;
}
