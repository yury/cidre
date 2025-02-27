#!/bin/bash
set -e

cargo b --target aarch64-apple-ios
cargo b --target aarch64-apple-ios-sim
cargo +nightly b -Zbuild-std --target aarch64-apple-tvos
cargo +nightly b -Zbuild-std --target aarch64-apple-tvos-sim
# cargo +nightly b -Zbuild-std --target arm64_32-apple-watchos
cargo b --target aarch64-apple-darwin
cargo b --target x86_64-apple-darwin
# cargo +nightly b -Zbuild-std --target aarch64-apple-visionos
# cargo +nightly b -Zbuild-std --target aarch64-apple-visionos-sim
cargo install --path ./cargo-box

cargo b --no-default-features --features="macos_12_0,vt,cm,mtl,dispatch"
