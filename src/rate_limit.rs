//! Per-minute in-memory rate limiting for API key and IP usage.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Rate-limit bounds applied by [`RateLimiter`].
pub struct RateLimitConfig {
    /// Maximum requests per key per minute.
    pub per_key_per_minute: u32,
    /// Maximum total requests per IP per minute.
    pub per_ip_per_minute: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            per_key_per_minute: 100,
            per_ip_per_minute: 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// Result of a rate-limit acquisition attempt.
pub enum AcquireResult {
    /// Request slot acquired.
    Acquired,
    /// Request must wait until the provided duration elapses.
    Wait {
        /// Duration remaining until the next available slot window.
        duration: Duration,
    },
}

#[derive(Debug)]
/// In-memory per-minute limiter shared by the executor.
pub struct RateLimiter {
    config: RateLimitConfig,
    state: Mutex<RateLimitState>,
}

#[derive(Debug, Default)]
struct RateLimitState {
    minute_index: u64,
    ip_count: u32,
    key_counts: HashMap<String, u32>,
}

impl RateLimiter {
    /// Creates a limiter with the provided configuration.
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            state: Mutex::new(RateLimitState::default()),
        }
    }

    /// Attempts to acquire a request slot for the given key and timestamp.
    pub fn try_acquire_at_unix_seconds(&self, key: &str, unix_seconds: u64) -> AcquireResult {
        let mut state = self
            .state
            .lock()
            .expect("rate limiter state mutex should not be poisoned");
        sync_minute(&mut state, unix_seconds);

        let key_count = state.key_counts.get(key).copied().unwrap_or(0);
        if key_count >= self.config.per_key_per_minute
            || state.ip_count >= self.config.per_ip_per_minute
        {
            return AcquireResult::Wait {
                duration: wait_until_next_minute(unix_seconds),
            };
        }

        state.ip_count = state.ip_count.saturating_add(1);
        state
            .key_counts
            .entry(key.to_string())
            .and_modify(|count| *count = count.saturating_add(1))
            .or_insert(1);

        AcquireResult::Acquired
    }

    /// Marks a key as exhausted for the current minute.
    pub fn mark_key_exhausted_at_unix_seconds(&self, key: &str, unix_seconds: u64) {
        let mut state = self
            .state
            .lock()
            .expect("rate limiter state mutex should not be poisoned");
        sync_minute(&mut state, unix_seconds);
        state
            .key_counts
            .insert(key.to_string(), self.config.per_key_per_minute);
    }

    /// Marks the IP budget as exhausted for the current minute.
    pub fn mark_ip_exhausted_at_unix_seconds(&self, unix_seconds: u64) {
        let mut state = self
            .state
            .lock()
            .expect("rate limiter state mutex should not be poisoned");
        sync_minute(&mut state, unix_seconds);
        state.ip_count = self.config.per_ip_per_minute;
    }

    /// Returns wait duration until next minute window.
    pub fn wait_duration_until_next_minute(unix_seconds: u64) -> Duration {
        wait_until_next_minute(unix_seconds)
    }
}

fn sync_minute(state: &mut RateLimitState, unix_seconds: u64) {
    let minute_index = unix_seconds / 60;
    if state.minute_index != minute_index {
        state.minute_index = minute_index;
        state.ip_count = 0;
        state.key_counts.clear();
    }
}

fn wait_until_next_minute(unix_seconds: u64) -> Duration {
    let remainder = unix_seconds % 60;
    let wait_secs = if remainder == 0 { 60 } else { 60 - remainder };
    Duration::from_secs(wait_secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enforces_per_key_limit_and_resets_on_minute_boundary() {
        let limiter = RateLimiter::new(RateLimitConfig {
            per_key_per_minute: 2,
            per_ip_per_minute: 1000,
        });
        let t = 30_u64;

        assert_eq!(
            limiter.try_acquire_at_unix_seconds("key-a", t),
            AcquireResult::Acquired
        );
        assert_eq!(
            limiter.try_acquire_at_unix_seconds("key-a", t),
            AcquireResult::Acquired
        );
        assert_eq!(
            limiter.try_acquire_at_unix_seconds("key-a", t),
            AcquireResult::Wait {
                duration: Duration::from_secs(30)
            }
        );

        assert_eq!(
            limiter.try_acquire_at_unix_seconds("key-a", 60),
            AcquireResult::Acquired
        );
    }

    #[test]
    fn enforces_per_ip_limit_across_keys() {
        let limiter = RateLimiter::new(RateLimitConfig {
            per_key_per_minute: 100,
            per_ip_per_minute: 2,
        });
        let t = 10_u64;

        assert_eq!(
            limiter.try_acquire_at_unix_seconds("key-a", t),
            AcquireResult::Acquired
        );
        assert_eq!(
            limiter.try_acquire_at_unix_seconds("key-b", t),
            AcquireResult::Acquired
        );
        assert_eq!(
            limiter.try_acquire_at_unix_seconds("key-c", t),
            AcquireResult::Wait {
                duration: Duration::from_secs(50)
            }
        );
    }

    #[test]
    fn marks_key_or_ip_exhausted_for_current_minute() {
        let limiter = RateLimiter::new(RateLimitConfig {
            per_key_per_minute: 2,
            per_ip_per_minute: 3,
        });
        let t = 61_u64;

        assert_eq!(
            limiter.try_acquire_at_unix_seconds("key-a", t),
            AcquireResult::Acquired
        );
        limiter.mark_key_exhausted_at_unix_seconds("key-a", t);
        assert_eq!(
            limiter.try_acquire_at_unix_seconds("key-a", t),
            AcquireResult::Wait {
                duration: Duration::from_secs(59)
            }
        );

        limiter.mark_ip_exhausted_at_unix_seconds(t);
        assert_eq!(
            limiter.try_acquire_at_unix_seconds("key-b", t),
            AcquireResult::Wait {
                duration: Duration::from_secs(59)
            }
        );
    }
}
