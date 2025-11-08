//! Time-related utilities

/// Seconds per day constant
pub const SECONDS_PER_DAY: u64 = 86400;

/// Convert days to seconds
pub fn days_to_seconds(days: u64) -> u64 {
    days.saturating_mul(SECONDS_PER_DAY)
}

/// Convert seconds to days
pub fn seconds_to_days(seconds: u64) -> u64 {
    seconds / SECONDS_PER_DAY
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_to_seconds() {
        assert_eq!(days_to_seconds(1), 86400);
    }

    #[test]
    fn test_seconds_to_days() {
        assert_eq!(seconds_to_days(172800), 2);
    }
}
