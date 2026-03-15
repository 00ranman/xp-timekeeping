# Universal Times v4.0

Hydrogen-anchored dual-system temporal infrastructure for multi-planetary timekeeping.

## Overview

Universal Times is a physics-first timekeeping framework that replaces arbitrary historical conventions with a system grounded in fundamental physics. The hydrogen-1 ground-state hyperfine transition frequency (1,420,405,751.768 Hz) serves as the universal time anchor, providing an immutable physical substrate upon which all temporal measurements are built.

The framework implements a dual-system architecture:
- **Solar Clock** — position within the local solar day (replaces hours/minutes/seconds)
- **Universal Duration** — planet-independent elapsed-time measurement (replaces legacy duration units)

## Key Concepts

### Solar Clock: `t:L:AA:TT`

Every planet's solar day is divided into exactly 100,000 ticks:

| Unit | Count | Per Day |
|------|-------|---------|
| Loop | 10 | 10 |
| Arc | 100 per Loop | 1,000 |
| Tick | 100 per Arc | 100,000 |

Format: `t:L:AA:TT` — e.g., `t:5:00:00` is solar noon on any planet.

### Universal Duration Units

Seven named duration units span human-relevant timescales, each a power-of-10 multiple of hydrogen hyperfine periods:

| Unit | H-Periods | Approx. Seconds |
|------|-----------|-----------------|
| Pulse | 10^11 | ~70 s |
| Wave | 10^12 | ~704 s |
| Tide | 10^13 | ~7,040 s |
| Spin | 10^14 | ~70,400 s |
| Current | 10^15 | ~704,000 s |
| Season | 10^16 | ~7,040,000 s |
| Epoch | 10^17 | ~70,400,000 s |

### Quant Accumulator

A monotonically increasing `u128`/`bigint` counter of hydrogen hyperfine periods since the epoch origin (t:0). This is the fundamental substrate from which all other time representations are derived.

### Planet Profiles

Each planet defines how universal time maps to local experience:

| Planet | Day Length | Seconds/Tick |
|--------|-----------|-------------|
| Earth | 86,400 s | 0.864 s |
| Mars | 88,775.244 s | 0.88775 s |

### 10-Month Calendar

- Months 1–9: equal length (Earth: 40 days, Mars: 74 sols)
- Month 10: short closing month absorbing the orbital remainder
- 5-day cycles replace 7-day weeks
- Earth intercalation uses the Gregorian leap-year rule

### 5-Layer Temporal Architecture

| Layer | Name | Function | Mutability |
|-------|------|----------|-----------|
| 0 | Epoch | Shared origin point (t:0) | Immutable |
| 1 | Quant | Physical substrate (hydrogen periods) | Immutable (physics) |
| 2 | Universal Duration | Planet-independent elapsed-time units | Immutable |
| 3 | Solar Clock | Planet-specific position-in-day | Per planet profile |
| 4 | Display | Human-facing rendering | Customizable |

### XP (Experience Points)

Two formulas for computing validated entropy reduction:

- **Philosophical**: `XP = S × c_L²` (entropy reduction × propagation speed²)
- **Operational**: `XP = B × D × T × V × S` (base × domain × temporal decay × validation × scarcity)

### 7-Prime Token Economy

| Token | Prime | Full Name |
|-------|-------|-----------|
| XP | 2 | Experience Points |
| CT | 3 | Contribution Tokens |
| CAT | 5 | Catalyst Tokens |
| IT | 7 | Insight Tokens |
| DT | 11 | Decay Tokens |
| EP | 13 | Entropy Points |
| GP | 17 | Governance Points |

Token sets encode as prime-factored products for collision-free representation.

### Temporal DAG

A directed acyclic graph substrate enforcing causal consistency — every node's timestamp must strictly exceed all parent timestamps.

## Project Structure

```
├── Cargo.toml              # Rust crate config
├── src/                    # Rust implementation
│   ├── lib.rs              # Module declarations and re-exports
│   ├── constants.rs        # Physical constants and derivation functions
│   ├── error.rs            # Error types
│   ├── quant.rs            # Quant accumulator
│   ├── duration.rs         # Universal Duration units
│   ├── solar_clock.rs      # Solar Clock time
│   ├── planet.rs           # Planet profiles (Earth, Mars)
│   ├── calendar.rs         # 10-month calendar
│   ├── layers.rs           # 5-layer architecture
│   ├── xp.rs               # XP computation
│   ├── dag.rs              # Temporal DAG
│   ├── tokens.rs           # 7-prime token economy
│   └── convert.rs          # Format conversion utilities
├── ts/                     # TypeScript implementation
│   ├── package.json
│   ├── tsconfig.json
│   ├── src/                # Source modules (mirrors Rust)
│   └── tests/              # Test suite
└── docs/
    └── Universal-Times-v4.0.pdf
```

## Rust

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

126 tests across 12 modules.

## TypeScript

### Install

```bash
cd ts && npm install
```

### Build

```bash
npm run build
```

### Test

```bash
npm test
```

175 tests across 10 test files.

## Usage

### Rust

```rust
use xp_timekeeping::{SolarTime, create_earth, Quant};

// Solar Clock time
let noon = SolarTime::new(5, 0, 0).unwrap();
println!("{}", noon); // t:5:00:00

// Planet-aware conversion
let earth = create_earth();
let time = earth.seconds_to_solar_time(43200.0);
assert!(time.is_noon());

// Quant accumulator
let q = Quant::from_seconds(86400.0);
println!("One day = {} quants", q.count());
```

### TypeScript

```typescript
import { SolarTime, createEarth, Quant } from "xp-timekeeping";

// Solar Clock time
const noon = SolarTime.new(5, 0, 0);
console.log(noon.toString()); // t:5:00:00

// Planet-aware conversion
const earth = createEarth();
const time = earth.secondsToSolarTime(43200);
console.log(time.isNoon()); // true

// Quant accumulator
const q = Quant.fromSeconds(86400);
console.log(`One day = ${q.count()} quants`);
```

## License

MIT
