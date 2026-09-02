//! StoreKit.framework native Swift ABI bindings.
//!
//! These bindings target StoreKit 2, the Swift-native API introduced in Apple
//! OS 12/15, and call framework and Swift runtime symbols directly. No C or
//! Objective-C wrapper functions are used.

mod product;

pub use product::Product;

#[link(name = "StoreKit", kind = "framework")]
unsafe extern "C" {}

#[cfg(test)]
mod tests {
    use super::*;

    /// A binary that is not an App Store app gets a StoreKit error rather
    /// than products, and that is the whole generic async call round-tripping:
    /// the task is spawned, the `[String]` instantiation dispatched, and the
    /// thrown error bridged back.
    #[test]
    fn products_for_reports_its_result() {
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        Product::products_handler(&["cidre.test.product"], move |res| {
            let _ = tx.send(res.map(|products| {
                products
                    .iter()
                    .map(|p| (p.id().to_string(), p.display_price().to_string()))
                    .collect::<Vec<_>>()
            }));
        });

        match rx
            .recv_timeout(std::time::Duration::from_secs(30))
            .expect("products callback")
        {
            Ok(products) => println!("products: {products:?}"),
            Err(err) => println!("refused: code {} :: {}", err.code(), err.localized_desc()),
        }
    }
}
