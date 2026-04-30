use std::{env, fs::read_to_string, path::PathBuf, process::Command, str::FromStr, thread};

#[derive(Debug, Clone, Copy)]
struct Version {
    major: u32,
    minor: u32,
}

#[derive(Debug, Clone, Default)]
struct DeploymentTargets {
    macos: Option<Version>,
    ios: Option<Version>,
    tvos: Option<Version>,
    watchos: Option<Version>,
    visionos: Option<Version>,
}

impl Version {
    fn with_table(table: &[(String, Self)]) -> Option<Self> {
        for record in table {
            if env::var_os(&record.0).is_some() {
                return Some(record.1);
            }
        }
        None
    }

    fn with_str(str: &str) -> Option<Self> {
        let mut res = Self { major: 0, minor: 0 };
        let mut iter = str.split("_").into_iter();
        let Some(s) = iter.next() else {
            return None;
        };

        res.major = str::parse(s).unwrap();

        let Some(s) = iter.next() else {
            return None;
        };

        res.minor = str::parse(s).unwrap();

        Some(res)
    }

    fn to_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

fn has_feature(name: &str) -> bool {
    env::var_os(format!("CARGO_FEATURE_{}", name.to_uppercase())).is_some()
}

fn build_pomace_target(name: &str, sdk: &str, deployment_targets: &DeploymentTargets) {
    if !has_feature(name) {
        return;
    }

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src = format!("{manifest_dir}/pomace/{name}/{name}.m");

    let mut build = cc::Build::new();
    build.file(&src);
    build.flag("-fobjc-arc");
    build.flag("-fobjc-weak");
    build.flag("-fno-common");
    // Ensure Foundation types (Class, NS_ASSUME_NONNULL_BEGIN, etc.) are
    // available even for targets that import C-only frameworks like CoreAudio.
    // Xcode does this implicitly for all Obj-C targets.
    build.flag("-include").flag("Foundation/Foundation.h");

    // Set deployment target flags
    match sdk {
        "macosx" | "maccatalyst" => {
            if let Some(v) = &deployment_targets.macos {
                build.flag(&format!("-mmacosx-version-min={}", v.to_string()));
            }
        }
        "iphoneos" | "iphonesimulator" => {
            if let Some(v) = &deployment_targets.ios {
                build.flag(&format!("-mios-version-min={}", v.to_string()));
            }
        }
        "appletvos" | "appletvsimulator" => {
            if let Some(v) = &deployment_targets.tvos {
                build.flag(&format!("-mtvos-version-min={}", v.to_string()));
            }
        }
        "watchos" | "watchsimulator" => {
            if let Some(v) = &deployment_targets.watchos {
                build.flag(&format!("-mwatchos-version-min={}", v.to_string()));
            }
        }
        _ => {}
    }

    build.compile(name);
}

fn collect_targets(sdk: &str) -> Vec<&'static str> {
    let mut targets = Vec::new();

    // Available on all platforms
    targets.extend_from_slice(&["ut", "un", "sn", "ns", "av", "cl", "nl", "ml", "at"]);

    // Not available on watchOS
    if sdk != "watchos" && sdk != "watchsimulator" {
        targets.extend_from_slice(&[
            "ca", "vn", "mps", "mpsg", "mc", "mtl", "mtk", "ci", "gc", "av_kit",
        ]);
        if sdk != "xros" && sdk != "xrsimulator" {
            targets.push("mlc");
        }

        if sdk != "appletvos"
            && sdk != "iphonesimulator"
            && sdk != "appletvsimulator"
            && sdk != "xrsimulator"
        {
            targets.push("mtl_fx");
        }
    }

    // Not available on tvOS
    if sdk != "appletvos" && sdk != "appletvsimulator" {
        targets.push("core_motion");
    }

    // Not available on tvOS or watchOS
    if sdk != "appletvos"
        && sdk != "appletvsimulator"
        && sdk != "watchos"
        && sdk != "watchsimulator"
    {
        targets.push("wk");
    }

    // iOS/tvOS/watchOS/visionOS/Catalyst only
    if [
        "iphoneos",
        "iphonesimulator",
        "maccatalyst",
        "appletvos",
        "appletvsimulator",
        "watchos",
        "watchsimulator",
        "xros",
        "xrsimulator",
    ]
    .contains(&sdk)
    {
        targets.push("ui");
    }

    // iPhone/iPad/Catalyst only
    if sdk == "iphoneos" || sdk == "iphonesimulator" {
        targets.push("wc");
    }
    if sdk == "iphoneos" || sdk == "iphonesimulator" || sdk == "maccatalyst" {
        targets.push("ar");
    }

    // macOS/Catalyst only
    if sdk == "macosx" || sdk == "maccatalyst" {
        targets.extend_from_slice(&["sc", "app"]);
    }

    // macOS only
    if sdk == "macosx" {
        targets.push("core_audio");
    }

    targets.retain(|target| has_feature(target));
    targets
}

