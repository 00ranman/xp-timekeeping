use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc, Datelike, Timelike};

use crate::calendar::{CalendarConfig, CalendarDate, is_earth_leap_year};
use crate::constants::*;
use crate::error::{UtError, UtResult};
use crate::planet::{Earth, PlanetProfile};
use crate::quant::Quant;
use crate::solar_clock::SolarTime;

/// Convert a Unix timestamp (seconds since 1970-01-01T00:00:00 UTC) to a SolarTime
/// on Earth (time-of-day only; the date portion is discarded).
pub fn unix_to_earth_solar_time(unix_secs: f64) -> UtResult<SolarTime> {
    let seconds_in_day = unix_secs.rem_euclid(EARTH_DAY_SECONDS);
    let earth = Earth;
    earth.seconds_to_solar_time(seconds_in_day)
}

/// Convert a SolarTime on Earth to the seconds-since-midnight portion.
/// To reconstruct a full Unix timestamp you also need the date.
pub fn earth_solar_time_to_seconds_in_day(time: SolarTime) -> f64 {
    let earth = Earth;
    earth.solar_time_to_seconds(time)
}

/// Convert an ISO 8601 datetime string to Earth SolarTime (time-of-day only).
pub fn iso8601_to_earth_solar_time(iso: &str) -> UtResult<SolarTime> {
    let dt = iso
        .parse::<DateTime<Utc>>()
        .map_err(|e| UtError::ParseError(format!("invalid ISO 8601: {e}")))?;
    let seconds_in_day =
        dt.hour() as f64 * 3600.0 + dt.minute() as f64 * 60.0 + dt.second() as f64;
    let earth = Earth;
    earth.seconds_to_solar_time(seconds_in_day)
}

/// Convert an Earth SolarTime + Gregorian date back to an ISO 8601 string.
pub fn earth_solar_time_to_iso8601(
    solar_time: SolarTime,
    year: i32,
    month: u32,
    day: u32,
) -> UtResult<String> {
    let earth = Earth;
    let secs = earth.solar_time_to_seconds(solar_time);
    let total_secs = secs as u32;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    let date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
        UtError::InvalidDate {
            reason: format!("invalid Gregorian date: {year}-{month}-{day}"),
        }
    })?;
    let time = NaiveTime::from_hms_opt(hours, minutes, seconds).ok_or_else(|| {
        UtError::InvalidDate {
            reason: format!("invalid time: {hours}:{minutes}:{seconds}"),
        }
    })?;
    let dt = NaiveDateTime::new(date, time);
    Ok(format!("{}Z", dt.format("%Y-%m-%dT%H:%M:%S")))
}

/// Convert a Unix timestamp to a quant count (given a quant-at-unix-epoch offset).
pub fn unix_to_quant(unix_secs: f64, quant_at_unix_epoch: Quant) -> Quant {
    let elapsed = Quant::from_seconds(unix_secs);
    Quant(quant_at_unix_epoch.0 + elapsed.0)
}

/// Convert a quant count back to a Unix timestamp.
pub fn quant_to_unix(quant: Quant, quant_at_unix_epoch: Quant) -> f64 {
    let elapsed = Quant(quant.0 - quant_at_unix_epoch.0);
    elapsed.to_seconds()
}

/// Convert a Gregorian date to a Universal Times CalendarDate.
///
/// Maps the Gregorian day-of-year to the UT 10-month calendar.
pub fn gregorian_to_ut_calendar(year: i32, month: u32, day: u32) -> UtResult<CalendarDate> {
    let date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| UtError::InvalidDate {
        reason: format!("invalid Gregorian date: {year}-{month:02}-{day:02}"),
    })?;
    let doy = date.ordinal();
    let is_leap = is_earth_leap_year(year);
    let config = CalendarConfig::earth();
    CalendarDate::from_day_of_year(year, doy, &config, is_leap)
}

/// Convert a Universal Times CalendarDate back to a Gregorian date.
pub fn ut_calendar_to_gregorian(ut_date: CalendarDate) -> UtResult<(i32, u32, u32)> {
    let _is_leap = is_earth_leap_year(ut_date.year);
    let config = CalendarConfig::earth();
    let doy = ut_date.day_of_year(&config);
    let date = NaiveDate::from_yo_opt(ut_date.year, doy).ok_or_else(|| UtError::InvalidDate {
        reason: format!("cannot convert UT date {} to Gregorian", ut_date),
    })?;
    Ok((date.year(), date.month(), date.day()))
}

/// Convert a legacy 24-hour time (hours, minutes, seconds) to SolarTime.
pub fn legacy_time_to_solar_time(hours: u32, minutes: u32, seconds: u32) -> UtResult<SolarTime> {
    let total_seconds = hours as f64 * 3600.0 + minutes as f64 * 60.0 + seconds as f64;
    let earth = Earth;
    earth.seconds_to_solar_time(total_seconds)
}

