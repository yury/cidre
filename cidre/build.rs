use std::env;
use std::path::PathBuf;
use std::process::Command;

fn xc_feature_build(pomace: &str, sdk: &str, arch: &str, configuration: &str) {
    let env_var = format!("CARGO_FEATURE_{}", pomace.to_uppercase());
    if env::var_os(env_var).is_some() {
        xc_build(pomace, sdk, arch, configuration)
    }
}

fn xc_build(pomace: &str, sdk: &str, arch: &str, configuration: &str) {
    let mut out_lib_dir = PathBuf::from(&env::var("OUT_DIR").unwrap());
    out_lib_dir.push(pomace);

    let status = if sdk.eq("maccatalyst") {
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
            .args(["-scheme", pomace])
            .args(["-sdk", "macosx"])
            .args(["-arch", arch])
            .args(["-configuration", configuration])
            .args(["-derivedDataPath", out_lib_dir.to_str().unwrap()])
            .args([
                "-destination 'platform=macOS,variant=Mac Catalyst'",
                "SUPPORTS_MACCATALYST=YES",
            ])
            .arg("build")
            .status()
            .unwrap()
    } else {
        Command::new("xcodebuild")
            .args(["-project", "./pomace/pomace.xcodeproj"])
            .args(["-scheme", pomace])
            .args(["-sdk", sdk])
            .args(["-arch", arch])
            .args(["-configuration", configuration])
            .args(["-derivedDataPath", out_lib_dir.to_str().unwrap()])
            .arg("build")
            .status()
            .unwrap()
    };

    out_lib_dir.push("Build");
    out_lib_dir.push("Products");
    out_lib_dir.push(configuration);

    let mut s = out_lib_dir.to_str().unwrap().to_string();

    if !sdk.eq("macosx") {
        s.push('-');
        s.push_str(sdk);
    }

    println!("cargo:rustc-link-search=native={s}");

    assert!(status.success());
}

fn main() {
    let sdk = match env::var("TARGET").unwrap().as_ref() {
        "aarch64-apple-darwin" | "x86_64-apple-darwin" => "macosx",
        "aarch64-apple-ios" => "iphoneos",
        "x86_64-apple-ios" | "aarch64-apple-ios-sim" => "iphonesimulator",
        "aarch64-apple-ios-macabi" => "maccatalyst",
        x => panic!("unknown tripple: {x}"),
    };

    let arch = match env::var("TARGET").unwrap().as_ref() {
        "aarch64-apple-ios-macabi"
        | "aarch64-apple-darwin"
        | "aarch64-apple-ios"
        | "aarch64-apple-ios-sim" => "arm64",
        "x86_64-apple-ios" | "x86_64-apple-darwin" => "x86_64",
        x => panic!("unknown tripple: {x}"),
    };

    let configuration = match env::var("PROFILE").unwrap().as_str() {
        "release" => "Release",
        "debug" => "Debug",
        x => panic!("unknown profile: {x}"),
    };

    xc_feature_build("vn", sdk, arch, configuration);
    xc_feature_build("sn", sdk, arch, configuration);
    xc_feature_build("mps", sdk, arch, configuration);
    xc_feature_build("mpsg", sdk, arch, configuration);
    xc_feature_build("ns", sdk, arch, configuration);
    xc_feature_build("mtl", sdk, arch, configuration);
    xc_feature_build("ci", sdk, arch, configuration);
    xc_feature_build("av", sdk, arch, configuration);
    xc_feature_build("ca", sdk, arch, configuration);
    xc_feature_build("mlc", sdk, arch, configuration);

    if sdk.eq("iphoneos") || sdk.eq("maccatalyst") {
        xc_build("ui", sdk, arch, configuration);
    }
    if sdk.eq("macosx") || sdk.eq("maccatalyst") {
        xc_feature_build("sc", sdk, arch, configuration);
        if env::var_os("CARGO_FEATURE_PRIVATE").is_some() {
            println!("cargo:rustc-link-search=framework=/System/Library/PrivateFrameworks");
            println!(
                "cargo:rustc-link-search=framework=/Library/Apple/System/Library/PrivateFrameworks"
            );
        }
    } else {
    }

    // let cc_args =
    //     format!("-fobjc-exceptions -fobjc-arc -fobjc-arc-exceptions -Wl,-objc_stubs_small -O1");
    // println!("cargo:cc_args={cc_args}");

    println!("cargo:rerun-if-changed=./pomace/");
}
