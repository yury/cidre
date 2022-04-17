use std::env;
use std::path::PathBuf;
use std::process::Command;

fn xc_build(pomace: &str, sdk: &str, arch: &str, configuration: &str) {
    let mut out_lib_dir = PathBuf::from(&env::var("OUT_DIR").unwrap());
    out_lib_dir.push(pomace);

    let status = Command::new("xcodebuild")
        .args(&["-project", &format!("./pomace/pomace.xcodeproj")])
        .args(&["-scheme", pomace])
        .args(&["-sdk", &sdk])
        .args(&["-arch", &arch])
        .args(&["-configuration", &configuration])
        .args(&["-derivedDataPath", out_lib_dir.to_str().unwrap()])
        .arg("build")
        .status()
        .unwrap();

    out_lib_dir.push("Build");
    out_lib_dir.push("Products");
    out_lib_dir.push(configuration);

    let s = out_lib_dir.to_str().unwrap();

    println!("cargo:rustc-link-search=native={s}");
    println!("cargo:rerun-if-changed=./pomace/{pomace}");

    assert!(status.success());
}

fn main() {
    let sdk = match env::var("TARGET").unwrap().as_ref() {
        "aarch64-apple-darwin" | "x86_64-apple-darwin" => "macosx",
        "aarch64-apple-ios" => "iphoneos",
        "x86_64-apple-ios" | "aarch64-apple-ios-sim" => "iphonesimulator",
        x => panic!("unknown tripple: {x}"),
    };

    let arch = match env::var("TARGET").unwrap().as_ref() {
        "aarch64-apple-darwin" | "aarch64-apple-ios" | "aarch64-apple-ios-sim" => "arm64",
        "x86_64-apple-ios" | "x86_64-apple-darwin" => "x86_64",
        x => panic!("unknown tripple: {x}"),
    };

    let configuration = match env::var("PROFILE").unwrap().as_str() {
        "release" => "Release",
        "debug" => "Debug",
        x => panic!("unknown profile: {x}"),
    };

    xc_build("mtl", sdk, arch, configuration);
    if sdk.eq("macosx") {
        xc_build("sc", sdk, arch, configuration);
    }
}
