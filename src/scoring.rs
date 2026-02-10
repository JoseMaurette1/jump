use chrono::{DateTime, Utc};

/// Represents a score for directory entries based on frequency and recency.
/// Score = (access_count * recency_weight) + recency_bonus
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Score(f64);

impl Score {
    /// Weight applied to access count to diminish old access over time
    const RECENCY_WEIGHT: f64 = 0.1;

    /// Base score for new entries
    const BASE_SCORE: f64 = 1.0;

    /// Maximum bonus for recent access (exponential decay)
    const MAX_RECENCY_BONUS: f64 = 10.0;

    /// Days after which recency bonus starts decaying
    const RECENCY_WINDOW_DAYS: i64 = 30;

    /// Decay rate per week for unused entries
    const WEEKLY_DECAY_RATE: f64 = 0.05;

    pub fn default() -> Self {
        Self(Self::BASE_SCORE)
    }

    /// Create a Score from raw database value
    pub fn from_raw(value: f64) -> Self {
        Self(value)
    }

    /// Convert to raw f64 for database storage
    pub fn to_raw(self) -> f64 {
        self.0
    }

    /// Recalculate score based on current access count and timestamp
    pub fn recalculate(self, access_count: u32, last_accessed: i64) -> Self {
        let now = Utc::now();
        let last_access = DateTime::from_timestamp(last_accessed, 0).unwrap_or(now);
        let days_since_access = (now - last_access).num_days().max(0);

        // Calculate recency bonus (exponential decay based on days since access)
        let recency_bonus = if days_since_access < Self::RECENCY_WINDOW_DAYS {
            let decay_factor = (Self::RECENCY_WINDOW_DAYS - days_since_access) as f64
                / Self::RECENCY_WINDOW_DAYS as f64;
            Self::MAX_RECENCY_BONUS * decay_factor * decay_factor
        } else {
            0.0
        };

        // Apply decay for old entries
        let weeks_since_access = days_since_access / 7;
        let decay_multiplier = if weeks_since_access > 0 {
            (1.0 - Self::WEEKLY_DECAY_RATE * weeks_since_access as f64).max(0.0)
        } else {
            1.0
        };

        let base_score = Self::BASE_SCORE + (access_count as f64 * Self::RECENCY_WEIGHT);
        let final_score = (base_score + recency_bonus) * decay_multiplier;

        Self(final_score.max(0.0))
    }

    /// Apply weekly decay to entries that haven't been accessed
    pub fn apply_decay(self, weeks_since_access: u32) -> Self {
        let decay_multiplier = (1.0 - Self::WEEKLY_DECAY_RATE * weeks_since_access as f64).max(0.0);
        Self(self.0 * decay_multiplier)
    }
}

impl Default for Score {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_score() {
        let score = Score::default();
        assert_eq!(score.0, Score::BASE_SCORE);
    }

    #[test]
    fn test_score_recalculate_increases_with_access() {
        let now = Utc::now().timestamp();
        let score1 = Score::default().recalculate(0, now);
        let score2 = Score::default().recalculate(5, now);

        assert!(score2.0 > score1.0);
    }

    #[test]
    fn test_score_recalculate_with_recent_access() {
        let now = Utc::now().timestamp();
        let recent_score = Score::default().recalculate(5, now);

        // Access from 60 days ago should have lower score due to decay
        let old_timestamp = now - (60 * 24 * 60 * 60);
        let old_score = Score::default().recalculate(5, old_timestamp);

        assert!(recent_score.0 > old_score.0);
    }

    #[test]
    fn test_score_from_raw() {
        let raw = 42.5;
        let score = Score::from_raw(raw);
        assert_eq!(score.to_raw(), raw);
    }
}
