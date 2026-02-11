# Repository Guidelines

## Project Structure & Module Organization
- `cidre/` is the main Rust crate with source in `cidre/src`, examples in `cidre/examples`, benches in `cidre/benches`, and a `build.rs` for platform config.
- `cidre-macros/` provides proc-macro support used by the main crate.
- `cargo-box/` contains a Cargo subcommand for device testing and Xcode integration.
- Top-level `build.sh` is a convenience script for multi-target builds and local tooling setup.

## Apple Binding Workflow
- Place new Apple SDK bindings in header-aligned files: `cidre/src/<module>/<header_name>.rs` (snake_case file names).
- Keep types in the module that matches the framework/header they come from; avoid catch-all files for unrelated symbols.
- Wire exports in the module root with `mod <file>;` and `pub use ...;` so public API surfaces from the module root.
- If a bound Objective-C class uses a static class symbol, add it to the corresponding `cidre/pomace/<framework>/<framework>.h` initializer.
- Mirror SDK availability in Rust using `#[objc::available(...)]` and keep `#[doc(alias = "...")]` for Apple symbol names.

## Build, Test, and Development Commands
- `cargo build` builds the workspace for the host target.
- `cargo test` runs unit tests (platform availability may gate some modules).
- `cargo bench -p cidre` runs Criterion benches defined in `cidre/benches`.
- `cargo run -p cidre --example <name>` runs examples from `cidre/examples` (e.g., `sc-record`).
- `./build.sh` builds multiple Apple targets and installs `cargo-box`.
- `cargo install --path ./cargo-box` installs the device runner plugin.
- For binding work, prefer `cargo check -p cidre --no-default-features --features <feature-set>` to validate feature gating quickly.

## Coding Style & Naming Conventions
- Use standard Rust formatting (`cargo fmt`) and idiomatic Rust style; keep APIs `snake_case` and types `CamelCase`.
- Follow the project’s abbreviation shortcuts from `README.md` (e.g., `fmt`, `cfg`, `opts`, `ptr`, `dst`).
- Feature flags encode deployment targets (e.g., `macos_15_0`, `ios_18_0`); keep new flags consistent.

## Feature-Gating Rules
- Guard imports from optional modules (`blocks`, `ca`, etc.) with `#[cfg(feature = "...")]`.
- Guard methods/types that use optional modules with the same feature gates.
- When adding APIs that are only available on specific OS versions, annotate with `#[objc::available(...)]`.
- Avoid adding unconditional references to optional modules in shared files; this breaks minimal feature builds.

## Testing Guidelines
- The project relies on `cargo test` for unit checks and Criterion for performance benches.
- Bench names map to files under `cidre/benches` and use `[[bench]]` entries in `cidre/Cargo.toml`.
- Device testing uses `cargo box` and a `.box` config file; see `README.md` for the iOS runner steps.
- For `ns/app` binding changes, validate at least `app,blocks,ca`.
- When practical, also validate reduced combinations relevant to the touched code (for example `app` or `app,ca`) to catch missing cfg-gates.

## Commit & Pull Request Guidelines
- Commit messages in this repo are short, imperative, and lowercase (e.g., “update criterion”, “tune api”).
- PRs should describe the Apple frameworks touched, target platforms, and any feature flags required.
- Include example commands or sample usage when adding new modules or API bindings.

## Platform & Configuration Notes
- Deployment targets are controlled by feature flags; check selector availability before unsafe calls.
- Some builds require `+nightly` and `-Zbuild-std` for non-host Apple targets.
- `cidre/build.rs` invokes `xcodebuild` for enabled framework targets, so Xcode toolchain access and writable build/derived-data paths are required.
