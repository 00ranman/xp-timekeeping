use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{UtError, UtResult};

/// The 7-prime token economy.
///
/// Seven token types, each mapped to a unique prime number.
/// Prime factorization ensures token types never collide and operations
/// are mathematically clean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenType {
    /// XP (Experience Points) — validated entropy reduction.
    XP = 0,
    /// CT (Contribution Tokens) — domain-specific work units.
    CT = 1,
    /// CAT (Catalyst Tokens) — system facilitation rewards.
    CAT = 2,
    /// IT (Insight Tokens) — knowledge and discovery.
    IT = 3,
    /// DT (Decay Tokens) — temporal depreciation.
    DT = 4,
    /// EP (Entropy Points) — raw entropy measurements.
    EP = 5,
    /// GP (Governance Points) — governance participation.
    GP = 6,
}

impl TokenType {
    /// The prime number associated with this token type.
    pub fn prime(self) -> u64 {
        match self {
            Self::XP => 2,
            Self::CT => 3,
            Self::CAT => 5,
            Self::IT => 7,
            Self::DT => 11,
            Self::EP => 13,
            Self::GP => 17,
        }
    }

    /// Short abbreviation of this token type.
    pub fn abbreviation(self) -> &'static str {
        match self {
            Self::XP => "XP",
            Self::CT => "CT",
            Self::CAT => "CAT",
            Self::IT => "IT",
            Self::DT => "DT",
            Self::EP => "EP",
            Self::GP => "GP",
        }
    }

    /// Full name of this token type.
    pub fn full_name(self) -> &'static str {
        match self {
            Self::XP => "Experience Points",
            Self::CT => "Contribution Tokens",
            Self::CAT => "Catalyst Tokens",
            Self::IT => "Insight Tokens",
            Self::DT => "Decay Tokens",
            Self::EP => "Entropy Points",
            Self::GP => "Governance Points",
        }
    }

    /// Description of the token's purpose.
    pub fn description(self) -> &'static str {
        match self {
            Self::XP => "Validated entropy reduction",
            Self::CT => "Domain-specific work units",
            Self::CAT => "System facilitation rewards",
            Self::IT => "Knowledge and discovery",
            Self::DT => "Temporal depreciation",
            Self::EP => "Raw entropy measurements",
            Self::GP => "Governance participation",
        }
    }

    /// All token types in order.
    pub fn all() -> [TokenType; 7] {
        [
            Self::XP,
            Self::CT,
            Self::CAT,
            Self::IT,
            Self::DT,
            Self::EP,
            Self::GP,
        ]
    }

    /// Look up a token type by its prime number.
    pub fn from_prime(prime: u64) -> UtResult<Self> {
        match prime {
            2 => Ok(Self::XP),
            3 => Ok(Self::CT),
            5 => Ok(Self::CAT),
            7 => Ok(Self::IT),
            11 => Ok(Self::DT),
            13 => Ok(Self::EP),
            17 => Ok(Self::GP),
            _ => Err(UtError::TokenError(format!(
                "no token type for prime {prime}"
            ))),
        }
    }

    /// Look up a token type by abbreviation.
    pub fn from_abbreviation(abbr: &str) -> UtResult<Self> {
        match abbr.to_uppercase().as_str() {
            "XP" => Ok(Self::XP),
            "CT" => Ok(Self::CT),
            "CAT" => Ok(Self::CAT),
            "IT" => Ok(Self::IT),
            "DT" => Ok(Self::DT),
            "EP" => Ok(Self::EP),
            "GP" => Ok(Self::GP),
            _ => Err(UtError::TokenError(format!(
                "unknown token abbreviation '{abbr}'"
            ))),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (p={})", self.abbreviation(), self.prime())
    }
}

/// A token amount: a quantity of a specific token type.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TokenAmount {
    pub token_type: TokenType,
    pub amount: f64,
}

impl TokenAmount {
    /// Create a new token amount.
    pub fn new(token_type: TokenType, amount: f64) -> Self {
        Self { token_type, amount }
    }

    /// Encode this token amount as a prime-factored value.
    /// The encoding is: prime^(amount as integer) for integer amounts.
    pub fn prime_encoding(&self) -> u128 {
        let prime = self.token_type.prime() as u128;
        let count = self.amount.round() as u32;
        prime.pow(count)
    }
}

