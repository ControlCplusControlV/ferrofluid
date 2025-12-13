//! Example of using the WebSocket provider for real-time data

use ferrofluid::{providers::WsProvider, types::ws::Message, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Install crypto provider for rustls
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install crypto provider");
    // Connect to WebSocket
    let mut ws = WsProvider::connect(Network::Mainnet).await?;
    println!("Connected to Hyperliquid WebSocket");

    // Subscribe to BTC order book
    let (_btc_book_id, mut btc_book_rx) = ws.subscribe_l2_book("BTC").await?;
    println!("Subscribed to BTC L2 book");

    // Subscribe to all mid prices
    let (_mids_id, mut mids_rx) = ws.subscribe_all_mids().await?;
    println!("Subscribed to all mids");

    // Start reading messages
    ws.start_reading().await?;

    // Handle messages for a limited time (10 seconds for demo)
    let mut message_count = 0;
    let timeout = tokio::time::sleep(std::time::Duration::from_secs(10));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            // Handle BTC book updates
            Some(msg) = btc_book_rx.recv() => {
                if let Message::L2Book(book) = msg {
                    println!("BTC book update:");
                    println!("  Coin: {}", book.data.coin);
                    println!("  Time: {}", book.data.time);
                    if let Some(bids) = book.data.levels.first() {
                        if let Some(best_bid) = bids.first() {
                            println!("  Best bid: {} @ {}", best_bid.sz, best_bid.px);
                        }
                    }
                    if let Some(asks) = book.data.levels.get(1) {
                        if let Some(best_ask) = asks.first() {
                            println!("  Best ask: {} @ {}", best_ask.sz, best_ask.px);
                        }
                    }
                    message_count += 1;
                }
            }

            // Handle all mids updates
            Some(msg) = mids_rx.recv() => {
                if let Message::AllMids(mids) = msg {
                    println!("\nMid prices update:");
                    for (coin, price) in mids.data.mids.iter().take(5) {
                        println!("  {}: {}", coin, price);
                    }
                    println!("  ... and {} more", mids.data.mids.len().saturating_sub(5));
                    message_count += 1;
                }
            }

            // Handle timeout
            _ = &mut timeout => {
                println!("\nDemo timeout reached after 10 seconds");
                break;
            }

            // Handle channel closure
            else => {
                println!("\nAll channels closed, exiting");
                break;
            }
        }

        // Optional: Exit after certain number of messages
        if message_count >= 20 {
            println!("\nReceived {} messages, exiting demo", message_count);
            break;
        }
    }

    println!("WebSocket demo completed successfully!");
    Ok(())
}
