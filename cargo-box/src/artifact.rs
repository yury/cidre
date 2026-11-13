//! Cargo hands runners an executable path, without a compiler-artifact message.
//! Use metadata to identify the workspace and target, including test harnesses,
//! rather than treating Cargo's internal build directory layout as an API.

use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Deserialize)]
pub(crate) struct Metadata {
    pub(crate) workspace_root: PathBuf,
    packages: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    targets: Vec<Target>,
}

#[derive(Deserialize)]
struct Target {
    name: String,
    kind: Vec<String>,
}

pub(crate) fn metadata() -> Metadata {
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .args(["metadata", "--format-version=1", "--no-deps", "--offline"])
        .output()
        .expect("cargo box: failed to run cargo metadata");
    assert!(
        output.status.success(),
        "cargo box: cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("cargo box: invalid cargo metadata")
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Binary,
    Example,
    Dep,
}

#[derive(Debug)]
pub(crate) struct Artifact {
    pub(crate) name: String,
    pub(crate) kind: Kind,
    pub(crate) sdk: &'static str,
    pub(crate) config: &'static str,
}

impl Artifact {
    pub(crate) fn resolve(path: &Path, metadata: &Metadata) -> Result<Self, String> {
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("invalid executable name")?;
        let targets = || metadata.packages.iter().flat_map(|p| &p.targets);
        // Prefer an exact target name: a binary may itself end in `-<hex>`.
        let is_example = path
            .parent()
            .and_then(Path::file_name)
            .is_some_and(|p| p == "examples");
        let direct = targets().find_map(|t| {
            if t.name != filename {
                return None;
            }
            if is_example && t.kind.iter().any(|k| k == "example") {
                Some((t.name.clone(), Kind::Example))
            } else if !is_example && t.kind.iter().any(|k| k == "bin") {
                Some((t.name.clone(), Kind::Binary))
            } else {
                None
            }
        });
        let (name, kind) = direct
            .or_else(|| {
                let (name, hash) = filename.rsplit_once('-')?;
                if hash.is_empty() || !hash.bytes().all(|c| c.is_ascii_hexdigit()) {
                    return None;
                }
                targets()
                    .find(|t| t.name.replace('-', "_") == name)
                    .map(|_| (name.to_owned(), Kind::Dep))
            })
            .ok_or_else(|| format!("no Cargo target matches {}", path.display()))?;

        // Platform/profile components are still needed by Xcode, but their
        // distance from the executable and the build directory name are not.
        let sdk = path
            .ancestors()
            .filter_map(|p| p.file_name()?.to_str())
            .find_map(|name| {
                Some(match name {
                    "aarch64-apple-darwin" | "x86_64-apple-darwin" => "macos",
                    "aarch64-apple-ios" => "iphoneos",
                    "aarch64-apple-ios-sim" | "x86_64-apple-ios" => "iphonesimulator",
                    "aarch64-apple-tvos" => "appletvos",
                    "aarch64-apple-tvos-sim" | "x86_64-apple-tvos" => "appletvsimulator",
                    "arm64_32-apple-watchos" | "aarch64-apple-watchos" => "watchos",
                    "aarch64-apple-watchos-sim" | "x86_64-apple-watchos-sim" => "watchsimulator",
                    "aarch64-apple-visionos" => "xros",
                    "aarch64-apple-visionos-sim" => "xrsimulator",
                    _ => return None,
                })
            })
            .unwrap_or("macos");
        // Custom Cargo profiles contain an already compiled executable; Debug
        // here controls only Xcode's packaging configuration.
        let config = path
            .ancestors()
            .filter_map(|p| p.file_name()?.to_str())
            .find_map(|p| match p {
                "release" => Some("Release"),
                "debug" => Some("Debug"),
                _ => None,
            })
            .unwrap_or("Debug");
        Ok(Self {
            name,
            kind,
            sdk,
            config,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn metadata() -> Metadata {
        serde_json::from_value(serde_json::json!({
            "workspace_root": "/workspace", "packages": [{"targets": [
                {"name": "demo", "kind": ["lib"]},
                {"name": "my-app", "kind": ["lib"]},
                {"name": "my-app", "kind": ["bin"]},
                {"name": "app-deadbeef", "kind": ["bin"]},
                {"name": "hello", "kind": ["example"]},
                {"name": "integration", "kind": ["test"]},
                {"name": "speed", "kind": ["bench"]}
            ]}]
        }))
        .unwrap()
    }
    #[test]
    fn test_layouts_and_platforms() {
        for (path, sdk, config) in [
            ("/workspace/target/debug/deps/demo-ab0123", "macos", "Debug"),
            (
                "/cache/aarch64-apple-ios/release/build/demo/123/out/demo-ab0123",
                "iphoneos",
                "Release",
            ),
            (
                "/cache/aarch64-apple-ios-sim/debug/build/demo/123/out/demo-ab0123",
                "iphonesimulator",
                "Debug",
            ),
            (
                "/cache/aarch64-apple-tvos/debug/build/demo/123/out/demo-ab0123",
                "appletvos",
                "Debug",
            ),
        ] {
            let a = Artifact::resolve(Path::new(path), &metadata()).unwrap();
            assert_eq!(
                (a.name.as_str(), a.kind, a.sdk, a.config),
                ("demo", Kind::Dep, sdk, config)
            );
        }
    }
    #[test]
    fn target_kinds_and_names() {
        for (filename, name, kind) in [
            ("my-app", "my-app", Kind::Binary),
            ("app-deadbeef", "app-deadbeef", Kind::Binary),
            ("my_app-ab0123", "my_app", Kind::Dep),
            ("hello-ab0123", "hello", Kind::Dep),
            ("integration-ab0123", "integration", Kind::Dep),
            ("speed-ab0123", "speed", Kind::Dep),
        ] {
            let a = Artifact::resolve(
                &Path::new("/cache/custom-profile").join(filename),
                &metadata(),
            )
            .unwrap();
            assert_eq!((a.name.as_str(), a.kind), (name, kind));
        }
        let example =
            Artifact::resolve(Path::new("/cache/debug/examples/hello"), &metadata()).unwrap();
        assert_eq!(example.kind, Kind::Example);
        assert!(Artifact::resolve(Path::new("/cache/unknown-ab0123"), &metadata()).is_err());
        assert!(Artifact::resolve(Path::new("/cache/demo-not-a-hash"), &metadata()).is_err());
    }
    #[test]
    fn resolves_actual_cargo_test_artifact() {
        let path = std::env::temp_dir().join(format!("cargo-box-layout-{}", std::process::id()));
        std::fs::create_dir_all(path.join("src")).unwrap();
        let path = std::mem::DropGuard::new(path, |p| {
            let _ = std::fs::remove_dir_all(p);
        });
        std::fs::write(
            path.join("Cargo.toml"),
            "[package]\nname=\"layout-fixture\"\nversion=\"0.1.0\"\nedition=\"2024\"\n",
        )
        .unwrap();
        std::fs::write(path.join("src/lib.rs"), "#[test] fn runs() {}\n").unwrap();
        let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
        let output = Command::new(&cargo)
            .current_dir(&*path)
            .args([
                "test",
                "--offline",
                "--no-run",
                "--message-format=json",
                "--target-dir",
                "cache",
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let executable = String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
            .find_map(|v| v["executable"].as_str().map(PathBuf::from))
            .unwrap();
        let output = Command::new(cargo)
            .current_dir(&*path)
            .args(["metadata", "--offline", "--no-deps", "--format-version=1"])
            .output()
            .unwrap();
        assert!(output.status.success());
        let metadata = serde_json::from_slice(&output.stdout).unwrap();
        let artifact = Artifact::resolve(&executable, &metadata).unwrap();
        assert_eq!(
            (artifact.name.as_str(), artifact.kind),
            ("layout_fixture", Kind::Dep)
        );
    }
}
