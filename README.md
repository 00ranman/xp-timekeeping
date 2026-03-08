# XP Timekeeping

Base-10 temporal architecture for the Extropy Engine ecosystem.

## Ecosystem Integration

> **Ecosystem Note:** This is the standalone Rust implementation. The deployed TypeScript re-implementation lives in the [extropy-engine](https://github.com/00ranman/extropy-engine) monorepo at `packages/temporal` (port 4011).
>
> See [ECOSYSTEM_MAP.md](https://github.com/00ranman/extropy-engine/blob/main/ECOSYSTEM_MAP.md) for the full repository mapping.


## Overview

XP Timekeeping implements a revolutionary base-10 temporal system that eliminates the complexities of traditional calendars (leap years, irregular month lengths) while providing precise time representation for physics-based governance systems.

## Key Features

- **Base-10 Architecture**: 10-hour days, 100-minute hours, 5-day weeks, 15-day months, 24 months/year
- **No Leap Years**: Mathematically consistent temporal progression
- **Seamless Conversion**: Bidirectional conversion with Gregorian calendar
- **Temporal Loops**: Support for nested temporal structures and causal loops
- **Physics Integration**: Time representation optimized for entropy calculations

## Time Structure

```
1 XP Year = 24 months = 360 days
1 XP Month = 15 days
1 XP Week = 5 days  
1 XP Day = 10 hours
1 XP Hour = 100 minutes
```

## Usage

```rust
use xp_timekeeping::{XpTime, TemporalLoop, LoopType};

// Current time in XP format
let current_time = XpTime::now();
println!("XP Time: {}", current_time);

// Time arithmetic
let future_time = current_time.add_minutes(150);

// Gregorian conversion
let gregorian_timestamp = current_time.to_gregorian_timestamp();
let converted_back = XpTime::from_gregorian_timestamp(gregorian_timestamp);

// Temporal loops for activity tracking
let daily_loop = TemporalLoop::new(LoopType::Daily, current_time, 5.2);
```

## Integration

This timekeeping system integrates with:
- **XP Calculation Engine**: Physics-based value computation
- **DAG Mesh**: Transaction timestamping and ordering
- **Entropy Tracking**: Temporal measurement for disorder reduction
- **Causal Loop Detection**: Cross-platform synergy identification

## License

MIT License - See LICENSE file for details
