import { EARTH_DAY_SECONDS } from "./constants";
import { CalendarDate, earthCalendarConfig, isEarthLeapYear } from "./calendar";
import { InvalidDateError, ParseError } from "./error";
import { createEarth } from "./planet";
import { Quant } from "./quant";
import { SolarTime } from "./solar-clock";

/**
 * Convert a Unix timestamp (seconds since 1970-01-01T00:00:00 UTC) to a SolarTime
 * on Earth (time-of-day only; the date portion is discarded).
 */
export function unixToEarthSolarTime(unixSecs: number): SolarTime {
  const secondsInDay = ((unixSecs % EARTH_DAY_SECONDS) + EARTH_DAY_SECONDS) % EARTH_DAY_SECONDS;
  const earth = createEarth();
  return earth.secondsToSolarTime(secondsInDay);
}

/** Convert a SolarTime on Earth to the seconds-since-midnight portion. */
export function earthSolarTimeToSecondsInDay(time: SolarTime): number {
  const earth = createEarth();
  return earth.solarTimeToSeconds(time);
}

/** Convert an ISO 8601 datetime string to Earth SolarTime (time-of-day only). */
export function iso8601ToEarthSolarTime(iso: string): SolarTime {
  const dt = new Date(iso);
  if (isNaN(dt.getTime())) {
    throw new ParseError(`invalid ISO 8601: ${iso}`);
  }
  const secondsInDay = dt.getUTCHours() * 3600 + dt.getUTCMinutes() * 60 + dt.getUTCSeconds();
  const earth = createEarth();
  return earth.secondsToSolarTime(secondsInDay);
}

/** Convert an Earth SolarTime + Gregorian date back to an ISO 8601 string. */
export function earthSolarTimeToIso8601(
  solarTime: SolarTime,
  year: number,
  month: number,
  day: number,
): string {
  const earth = createEarth();
  const secs = earth.solarTimeToSeconds(solarTime);
  const totalSecs = Math.floor(secs);
  const hours = Math.floor(totalSecs / 3600);
  const minutes = Math.floor((totalSecs % 3600) / 60);
  const seconds = totalSecs % 60;

  const yStr = String(year).padStart(4, "0");
  const mStr = String(month).padStart(2, "0");
  const dStr = String(day).padStart(2, "0");
  const hStr = String(hours).padStart(2, "0");
  const minStr = String(minutes).padStart(2, "0");
  const sStr = String(seconds).padStart(2, "0");

  return `${yStr}-${mStr}-${dStr}T${hStr}:${minStr}:${sStr}Z`;
}

/** Convert a Unix timestamp to a quant count (given a quant-at-unix-epoch offset). */
export function unixToQuant(unixSecs: number, quantAtUnixEpoch: Quant): Quant {
  const elapsed = Quant.fromSeconds(unixSecs);
  return new Quant(quantAtUnixEpoch.value + elapsed.value);
}

/** Convert a quant count back to a Unix timestamp. */
export function quantToUnix(quant: Quant, quantAtUnixEpoch: Quant): number {
  const elapsed = new Quant(quant.value - quantAtUnixEpoch.value);
  return elapsed.toSeconds();
}

/** Convert a Gregorian date to a Universal Times CalendarDate. */
export function gregorianToUtCalendar(year: number, month: number, day: number): CalendarDate {
  // Calculate day of year from Gregorian date
  const date = new Date(Date.UTC(year, month - 1, day));
  const jan1 = new Date(Date.UTC(year, 0, 1));
  const doy = Math.floor((date.getTime() - jan1.getTime()) / (86400 * 1000)) + 1;
  const isLeap = isEarthLeapYear(year);
  const config = earthCalendarConfig();
  return CalendarDate.fromDayOfYear(year, doy, config, isLeap);
}

/** Convert a Universal Times CalendarDate back to a Gregorian date. */
export function utCalendarToGregorian(utDate: CalendarDate): [number, number, number] {
  const config = earthCalendarConfig();
  const doy = utDate.dayOfYear(config);
  // Convert day of year back to Gregorian
  const date = new Date(Date.UTC(utDate.year, 0, doy));
  return [date.getUTCFullYear(), date.getUTCMonth() + 1, date.getUTCDate()];
}

/** Convert a legacy 24-hour time (hours, minutes, seconds) to SolarTime. */
export function legacyTimeToSolarTime(hours: number, minutes: number, seconds: number): SolarTime {
  const totalSeconds = hours * 3600 + minutes * 60 + seconds;
  const earth = createEarth();
  return earth.secondsToSolarTime(totalSeconds);
}

/** Convert a SolarTime to legacy 24-hour time (hours, minutes, seconds). */
export function solarTimeToLegacyTime(time: SolarTime): [number, number, number] {
  const earth = createEarth();
  const secs = earth.solarTimeToSeconds(time);
  const total = Math.round(secs);
  const hours = Math.floor(total / 3600);
  const minutes = Math.floor((total % 3600) / 60);
  const seconds = total % 60;
  return [hours, minutes, seconds];
}
