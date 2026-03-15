export class UtError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "UtError";
  }
}

export class InvalidSolarTimeError extends UtError {
  constructor(
    public readonly field: string,
    public readonly value: number,
    public readonly max: number,
  ) {
    super(`invalid solar time: ${field}=${value}, max=${max}`);
    this.name = "InvalidSolarTimeError";
  }
}

export class ParseError extends UtError {
  constructor(message: string) {
    super(message);
    this.name = "ParseError";
  }
}

export class DurationOverflowError extends UtError {
  constructor() {
    super("duration overflow");
    this.name = "DurationOverflowError";
  }
}

export class QuantOverflowError extends UtError {
  constructor() {
    super("quant overflow");
    this.name = "QuantOverflowError";
  }
}

export class InvalidDateError extends UtError {
  constructor(public readonly reason: string) {
    super(`invalid date: ${reason}`);
    this.name = "InvalidDateError";
  }
}

export class UnknownPlanetError extends UtError {
  constructor(public readonly planet: string) {
    super(`unknown planet: ${planet}`);
    this.name = "UnknownPlanetError";
  }
}

export class DagError extends UtError {
  constructor(message: string) {
    super(message);
    this.name = "DagError";
  }
}

export class TokenError extends UtError {
  constructor(message: string) {
    super(message);
    this.name = "TokenError";
  }
}
