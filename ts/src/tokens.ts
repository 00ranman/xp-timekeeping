import { TokenError } from "./error";

/** The 7-prime token economy. */
export enum TokenType {
  XP = 0,
  CT = 1,
  CAT = 2,
  IT = 3,
  DT = 4,
  EP = 5,
  GP = 6,
}

const TOKEN_PRIMES: Record<TokenType, number> = {
  [TokenType.XP]: 2,
  [TokenType.CT]: 3,
  [TokenType.CAT]: 5,
  [TokenType.IT]: 7,
  [TokenType.DT]: 11,
  [TokenType.EP]: 13,
  [TokenType.GP]: 17,
};

const TOKEN_ABBREVIATIONS: Record<TokenType, string> = {
  [TokenType.XP]: "XP",
  [TokenType.CT]: "CT",
  [TokenType.CAT]: "CAT",
  [TokenType.IT]: "IT",
  [TokenType.DT]: "DT",
  [TokenType.EP]: "EP",
  [TokenType.GP]: "GP",
};

const TOKEN_FULL_NAMES: Record<TokenType, string> = {
  [TokenType.XP]: "Experience Points",
  [TokenType.CT]: "Contribution Tokens",
  [TokenType.CAT]: "Catalyst Tokens",
  [TokenType.IT]: "Insight Tokens",
  [TokenType.DT]: "Decay Tokens",
  [TokenType.EP]: "Entropy Points",
  [TokenType.GP]: "Governance Points",
};

const TOKEN_DESCRIPTIONS: Record<TokenType, string> = {
  [TokenType.XP]: "Validated entropy reduction",
  [TokenType.CT]: "Domain-specific work units",
  [TokenType.CAT]: "System facilitation rewards",
  [TokenType.IT]: "Knowledge and discovery",
  [TokenType.DT]: "Temporal depreciation",
  [TokenType.EP]: "Raw entropy measurements",
  [TokenType.GP]: "Governance participation",
};

export function tokenPrime(tt: TokenType): number {
  return TOKEN_PRIMES[tt];
}

export function tokenAbbreviation(tt: TokenType): string {
  return TOKEN_ABBREVIATIONS[tt];
}

export function tokenFullName(tt: TokenType): string {
  return TOKEN_FULL_NAMES[tt];
}

export function tokenDescription(tt: TokenType): string {
  return TOKEN_DESCRIPTIONS[tt];
}

export function allTokenTypes(): TokenType[] {
  return [TokenType.XP, TokenType.CT, TokenType.CAT, TokenType.IT, TokenType.DT, TokenType.EP, TokenType.GP];
}

export function tokenTypeFromPrime(prime: number): TokenType {
  for (const tt of allTokenTypes()) {
    if (TOKEN_PRIMES[tt] === prime) return tt;
  }
  throw new TokenError(`no token type for prime ${prime}`);
}

export function tokenTypeFromAbbreviation(abbr: string): TokenType {
  const upper = abbr.toUpperCase();
  for (const tt of allTokenTypes()) {
    if (TOKEN_ABBREVIATIONS[tt] === upper) return tt;
  }
  throw new TokenError(`unknown token abbreviation '${abbr}'`);
}

/** A token amount: a quantity of a specific token type. */
export class TokenAmount {
  readonly tokenType: TokenType;
  readonly amount: number;

  constructor(tokenType: TokenType, amount: number) {
    this.tokenType = tokenType;
    this.amount = amount;
  }

  primeEncoding(): bigint {
    const prime = BigInt(tokenPrime(this.tokenType));
    const count = Math.round(this.amount);
    return prime ** BigInt(count);
  }

  toString(): string {
    return `${this.amount.toFixed(2)} ${tokenAbbreviation(this.tokenType)}`;
  }
}

/** Encode a set of token amounts as a single prime-factored product. */
export function encodeTokenSet(amounts: TokenAmount[]): bigint {
  let product = 1n;
  for (const ta of amounts) {
    const prime = BigInt(tokenPrime(ta.tokenType));
    const count = Math.round(ta.amount);
    product *= prime ** BigInt(count);
  }
  return product;
}

/** Decode a prime-factored token value into individual token amounts. */
export function decodeTokenSet(value: bigint): TokenAmount[] {
  const result: TokenAmount[] = [];
  let remaining = value;
  for (const tt of allTokenTypes()) {
    const prime = BigInt(tokenPrime(tt));
    let count = 0;
    while (remaining % prime === 0n && remaining > 0n) {
      remaining /= prime;
      count++;
    }
    if (count > 0) {
      result.push(new TokenAmount(tt, count));
    }
  }
  return result;
}
