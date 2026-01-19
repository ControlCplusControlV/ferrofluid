//! Tests for ManagedExchangeProvider

use std::time::Duration;

use alloy::signers::local::PrivateKeySigner;
use ferrofluid::{
    constants::*,
    providers::{ManagedExchangeProvider, OrderHandle},
    types::requests::{Limit, OrderRequest, OrderType},
};

#[tokio::test]
async fn test_managed_provider_creation() {
    // Initialize CryptoProvider for rustls
    rustls::crypto::CryptoProvider::install_default(
        rustls::crypto::ring::default_provider(),
    )
    .ok();

    // Create a test signer
    let signer = PrivateKeySigner::random();

    // Create managed provider with default config
    let exchange = ManagedExchangeProvider::builder(signer)
        .with_network(ferrofluid::Network::Testnet)
        .build()
        .await;

    assert!(exchange.is_ok());
}

#[tokio::test]
async fn test_managed_provider_with_batching() {
    // Initialize CryptoProvider for rustls
    rustls::crypto::CryptoProvider::install_default(
        rustls::crypto::ring::default_provider(),
    )
    .ok();

    let signer = PrivateKeySigner::random();

    // Create with batching enabled
    let exchange = ManagedExchangeProvider::builder(signer)
        .with_network(ferrofluid::Network::Testnet)
        .with_auto_batching(Duration::from_millis(50))
        .without_agent_rotation() // Disable for testing
        .build()
        .await
        .unwrap();

    // Create a test order
    let order = OrderRequest {
        asset: 0,
        is_buy: true,
        limit_px: "50000".to_string(),
        sz: "0.01".to_string(),
        reduce_only: false,
        order_type: OrderType::Limit(Limit {
            tif: TIF_GTC.to_string(),
        }),
        cloid: None,
    };

    // Place order should return pending handle
    let handle = exchange.place_order(&order).await.unwrap();

    match handle {
        OrderHandle::Pending { .. } => {
            // Expected for batched orders
        }
        OrderHandle::Immediate(_) => {
            panic!("Expected pending handle for batched order");
        }
    }
}

#[tokio::test]
async fn test_alo_order_detection() {
    let order = OrderRequest {
        asset: 0,
        is_buy: true,
        limit_px: "50000".to_string(),
        sz: "0.01".to_string(),
        reduce_only: false,
        order_type: OrderType::Limit(Limit {
            tif: "Alo".to_string(),
        }),
        cloid: None,
    };

    assert!(order.is_alo());

    let regular_order = OrderRequest {
        asset: 0,
        is_buy: true,
        limit_px: "50000".to_string(),
        sz: "0.01".to_string(),
        reduce_only: false,
        order_type: OrderType::Limit(Limit {
            tif: "Gtc".to_string(),
        }),
        cloid: None,
    };

    assert!(!regular_order.is_alo());
}

#[test]
fn test_nonce_generation() {
    use ferrofluid::providers::nonce::NonceManager;

    let manager = NonceManager::new(false);

    let nonce1 = manager.next_nonce(None);
    let nonce2 = manager.next_nonce(None);

    assert!(nonce2 > nonce1);
    assert!(NonceManager::is_valid_nonce(nonce1));
    assert!(NonceManager::is_valid_nonce(nonce2));
}

#[test]
fn test_nonce_isolation() {
    use alloy::primitives::Address;
    use ferrofluid::providers::nonce::NonceManager;

    let manager = NonceManager::new(true);
    let addr1 = Address::new([1u8; 20]);
    let addr2 = Address::new([2u8; 20]);

    // Get initial nonces - these should have different millisecond timestamps
    let n1_1 = manager.next_nonce(Some(addr1));
    std::thread::sleep(std::time::Duration::from_millis(1));
    let n2_1 = manager.next_nonce(Some(addr2));
    std::thread::sleep(std::time::Duration::from_millis(1));
    let n1_2 = manager.next_nonce(Some(addr1));
    std::thread::sleep(std::time::Duration::from_millis(1));
    let n2_2 = manager.next_nonce(Some(addr2));

    // Each address should have independent, increasing nonces
    assert!(n1_2 > n1_1, "addr1 nonces should increase");
    assert!(n2_2 > n2_1, "addr2 nonces should increase");

    // Verify counter independence using the manager's get_counter method
    assert_eq!(manager.get_counter(Some(addr1)), 2); // addr1 has 2 nonces
    assert_eq!(manager.get_counter(Some(addr2)), 2); // addr2 has 2 nonces

    // The nonces themselves should be unique
    assert_ne!(n1_1, n2_1);
    assert_ne!(n1_2, n2_2);
}
