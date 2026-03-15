import { describe, it } from "node:test";
import * as assert from "node:assert/strict";
import {
  earthCalendarConfig,
  marsCalendarConfig,
  CalendarDate,
  daysInMonth,
  daysInYear,
  daysPerYearNormal,
  daysPerYearLeap,
  isEarthLeapYear,
} from "../src/calendar";
import { InvalidDateError } from "../src/error";

describe("earthCalendarConfig", () => {
  const config = earthCalendarConfig();

  it("10 months", () => {
    assert.equal(config.monthsPerOrbit, 10);
  });

  it("40 days per standard month", () => {
    assert.equal(config.daysPerStandardMonth, 40);
  });

  it("normal year = 365 days", () => {
    assert.equal(daysPerYearNormal(config), 365);
  });

  it("leap year = 366 days", () => {
    assert.equal(daysPerYearLeap(config), 366);
  });

  it("5-day cycles", () => {
    assert.equal(config.daysPerCycle, 5);
  });

  it("8 cycles per standard month", () => {
    assert.equal(config.cyclesPerStandardMonth, 8);
  });
});

describe("marsCalendarConfig", () => {
  const config = marsCalendarConfig();

  it("74 days per standard month", () => {
    assert.equal(config.daysPerStandardMonth, 74);
  });

  it("normal year = 669 sols", () => {
    assert.equal(daysPerYearNormal(config), 669);
  });

  it("leap year = 670 sols", () => {
    assert.equal(daysPerYearLeap(config), 670);
  });
});

describe("isEarthLeapYear", () => {
  it("2000 is leap", () => {
    assert.ok(isEarthLeapYear(2000));
  });

  it("1900 is not leap", () => {
    assert.ok(!isEarthLeapYear(1900));
  });

  it("2024 is leap", () => {
    assert.ok(isEarthLeapYear(2024));
  });

  it("2023 is not leap", () => {
    assert.ok(!isEarthLeapYear(2023));
  });
});

describe("daysInMonth", () => {
  const config = earthCalendarConfig();

  it("months 1-9 equal 40", () => {
    for (let m = 1; m <= 9; m++) {
      assert.equal(daysInMonth(config, m, false), 40);
    }
  });

  it("month 10 normal = 5", () => {
    assert.equal(daysInMonth(config, 10, false), 5);
  });

  it("month 10 leap = 6", () => {
    assert.equal(daysInMonth(config, 10, true), 6);
  });

  it("invalid month throws", () => {
    assert.throws(() => daysInMonth(config, 0, false), InvalidDateError);
    assert.throws(() => daysInMonth(config, 11, false), InvalidDateError);
  });
});

describe("CalendarDate", () => {
  const config = earthCalendarConfig();

  it("construct valid", () => {
    const d = CalendarDate.new(2026, 1, 1, config, false);
    assert.equal(d.year, 2026);
    assert.equal(d.month, 1);
    assert.equal(d.day, 1);
  });

  it("invalid day throws", () => {
    assert.throws(() => CalendarDate.new(2026, 1, 41, config, false), InvalidDateError);
    assert.throws(() => CalendarDate.new(2026, 1, 0, config, false), InvalidDateError);
  });

  it("day of year first day", () => {
    const d = new CalendarDate(2026, 1, 1);
    assert.equal(d.dayOfYear(config), 1);
  });

  it("day of year month 2 day 1", () => {
    const d = new CalendarDate(2026, 2, 1);
    assert.equal(d.dayOfYear(config), 41);
  });

  it("from day of year roundtrip", () => {
    const original = new CalendarDate(2026, 3, 15);
    const doy = original.dayOfYear(config);
    const restored = CalendarDate.fromDayOfYear(2026, doy, config, false);
    assert.equal(restored.month, 3);
    assert.equal(restored.day, 15);
  });

  it("from day of year last day normal", () => {
    const d = CalendarDate.fromDayOfYear(2023, 365, config, false);
    assert.equal(d.month, 10);
    assert.equal(d.day, 5);
  });

  it("from day of year last day leap", () => {
    const d = CalendarDate.fromDayOfYear(2024, 366, config, true);
    assert.equal(d.month, 10);
    assert.equal(d.day, 6);
  });

  it("from day of year out of range", () => {
    assert.throws(() => CalendarDate.fromDayOfYear(2023, 0, config, false), InvalidDateError);
    assert.throws(() => CalendarDate.fromDayOfYear(2023, 366, config, false), InvalidDateError);
  });

  it("cycle and day", () => {
    const d = new CalendarDate(2026, 1, 7);
    const [cycle, dayInCycle] = d.cycleAndDay(config);
    assert.equal(cycle, 2);
    assert.equal(dayInCycle, 2);
  });

  it("toString", () => {
    const d = new CalendarDate(2026, 3, 15);
    assert.equal(d.toString(), "Y2026:M3:D15");
  });
});