impl fmt::Display for TokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} {}", self.amount, self.token_type.abbreviation())
    }
}

/// Encode a set of token amounts as a single prime-factored product.
/// Each token type contributes: prime_i ^ amount_i.
/// This ensures different token combinations never collide.
pub fn encode_token_set(amounts: &[TokenAmount]) -> u128 {
    let mut product: u128 = 1;
    for ta in amounts {
        let prime = ta.token_type.prime() as u128;
        let count = ta.amount.round() as u32;
        product = product.saturating_mul(prime.pow(count));
    }
    product
}

/// Decode a prime-factored token value into individual token amounts.
pub fn decode_token_set(mut value: u128) -> Vec<TokenAmount> {
    let mut result = Vec::new();
    for tt in TokenType::all() {
        let prime = tt.prime() as u128;
        let mut count = 0u32;
        while value % prime == 0 && value > 0 {
            value /= prime;
            count += 1;
        }
        if count > 0 {
            result.push(TokenAmount::new(tt, count as f64));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_primes() {
        assert_eq!(TokenType::XP.prime(), 2);
        assert_eq!(TokenType::CT.prime(), 3);
        assert_eq!(TokenType::CAT.prime(), 5);
        assert_eq!(TokenType::IT.prime(), 7);
        assert_eq!(TokenType::DT.prime(), 11);
        assert_eq!(TokenType::EP.prime(), 13);
        assert_eq!(TokenType::GP.prime(), 17);
    }

    #[test]
    fn test_all_primes_are_prime() {
        fn is_prime(n: u64) -> bool {
            if n < 2 { return false; }
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 { return false; }
                i += 1;
            }
            true
        }
        for tt in TokenType::all() {
            assert!(is_prime(tt.prime()), "{} prime {} is not prime", tt, tt.prime());
        }
    }

    #[test]
    fn test_all_primes_unique() {
        let primes: Vec<u64> = TokenType::all().iter().map(|t| t.prime()).collect();
        for i in 0..primes.len() {
            for j in (i + 1)..primes.len() {
                assert_ne!(primes[i], primes[j]);
            }
        }
    }

    #[test]
    fn test_from_prime() {
        assert_eq!(TokenType::from_prime(2).unwrap(), TokenType::XP);
        assert_eq!(TokenType::from_prime(17).unwrap(), TokenType::GP);
        assert!(TokenType::from_prime(4).is_err());
    }

    #[test]
    fn test_from_abbreviation() {
        assert_eq!(TokenType::from_abbreviation("XP").unwrap(), TokenType::XP);
        assert_eq!(TokenType::from_abbreviation("cat").unwrap(), TokenType::CAT);
        assert!(TokenType::from_abbreviation("INVALID").is_err());
    }

    #[test]
    fn test_token_amount_display() {
        let ta = TokenAmount::new(TokenType::XP, 42.5);
        assert_eq!(format!("{ta}"), "42.50 XP");
    }

    #[test]
    fn test_prime_encoding_single() {
        // 3 XP tokens → 2^3 = 8
        let ta = TokenAmount::new(TokenType::XP, 3.0);
        assert_eq!(ta.prime_encoding(), 8);
    }

    #[test]
    fn test_encode_decode_round_trip() {
        let amounts = vec![
            TokenAmount::new(TokenType::XP, 2.0),  // 2^2 = 4
            TokenAmount::new(TokenType::CT, 1.0),   // 3^1 = 3
        ];
        let encoded = encode_token_set(&amounts);
        assert_eq!(encoded, 12); // 4 * 3 = 12

        let decoded = decode_token_set(encoded);
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].token_type, TokenType::XP);
        assert!((decoded[0].amount - 2.0).abs() < 1e-10);
        assert_eq!(decoded[1].token_type, TokenType::CT);
        assert!((decoded[1].amount - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_encode_no_collision() {
        // Different combinations should produce different encodings
        let a = encode_token_set(&[TokenAmount::new(TokenType::XP, 3.0)]);
        let b = encode_token_set(&[TokenAmount::new(TokenType::CT, 2.0)]);
        assert_ne!(a, b);
    }

    #[test]
    fn test_token_count() {
        assert_eq!(TokenType::all().len(), 7);
    }
}
