//! Requests StoreKit 2 products through the native Swift ABI.
//!
//! ```sh
//! cargo run --example sk-products --features "sk,async,macos_15_0" -- yoml.pro.monthly
//! ```
//!
//! Without an App Store configuration for the running binary the call comes
//! back with either an empty array or a StoreKit error; both prove the async
//! generic call round-trips.

use cidre::swift::store_kit::Product;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ids: Vec<&str> = if args.is_empty() {
        vec!["yoml.pro.monthly"]
    } else {
        args.iter().map(String::as_str).collect()
    };

    println!("requesting {} product(s): {ids:?}", ids.len());

    match Product::products(&ids).await {
        Ok(products) => {
            println!("received {} product(s)", products.len());
            for product in products.iter() {
                println!(
                    "  {} — {} ({})",
                    product.id(),
                    product.display_name(),
                    product.display_price()
                );
            }
        }
        Err(err) => {
            println!(
                "error: domain={:?} code={} :: {}",
                err.domain(),
                err.code(),
                err.localized_desc()
            );
        }
    }
}
