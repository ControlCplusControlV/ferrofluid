use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;
use ferrofluid::constants::TIF_GTC;
use ferrofluid::types::requests::OrderRequest;
use ferrofluid::{signers::AlloySigner, ExchangeProvider};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the crypto provider for TLS
    rustls::crypto::CryptoProvider::install_default(
        rustls::crypto::aws_lc_rs::default_provider(),
    )
    .expect("Failed to install rustls crypto provider");
    // Create a test signer (DO NOT USE IN PRODUCTION)
    let private_key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer = private_key.parse::<PrivateKeySigner>()?;
    let alloy_signer = AlloySigner { inner: signer };

    // Create ExchangeProvider for testnet
    let exchange = ExchangeProvider::testnet(alloy_signer);

    println!("ExchangeProvider created successfully!");

    // Example 1: Create an order with client order ID
    let cloid = Uuid::new_v4();
    let _order = OrderRequest::limit(
        0,         // BTC-USD asset ID
        true,      // buy
        "45000.0", // price
        "0.01",    // size
        TIF_GTC,
    );

    println!("\nOrder created:");
    println!("- Asset: 0 (BTC-USD)");
    println!("- Side: Buy");
    println!("- Price: $45,000");
    println!("- Size: 0.01");
    println!("- Client Order ID: {}", cloid);

    // Example 2: Using OrderBuilder pattern
    let _builder_order = exchange
        .order(0)
        .buy()
        .limit_px("45000.0")
        .size("0.01")
        .cloid(Uuid::new_v4());

    println!("\nOrderBuilder created successfully!");

    // Example 3: Create bulk orders with mixed tracking
    let orders_with_ids = vec![
        (
            OrderRequest::limit(0, true, "44900.0", "0.01", TIF_GTC),
            Some(Uuid::new_v4()),
        ),
        (
            OrderRequest::limit(0, true, "44800.0", "0.01", TIF_GTC),
            None,
        ),
        (
            OrderRequest::limit(0, true, "44700.0", "0.01", TIF_GTC),
            Some(Uuid::new_v4()),
        ),
    ];

    println!("\nBulk orders created:");
    for (i, (order, cloid)) in orders_with_ids.iter().enumerate() {
        println!(
            "- Order {}: price={}, cloid={:?}",
            i + 1,
            &order.limit_px,
            cloid.as_ref().map(|id| id.to_string())
        );
    }

    // Example 4: Different constructor types
    let vault_address: Address = "0x742d35Cc6634C0532925a3b844Bc9e7595f8fA49".parse()?;
    let vault_signer = private_key.parse::<PrivateKeySigner>()?;
    let _vault_exchange = ExchangeProvider::testnet_vault(
        AlloySigner {
            inner: vault_signer,
        },
        vault_address,
    );
    println!(
        "\nVault ExchangeProvider created for address: {}",
        vault_address
    );

    let agent_address: Address = "0x742d35Cc6634C0532925a3b844Bc9e7595f8fA49".parse()?;
    let agent_signer = private_key.parse::<PrivateKeySigner>()?;
    let _agent_exchange = ExchangeProvider::testnet_agent(
        AlloySigner {
            inner: agent_signer,
        },
        agent_address,
    );
    println!(
        "Agent ExchangeProvider created for address: {}",
        agent_address
    );

    println!("\nAll examples completed successfully!");
    println!("\nNOTE: This example only tests object creation, not actual API calls.");
    println!("To test actual trading, you would need:");
    println!("1. A funded testnet account");
    println!("2. Valid asset IDs for the testnet");
    println!("3. Appropriate risk controls");

    Ok(())
}
