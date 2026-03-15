import { describe, it } from "node:test";
import * as assert from "node:assert/strict";
import {
  TokenType,
  tokenPrime,
  tokenAbbreviation,
  tokenFullName,
  tokenDescription,
  allTokenTypes,
  tokenTypeFromPrime,
  tokenTypeFromAbbreviation,
  TokenAmount,
  encodeTokenSet,
  decodeTokenSet,
} from "../src/tokens";
import { TokenError } from "../src/error";

describe("TokenType", () => {
  it("seven types", () => {
    assert.equal(allTokenTypes().length, 7);
  });

  it("primes", () => {
    assert.equal(tokenPrime(TokenType.XP), 2);
    assert.equal(tokenPrime(TokenType.CT), 3);
    assert.equal(tokenPrime(TokenType.CAT), 5);
    assert.equal(tokenPrime(TokenType.IT), 7);
    assert.equal(tokenPrime(TokenType.DT), 11);
    assert.equal(tokenPrime(TokenType.EP), 13);
    assert.equal(tokenPrime(TokenType.GP), 17);
  });

  it("abbreviations", () => {
    assert.equal(tokenAbbreviation(TokenType.XP), "XP");
    assert.equal(tokenAbbreviation(TokenType.CT), "CT");
    assert.equal(tokenAbbreviation(TokenType.GP), "GP");
  });

  it("full names", () => {
    assert.equal(tokenFullName(TokenType.XP), "Experience Points");
    assert.equal(tokenFullName(TokenType.GP), "Governance Points");
  });

  it("descriptions non-empty", () => {
    for (const tt of allTokenTypes()) {
      assert.ok(tokenDescription(tt).length > 0);
    }
  });

  it("from prime", () => {
    assert.equal(tokenTypeFromPrime(2), TokenType.XP);
    assert.equal(tokenTypeFromPrime(17), TokenType.GP);
  });

  it("from prime invalid throws", () => {
    assert.throws(() => tokenTypeFromPrime(4), TokenError);
  });

  it("from abbreviation", () => {
    assert.equal(tokenTypeFromAbbreviation("XP"), TokenType.XP);
    assert.equal(tokenTypeFromAbbreviation("gp"), TokenType.GP);
  });

  it("from abbreviation invalid throws", () => {
    assert.throws(() => tokenTypeFromAbbreviation("NOPE"), TokenError);
  });
});

describe("TokenAmount", () => {
  it("constructor", () => {
    const ta = new TokenAmount(TokenType.XP, 10);
    assert.equal(ta.tokenType, TokenType.XP);
    assert.equal(ta.amount, 10);
  });

  it("prime encoding", () => {
    const ta = new TokenAmount(TokenType.XP, 3);
    assert.equal(ta.primeEncoding(), 8n); // 2^3 = 8
  });

  it("toString", () => {
    const ta = new TokenAmount(TokenType.CT, 5);
    assert.equal(ta.toString(), "5.00 CT");
  });
});

describe("encodeTokenSet / decodeTokenSet", () => {
  it("single token roundtrip", () => {
    const amounts = [new TokenAmount(TokenType.XP, 3)];
    const encoded = encodeTokenSet(amounts);
    const decoded = decodeTokenSet(encoded);
    assert.equal(decoded.length, 1);
    assert.equal(decoded[0].tokenType, TokenType.XP);
    assert.equal(decoded[0].amount, 3);
  });

  it("multiple tokens roundtrip", () => {
    const amounts = [
      new TokenAmount(TokenType.XP, 2),
      new TokenAmount(TokenType.CT, 3),
      new TokenAmount(TokenType.GP, 1),
    ];
    const encoded = encodeTokenSet(amounts);
    const decoded = decodeTokenSet(encoded);

    const xp = decoded.find((d) => d.tokenType === TokenType.XP);
    const ct = decoded.find((d) => d.tokenType === TokenType.CT);
    const gp = decoded.find((d) => d.tokenType === TokenType.GP);
    assert.equal(xp!.amount, 2);
    assert.equal(ct!.amount, 3);
    assert.equal(gp!.amount, 1);
  });

  it("encode empty set is 1", () => {
    assert.equal(encodeTokenSet([]), 1n);
  });

  it("known value", () => {
    // 2^2 * 3^1 = 4 * 3 = 12
    const encoded = encodeTokenSet([
      new TokenAmount(TokenType.XP, 2),
      new TokenAmount(TokenType.CT, 1),
    ]);
    assert.equal(encoded, 12n);
  });
});
