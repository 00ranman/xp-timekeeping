use serde::{Deserialize, Serialize};
use std::fmt;

/// The 5-layer temporal architecture.
///
/// Each layer depends only on lower layers:
/// - Layer 0 (Epoch): Shared origin point (t:0) — immutable
/// - Layer 1 (Quant): Physical substrate (hydrogen periods) — immutable (physics)
/// - Layer 2 (Universal Duration): Planet-independent elapsed time — immutable
/// - Layer 3 (Solar Clock): Planet-specific position-in-day — per planet profile
/// - Layer 4 (Display): Human-facing rendering — customizable
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TemporalLayer {
    /// Layer 0: The shared epoch origin point.
    Epoch = 0,
    /// Layer 1: Physical substrate — hydrogen hyperfine period counting.
    Quant = 1,
    /// Layer 2: Planet-independent elapsed-time units.
    UniversalDuration = 2,
    /// Layer 3: Planet-specific position-in-day coordinates.
    SolarClock = 3,
    /// Layer 4: Human-facing rendering (clock faces, calendars, etc.).
    Display = 4,
}

impl TemporalLayer {
    /// Get the layer number (0-4).
    pub fn number(self) -> u8 {
        self as u8
    }

    /// Get the canonical name of this layer.
    pub fn name(self) -> &'static str {
        match self {
            Self::Epoch => "Epoch",
            Self::Quant => "Quant",
            Self::UniversalDuration => "Universal Duration",
            Self::SolarClock => "Solar Clock",
            Self::Display => "Display",
        }
    }

    /// Get the function/purpose of this layer.
    pub fn function(self) -> &'static str {
        match self {
            Self::Epoch => "Shared origin point (t:0)",
            Self::Quant => "Physical substrate (hydrogen hyperfine periods)",
            Self::UniversalDuration => "Planet-independent elapsed-time units",
            Self::SolarClock => "Planet-specific position-in-day coordinates",
            Self::Display => "Human-facing rendering (clock faces, calendars, etc.)",
        }
    }

    /// Get the mutability characteristics of this layer.
    pub fn mutability(self) -> LayerMutability {
        match self {
            Self::Epoch => LayerMutability::Immutable,
            Self::Quant => LayerMutability::ImmutablePhysics,
            Self::UniversalDuration => LayerMutability::Immutable,
            Self::SolarClock => LayerMutability::PerPlanetProfile,
            Self::Display => LayerMutability::Customizable,
        }
    }

    /// Check if this layer depends on another (lower layers are dependencies).
    pub fn depends_on(self, other: TemporalLayer) -> bool {
        (other as u8) < (self as u8)
    }

    /// All layers in ascending order.
    pub fn all() -> [TemporalLayer; 5] {
        [
            Self::Epoch,
            Self::Quant,
            Self::UniversalDuration,
            Self::SolarClock,
            Self::Display,
        ]
    }
}

impl fmt::Display for TemporalLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Layer {} ({})", self.number(), self.name())
    }
}

/// Mutability characteristics of a temporal layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerMutability {
    /// Cannot be changed.
    Immutable,
    /// Immutable — defined by physics.
    ImmutablePhysics,
    /// Varies per planet profile.
    PerPlanetProfile,
    /// Can be customized by end users.
    Customizable,
}

impl fmt::Display for LayerMutability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Immutable => write!(f, "Immutable"),
            Self::ImmutablePhysics => write!(f, "Immutable (physics)"),
            Self::PerPlanetProfile => write!(f, "Per planet profile"),
            Self::Customizable => write!(f, "Customizable"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_numbers() {
        assert_eq!(TemporalLayer::Epoch.number(), 0);
        assert_eq!(TemporalLayer::Quant.number(), 1);
        assert_eq!(TemporalLayer::UniversalDuration.number(), 2);
        assert_eq!(TemporalLayer::SolarClock.number(), 3);
        assert_eq!(TemporalLayer::Display.number(), 4);
    }

    #[test]
    fn test_layer_dependencies() {
        assert!(TemporalLayer::Display.depends_on(TemporalLayer::SolarClock));
        assert!(TemporalLayer::SolarClock.depends_on(TemporalLayer::Quant));
        assert!(!TemporalLayer::Epoch.depends_on(TemporalLayer::Quant));
    }

    #[test]
    fn test_layer_mutability() {
        assert_eq!(TemporalLayer::Epoch.mutability(), LayerMutability::Immutable);
        assert_eq!(TemporalLayer::Quant.mutability(), LayerMutability::ImmutablePhysics);
        assert_eq!(TemporalLayer::Display.mutability(), LayerMutability::Customizable);
    }

    #[test]
    fn test_layer_ordering() {
        assert!(TemporalLayer::Epoch < TemporalLayer::Quant);
        assert!(TemporalLayer::Quant < TemporalLayer::UniversalDuration);
        assert!(TemporalLayer::SolarClock < TemporalLayer::Display);
    }

    #[test]
    fn test_all_layers() {
        let layers = TemporalLayer::all();
        assert_eq!(layers.len(), 5);
        assert_eq!(layers[0], TemporalLayer::Epoch);
        assert_eq!(layers[4], TemporalLayer::Display);
    }
}
