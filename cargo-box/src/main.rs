use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Used via cargo.toml runner
    #[command()]
    Runner(runner::Args),

    /// List dev teams on this mac
    #[command()]
    Teams,

    /// List connected devices on this mac
    #[command()]
    Devices,

    /// Create configured xcode project for binary, test or example
    /// in target/boxes for runner.
    #[command()]
    Proj(xcode::ProjArgs),
}

fn main() {
    // Handle different args from different calls:
    //
    // cargo-box -> cargo-box (no extra box)
    // cargo box -> cargo-box box (extra box)
    let mut args: Vec<_> = env::args().collect();
    if args.get(1).map(|v| v == "box") == Some(true) {
        args.remove(1);
    }

    // special case for runner to skip Cli::parse_from
    if args.get(1).map(|v| v == "runner") == Some(true) {
        return runner::run(runner::Args { args });
    }

    match Cli::parse_from(args).cmd {
        Cmd::Teams => teams::list(),
        Cmd::Devices => device_ctl::list_devices(),
        Cmd::Proj(args) => xcode::proj(args),
        _ => panic!("unknown command"),
    }
}

mod runner {
    use std::{
        env,
        path::{Path, PathBuf},
    };

    use clap::Parser;

    use crate::{device_ctl, xcode};

    #[derive(Parser, Debug)]
    pub(crate) struct Args {
        /// Arguments for the runner
        #[arg()]
        pub(crate) args: Vec<String>,
    }

    pub(crate) fn run(args: Args) {
        assert!(args.args.len() > 2);
        let binary = &args.args[2];
        let path = Path::new(binary);

        let err = "can't parse filename";

        let Some(name) = path.file_name() else {
            panic!("{err}");
        };

        let Some(name) = name.to_str() else {
            panic!("{err}");
        };

        let mut name = name.to_string();

        let Some(parent) = path.parent() else {
            panic!("{err}");
        };

        let Some(folder) = parent.file_name() else {
            panic!("{err}");
        };

        let Some(folder) = folder.to_str() else {
            panic!("{err}");
        };
        let mut is_example = false;
        let mut is_dep = false;
        let mut is_binary = false;
        let parent = match folder {
            "examples" => {
                // target/[optional-tripple/]profile|/examples/example
                is_example = true;
                parent.parent()
            }
            "deps" => {
                // [fullpath/]target/[optional-tripple/]profile|/deps/test-or-bench
                is_dep = true;
                parent.parent()
            }
            "debug" => {
                // target/[optional-tripple/]profile|/binary
                // binary
                is_binary = true;
                Some(parent)
            }
            "release" => {
                // target/[optional-tripple/]profile|/binary
                // binary
                is_binary = true;
                Some(parent)
            }
            _ => panic!("{err}"),
        };

        let Some(parent) = parent else {
            panic!("{err}");
        };
        let Some(folder) = parent.file_name() else {
            panic!("{err}");
        };
        let Some(folder) = folder.to_str() else {
            panic!("{err}");
        };

        let config = match folder {
            "debug" => "Debug",
            "release" => "Release",
            _ => panic!("{err}"),
        };

        let Some(mut parent) = parent.parent() else {
            panic!("{err}");
        };
        let Some(folder) = parent.file_name() else {
            panic!("{err}");
        };

        let Some(folder) = folder.to_str() else {
            panic!("{err}");
        };

        let sdk = match folder {
            "target" | "aarch64-apple-darwin" | "x86_64-apple-darwin" => "macos",
            "aarch64-apple-ios" => "iphoneos",
            "x86_64-apple-ios" | "aarch64-apple-ios-sim" => "iphonesimulator",
            "aarch64-apple-tvos" => "appletvos",
            "aarch64-apple-tvos-sim" => "appletvsimulator",
            "arm64_32-apple-watchos" => "watchos",
            "aarch64-apple-watchos" => "watchos",
            "aarch64-apple-watchos-sim" => "watchsimulator",
            "aarch64-apple-visionos" => "xros",
            "aarch64-apple-visionos-sim" => "xrsimulator",
            _ => panic!("{err}"),
        };

        let platform = match sdk {
            "macos" => "macOS",
            "iphoneos" => "iOS",
            "tvos" => "tvOS",
            "watchos" => "watchOS",
            "xros" => "visionOS",
            "iphoneosimulator" => "iOS Simulator",
            "appletvsimulator" => "tvOS Simulator",
            "watchsimulator" => "watchOS Simulator",
            "xrsimulator" => "visionOS Simulator",
            x => panic!("unknown platform {x}"),
        };

        if folder != "target" {
            parent = parent.parent().unwrap();
        }

        let Some(folder) = parent.file_name() else {
            panic!("{err}");
        };

        if folder == "target" {
            parent = parent.parent().unwrap();
        }

        if !parent.as_os_str().is_empty() {
            env::set_current_dir(parent).unwrap();
        }

        if is_dep {
            // /Users/yury/Projects/cidre/target/aarch64-apple-ios/release/deps/uuid-3968b503a26aa57f
            if let Some((name_part, _hash)) = name.rsplit_once('-') {
                name = name_part.to_string();
            }
        }

        let mut proj_args = xcode::ProjArgs {
            bin: None,
            example: None,
            dep: None,
        };

        if is_example {
            proj_args.example = Some(name.clone());
        } else if is_binary {
            proj_args.bin = Some(name.clone());
        } else if is_dep {
            proj_args.dep = Some(name.clone());
        }

        xcode::proj(proj_args);

        let mut project = PathBuf::from("./target/boxes");
        if is_example {
            project.push("examples");
        }
        if is_dep {
            project.push("deps");
        }

        let mut target = project.clone();
        target.push(format!("build-{name}"));

        project.push(&name);
        project.push(&name);

        // TODO: if is_binary, try replace with BOX_BIN_PATH instead of copy
        std::fs::copy(binary, &project).unwrap();
        project.pop();

        project.push("box.xcodeproj");

        xcode::build(&project, "box", platform, config, &target);

        target.push("Build");
        target.push("Products");

        target.push(format!("{config}-{sdk}"));
        target.push(format!("{name}.app"));

        let box_org_id = std::env::var("BOX_ORG_ID").unwrap();
        let device_id = std::env::var("DEVICE_ID").unwrap();

        device_ctl::install_app(&device_id, &target);
        device_ctl::run_app(&device_id, &format!("{box_org_id}.{name}"), &args.args[3..]);
    }
}