fn parse_deployment_targets() -> DeploymentTargets {
    let path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from_str(&path).unwrap();
    let path = path.join("Cargo.toml");
    let str = read_to_string(path).unwrap();
    let mut lines_iter = str.lines().into_iter();
    while let Some(line) = lines_iter.next() {
        if !line.starts_with("# deployment targets") {
            continue;
        }
        break;
    }

    let mut macos = Vec::new();
    let mut ios = Vec::new();
    let mut tvos = Vec::new();
    let mut maccatalyst = Vec::new();
    let mut watchos = Vec::new();
    let mut visionos = Vec::new();

    while let Some(line) = lines_iter.next() {
        if line.starts_with("# end of deployment targets") {
            break;
        }

        let Some((val, _)) = line.split_once(" ") else {
            continue;
        };

        let Some((platform, ver_str)) = val.split_once("_") else {
            continue;
        };

        let Some(ver) = Version::with_str(ver_str) else {
            continue;
        };

        match platform {
            "macos" => macos.push((format!("CARGO_FEATURE_MACOS_{ver_str}"), ver)),
            "ios" => ios.push((format!("CARGO_FEATURE_IOS_{ver_str}"), ver)),
            "tvos" => tvos.push((format!("CARGO_FEATURE_TVOS_{ver_str}"), ver)),
            "maccatalyst" => {
                maccatalyst.push((format!("CARGO_FEATURE_MACCATALYST_{ver_str}"), ver))
            }
            "watchos" => watchos.push((format!("CARGO_FEATURE_WATCHOS_{ver_str}"), ver)),
            "visionos" => visionos.push((format!("CARGO_FEATURE_VISIONOS_{ver_str}"), ver)),
            _ => {
                panic!("unknown platform {platform:?}");
            }
        }
    }

    macos.reverse();
    ios.reverse();
    tvos.reverse();
    maccatalyst.reverse();
    watchos.reverse();
    visionos.reverse();

    let mut res = DeploymentTargets::default();

    if let Some(v) = Version::with_table(&macos) {
        res.macos = Some(v);
    }
    if let Some(v) = Version::with_table(&ios) {
        res.ios = Some(v);
    }
    if let Some(v) = Version::with_table(&tvos) {
        res.tvos = Some(v);
    }
    if let Some(v) = Version::with_table(&watchos) {
        res.watchos = Some(v);
    }
    if let Some(v) = Version::with_table(&maccatalyst) {
        // Mac Catalyst uses the iOS deployment target
        res.ios = Some(v);
    }
    if let Some(v) = Version::with_table(&visionos) {
        res.visionos = Some(v);
    }

    res
}

fn clang_link_search_path() -> String {
    // Use the CC env var if set (e.g. by Nix), otherwise fall back to
    // the cc crate's compiler detection.
    let compiler = env::var("CC").unwrap_or_else(|_| {
        cc::Build::new()
            .get_compiler()
            .path()
            .to_str()
            .unwrap()
            .to_string()
    });

    let output = Command::new(&compiler)
        .arg("--print-search-dirs")
        .output()
        .unwrap();
    if !output.status.success() {
        panic!("Can't get search paths from {compiler}");
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some((a, b)) = line.split_once('=') {
            if a == "libraries: " {
                return format!("{}/lib/darwin", b);
            }
        }
    }
    panic!("{compiler} is missing search paths");
}

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    let deployment_targets = parse_deployment_targets();

    let sdk = match env::var("TARGET").unwrap().as_ref() {
        "aarch64-apple-darwin" | "x86_64-apple-darwin" => "macosx",
        "aarch64-apple-ios" => "iphoneos",
        "x86_64-apple-ios" | "aarch64-apple-ios-sim" => "iphonesimulator",
        "aarch64-apple-ios-macabi" => "maccatalyst",
        "aarch64-apple-tvos" => "appletvos",
        "aarch64-apple-tvos-sim" => "appletvsimulator",
        "arm64_32-apple-watchos" => "watchos",
        "aarch64-apple-watchos" => "watchos",
        "aarch64-apple-watchos-sim" => "watchsimulator",
        "aarch64-apple-visionos" => "xros",
        "aarch64-apple-visionos-sim" => "xrsimulator",
        x => panic!("unknown tripple: {x}"),
    };

    let clang_rt = match sdk {
        "macosx" => "clang_rt.osx",
        "iphoneos" => "clang_rt.ios",
        "iphonesimulator" => "clang_rt.iossim",
        "appletvos" => "clang_rt.tvos",
        "appletvsimulator" => "clang_rt.tvossim",
        "watchos" => "clang_rt.watchos",
        "watchsimulator" => "clang_rt.watchossim",
        "xros" => "clang_rt.xros",
        "xrsimulator" => "clang_rt.xrossim",
        "maccatalyst" => "clang_rt.ios", // check
        x => panic!("unknown sdk {x}"),
    };

    println!("cargo:rustc-link-lib={}", clang_rt);
    println!("cargo:rustc-link-search={}", clang_link_search_path());

    if sdk == "maccatalyst" {
        // Mac Catalyst needs iOSSupport paths for framework and system headers
        let sdkroot = env::var("SDKROOT").unwrap_or_else(|_| {
            let c = Command::new("xcrun")
                .arg("--show-sdk-path")
                .output()
                .unwrap();
            String::from_utf8(c.stdout).unwrap().trim().to_string()
        });
        println!("cargo:rustc-link-search=system={sdkroot}/System/iOSSupport/usr");
        println!(
            "cargo:rustc-link-search=framework={sdkroot}/System/iOSSupport/System/Library/Frameworks"
        );
    }

    if (sdk == "macosx" || sdk == "maccatalyst") && has_feature("private") {
        println!("cargo:rustc-link-search=framework=/System/Library/PrivateFrameworks");
        println!(
            "cargo:rustc-link-search=framework=/Library/Apple/System/Library/PrivateFrameworks"
        );
    }

    println!("cargo:rerun-if-changed=./pomace/");

    let targets = collect_targets(sdk);
    let deployment_targets = &deployment_targets;

    thread::scope(|scope| {
        for target in targets {
            scope.spawn(move || {
                build_pomace_target(target, sdk, deployment_targets);
            });
        }
    });
}
