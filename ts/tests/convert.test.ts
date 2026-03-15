import { describe, it } from "node:test";
import * as assert from "node:assert/strict";
import {
  unixToEarthSolarTime,
  earthSolarTimeToSecondsInDay,
  iso8601ToEarthSolarTime,
  earthSolarTimeToIso8601,
  unixToQuant,
  quantToUnix,
  gregorianToUtCalendar,
  utCalendarToGregorian,
  legacyTimeToSolarTime,
  solarTimeToLegacyTime,
} from "../src/convert";
import { SolarTime } from "../src/solar-clock";
import { Quant } from "../src/quant";
import { ParseError } from "../src/error";

describe("unixToEarthSolarTime", () => {
  it("epoch midnight", () => {
    const t = unixToEarthSolarTime(0);
    assert.ok(t.isMidnight());
  });

  it("epoch noon", () => {
    const t = unixToEarthSolarTime(43200);
    assert.ok(t.isNoon());
  });

  it("negative wraps correctly", () => {
    const t = unixToEarthSolarTime(-43200);
    assert.ok(t.isNoon());
  });
});

describe("earthSolarTimeToSecondsInDay", () => {
  it("midnight is 0", () => {
    const secs = earthSolarTimeToSecondsInDay(SolarTime.new(0, 0, 0));
    assert.ok(Math.abs(secs) < 0.001);
  });

  it("noon is half day", () => {
    const secs = earthSolarTimeToSecondsInDay(SolarTime.new(5, 0, 0));
    assert.ok(Math.abs(secs - 43200) < 1);
  });

  it("roundtrip", () => {
    const original = SolarTime.new(3, 45, 67);
    const secs = earthSolarTimeToSecondsInDay(original);
    const restored = unixToEarthSolarTime(secs);
    assert.ok(original.equals(restored));
  });
});

describe("iso8601ToEarthSolarTime", () => {
  it("midnight", () => {
    const t = iso8601ToEarthSolarTime("2026-01-01T00:00:00Z");
    assert.ok(t.isMidnight());
  });

  it("noon", () => {
    const t = iso8601ToEarthSolarTime("2026-01-01T12:00:00Z");
    assert.ok(t.isNoon());
  });

  it("invalid throws", () => {
    assert.throws(() => iso8601ToEarthSolarTime("not-a-date"), ParseError);
  });
});

describe("earthSolarTimeToIso8601", () => {
  it("midnight", () => {
    const iso = earthSolarTimeToIso8601(SolarTime.new(0, 0, 0), 2026, 1, 1);
    assert.equal(iso, "2026-01-01T00:00:00Z");
  });

  it("noon", () => {
    const iso = earthSolarTimeToIso8601(SolarTime.new(5, 0, 0), 2026, 6, 15);
    assert.ok(iso.includes("T12:00:00Z"));
  });
});

describe("unixToQuant / quantToUnix", () => {
  it("roundtrip from epoch", () => {
    const q = unixToQuant(1000, Quant.zero());
    const unix = quantToUnix(q, Quant.zero());
    assert.ok(Math.abs(unix - 1000) < 0.001);
  });

  it("with nonzero epoch offset", () => {
    const epochOffset = new Quant(1_000_000_000_000n);
    const q = unixToQuant(500, epochOffset);
    const unix = quantToUnix(q, epochOffset);
    assert.ok(Math.abs(unix - 500) < 0.001);
  });
});

describe("gregorianToUtCalendar / utCalendarToGregorian", () => {
  it("jan 1 roundtrip", () => {
    const cal = gregorianToUtCalendar(2026, 1, 1);
    assert.equal(cal.month, 1);
    assert.equal(cal.day, 1);
    const [y, m, d] = utCalendarToGregorian(cal);
    assert.equal(y, 2026);
    assert.equal(m, 1);
    assert.equal(d, 1);
  });

  it("mid-year roundtrip", () => {
    const cal = gregorianToUtCalendar(2026, 6, 15);
    const [y, m, d] = utCalendarToGregorian(cal);
    assert.equal(y, 2026);
    assert.equal(m, 6);
    assert.equal(d, 15);
  });
});

describe("legacyTimeToSolarTime / solarTimeToLegacyTime", () => {
  it("midnight", () => {
    const t = legacyTimeToSolarTime(0, 0, 0);
    assert.ok(t.isMidnight());
  });

  it("noon", () => {
    const t = legacyTimeToSolarTime(12, 0, 0);
    assert.ok(t.isNoon());
  });

  it("roundtrip", () => {
    const t = legacyTimeToSolarTime(14, 30, 0);
    const [h, m, s] = solarTimeToLegacyTime(t);
    assert.equal(h, 14);
    assert.equal(m, 30);
    assert.equal(s, 0);
  });
});
