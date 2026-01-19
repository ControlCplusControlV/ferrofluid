//! Example showing how to use the Symbol type

use ferrofluid::types::{symbols, Symbol};

fn main() {
    // Using predefined constants
    println!("BTC symbol: {}", symbols::BTC);
    println!("Is BTC a perp? {}", symbols::BTC.is_perp());

    println!("HYPE spot symbol: {}", symbols::HYPE_USDC);
    println!("Is HYPE a spot? {}", symbols::HYPE_USDC.is_spot());

    // Creating symbols at runtime
    let new_coin = symbols::symbol("NEWCOIN");
    println!("New coin: {}", new_coin);

    // From string literals
    let btc: Symbol = "BTC".into();
    println!("BTC from string: {}", btc);

    // From String
    let eth = String::from("ETH");
    let eth_symbol: Symbol = eth.into();
    println!("ETH from String: {}", eth_symbol);

    // Function that accepts symbols
    print_symbol_info(symbols::BTC);
    print_symbol_info("DOGE");
    print_symbol_info(String::from("@999"));
}

fn print_symbol_info(symbol: impl Into<Symbol>) {
    let sym = symbol.into();
    println!(
        "Symbol: {}, Type: {}",
        sym,
        if sym.is_perp() { "Perpetual" } else { "Spot" }
    );
}
