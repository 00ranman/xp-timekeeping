use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{UtError, UtResult};

/// Calendar configuration for a planet.
///
/// Universal Times uses 10 months per orbit. Months 1-9 have equal length.
/// Month 10 is a short closing month that absorbs the orbital remainder.
/// 5-day cycles replace 7-day weeks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarConfig {
    /// Number of months per orbit (always 10).
    pub months_per_orbit: u32,
    /// Days per month for months 1-9.
    pub days_per_standard_month: u32,
    /// Days in month 10 (normal year).
    pub days_month_10_normal: u32,
    /// Days in month 10 (leap year).
    pub days_month_10_leap: u32,
    /// Days per cycle (replacing weeks).
    pub days_per_cycle: u32,
    /// Cycles per standard month.
    pub cycles_per_standard_month: u32,
    /// Remainder days after full cycles in a standard month.
    pub remainder_days_per_month: u32,
}

impl CalendarConfig {
    /// Create the canonical Earth calendar configuration.
    pub fn earth() -> Self {
        Self {
            months_per_orbit: 10,
            days_per_standard_month: 40,
            days_month_10_normal: 5,
            days_month_10_leap: 6,
            days_per_cycle: 5,
            cycles_per_standard_month: 8,
            remainder_days_per_month: 0,
        }
    }

    /// Create the canonical Mars calendar configuration.
    pub fn mars() -> Self {
        Self {
            months_per_orbit: 10,
            days_per_standard_month: 74,
            days_month_10_normal: 3,
            days_month_10_leap: 4,
            days_per_cycle: 5,
            cycles_per_standard_month: 14,
            remainder_days_per_month: 4,
        }
    }

    /// Total days in a normal (non-leap) year.
    pub fn days_per_year_normal(&self) -> u32 {
        self.days_per_standard_month * 9 + self.days_month_10_normal
    }

    /// Total days in a leap year.
    pub fn days_per_year_leap(&self) -> u32 {
        self.days_per_standard_month * 9 + self.days_month_10_leap
    }

    /// Total days in a given year.
    pub fn days_in_year(&self, _year: i32, is_leap: bool) -> u32 {
        if is_leap {
            self.days_per_year_leap()
        } else {
            self.days_per_year_normal()
        }
    }

    /// Days in a specific month (1-indexed).
    pub fn days_in_month(&self, month: u32, is_leap: bool) -> UtResult<u32> {
        if month < 1 || month > self.months_per_orbit {
            return Err(UtError::InvalidDate {
                reason: format!("month {month} out of range 1-{}", self.months_per_orbit),
            });
        }
        if month <= 9 {
            Ok(self.days_per_standard_month)
        } else if is_leap {
            Ok(self.days_month_10_leap)
        } else {
            Ok(self.days_month_10_normal)
        }
    }
}

/// Determine if an Earth year is a leap year using the Gregorian rule.
pub fn is_earth_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// A calendar date in Universal Times.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarDate {
    /// Year number.
    pub year: i32,
    /// Month (1-10).
    pub month: u32,
    /// Day within month (1-indexed).
    pub day: u32,
}

impl CalendarDate {
    /// Create a new calendar date, validating ranges.
    pub fn new(year: i32, month: u32, day: u32, config: &CalendarConfig, is_leap: bool) -> UtResult<Self> {
        let max_days = config.days_in_month(month, is_leap)?;
        if day < 1 || day > max_days {
            return Err(UtError::InvalidDate {
                reason: format!("day {day} out of range 1-{max_days} for month {month}"),
            });
        }
        Ok(Self { year, month, day })
    }

    /// Get the day-of-year (1-indexed).
    pub fn day_of_year(&self, config: &CalendarConfig) -> u32 {
        let full_months = self.month - 1;
        let days_from_months = full_months.min(9) * config.days_per_standard_month;
        days_from_months + self.day
    }

    /// Create from a day-of-year (1-indexed).
    pub fn from_day_of_year(year: i32, doy: u32, config: &CalendarConfig, is_leap: bool) -> UtResult<Self> {
        let total_days = config.days_in_year(year, is_leap);
        if doy < 1 || doy > total_days {
            return Err(UtError::InvalidDate {
                reason: format!("day-of-year {doy} out of range 1-{total_days}"),
            });
        }

        let mut remaining = doy;
        for m in 1..=10u32 {
            let md = config.days_in_month(m, is_leap)?;
            if remaining <= md {
                return Ok(Self { year, month: m, day: remaining });
            }
            remaining -= md;
        }

        Err(UtError::InvalidDate {
            reason: format!("could not resolve day-of-year {doy}"),
        })
    }

    /// Get the cycle number within the month (1-indexed) and day within the cycle (1-indexed).
    pub fn cycle_and_day(&self, config: &CalendarConfig) -> (u32, u32) {
        let day_zero = self.day - 1;
        let cycle = day_zero / config.days_per_cycle + 1;
        let day_in_cycle = day_zero % config.days_per_cycle + 1;
        (cycle, day_in_cycle)
    }
}

