import { describe, it } from "node:test";
import * as assert from "node:assert/strict";
import { Quant, QuantAccumulator } from "../src/quant";

describe("Quant", () => {
  it("zero", () => {
    const q = Quant.zero();
    assert.equal(q.value, 0n);
    assert.equal(q.count(), 0n);
  });

  it("from value", () => {
    const q = new Quant(42n);
    assert.equal(q.value, 42n);
  });

  it("from seconds", () => {
    const q = Quant.fromSeconds(1.0);
    assert.ok(q.value > 1_000_000_000n);
  });

  it("to seconds roundtrip", () => {
    const q = Quant.fromSeconds(100.0);
    assert.ok(Math.abs(q.toSeconds() - 100.0) < 0.001);
  });

  it("add", () => {
    const a = new Quant(10n);
    const b = new Quant(20n);
    assert.equal(a.add(b).value, 30n);
  });

  it("sub", () => {
    const a = new Quant(30n);
    const b = new Quant(10n);
    assert.equal(a.sub(b).value, 20n);
  });

  it("abs diff", () => {
    const a = new Quant(10n);
    const b = new Quant(30n);
    assert.equal(a.absDiff(b).value, 20n);
    assert.equal(b.absDiff(a).value, 20n);
  });

  it("toString", () => {
    const q = new Quant(42n);
    assert.equal(q.toString(), "Q:42");
  });
});

describe("QuantAccumulator", () => {
  it("starts at zero", () => {
    const acc = new QuantAccumulator();
    assert.equal(acc.current().value, 0n);
    assert.equal(acc.epochOrigin().value, 0n);
  });

  it("advance", () => {
    const acc = new QuantAccumulator();
    acc.advance(new Quant(100n));
    assert.equal(acc.current().value, 100n);
  });

  it("advance seconds", () => {
    const acc = new QuantAccumulator();
    acc.advanceSeconds(1.0);
    assert.ok(acc.current().value > 0n);
  });

  it("elapsed", () => {
    const acc = new QuantAccumulator();
    acc.advance(new Quant(500n));
    assert.equal(acc.elapsed().value, 500n);
  });

  it("elapsed seconds", () => {
    const acc = new QuantAccumulator();
    acc.advanceSeconds(60.0);
    assert.ok(Math.abs(acc.elapsedSeconds() - 60.0) < 0.01);
  });

  it("from quant", () => {
    const acc = QuantAccumulator.fromQuant(new Quant(1000n));
    assert.equal(acc.current().value, 1000n);
  });

  it("from unix timestamp", () => {
    const acc = QuantAccumulator.fromUnixTimestamp(0, Quant.zero());
    assert.equal(acc.current().value, 0n);
  });
});
