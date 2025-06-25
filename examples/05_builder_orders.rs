//! Example of using builder functionality for orders

use alloy::primitives::{address, B256};
use alloy::signers::local::PrivateKeySigner;
use ferrofluid::{providers::ExchangeProvider, signers::AlloySigner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example private key (DO NOT USE IN PRODUCTION)
    let private_key = B256::from([1u8; 32]);
    let signer = PrivateKeySigner::from_bytes(&private_key)?;
    let hyperliquid_signer = AlloySigner { inner: signer };

    // Builder address (example)
    let builder_address = address!("1234567890123456789012345678901234567890");

    // Create exchange provider with builder configured
    let exchange = ExchangeProvider::mainnet_builder(hyperliquid_signer, builder_address);

    println!(
        "Exchange provider configured with builder: {:?}",
        exchange.builder()
    );

    // Now all orders placed through this provider will automatically include builder info

    // Example 1: Simple order (builder fee = 0)
    // Note: In a real application, you would check the result
    // BTC perpetual is asset index 0
    let _order = exchange.order(0).limit_buy("109000", "0.001").send();
    println!("Order would be placed with default builder fee");

    // Example 2: Order with specific builder fee
    // ETH perpetual is asset index 1
    let order_request = exchange.order(1).limit_sell("2700", "0.1").build()?;

    // Place with specific builder fee (e.g., 10 bps = 10)
    // Note: In a real application, you would await this
    let _result = exchange.place_order_with_builder_fee(&order_request, 10);
    println!("Order would be placed with 10 bps builder fee");

    // Example 3: Bulk orders with builder
    let orders = vec![
        exchange.order(0).limit_buy("108000", "0.001").build()?,
        exchange.order(0).limit_buy("107000", "0.001").build()?,
    ];

    // All orders in the bulk will include builder info
    let _result = exchange.bulk_orders(orders);
    println!("Bulk orders would be placed with builder");

    // Example 4: Creating provider with all options
    let signer2 = PrivateKeySigner::from_bytes(&private_key)?;
    let hyperliquid_signer2 = AlloySigner { inner: signer2 };

    let _exchange_full = ExchangeProvider::mainnet_with_options(
        hyperliquid_signer2,
        None,                  // No vault
        None,                  // No agent
        Some(builder_address), // With builder
    );

    println!("Provider created with builder support!");

    // Example 5: Using builder with approved fee
    // First approve the builder for a max fee rate (done once)
    let private_key3 = B256::from([2u8; 32]);
    let signer3 = PrivateKeySigner::from_bytes(&private_key3)?;
    let hyperliquid_signer3 = AlloySigner { inner: signer3 };
    let exchange3 = ExchangeProvider::mainnet(hyperliquid_signer3);

    // Approve builder for max 50 bps (0.5%)
    let _approval = exchange3.approve_builder_fee(builder_address, "0.005".to_string());
    println!("Builder fee approval would be sent");

    Ok(())
}
