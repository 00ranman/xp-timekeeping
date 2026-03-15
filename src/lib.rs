//! # Universal Times v4.0
//!
//! Hydrogen-anchored dual-system temporal infrastructure for multi-planetary timekeeping.
//!
//! This crate implements the full Universal Times v4.0 specification:
//!
//! - **System 1 — Solar Clock**: Planet-specific local time using Loop/Arc/Tick coordinates
//! - **System 2 — Universal Duration**: Planet-independent elapsed time (Pulse through Epoch)
//! - **System 3 — Quant Accumulator**: Universal timestamps counting hydrogen hyperfine periods
//! - **5-Layer Temporal Architecture**: From physical substrate to human display
//! - **Planet Profiles**: Earth and Mars with full calendar support
//! - **XP Formula**: Entropy-based value calculation
//! - **DAG Temporal Substrate**: Directed acyclic graph for causal ordering
//! - **7-Prime Token Economy**: Seven token types mapped to prime numbers
//!
//! ## Physical Anchor
//!
//! All time measurements are anchored to the hydrogen-1 ground-state hyperfine transition
//! at 1,420,405,751.768 Hz. Each period is one "quant" — the smallest named unit.

pub mod calendar;
pub mod constants;
pub mod convert;
pub mod dag;
pub mod duration;
pub mod error;
pub mod layers;
pub mod planet;
pub mod quant;
pub mod solar_clock;
pub mod tokens;
pub mod xp;

// Re-export primary types for convenience.
pub use calendar::{CalendarConfig, CalendarDate};
pub use constants::HYDROGEN_HYPERFINE_FREQ;
pub use convert::{
    earth_solar_time_to_iso8601, gregorian_to_ut_calendar, iso8601_to_earth_solar_time,
    legacy_time_to_solar_time, solar_time_to_legacy_time, unix_to_earth_solar_time,
    ut_calendar_to_gregorian,
};
pub use dag::{TemporalDag, TemporalEdge, TemporalNode};
pub use duration::{Duration, DurationUnit};
pub use error::{UtError, UtResult};
pub use layers::{LayerMutability, TemporalLayer};
pub use planet::{Earth, Mars, PlanetProfile};
pub use quant::{Quant, QuantAccumulator};
pub use solar_clock::SolarTime;
pub use tokens::{TokenAmount, TokenType};
pub use xp::{xp_operational, xp_philosophical, XpOperationalParams};
