import { describe, it } from "node:test";
import * as assert from "node:assert/strict";
import {
  DurationUnit,
  Duration,
  durationUnitName,
  durationUnitHPeriods,
  durationUnitSeconds,
  allDurationUnits,
  bestUnitForSeconds,
  durationUnitFromIndex,
} from "../src/duration";
import { DurationOverflowError } from "../src/error";

describe("DurationUnit", () => {
  it("seven units", () => {
    assert.equal(allDurationUnits().length, 7);
  });

  it("names", () => {
    assert.equal(durationUnitName(DurationUnit.Pulse), "Pulse");
    assert.equal(durationUnitName(DurationUnit.Wave), "Wave");
    assert.equal(durationUnitName(DurationUnit.Tide), "Tide");
    assert.equal(durationUnitName(DurationUnit.Spin), "Spin");
    assert.equal(durationUnitName(DurationUnit.Current), "Current");
    assert.equal(durationUnitName(DurationUnit.Season), "Season");
    assert.equal(durationUnitName(DurationUnit.Epoch), "Epoch");
  });

  it("h periods increase", () => {
    const units = allDurationUnits();
    for (let i = 1; i < units.length; i++) {
      assert.ok(durationUnitHPeriods(units[i]) > durationUnitHPeriods(units[i - 1]));
    }
  });

  it("seconds increase", () => {
    const units = allDurationUnits();
    for (let i = 1; i < units.length; i++) {
      assert.ok(durationUnitSeconds(units[i]) > durationUnitSeconds(units[i - 1]));
    }
  });

  it("from index invalid", () => {
    assert.throws(() => durationUnitFromIndex(-1), DurationOverflowError);
    assert.throws(() => durationUnitFromIndex(7), DurationOverflowError);
  });

  it("pulse h periods", () => {
    assert.equal(durationUnitHPeriods(DurationUnit.Pulse), 10n ** 11n);
  });

  it("epoch h periods", () => {
    assert.equal(durationUnitHPeriods(DurationUnit.Epoch), 10n ** 17n);
  });
});

describe("Duration", () => {
  it("to seconds", () => {
    const d = new Duration(1, DurationUnit.Pulse);
    assert.ok(d.toSeconds() > 0);
  });

  it("from seconds", () => {
    const secs = durationUnitSeconds(DurationUnit.Wave);
    const d = Duration.fromSeconds(secs, DurationUnit.Wave);
    assert.ok(Math.abs(d.count - 1.0) < 0.01);
  });

  it("to quants", () => {
    const d = new Duration(1, DurationUnit.Pulse);
    assert.ok(d.toQuants().value > 0n);
  });

  it("convert between units", () => {
    const d = new Duration(1, DurationUnit.Wave);
    const d2 = d.convertTo(DurationUnit.Pulse);
    assert.ok(Math.abs(d2.count - 10.0) < 0.01);
  });

  it("toString singular", () => {
    const d = new Duration(1, DurationUnit.Pulse);
    assert.equal(d.toString(), "1 Pulse");
  });

  it("toString plural", () => {
    const d = new Duration(3, DurationUnit.Spin);
    assert.equal(d.toString(), "3.00 Spins");
  });
});

describe("bestUnitForSeconds", () => {
  it("small seconds -> Pulse", () => {
    assert.equal(bestUnitForSeconds(100), DurationUnit.Pulse);
  });

  it("larger seconds get larger units", () => {
    const unit = bestUnitForSeconds(1e10);
    assert.ok(unit >= DurationUnit.Tide);
  });
});
