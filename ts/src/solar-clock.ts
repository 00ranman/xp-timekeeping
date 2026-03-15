import { ARCS_PER_LOOP, LOOPS_PER_DAY, TICKS_PER_ARC, TICKS_PER_DAY } from "./constants";
import { InvalidSolarTimeError, ParseError } from "./error";

/**
 * Solar Clock time — a position in the local solar day.
 *
 * Display format: `t:L:AA:TT`
 * - L = loop (0-9)
 * - AA = arc (00-99)
 * - TT = tick (00-99)
 *
 * Loop 0 = local solar midnight, Loop 5 = local solar noon.
 */
export class SolarTime {
  private readonly _loop: number;
  private readonly _arc: number;
  private readonly _tick: number;

  private constructor(loopVal: number, arc: number, tick: number) {
    this._loop = loopVal;
    this._arc = arc;
    this._tick = tick;
  }

  static new(loopVal: number, arc: number, tick: number): SolarTime {
    if (loopVal < 0 || loopVal >= LOOPS_PER_DAY) {
      throw new InvalidSolarTimeError("loop", loopVal, LOOPS_PER_DAY - 1);
    }
    if (arc < 0 || arc >= ARCS_PER_LOOP) {
      throw new InvalidSolarTimeError("arc", arc, ARCS_PER_LOOP - 1);
    }
    if (tick < 0 || tick >= TICKS_PER_ARC) {
      throw new InvalidSolarTimeError("tick", tick, TICKS_PER_ARC - 1);
    }
    return new SolarTime(loopVal, arc, tick);
  }

  static fromTotalTicks(total: number): SolarTime {
    if (total < 0 || total >= TICKS_PER_DAY) {
      throw new InvalidSolarTimeError("total_ticks", total, TICKS_PER_DAY - 1);
    }
    const loopVal = Math.floor(total / (ARCS_PER_LOOP * TICKS_PER_ARC));
    const remainder = total % (ARCS_PER_LOOP * TICKS_PER_ARC);
    const arc = Math.floor(remainder / TICKS_PER_ARC);
    const tick = remainder % TICKS_PER_ARC;
    return new SolarTime(loopVal, arc, tick);
  }

  toTotalTicks(): number {
    return this._loop * ARCS_PER_LOOP * TICKS_PER_ARC + this._arc * TICKS_PER_ARC + this._tick;
  }

  loopVal(): number {
    return this._loop;
  }

  arc(): number {
    return this._arc;
  }

  tick(): number {
    return this._tick;
  }

  isMidnight(): boolean {
    return this.toTotalTicks() === 0;
  }

  isNoon(): boolean {
    return this._loop === 5 && this._arc === 0 && this._tick === 0;
  }

  dayFraction(): number {
    return this.toTotalTicks() / TICKS_PER_DAY;
  }

  static fromDayFraction(fraction: number): SolarTime {
    const total = Math.round(fraction * TICKS_PER_DAY);
    const clamped = Math.min(total, TICKS_PER_DAY - 1);
    return SolarTime.fromTotalTicks(clamped);
  }

  static parse(s: string): SolarTime {
    const trimmed = s.trim();
    if (!trimmed.startsWith("t:")) {
      throw new ParseError(`solar time must start with 't:', got '${trimmed}'`);
    }
    const rest = trimmed.slice(2);
    const parts = rest.split(":");
    if (parts.length !== 3) {
      throw new ParseError(`expected format t:L:AA:TT, got '${trimmed}'`);
    }
    const loopVal = parseInt(parts[0], 10);
    const arc = parseInt(parts[1], 10);
    const tick = parseInt(parts[2], 10);
    if (isNaN(loopVal)) throw new ParseError(`invalid loop: '${parts[0]}'`);
    if (isNaN(arc)) throw new ParseError(`invalid arc: '${parts[1]}'`);
    if (isNaN(tick)) throw new ParseError(`invalid tick: '${parts[2]}'`);
    return SolarTime.new(loopVal, arc, tick);
  }

  toString(): string {
    return `t:${this._loop}:${String(this._arc).padStart(2, "0")}:${String(this._tick).padStart(2, "0")}`;
  }

  equals(other: SolarTime): boolean {
    return this._loop === other._loop && this._arc === other._arc && this._tick === other._tick;
  }
}
