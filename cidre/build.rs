use std::{
    env, fs::read_to_string, path::PathBuf, process::Command, str::FromStr, time::SystemTime,
};

#[derive(Debug, Clone, Copy)]
struct Version {
    major: u32,
    minor: u32,
}

#[derive(Debug, Clone, Default)]
struct DeploymentTargets {
    macos: String,
    ios: String,
    tvos: String,
    watchos: String,
    visionos: String,
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

fn add_features_to_xc_target_args(features: &[&'static str], target_args: &mut Vec<&'static str>) {
    for feature in features {
        let env_var = format!("CARGO_FEATURE_{}", feature.to_uppercase());
        if env::var_os(env_var).is_some() {
            target_args.push("-target");
            target_args.push(feature);
        }
    }
}

fn xc_build(
    targets: &[&'static str],
    sdk: &str,
    arch: &str,
    configuration: &str,
    deployment_targets: &DeploymentTargets,
) {
    let start = SystemTime::now();
    let mut out_lib_dir = PathBuf::from(&env::var("OUT_DIR").unwrap());
    let build_products_dir = format!("SYMROOT={}", out_lib_dir.to_str().unwrap());
    // panic!("{}", build_products_dir);
    // out_lib_dir.push(pomace);

    let status = if sdk == "maccatalyst" {
        let c = Command::new("xcrun")
            .arg("--show-sdk-path")
            .output()
            .unwrap();
        let line = String::from_utf8(c.stdout).unwrap();
        let line = line.lines().next().unwrap();

        println!("cargo:rustc-link-search=system={line}/System/iOSSupport/usr");
        println!(
            "cargo:rustc-link-search=framework={line}/System/iOSSupport/System/Library/Frameworks"
        );

        // -isystem $(MACOSX_SDK_DIR)/System/iOSSupport/usr/include \
        // -iframework $(MACOSX_SDK_DIR)/System/iOSSupport/System/Library/Frameworks
        Command::new("xcodebuild")
            .args(["-project", "./pomace/pomace.xcodeproj"])
            .args(targets)
            .args(["-sdk", "macosx"])
            .args(["-arch", arch])
            .args(["-configuration", configuration])
            // .args(["-derivedDataPath", out_lib_dir.to_str().unwrap()])
            .args([
                "-destination 'generic/platform=macOS,variant=Mac Catalyst'",
                "SUPPORTS_MACCATALYST=YES",
            ])
            .arg("build")
            .arg(&deployment_targets.macos)
            .arg(&deployment_targets.ios)
            .arg(&deployment_targets.tvos)
            .arg(&deployment_targets.watchos)
            .arg(&deployment_targets.visionos)
            .arg(&build_products_dir)
            .status()
            .unwrap()
    } else {
        Command::new("xcodebuild")
            .args(["-project", "./pomace/pomace.xcodeproj"])
            .args(targets)
            .args(["-sdk", sdk])
            .args(["-arch", arch])
            .args(["-configuration", configuration])
            // .args(["-derivedDataPath", out_lib_dir.to_str().unwrap()])
            .arg("build")
            .arg(&deployment_targets.macos)
            .arg(&deployment_targets.ios)
            .arg(&deployment_targets.tvos)
            .arg(&deployment_targets.watchos)
            .arg(&deployment_targets.visionos)
            .arg(&build_products_dir)
            .status()
            .unwrap()
    };

    // out_lib_dir.push("Build");
    // out_lib_dir.push("Products");
    out_lib_dir.push(configuration);

    let mut s = out_lib_dir.to_str().unwrap().to_string();

    if sdk != "macosx" {
        s.push('-');
        s.push_str(sdk);
    }

    println!("cargo:rustc-link-search=native={s}");

    assert!(status.success());

    let dur = SystemTime::now().duration_since(start);
    eprintln!("time spend on pomace {:?}", dur);
}

fn parse_deployment_targets() -> DeploymentTargets {
    let var = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = var;
    let path = PathBuf::from_str(&manifest_dir).unwrap();
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
        res.macos = format!("MACOSX_DEPLOYMENT_TARGET={}", v.to_string());
    }
    if let Some(v) = Version::with_table(&ios) {
        res.ios = format!("IPHONEOS_DEPLOYMENT_TARGET={}", v.to_string());
    }
    if let Some(v) = Version::with_table(&tvos) {
        res.tvos = format!("TVOS_DEPLOYMENT_TARGET={}", v.to_string());
    }
    if let Some(v) = Version::with_table(&watchos) {
        res.watchos = format!("WATCHOS_DEPLOYMENT_TARGET={}", v.to_string());
    }
    if let Some(v) = Version::with_table(&maccatalyst) {
        // TODO: investigate
        res.ios = format!("IPHONEOS_DEPLOYMENT_TARGET={}", v.to_string());
    }
    if let Some(v) = Version::with_table(&visionos) {
        res.visionos = format!("XROS_DEPLOYMENT_TARGET={}", v.to_string());
    }

    res
}

fn main() {
    let versions = parse_deployment_targets();

    let sdk = match env::var("TARGET").unwrap().as_ref() {
        "aarch64-apple-darwin" | "x86_64-apple-darwin" => "macosx",
        "aarch64-apple-ios" => "iphoneos",
        "x86_64-apple-ios" | "aarch64-apple-ios-sim" => "iphonesimulator",
        "aarch64-apple-ios-macabi" => "maccatalyst",
        "aarch64-apple-tvos" => "appletvos",
        "aarch64-apple-tvos-sim" => "appletvsimulator",
        "arm64_32-apple-watchos" => "watchos",
        "aarch64-apple-watchos-sim" => "watchsimulator",
        x => panic!("unknown tripple: {x}"),
    };

    let arch = match env::var("TARGET").unwrap().as_ref() {
        "aarch64-apple-ios-macabi"
        | "aarch64-apple-darwin"
        | "aarch64-apple-ios"
        | "aarch64-apple-tvos"
        | "aarch64-apple-tvos-sim"
        | "aarch64-apple-ios-sim"
        | "aarch64-apple-watchos-sim" => "arm64",
        "x86_64-apple-ios" | "x86_64-apple-darwin" => "x86_64",
        "arm64_32-apple-watchos" => "arm64_32",
        x => panic!("unknown tripple: {x}"),
    };

    let configuration = match env::var("PROFILE").unwrap().as_str() {
        "release" => "Release",
        "debug" => "Debug",
        x => panic!("unknown profile: {x}"),
    };

    let mut target_args = Vec::new();

    add_features_to_xc_target_args(
        &["ut", "un", "sn", "ns", "av", "cl", "nl"],
        &mut target_args,
    );

    if sdk != "watchos" && sdk != "watchsimulator" {
        add_features_to_xc_target_args(
            &["ca", "vn", "mps", "mpsg", "mc", "mtl", "ci", "mlc"],
            &mut target_args,
        );
    }
    if sdk != "appletvos"
        && sdk != "appletvsimulator"
        && sdk != "watchos"
        && sdk != "watchsimulator"
    {
        add_features_to_xc_target_args(&["wk", "core_motion"], &mut target_args);
    }
    if sdk != "watchos" && sdk != "watchsimulator" {
        add_features_to_xc_target_args(&["gc"], &mut target_args);
    }

    if sdk == "iphoneos"
        || sdk == "iphonesimulator"
        || sdk == "maccatalyst"
        || sdk == "appletvos"
        || sdk == "appletvsimulator"
    {
        add_features_to_xc_target_args(&["ui"], &mut target_args);
    }
    if sdk == "iphoneos" || sdk == "iphonesimulator" {
        add_features_to_xc_target_args(&["wc"], &mut target_args);
    }
    if sdk == "macosx" || sdk == "maccatalyst" {
        add_features_to_xc_target_args(&["sc", "app"], &mut target_args);
        if env::var_os("CARGO_FEATURE_PRIVATE").is_some() {
            println!("cargo:rustc-link-search=framework=/System/Library/PrivateFrameworks");
            println!(
                "cargo:rustc-link-search=framework=/Library/Apple/System/Library/PrivateFrameworks"
            );
        }
    }

    xc_build(&target_args, sdk, arch, configuration, &versions);

    println!("cargo:rerun-if-changed=./pomace/");
}
