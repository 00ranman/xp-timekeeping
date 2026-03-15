import { describe, it } from "node:test";
import * as assert from "node:assert/strict";
import { SolarTime } from "../src/solar-clock";
import { InvalidSolarTimeError, ParseError } from "../src/error";

describe("SolarTime", () => {
  it("midnight", () => {
    const t = SolarTime.new(0, 0, 0);
    assert.ok(t.isMidnight());
    assert.equal(t.toTotalTicks(), 0);
  });

  it("noon", () => {
    const t = SolarTime.new(5, 0, 0);
    assert.ok(t.isNoon());
    assert.equal(t.toTotalTicks(), 50_000);
  });

  it("components", () => {
    const t = SolarTime.new(3, 45, 67);
    assert.equal(t.loopVal(), 3);
    assert.equal(t.arc(), 45);
    assert.equal(t.tick(), 67);
  });

  it("total ticks", () => {
    const t = SolarTime.new(3, 45, 67);
    assert.equal(t.toTotalTicks(), 3 * 10000 + 45 * 100 + 67);
  });

  it("from total ticks", () => {
    const t = SolarTime.fromTotalTicks(34567);
    assert.equal(t.loopVal(), 3);
    assert.equal(t.arc(), 45);
    assert.equal(t.tick(), 67);
  });

  it("from day fraction", () => {
    const t = SolarTime.fromDayFraction(0.5);
    assert.ok(t.isNoon());
  });

  it("day fraction roundtrip", () => {
    const t = SolarTime.new(7, 25, 50);
    const frac = t.dayFraction();
    const t2 = SolarTime.fromDayFraction(frac);
    assert.ok(t.equals(t2));
  });

  it("parse valid", () => {
    const t = SolarTime.parse("t:3:45:67");
    assert.equal(t.loopVal(), 3);
    assert.equal(t.arc(), 45);
    assert.equal(t.tick(), 67);
  });

  it("parse midnight", () => {
    const t = SolarTime.parse("t:0:00:00");
    assert.ok(t.isMidnight());
  });

  it("toString", () => {
    const t = SolarTime.new(3, 5, 7);
    assert.equal(t.toString(), "t:3:05:07");
  });

  it("parse roundtrip", () => {
    const original = SolarTime.new(9, 99, 99);
    const parsed = SolarTime.parse(original.toString());
    assert.ok(original.equals(parsed));
  });

  it("invalid loop", () => {
    assert.throws(() => SolarTime.new(10, 0, 0), InvalidSolarTimeError);
  });

  it("invalid arc", () => {
    assert.throws(() => SolarTime.new(0, 100, 0), InvalidSolarTimeError);
  });

  it("invalid tick", () => {
    assert.throws(() => SolarTime.new(0, 0, 100), InvalidSolarTimeError);
  });

  it("invalid total ticks", () => {
    assert.throws(() => SolarTime.fromTotalTicks(100_000), InvalidSolarTimeError);
  });

  it("invalid total ticks negative", () => {
    assert.throws(() => SolarTime.fromTotalTicks(-1), InvalidSolarTimeError);
  });

  it("parse bad prefix", () => {
    assert.throws(() => SolarTime.parse("x:1:00:00"), ParseError);
  });

  it("parse missing parts", () => {
    assert.throws(() => SolarTime.parse("t:1:00"), ParseError);
  });

  it("equals", () => {
    const a = SolarTime.new(1, 2, 3);
    const b = SolarTime.new(1, 2, 3);
    const c = SolarTime.new(1, 2, 4);
    assert.ok(a.equals(b));
    assert.ok(!a.equals(c));
  });
});