/// Convert a SolarTime to legacy 24-hour time (hours, minutes, seconds).
pub fn solar_time_to_legacy_time(time: SolarTime) -> (u32, u32, u32) {
    let earth = Earth;
    let secs = earth.solar_time_to_seconds(time);
    let total = secs.round() as u32;
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let seconds = total % 60;
    (hours, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unix_midnight_to_solar_time() {
        // Unix timestamp 0 = 1970-01-01T00:00:00Z = midnight
        let t = unix_to_earth_solar_time(0.0).unwrap();
        assert_eq!(t.loop_val(), 0);
        assert_eq!(t.arc(), 0);
        assert_eq!(t.tick(), 0);
    }

    #[test]
    fn test_unix_noon_to_solar_time() {
        // 43200 seconds = noon
        let t = unix_to_earth_solar_time(43200.0).unwrap();
        assert_eq!(t.loop_val(), 5);
        assert_eq!(t.arc(), 0);
        assert_eq!(t.tick(), 0);
    }

    #[test]
    fn test_unix_solar_time_round_trip() {
        let original_secs = 36000.0; // 10:00 AM
        let st = unix_to_earth_solar_time(original_secs).unwrap();
        let back_secs = earth_solar_time_to_seconds_in_day(st);
        assert!((back_secs - original_secs).abs() < 1.0);
    }

    #[test]
    fn test_iso8601_noon() {
        let t = iso8601_to_earth_solar_time("2026-03-15T12:00:00Z").unwrap();
        assert_eq!(t.loop_val(), 5);
        assert_eq!(t.arc(), 0);
        assert_eq!(t.tick(), 0);
    }

    #[test]
    fn test_iso8601_round_trip() {
        let iso = "2026-03-15T12:00:00Z";
        let st = iso8601_to_earth_solar_time(iso).unwrap();
        let back = earth_solar_time_to_iso8601(st, 2026, 3, 15).unwrap();
        assert_eq!(back, "2026-03-15T12:00:00Z");
    }

    #[test]
    fn test_legacy_time_midnight() {
        let t = legacy_time_to_solar_time(0, 0, 0).unwrap();
        assert!(t.is_midnight());
    }

    #[test]
    fn test_legacy_time_noon() {
        let t = legacy_time_to_solar_time(12, 0, 0).unwrap();
        assert!(t.is_noon());
    }

    #[test]
    fn test_solar_to_legacy_round_trip() {
        let original = SolarTime::new(5, 0, 0).unwrap();
        let (h, m, s) = solar_time_to_legacy_time(original);
        assert_eq!(h, 12);
        assert_eq!(m, 0);
        assert_eq!(s, 0);
    }

    #[test]
    fn test_legacy_time_round_trip() {
        let (hours, minutes, seconds) = (14, 30, 0);
        let st = legacy_time_to_solar_time(hours, minutes, seconds).unwrap();
        let (h2, m2, s2) = solar_time_to_legacy_time(st);
        assert_eq!(h2, hours);
        assert_eq!(m2, minutes);
        // Small rounding tolerance
        assert!((s2 as i32 - seconds as i32).unsigned_abs() <= 1);
    }

    #[test]
    fn test_quant_unix_round_trip() {
        let epoch_offset = Quant::new(0);
        let original_unix = 1710460800.0; // some Unix timestamp
        let q = unix_to_quant(original_unix, epoch_offset);
        let back = quant_to_unix(q, epoch_offset);
        assert!((back - original_unix).abs() < 0.001);
    }

    #[test]
    fn test_gregorian_to_ut_jan1() {
        let d = gregorian_to_ut_calendar(2026, 1, 1).unwrap();
        assert_eq!(d.year, 2026);
        assert_eq!(d.month, 1);
        assert_eq!(d.day, 1);
    }

    #[test]
    fn test_gregorian_to_ut_feb10() {
        // Feb 10 = day 41 of year → should be month 2, day 1 in UT
        let d = gregorian_to_ut_calendar(2026, 2, 10).unwrap();
        assert_eq!(d.year, 2026);
        assert_eq!(d.month, 2);
        assert_eq!(d.day, 1);
    }

    #[test]
    fn test_gregorian_ut_round_trip() {
        let (y, m, d) = (2026, 3, 15);
        let ut = gregorian_to_ut_calendar(y, m, d).unwrap();
        let (y2, m2, d2) = ut_calendar_to_gregorian(ut).unwrap();
        assert_eq!(y2, y);
        assert_eq!(m2, m);
        assert_eq!(d2, d);
    }

    #[test]
    fn test_gregorian_ut_round_trip_leap_year() {
        let (y, m, d) = (2024, 12, 31);
        let ut = gregorian_to_ut_calendar(y, m, d).unwrap();
        let (y2, m2, d2) = ut_calendar_to_gregorian(ut).unwrap();
        assert_eq!(y2, y);
        assert_eq!(m2, m);
        assert_eq!(d2, d);
    }

    #[test]
    fn test_gregorian_ut_round_trip_all_year() {
        // Test every day in 2026 (non-leap)
        let mut date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        while date <= end {
            let ut = gregorian_to_ut_calendar(date.year(), date.month(), date.day()).unwrap();
            let (y2, m2, d2) = ut_calendar_to_gregorian(ut).unwrap();
            assert_eq!((date.year(), date.month(), date.day()), (y2, m2, d2),
                "failed for {date}");
            date = date.succ_opt().unwrap();
        }
    }
}
