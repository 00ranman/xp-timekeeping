import {
  EARTH_DAY_SECONDS,
  EARTH_ORBITAL_PERIOD_DAYS,
  EARTH_QUANTS_PER_DAY,
  EARTH_QUANTS_PER_TICK,
  MARS_ORBITAL_PERIOD_SOLS,
  MARS_QUANTS_PER_SOL,
  MARS_QUANTS_PER_TICK,
  MARS_SOL_SECONDS,
  TICKS_PER_DAY,
} from "./constants";
import { SolarTime } from "./solar-clock";

/** A planet profile fully defines how universal time maps to local experience. */
export interface PlanetProfile {
  name(): string;
  star(): string;
  rotationalPeriodSeconds(): number;
  orbitalPeriodDays(): number;
  ticksPerDay(): number;
  secondsPerTick(): number;
  quantsPerTick(): bigint;
  quantsPerDay(): bigint;
  secondsToSolarTime(secondsSinceMidnight: number): SolarTime;
  solarTimeToSeconds(time: SolarTime): number;
  quantsToSolarTime(quantsSinceDayStart: bigint): SolarTime;
  solarTimeToQuants(time: SolarTime): bigint;
}

function createBasePlanet(params: {
  name: string;
  star: string;
  rotationalPeriodSeconds: number;
  orbitalPeriodDays: number;
  quantsPerTick: bigint;
  quantsPerDay: bigint;
}): PlanetProfile {
  return {
    name: () => params.name,
    star: () => params.star,
    rotationalPeriodSeconds: () => params.rotationalPeriodSeconds,
    orbitalPeriodDays: () => params.orbitalPeriodDays,
    ticksPerDay: () => TICKS_PER_DAY,
    secondsPerTick: () => params.rotationalPeriodSeconds / TICKS_PER_DAY,
    quantsPerTick: () => params.quantsPerTick,
    quantsPerDay: () => params.quantsPerDay,
    secondsToSolarTime(secondsSinceMidnight: number): SolarTime {
      const fraction = secondsSinceMidnight / params.rotationalPeriodSeconds;
      return SolarTime.fromDayFraction(fraction);
    },
    solarTimeToSeconds(time: SolarTime): number {
      return time.dayFraction() * params.rotationalPeriodSeconds;
    },
    quantsToSolarTime(quantsSinceDayStart: bigint): SolarTime {
      const totalTicks = Number(quantsSinceDayStart / params.quantsPerTick);
      const clamped = Math.min(totalTicks, TICKS_PER_DAY - 1);
      return SolarTime.fromTotalTicks(clamped);
    },
    solarTimeToQuants(time: SolarTime): bigint {
      return BigInt(time.toTotalTicks()) * params.quantsPerTick;
    },
  };
}

export function createEarth(): PlanetProfile {
  return createBasePlanet({
    name: "EARTH",
    star: "Sol",
    rotationalPeriodSeconds: EARTH_DAY_SECONDS,
    orbitalPeriodDays: EARTH_ORBITAL_PERIOD_DAYS,
    quantsPerTick: EARTH_QUANTS_PER_TICK,
    quantsPerDay: EARTH_QUANTS_PER_DAY,
  });
}

export function createMars(): PlanetProfile {
  return createBasePlanet({
    name: "MARS",
    star: "Sol",
    rotationalPeriodSeconds: MARS_SOL_SECONDS,
    orbitalPeriodDays: MARS_ORBITAL_PERIOD_SOLS,
    quantsPerTick: MARS_QUANTS_PER_TICK,
    quantsPerDay: MARS_QUANTS_PER_SOL,
  });
}

export function getPlanet(name: string): PlanetProfile | null {
  switch (name.toUpperCase()) {
    case "EARTH":
      return createEarth();
    case "MARS":
      return createMars();
    default:
      return null;
  }
}
