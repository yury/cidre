cargo b --target aarch64-apple-ios
cargo b --target aarch64-apple-ios-sim
cargo +nightly b -Zbuild-std --target aarch64-apple-tvos
cargo +nightly b -Zbuild-std --target aarch64-apple-tvos-sim
cargo b --target aarch64-apple-darwin
cargo b --target x86_64-apple-darwin