mod teams {
    use cidre::{arc, cf, sec};

    pub(crate) fn list() {
        let query = cf::DictionaryOf::with_keys_values(
            &[
                sec::class_key(),
                sec::match_keys::limit(),
                sec::match_keys::policy(),
            ],
            &[
                sec::class::certificate().as_type_ref(),
                sec::match_limit::all(),
                &sec::Policy::with_props(sec::Policy::apple_code_signing(), None).unwrap(),
            ],
        );

        let certs = sec::item_matching(&query).unwrap();

        assert_eq!(certs.get_type_id(), cf::Array::type_id());
        let certs: arc::R<cf::ArrayOf<sec::Cert>> = unsafe { std::mem::transmute(certs) };

        let mut filter_set = std::collections::HashSet::new();
        let subject_key = sec::cert_oids::x509_v1_subject_name();
        let org_name_label = sec::cert_oids::organization_name();
        let unit_name_label = sec::cert_oids::organizational_unit_name();
        let prop_value_key = sec::prop_keys::value();
        let prop_label_key = sec::prop_keys::label();
        let keys = cf::ArrayOf::from_slice(&[subject_key]);
        for cert in certs.iter() {
            let Ok(vals) = cert.values(&keys) else {
                continue;
            };
            let Some(value) = vals.get(subject_key) else {
                continue;
            };
            let Some(section) = value.get(prop_value_key) else {
                continue;
            };
            assert_eq!(section.get_type_id(), cf::Array::type_id());

            let section: &cf::ArrayOf<cf::DictionaryOf<cf::String, cf::Type>> =
                unsafe { std::mem::transmute(section) };

            let mut team_id = None;
            let mut team_name = None;
            for dict in section.iter() {
                let Some(label) = dict.get(prop_label_key) else {
                    continue;
                };
                let Some(value) = dict.get(prop_value_key) else {
                    continue;
                };

                // todo: build safer api
                if value.get_type_id() != cf::String::type_id() {
                    continue;
                }
                let value: &cf::String = unsafe { std::mem::transmute(value) };

                if label.equal(org_name_label) {
                    team_name = Some(value);
                } else if label.equal(unit_name_label) {
                    team_id = Some(value);
                }
            }

            if let (Some(id), Some(name)) = (team_id, team_name) {
                if filter_set.contains(id) {
                    continue;
                };
                println!("{id}: {name}");
                filter_set.insert(id);
            }
        }
        if filter_set.is_empty() {
            println!("no teams are found");
        }
    }
}

mod cargo {
    use cargo_toml::{Manifest, Workspace};
    use std::{env, path::PathBuf};

