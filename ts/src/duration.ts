import { durationHPeriods, durationSeconds, DURATION_UNIT_COUNT, DURATION_UNIT_NAMES } from "./constants";
import { DurationOverflowError } from "./error";
import { Quant } from "./quant";

/** Universal Duration unit levels, from smallest to largest. */
export enum DurationUnit {
  Pulse = 0,
  Wave = 1,
  Tide = 2,
  Spin = 3,
  Current = 4,
  Season = 5,
  Epoch = 6,
}

export function durationUnitFromIndex(index: number): DurationUnit {
  if (index < 0 || index >= DURATION_UNIT_COUNT) {
    throw new DurationOverflowError();
  }
  return index as DurationUnit;
}

export function durationUnitName(unit: DurationUnit): string {
  return DURATION_UNIT_NAMES[unit];
}

export function durationUnitHPeriods(unit: DurationUnit): bigint {
  return durationHPeriods(unit);
}

export function durationUnitSeconds(unit: DurationUnit): number {
  return durationSeconds(unit);
}

export function allDurationUnits(): DurationUnit[] {
  return [
    DurationUnit.Pulse,
    DurationUnit.Wave,
    DurationUnit.Tide,
    DurationUnit.Spin,
    DurationUnit.Current,
    DurationUnit.Season,
    DurationUnit.Epoch,
  ];
}

/** A duration value: a count of a specific duration unit. */
export class Duration {
  readonly count: number;
  readonly unit: DurationUnit;

  constructor(count: number, unit: DurationUnit) {
    this.count = count;
    this.unit = unit;
  }

  toSeconds(): number {
    return this.count * durationUnitSeconds(this.unit);
  }

  static fromSeconds(seconds: number, unit: DurationUnit): Duration {
    return new Duration(seconds / durationUnitSeconds(unit), unit);
  }

  toQuants(): Quant {
    const hPeriods = BigInt(Math.floor(this.count * Number(durationUnitHPeriods(this.unit))));
    return new Quant(hPeriods);
  }

  static fromQuants(quants: Quant, unit: DurationUnit): Duration {
    return new Duration(Number(quants.count()) / Number(durationUnitHPeriods(unit)), unit);
  }

  convertTo(target: DurationUnit): Duration {
    const seconds = this.toSeconds();
    return Duration.fromSeconds(seconds, target);
  }

  totalHPeriods(): number {
    return this.count * Number(durationUnitHPeriods(this.unit));
  }

  toString(): string {
    if (this.count === 1.0) {
      return `1 ${durationUnitName(this.unit)}`;
    }
    return `${this.count.toFixed(2)} ${durationUnitName(this.unit)}s`;
  }
}

/** Find the most human-readable duration unit for a given number of seconds. */
export function bestUnitForSeconds(seconds: number): DurationUnit {
  const units = allDurationUnits();
  for (let i = units.length - 1; i >= 0; i--) {
    if (seconds >= durationUnitSeconds(units[i]) * 0.5) {
      return units[i];
    }
  }
  return DurationUnit.Pulse;
}
