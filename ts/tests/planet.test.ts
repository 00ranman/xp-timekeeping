import { describe, it } from "node:test";
import * as assert from "node:assert/strict";
import { createEarth, createMars, getPlanet } from "../src/planet";
import { SolarTime } from "../src/solar-clock";
import { EARTH_DAY_SECONDS, MARS_SOL_SECONDS, TICKS_PER_DAY } from "../src/constants";

describe("Earth", () => {
  const earth = createEarth();

  it("name", () => {
    assert.equal(earth.name(), "EARTH");
  });

  it("star", () => {
    assert.equal(earth.star(), "Sol");
  });

  it("rotational period", () => {
    assert.equal(earth.rotationalPeriodSeconds(), EARTH_DAY_SECONDS);
  });

  it("ticks per day", () => {
    assert.equal(earth.ticksPerDay(), TICKS_PER_DAY);
  });

  it("seconds per tick", () => {
    assert.ok(Math.abs(earth.secondsPerTick() - 0.864) < 0.001);
  });

  it("seconds to solar time midnight", () => {
    const t = earth.secondsToSolarTime(0);
    assert.ok(t.isMidnight());
  });

  it("seconds to solar time noon", () => {
    const t = earth.secondsToSolarTime(EARTH_DAY_SECONDS / 2);
    assert.ok(t.isNoon());
  });

  it("solar time to seconds roundtrip", () => {
    const t = SolarTime.new(3, 45, 67);
    const secs = earth.solarTimeToSeconds(t);
    const t2 = earth.secondsToSolarTime(secs);
    assert.ok(t.equals(t2));
  });

  it("quants to solar time", () => {
    const t = earth.quantsToSolarTime(0n);
    assert.ok(t.isMidnight());
  });

  it("solar time to quants roundtrip", () => {
    const t = SolarTime.new(5, 0, 0);
    const q = earth.solarTimeToQuants(t);
    const t2 = earth.quantsToSolarTime(q);
    assert.ok(t.equals(t2));
  });
});

describe("Mars", () => {
  const mars = createMars();

  it("name", () => {
    assert.equal(mars.name(), "MARS");
  });

  it("star", () => {
    assert.equal(mars.star(), "Sol");
  });

  it("rotational period", () => {
    assert.equal(mars.rotationalPeriodSeconds(), MARS_SOL_SECONDS);
  });

  it("ticks per day", () => {
    assert.equal(mars.ticksPerDay(), TICKS_PER_DAY);
  });

  it("seconds per tick differs from earth", () => {
    const earth = createEarth();
    assert.notEqual(mars.secondsPerTick(), earth.secondsPerTick());
  });
});

describe("getPlanet", () => {
  it("earth", () => {
    const p = getPlanet("Earth");
    assert.notEqual(p, null);
    assert.equal(p!.name(), "EARTH");
  });

  it("mars", () => {
    const p = getPlanet("MARS");
    assert.notEqual(p, null);
    assert.equal(p!.name(), "MARS");
  });

  it("unknown returns null", () => {
    assert.equal(getPlanet("Pluto"), null);
  });

  it("case insensitive", () => {
    const p = getPlanet("earth");
    assert.notEqual(p, null);
  });
});