    pub(crate) fn manifests() -> Option<(PathBuf, Vec<Manifest>, Option<Workspace>)> {
        let mut path = root()?.join("Cargo.toml");
        if let Ok(mut res) = Manifest::from_path(&path) {
            path.pop();
            res.complete_from_path(&path).unwrap();
            if let Some(ws) = res.workspace {
                let mut vec = Vec::with_capacity(ws.members.len());
                for member in ws.members.iter() {
                    path = path.join(format!("{member}/Cargo.toml"));
                    let mut man = Manifest::from_path(&path).unwrap();
                    path.pop();
                    man.complete_from_path(&path).unwrap();
                    path.pop();
                    vec.push(man);
                }
                Some((path, vec, Some(ws)))
            } else {
                Some((path, vec![res], None))
            }
        } else {
            None
        }
    }

    fn root() -> Option<PathBuf> {
        let cwd = env::current_dir().unwrap();
        let mut cwd_clone = cwd.clone();
        cwd_clone.push("target");
        if cwd_clone.exists() {
            // we are good
            return Some(cwd);
        }
        cwd_clone.pop(); // target
        cwd_clone.pop(); // ..
        cwd_clone.push("target");
        if cwd_clone.exists() {
            // we are good
            cwd_clone.pop();
            return Some(cwd_clone);
        }
        None
    }
}

mod xcode {
    use std::{fs, path::Path};

    use cargo_toml::{Manifest, Product};

    use crate::cargo;

    pub(crate) fn build(project: &Path, scheme: &str, platform: &str, conf: &str, target: &Path) {
        std::process::Command::new("xcodebuild")
            .args(["-project".as_ref(), project.as_os_str()])
            .args(["-destination", &format!("generic/platform={platform}")])
            .args(["-configuration", conf])
            .args(["-scheme", scheme, "-quiet"])
            .args(["-derivedDataPath".as_ref(), target.as_os_str()])
            // .args(env_args)
            .status()
            .unwrap();
    }

    #[derive(clap::Args, Debug)]
    pub(crate) struct ProjArgs {
        #[arg(long)]
        pub(crate) bin: Option<String>,
        #[arg(long)]
        pub(crate) example: Option<String>,
        #[arg(long)]
        pub(crate) dep: Option<String>,
    }

    fn find_product<'a>(
        mans: &'a [Manifest],
        args: &ProjArgs,
    ) -> Option<(&'a Manifest, &'a Product)> {
        let (is_bin, name, nothing, not_found, available) = if args.example.is_some() {
            (
                false,
                args.example.as_ref(),
                "No examples.",
                "No examples found with name",
                "Available examples:",
            )
        } else {
            (
                true,
                args.bin.as_ref(),
                "No binaries.",
                "No binary found with name",
                "Available binaries:",
            )
        };

        let products =
            |man: &'a Manifest| -> &'a [Product] { if is_bin { &man.bin } else { &man.example } };

        if let Some(product_name) = name {
            let mut count = 0;
            for man in mans {
                for product in products(man) {
                    if product.name.as_ref() == name {
                        return Some((man, product));
                    }
                    count += 1;
                }
            }
            if count == 0 {
                println!("{nothing}");
            } else {
                println!("{not_found} `{product_name}`");
                println!("{available}");
                for man in mans {
                    for product in products(man) {
                        if let Some(ref name) = product.name {
                            println!("\t{name}");
                        }
                    }
                }
            }

            return None;
        }

        for man in mans {
            for product in man.bin.iter() {
                if product.name.is_some() {
                    return Some((man, product));
                }
            }
        }

        println!("No binaries");

        None
    }

    pub(crate) fn proj(args: ProjArgs) {
        let (mut path, mans, _ws) = cargo::manifests().unwrap();

        path.push(".box");
        _ = dotenv::from_filename(".box");
        path.pop();

        let product = if args.dep.is_some() {
            &Product {
                name: args.dep.clone(),
                ..Default::default()
            }
        } else {
            let Some((_man, product)) = find_product(&mans, &args) else {
                return;
            };
            product
        };

        let Ok(box_org_id) = std::env::var("BOX_ORG_ID") else {
            println!("BOX_ORG_ID env is required. You can add it .box file");
            return;
        };
        let Ok(dev_team_id) = std::env::var("DEVELOPMENT_TEAM") else {
            println!("DEVELOPMENT_TEAM env is required. You can add it .box file");
            println!("use cidre-box teams command to list available team ids");
            return;
        };

        path.push("target/boxes");
        if args.example.is_some() {
            path.push("examples")
        }
        if args.dep.is_some() {
            path.push("deps")
        }
        let product_name = product.name.as_ref().unwrap();
        path.push(product_name);

        fs::create_dir_all(&path).unwrap();
        path.push("box.entitlements");
        fs::write(&path, include_str!("../box/box.entitlements")).unwrap();
        path.pop();
        path.push("cfg.xcconfig");
        fs::write(&path, include_str!("../box/cfg.xcconfig")).unwrap();
        path.pop();
        path.push("box.xcconfig");
        fs::write(
            &path,
            format!(
                r#"
PRODUCT_NAME = {product_name}
BOX_ID = {product_name}
DEVELOPMENT_TEAM = {dev_team_id}
BOX_ORG_ID = {box_org_id}
"#
            ),
        )
        .unwrap();

        path.pop();

        path.push("box.xcodeproj");
        fs::create_dir_all(&path).unwrap();
        path.push("project.pbxproj");
        fs::write(&path, include_str!("../box/box.xcodeproj/project.pbxproj")).unwrap();
        path.pop();
        path.push("xcshareddata/xcschemes");
        fs::create_dir_all(&path).unwrap();
        path.push("box.xcscheme");
        fs::write(
            &path,
            include_str!("../box/box.xcodeproj/xcshareddata/xcschemes/box.xcscheme"),
        )
        .unwrap();
    }
}

