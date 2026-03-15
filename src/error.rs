use std::fmt;

/// Errors that can occur in Universal Times operations.
#[derive(Debug, Clone, PartialEq)]
pub enum UtError {
    /// Solar clock value out of valid range.
    InvalidSolarTime { field: &'static str, value: u32, max: u32 },
    /// Failed to parse a solar time display string.
    ParseError(String),
    /// Duration conversion produced an out-of-range value.
    DurationOverflow,
    /// Quant accumulator arithmetic overflow.
    QuantOverflow,
    /// Invalid calendar date.
    InvalidDate { reason: String },
    /// Unknown planet identifier.
    UnknownPlanet(String),
    /// Invalid DAG operation.
    DagError(String),
    /// Invalid token operation.
    TokenError(String),
}

impl fmt::Display for UtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSolarTime { field, value, max } => {
                write!(f, "invalid solar time: {field} = {value}, max = {max}")
            }
            Self::ParseError(msg) => write!(f, "parse error: {msg}"),
            Self::DurationOverflow => write!(f, "duration overflow"),
            Self::QuantOverflow => write!(f, "quant accumulator overflow"),
            Self::InvalidDate { reason } => write!(f, "invalid date: {reason}"),
            Self::UnknownPlanet(name) => write!(f, "unknown planet: {name}"),
            Self::DagError(msg) => write!(f, "DAG error: {msg}"),
            Self::TokenError(msg) => write!(f, "token error: {msg}"),
        }
    }
}

impl std::error::Error for UtError {}

pub type UtResult<T> = Result<T, UtError>;