impl fmt::Display for CalendarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Y{}:M{}:D{}", self.year, self.month, self.day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_earth_calendar_normal_year() {
        let cal = CalendarConfig::earth();
        assert_eq!(cal.days_per_year_normal(), 365); // 9*40 + 5
    }

    #[test]
    fn test_earth_calendar_leap_year() {
        let cal = CalendarConfig::earth();
        assert_eq!(cal.days_per_year_leap(), 366); // 9*40 + 6
    }

    #[test]
    fn test_earth_month_lengths() {
        let cal = CalendarConfig::earth();
        for m in 1..=9 {
            assert_eq!(cal.days_in_month(m, false).unwrap(), 40);
        }
        assert_eq!(cal.days_in_month(10, false).unwrap(), 5);
        assert_eq!(cal.days_in_month(10, true).unwrap(), 6);
    }

    #[test]
    fn test_mars_calendar_normal() {
        let cal = CalendarConfig::mars();
        // 9 * 74 + 3 = 669
        assert_eq!(cal.days_per_year_normal(), 669);
    }

    #[test]
    fn test_mars_month_lengths() {
        let cal = CalendarConfig::mars();
        for m in 1..=9 {
            assert_eq!(cal.days_in_month(m, false).unwrap(), 74);
        }
        assert_eq!(cal.days_in_month(10, false).unwrap(), 3);
        assert_eq!(cal.days_in_month(10, true).unwrap(), 4);
    }

    #[test]
    fn test_invalid_month() {
        let cal = CalendarConfig::earth();
        assert!(cal.days_in_month(0, false).is_err());
        assert!(cal.days_in_month(11, false).is_err());
    }

    #[test]
    fn test_leap_year_gregorian() {
        assert!(is_earth_leap_year(2000));  // div by 400
        assert!(!is_earth_leap_year(1900)); // div by 100 but not 400
        assert!(is_earth_leap_year(2024));  // div by 4
        assert!(!is_earth_leap_year(2023)); // not div by 4
    }

    #[test]
    fn test_calendar_date_creation() {
        let cal = CalendarConfig::earth();
        let d = CalendarDate::new(1, 5, 20, &cal, false).unwrap();
        assert_eq!(d.year, 1);
        assert_eq!(d.month, 5);
        assert_eq!(d.day, 20);
    }

    #[test]
    fn test_calendar_date_invalid_day() {
        let cal = CalendarConfig::earth();
        assert!(CalendarDate::new(1, 1, 41, &cal, false).is_err());
        assert!(CalendarDate::new(1, 10, 6, &cal, false).is_err()); // only 5 days in month 10
        assert!(CalendarDate::new(1, 10, 6, &cal, true).is_ok());   // leap year: 6 days
    }

    #[test]
    fn test_day_of_year() {
        let cal = CalendarConfig::earth();
        let d = CalendarDate::new(1, 1, 1, &cal, false).unwrap();
        assert_eq!(d.day_of_year(&cal), 1);

        let d2 = CalendarDate::new(1, 2, 1, &cal, false).unwrap();
        assert_eq!(d2.day_of_year(&cal), 41); // 40 + 1

        let d3 = CalendarDate::new(1, 10, 5, &cal, false).unwrap();
        assert_eq!(d3.day_of_year(&cal), 365); // 360 + 5
    }

    #[test]
    fn test_from_day_of_year() {
        let cal = CalendarConfig::earth();
        let d = CalendarDate::from_day_of_year(1, 1, &cal, false).unwrap();
        assert_eq!(d.month, 1);
        assert_eq!(d.day, 1);

        let d2 = CalendarDate::from_day_of_year(1, 41, &cal, false).unwrap();
        assert_eq!(d2.month, 2);
        assert_eq!(d2.day, 1);

        let d3 = CalendarDate::from_day_of_year(1, 365, &cal, false).unwrap();
        assert_eq!(d3.month, 10);
        assert_eq!(d3.day, 5);
    }

    #[test]
    fn test_from_day_of_year_round_trip() {
        let cal = CalendarConfig::earth();
        for doy in 1..=365 {
            let d = CalendarDate::from_day_of_year(1, doy, &cal, false).unwrap();
            assert_eq!(d.day_of_year(&cal), doy);
        }
    }

    #[test]
    fn test_cycle_and_day() {
        let cal = CalendarConfig::earth();
        let d = CalendarDate::new(1, 1, 1, &cal, false).unwrap();
        assert_eq!(d.cycle_and_day(&cal), (1, 1));

        let d2 = CalendarDate::new(1, 1, 6, &cal, false).unwrap();
        assert_eq!(d2.cycle_and_day(&cal), (2, 1));

        let d3 = CalendarDate::new(1, 1, 40, &cal, false).unwrap();
        assert_eq!(d3.cycle_and_day(&cal), (8, 5));
    }

    #[test]
    fn test_calendar_date_display() {
        let cal = CalendarConfig::earth();
        let d = CalendarDate::new(2026, 3, 15, &cal, false).unwrap();
        assert_eq!(format!("{d}"), "Y2026:M3:D15");
    }
}
