// Timestamp utility functions for the orqa-engine crate.
//
// Provides calendar arithmetic (days-since-epoch to year/month/day),
// Unix-timestamp decomposition, and ISO-8601 formatting helpers without
// pulling in the `chrono` crate. All arithmetic is done in UTC.

// ── Core calendar helpers ──────────────────────────────────────────────────

/// Return `true` if `year` is a Gregorian leap year.
pub fn is_leap_year(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

/// Convert days elapsed since the Unix epoch (1970-01-01) to `(year, month, day)`.
///
/// Both `month` and `day` are 1-based.
pub fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    let mut year = 1970u64;
    let mut remaining = days;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }

    let month_lengths: [u64; 12] = [
        31,
        if is_leap_year(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    let mut month = 1u64;
    for &len in &month_lengths {
        if remaining < len {
            break;
        }
        remaining -= len;
        month += 1;
    }

    (year, month, remaining + 1)
}

/// Decompose a Unix timestamp (seconds since 1970-01-01T00:00:00Z) into
/// `(year, month, day, hour, minute, second)`.
///
/// Month and day are 1-based.
pub fn unix_to_datetime(secs: u64) -> (u64, u64, u64, u64, u64, u64) {
    let second = secs % 60;
    let minutes = secs / 60;
    let minute = minutes % 60;
    let hours = minutes / 60;
    let hour = hours % 24;
    let total_days = hours / 24;

    let (year, month, day) = days_to_ymd(total_days);

    (year, month, day, hour, minute, second)
}

// ── Formatting helpers ─────────────────────────────────────────────────────

/// Format a Unix timestamp as `"YYYY-MM-DDTHH:MM:SS.000Z"`.
///
/// This variant includes the millisecond component (always `.000`) to match
/// the format produced by SQLite's `strftime` default and the governance
/// commands layer.
pub fn format_unix_timestamp(secs: u64) -> String {
    let (year, month, day, hour, minute, second) = unix_to_datetime(secs);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.000Z")
}

/// Return the current UTC time formatted as `"YYYY-MM-DDTHH:MM:SSZ"` (no
/// millisecond component).
///
/// Used by artifact commands that store lightweight timestamps.
pub fn now_iso_basic() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let (year, month, day, hour, minute, second) = unix_to_datetime(secs);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// Return the current UTC date formatted as `"YYYY-MM-DD"`.
///
/// Used by the lesson repository to stamp lesson files.
pub fn today_date_string() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = secs / 86_400;
    let (year, month, day) = days_to_ymd(days);
    format!("{year:04}-{month:02}-{day:02}")
}

/// Return the current UTC timestamp as a Unix seconds value.
///
/// Convenience wrapper so callers do not need to import `std::time` directly.
pub fn now_unix_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_leap_year ──────────────────────────────────────────────────────

    #[test]
    fn leap_year_divisible_by_400() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(1600));
    }

    #[test]
    fn leap_year_divisible_by_4_not_100() {
        assert!(is_leap_year(2024));
        assert!(is_leap_year(1996));
    }

    #[test]
    fn not_leap_year_divisible_by_100_not_400() {
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn not_leap_year_not_divisible_by_4() {
        assert!(!is_leap_year(2023));
        assert!(!is_leap_year(2019));
    }

    // ── days_to_ymd ───────────────────────────────────────────────────────

    #[test]
    fn days_to_ymd_epoch() {
        assert_eq!(days_to_ymd(0), (1970, 1, 1));
    }

    #[test]
    fn days_to_ymd_one_day() {
        assert_eq!(days_to_ymd(1), (1970, 1, 2));
    }

    #[test]
    fn days_to_ymd_end_of_january_1970() {
        assert_eq!(days_to_ymd(30), (1970, 1, 31));
    }

    #[test]
    fn days_to_ymd_start_of_february_1970() {
        assert_eq!(days_to_ymd(31), (1970, 2, 1));
    }

    #[test]
    fn days_to_ymd_start_of_1971() {
        // 1970 has 365 days (not a leap year)
        assert_eq!(days_to_ymd(365), (1971, 1, 1));
    }

    #[test]
    fn days_to_ymd_known_date_2024_01_01() {
        // 2024-01-01 is 19723 days from the Unix epoch
        assert_eq!(days_to_ymd(19723), (2024, 1, 1));
    }

    #[test]
    fn days_to_ymd_leap_day_2024() {
        // 2024-02-29: 2024 is a leap year; 19723 + 31 + 28 = 19782
        assert_eq!(days_to_ymd(19782), (2024, 2, 29));
    }

    // ── unix_to_datetime ──────────────────────────────────────────────────

    #[test]
    fn unix_to_datetime_epoch() {
        assert_eq!(unix_to_datetime(0), (1970, 1, 1, 0, 0, 0));
    }

    #[test]
    fn unix_to_datetime_one_hour() {
        assert_eq!(unix_to_datetime(3600), (1970, 1, 1, 1, 0, 0));
    }

    #[test]
    fn unix_to_datetime_one_day() {
        assert_eq!(unix_to_datetime(86_400), (1970, 1, 2, 0, 0, 0));
    }

    #[test]
    fn unix_to_datetime_components() {
        // 2026-03-06T15:30:45Z
        // Days to 2026-01-01: days_to_ymd inverse gives 20454 days
        // 2026-03-06 = 20454 + 31 (Jan) + 28 (Feb, not leap) + 5 = 20518 days
        // seconds = 20518 * 86400 + 15 * 3600 + 30 * 60 + 45
        let secs = 20518u64 * 86_400 + 15 * 3_600 + 30 * 60 + 45;
        let (y, mo, d, h, mi, s) = unix_to_datetime(secs);
        assert_eq!(y, 2026);
        assert_eq!(mo, 3);
        assert_eq!(d, 6);
        assert_eq!(h, 15);
        assert_eq!(mi, 30);
        assert_eq!(s, 45);
    }

    // ── format_unix_timestamp ─────────────────────────────────────────────

    #[test]
    fn format_unix_timestamp_epoch() {
        assert_eq!(format_unix_timestamp(0), "1970-01-01T00:00:00.000Z");
    }

    #[test]
    #[allow(clippy::case_sensitive_file_extension_comparisons)] // ".000Z" is an ISO timestamp suffix, not a file extension
    fn format_unix_timestamp_length_and_structure() {
        let ts = format_unix_timestamp(0);
        assert_eq!(ts.len(), 24);
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
        assert!(ts.ends_with(".000Z"));
    }

    // ── now_iso_basic ─────────────────────────────────────────────────────

    #[test]
    fn now_iso_basic_format() {
        let ts = now_iso_basic();
        // "YYYY-MM-DDTHH:MM:SSZ" = 20 chars
        assert_eq!(ts.len(), 20);
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
        assert!(ts.ends_with('Z'));
    }

    // ── today_date_string ─────────────────────────────────────────────────

    #[test]
    fn today_date_string_format() {
        let date = today_date_string();
        // "YYYY-MM-DD" = 10 chars
        assert_eq!(date.len(), 10);
        assert_eq!(&date[4..5], "-");
        assert_eq!(&date[7..8], "-");
    }
}
