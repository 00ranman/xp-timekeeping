import { describe, it } from "node:test";
import * as assert from "node:assert/strict";
import {
  HYDROGEN_HYPERFINE_FREQ,
  ONE_QUANT_SECONDS,
  EARTH_DAY_SECONDS,
  MARS_SOL_SECONDS,
  TICKS_PER_DAY,
  LOOPS_PER_DAY,
  ARCS_PER_LOOP,
  TICKS_PER_ARC,
  EARTH_SECONDS_PER_TICK,
  MARS_SECONDS_PER_TICK,
  EARTH_QUANTS_PER_TICK,
  MARS_QUANTS_PER_TICK,
  EARTH_QUANTS_PER_DAY,
  MARS_QUANTS_PER_SOL,
  DURATION_UNIT_BASE_EXPONENT,
  DURATION_UNIT_COUNT,
  DURATION_UNIT_NAMES,
  durationHPeriods,
  durationSeconds,
} from "../src/constants";

describe("constants", () => {
  it("hydrogen frequency", () => {
    assert.ok(Math.abs(HYDROGEN_HYPERFINE_FREQ - 1_420_405_751.768) < 0.001);
  });

  it("one quant seconds inverse", () => {
    assert.ok(Math.abs(ONE_QUANT_SECONDS - 1.0 / 1_420_405_751.768) < 1e-20);
  });

  it("ticks structure", () => {
    assert.equal(TICKS_PER_DAY, 100_000);
    assert.equal(LOOPS_PER_DAY * ARCS_PER_LOOP * TICKS_PER_ARC, TICKS_PER_DAY);
  });

  it("earth seconds per tick", () => {
    assert.ok(Math.abs(EARTH_SECONDS_PER_TICK - 0.864) < 0.001);
  });

  it("mars seconds per tick", () => {
    assert.ok(Math.abs(MARS_SECONDS_PER_TICK - 0.88775244) < 0.0001);
  });

  it("duration unit count", () => {
    assert.equal(DURATION_UNIT_COUNT, 7);
    assert.equal(DURATION_UNIT_NAMES.length, 7);
  });

  it("duration h periods power of 10", () => {
    assert.equal(durationHPeriods(0), 10n ** 11n); // Pulse
    assert.equal(durationHPeriods(1), 10n ** 12n); // Wave
    assert.equal(durationHPeriods(6), 10n ** 17n); // Epoch
  });

  it("duration seconds positive", () => {
    for (let i = 0; i < DURATION_UNIT_COUNT; i++) {
      assert.ok(durationSeconds(i) > 0);
    }
  });

  it("duration seconds increasing", () => {
    for (let i = 1; i < DURATION_UNIT_COUNT; i++) {
      assert.ok(durationSeconds(i) > durationSeconds(i - 1));
    }
  });
});
