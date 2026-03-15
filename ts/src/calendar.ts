import { InvalidDateError } from "./error";

/**
 * Calendar configuration for a planet.
 *
 * Universal Times uses 10 months per orbit. Months 1-9 have equal length.
 * Month 10 is a short closing month that absorbs the orbital remainder.
 * 5-day cycles replace 7-day weeks.
 */
export interface CalendarConfig {
  monthsPerOrbit: number;
  daysPerStandardMonth: number;
  daysMonth10Normal: number;
  daysMonth10Leap: number;
  daysPerCycle: number;
  cyclesPerStandardMonth: number;
  remainderDaysPerMonth: number;
}

export function earthCalendarConfig(): CalendarConfig {
  return {
    monthsPerOrbit: 10,
    daysPerStandardMonth: 40,
    daysMonth10Normal: 5,
    daysMonth10Leap: 6,
    daysPerCycle: 5,
    cyclesPerStandardMonth: 8,
    remainderDaysPerMonth: 0,
  };
}

export function marsCalendarConfig(): CalendarConfig {
  return {
    monthsPerOrbit: 10,
    daysPerStandardMonth: 74,
    daysMonth10Normal: 3,
    daysMonth10Leap: 4,
    daysPerCycle: 5,
    cyclesPerStandardMonth: 14,
    remainderDaysPerMonth: 4,
  };
}

export function daysPerYearNormal(config: CalendarConfig): number {
  return config.daysPerStandardMonth * 9 + config.daysMonth10Normal;
}

export function daysPerYearLeap(config: CalendarConfig): number {
  return config.daysPerStandardMonth * 9 + config.daysMonth10Leap;
}

export function daysInYear(config: CalendarConfig, isLeap: boolean): number {
  return isLeap ? daysPerYearLeap(config) : daysPerYearNormal(config);
}

export function daysInMonth(config: CalendarConfig, month: number, isLeap: boolean): number {
  if (month < 1 || month > config.monthsPerOrbit) {
    throw new InvalidDateError(`month ${month} out of range 1-${config.monthsPerOrbit}`);
  }
  if (month <= 9) return config.daysPerStandardMonth;
  return isLeap ? config.daysMonth10Leap : config.daysMonth10Normal;
}

/** Determine if an Earth year is a leap year using the Gregorian rule. */
export function isEarthLeapYear(year: number): boolean {
  return (year % 4 === 0 && year % 100 !== 0) || year % 400 === 0;
}

/** A calendar date in Universal Times. */
export class CalendarDate {
  readonly year: number;
  readonly month: number;
  readonly day: number;

  constructor(year: number, month: number, day: number) {
    this.year = year;
    this.month = month;
    this.day = day;
  }

  static new(year: number, month: number, day: number, config: CalendarConfig, isLeap: boolean): CalendarDate {
    const maxDays = daysInMonth(config, month, isLeap);
    if (day < 1 || day > maxDays) {
      throw new InvalidDateError(`day ${day} out of range 1-${maxDays} for month ${month}`);
    }
    return new CalendarDate(year, month, day);
  }

  dayOfYear(config: CalendarConfig): number {
    const fullMonths = this.month - 1;
    const daysFromMonths = Math.min(fullMonths, 9) * config.daysPerStandardMonth;
    return daysFromMonths + this.day;
  }

  static fromDayOfYear(year: number, doy: number, config: CalendarConfig, isLeap: boolean): CalendarDate {
    const totalDays = daysInYear(config, isLeap);
    if (doy < 1 || doy > totalDays) {
      throw new InvalidDateError(`day-of-year ${doy} out of range 1-${totalDays}`);
    }

    let remaining = doy;
    for (let m = 1; m <= 10; m++) {
      const md = daysInMonth(config, m, isLeap);
      if (remaining <= md) {
        return new CalendarDate(year, m, remaining);
      }
      remaining -= md;
    }

    throw new InvalidDateError(`could not resolve day-of-year ${doy}`);
  }

  cycleAndDay(config: CalendarConfig): [number, number] {
    const dayZero = this.day - 1;
    const cycle = Math.floor(dayZero / config.daysPerCycle) + 1;
    const dayInCycle = (dayZero % config.daysPerCycle) + 1;
    return [cycle, dayInCycle];
  }

  toString(): string {
    return `Y${this.year}:M${this.month}:D${this.day}`;
  }
}