mod device_ctl {
    use std::{env, fs, path::Path, process};

    fn run_cmd(args: &[&str]) -> String {
        let json_output_path = env::temp_dir().join(format!("devicectl-{}.json", process::id()));
        let mut child = process::Command::new("xcrun")
            .args(["devicectl", "-q", "--json-output"])
            .arg(&json_output_path)
            .args(args)
            .spawn()
            .unwrap();
        child.wait().unwrap();
        let buf = fs::read_to_string(&json_output_path).unwrap();
        let _ = fs::remove_file(json_output_path);
        buf
    }

    pub(crate) fn install_app(device_id: &str, bundle: &Path) {
        run_cmd(&[
            "device",
            "install",
            "app",
            "-d",
            device_id,
            bundle.to_str().unwrap(),
        ]);
    }

    pub(crate) fn run_app(device_id: &str, id: &str, args: &[String]) {
        let mut args_vec = vec![
            "device",
            "process",
            "launch",
            "--console",
            "-d",
            device_id,
            "--terminate-existing",
            id,
        ];
        for s in args {
            args_vec.push(s);
        }
        let buf = run_cmd(&args_vec);
        let run = serde_json::from_str::<json::AppRun>(&buf).unwrap();
        let code = match (run.result.termination.signal, run.result.termination.code) {
            (Some(signal), None) => signal,
            (None, Some(code)) => code,
            _ => 0,
        };
        std::process::exit(code);
    }

    pub(crate) fn list_devices() {
        let buf = run_cmd(&["list", "devices"]);
        let list = serde_json::from_str::<json::DeviceList>(&buf).unwrap();
        println!("{:#?}", list.result.devices);
    }

    #[allow(unused)]
    mod json {
        use serde::Deserialize;
        use std::borrow::Cow;

        #[derive(Deserialize, Debug)]
        pub(crate) struct AppRun {
            pub(crate) result: AppRunResult,
        }

        #[derive(Deserialize, Debug)]
        pub(crate) struct AppRunResult {
            #[serde(rename = "terminationResult")]
            pub(crate) termination: AppRunTermination,
        }

        #[derive(Deserialize, Debug)]
        pub(crate) struct AppRunTermination {
            #[serde(rename = "terminatingSignal")]
            pub(crate) signal: Option<i32>,
            #[serde(rename = "exitCode")]
            pub(crate) code: Option<i32>,
        }

        #[derive(Deserialize, Debug)]
        pub(crate) struct DeviceList<'a> {
            #[serde(borrow)]
            pub(crate) result: ListResult<'a>,
        }

        #[derive(Deserialize, Debug)]
        pub(crate) struct ListResult<'a> {
            #[serde(borrow)]
            pub(crate) devices: Vec<Device<'a>>,
        }

        // #[serde(rename_all = "camelCase")]
        #[derive(Deserialize, Debug)]
        pub(crate) struct Device<'a> {
            #[serde(rename = "identifier")]
            pub(crate) id: &'a str,

            #[serde(borrow)]
            #[serde(rename = "deviceProperties")]
            pub(crate) props: DeviceProps<'a>,

            // pub(crate) visibility_class: &'a str,
            #[serde(borrow)]
            #[serde(rename = "connectionProperties")]
            pub(crate) connection: ConnectionProps<'a>,

            #[serde(borrow)]
            #[serde(rename = "hardwareProperties")]
            pub(crate) hardware: HwProps<'a>,
        }

        #[derive(Deserialize, Debug)]
        pub(crate) struct DeviceProps<'a> {
            pub(crate) name: Cow<'a, str>,
            #[serde(rename = "developerModeStatus")]
            pub(crate) dev_mode_status: Option<&'a str>,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub(crate) struct HwProps<'a> {
            pub(crate) device_type: &'a str,
            pub(crate) platform: &'a str,
            pub(crate) udid: &'a str,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub(crate) struct ConnectionProps<'a> {
            pub(crate) pairing_state: &'a str,
        }
    }
}
