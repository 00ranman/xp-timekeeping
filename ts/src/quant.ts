import { HYDROGEN_HYPERFINE_FREQ, ONE_QUANT_SECONDS } from "./constants";

/**
 * A single quant value — one hydrogen-1 hyperfine period count.
 *
 * Quants are the fundamental unit of Universal Times. Each quant represents
 * one period of the hydrogen-1 ground-state hyperfine transition at
 * 1,420,405,751.768 Hz.
 */
export class Quant {
  readonly value: bigint;

  constructor(count: bigint) {
    this.value = count;
  }

  static zero(): Quant {
    return new Quant(0n);
  }

  static fromSeconds(seconds: number): Quant {
    return new Quant(BigInt(Math.floor(seconds * HYDROGEN_HYPERFINE_FREQ)));
  }

  toSeconds(): number {
    return Number(this.value) * ONE_QUANT_SECONDS;
  }

  count(): bigint {
    return this.value;
  }

  add(other: Quant): Quant {
    return new Quant(this.value + other.value);
  }

  sub(other: Quant): Quant {
    return new Quant(this.value - other.value);
  }

  absDiff(other: Quant): Quant {
    return this.value >= other.value
      ? new Quant(this.value - other.value)
      : new Quant(other.value - this.value);
  }

  toString(): string {
    return `Q:${this.value}`;
  }
}

/**
 * The Quant Accumulator — a monotonically increasing counter of hydrogen
 * hyperfine periods since the epoch origin (t:0).
 */
export class QuantAccumulator {
  private _current: Quant;
  private readonly _epochOrigin: Quant;

  constructor() {
    this._current = Quant.zero();
    this._epochOrigin = Quant.zero();
  }

  static fromQuant(q: Quant): QuantAccumulator {
    const acc = new QuantAccumulator();
    (acc as any)._current = q;
    return acc;
  }

  static fromUnixTimestamp(unixSecs: number, quantAtUnixEpoch: Quant): QuantAccumulator {
    const elapsedQuants = Quant.fromSeconds(unixSecs);
    const acc = new QuantAccumulator();
    (acc as any)._current = new Quant(quantAtUnixEpoch.value + elapsedQuants.value);
    return acc;
  }

  current(): Quant {
    return this._current;
  }

  epochOrigin(): Quant {
    return this._epochOrigin;
  }

  advance(delta: Quant): void {
    this._current = this._current.add(delta);
  }

  advanceSeconds(seconds: number): void {
    this.advance(Quant.fromSeconds(seconds));
  }

  elapsed(): Quant {
    return new Quant(this._current.value - this._epochOrigin.value);
  }

  elapsedSeconds(): number {
    return this.elapsed().toSeconds();
  }
}
