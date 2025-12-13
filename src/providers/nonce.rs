//! Nonce management for Hyperliquid's sliding window system

use alloy::primitives::Address;
use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Manages nonces for Hyperliquid's sliding window system
///
/// Hyperliquid stores the 100 highest nonces per address and requires:
/// - New nonce > smallest in the set
/// - Never reuse a nonce
/// - Nonces must be within (T - 2 days, T + 1 day)
#[derive(Debug)]
pub struct NonceManager {
    /// Separate nonce counters per address for subaccount isolation
    counters: DashMap<Address, AtomicU64>,
    /// Global counter for addresses without isolation
    global_counter: AtomicU64,
    /// Whether to isolate nonces per address
    isolate_per_address: bool,
}

impl NonceManager {
    /// Create a new nonce manager
    pub fn new(isolate_per_address: bool) -> Self {
        Self {
            counters: DashMap::new(),
            global_counter: AtomicU64::new(0),
            isolate_per_address,
        }
    }

    /// Get the next nonce for an optional address
    pub fn next_nonce(&self, address: Option<Address>) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // Get counter increment
        let counter = if self.isolate_per_address && address.is_some() {
            let addr = address.unwrap();
            self.counters
                .entry(addr)
                .or_insert_with(|| AtomicU64::new(0))
                .fetch_add(1, Ordering::Relaxed)
        } else {
            self.global_counter.fetch_add(1, Ordering::Relaxed)
        };

        // Add sub-millisecond offset to ensure uniqueness
        // This handles rapid-fire orders within the same millisecond
        now.saturating_add(counter % 1000)
    }

    /// Reset counter for a specific address (useful after agent rotation)
    pub fn reset_address(&self, address: Address) {
        if let Some(counter) = self.counters.get_mut(&address) {
            counter.store(0, Ordering::Relaxed);
        }
    }

    /// Get current counter value for monitoring
    pub fn get_counter(&self, address: Option<Address>) -> u64 {
        if let Some(addr) = address {
            if self.isolate_per_address {
                self.counters
                    .get(&addr)
                    .map(|c| c.load(Ordering::Relaxed))
                    .unwrap_or(0)
            } else {
                self.global_counter.load(Ordering::Relaxed)
            }
        } else {
            self.global_counter.load(Ordering::Relaxed)
        }
    }

    /// Check if a nonce is within valid time bounds
    pub fn is_valid_nonce(nonce: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // Must be within (T - 2 days, T + 1 day)
        const TWO_DAYS_MS: u64 = 2 * 24 * 60 * 60 * 1000;
        const ONE_DAY_MS: u64 = 24 * 60 * 60 * 1000;

        nonce > now.saturating_sub(TWO_DAYS_MS) && nonce < now.saturating_add(ONE_DAY_MS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonce_uniqueness() {
        let manager = NonceManager::new(false);

        let nonce1 = manager.next_nonce(None);
        let nonce2 = manager.next_nonce(None);

        assert_ne!(nonce1, nonce2);
        assert!(nonce2 > nonce1);
    }

    #[test]
    fn test_address_isolation() {
        let manager = NonceManager::new(true);
        let addr1 = Address::new([1u8; 20]);
        let addr2 = Address::new([2u8; 20]);

        // Get nonces for different addresses
        let n1_1 = manager.next_nonce(Some(addr1));
        let n2_1 = manager.next_nonce(Some(addr2));
        let n1_2 = manager.next_nonce(Some(addr1));
        let n2_2 = manager.next_nonce(Some(addr2));

        // Each address should have independent counters
        assert!(n1_2 > n1_1);
        assert!(n2_2 > n2_1);

        // Counters should be independent
        assert_eq!(manager.get_counter(Some(addr1)), 2);
        assert_eq!(manager.get_counter(Some(addr2)), 2);
    }

    #[test]
    fn test_nonce_validity() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // Valid: current time
        assert!(NonceManager::is_valid_nonce(now));

        // Valid: 1 day ago
        assert!(NonceManager::is_valid_nonce(now - 24 * 60 * 60 * 1000));

        // Invalid: 3 days ago
        assert!(!NonceManager::is_valid_nonce(now - 3 * 24 * 60 * 60 * 1000));

        // Invalid: 2 days in future
        assert!(!NonceManager::is_valid_nonce(now + 2 * 24 * 60 * 60 * 1000));
    }
}
