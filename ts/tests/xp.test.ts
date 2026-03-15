import { describe, it } from "node:test";
import * as assert from "node:assert/strict";
import {
  xpPhilosophical,
  xpOperational,
  temporalDecay,
  temporalDecayQuants,
} from "../src/xp";

describe("xpPhilosophical", () => {
  it("basic formula S * c_L^2", () => {
    assert.equal(xpPhilosophical(10, 3), 90);
  });

  it("zero entropy returns zero", () => {
    assert.equal(xpPhilosophical(0, 100), 0);
  });

  it("zero speed returns zero", () => {
    assert.equal(xpPhilosophical(100, 0), 0);
  });

  it("unit values", () => {
    assert.equal(xpPhilosophical(1, 1), 1);
  });
});

describe("xpOperational", () => {
  it("product of factors", () => {
    const result = xpOperational({
      base: 2,
      domain: 3,
      temporalDecay: 4,
      validation: 5,
      scarcity: 6,
    });
    assert.equal(result, 2 * 3 * 4 * 5 * 6);
  });

  it("any zero factor returns zero", () => {
    const result = xpOperational({
      base: 100,
      domain: 0,
      temporalDecay: 1,
      validation: 1,
      scarcity: 1,
    });
    assert.equal(result, 0);
  });
});

describe("temporalDecay", () => {
  it("zero elapsed returns 1", () => {
    assert.ok(Math.abs(temporalDecay(0, 0.01) - 1.0) < 1e-10);
  });

  it("decays over time", () => {
    const d1 = temporalDecay(100, 0.01);
    const d2 = temporalDecay(200, 0.01);
    assert.ok(d1 > d2);
    assert.ok(d1 > 0 && d1 < 1);
  });

  it("higher rate decays faster", () => {
    const slow = temporalDecay(100, 0.001);
    const fast = temporalDecay(100, 0.01);
    assert.ok(slow > fast);
  });
});

describe("temporalDecayQuants", () => {
  it("zero elapsed returns 1", () => {
    assert.ok(Math.abs(temporalDecayQuants(0n, 1e-15) - 1.0) < 1e-10);
  });

  it("decays over quants", () => {
    const d = temporalDecayQuants(1_000_000_000_000n, 1e-12);
    assert.ok(d > 0 && d < 1);
  });
});
