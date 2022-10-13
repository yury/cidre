use std::env;
use std::path::PathBuf;
use std::process::Command;

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
            .args(&["-project", "./pomace/pomace.xcodeproj"])
            .args(&["-scheme", pomace])
            .args(&["-sdk", "macosx"])
            .args(&["-arch", arch])
            .args(&["-configuration", configuration])
            .args(&["-derivedDataPath", out_lib_dir.to_str().unwrap()])
            .args(&[
                "-destination 'platform=macOS,variant=Mac Catalyst'",
                "SUPPORTS_MACCATALYST=YES",
            ])
            .arg("build")
            .status()
            .unwrap()
    } else {
        Command::new("xcodebuild")
            .args(&["-project", "./pomace/pomace.xcodeproj"])
            .args(&["-scheme", pomace])
            .args(&["-sdk", sdk])
            .args(&["-arch", arch])
            .args(&["-configuration", configuration])
            .args(&["-derivedDataPath", out_lib_dir.to_str().unwrap()])
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

    xc_build("common", sdk, arch, configuration);
    xc_build("ns", sdk, arch, configuration);
    xc_build("av", sdk, arch, configuration);
    xc_build("mtl", sdk, arch, configuration);
    xc_build("ci", sdk, arch, configuration);
    xc_build("vn", sdk, arch, configuration);

    if sdk.eq("iphoneos") || sdk.eq("maccatalyst") {
        xc_build("ca", sdk, arch, configuration);
        xc_build("ui", sdk, arch, configuration);
    }
    xc_build("ci", sdk, arch, configuration);
    if sdk.eq("macosx") || sdk.eq("maccatalyst") {
        xc_build("sc", sdk, arch, configuration);
        println!("cargo:rustc-link-search=framework=/System/Library/PrivateFrameworks");
        println!(
            "cargo:rustc-link-search=framework=/Library/Apple/System/Library/PrivateFrameworks"
        );
    } else {
    }

    println!("cargo:rerun-if-changed=./pomace/");
}
