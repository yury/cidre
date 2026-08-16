//! Prints the orientation reported by motion-capable AirPods.
//!
//! A plain macOS command-line executable has no `Info.plist` or code-signing
//! entitlements. Run through `cargo-box` so the executable is hosted in a
//! signed app bundle with `NSMotionUsageDescription` and the Head Pose
//! entitlement:
//!
//! ```text
//! CARGO_TARGET_AARCH64_APPLE_DARWIN_RUNNER="cargo-box runner" \
//!   cargo run -p cidre --target aarch64-apple-darwin \
//!   --example cm-headphone --no-default-features \
//!   --features core_motion,blocks
//! ```
//!
//! Connect compatible AirPods and put them in your ears before launching it.

use cidre::{core_motion as cm, ns};

fn main() {
    println!(
        "headphones motion authorization: {:?}",
        cm::HeadphoneMotionManager::authorization_status()
    );

    let mut manager = cm::HeadphoneMotionManager::new();
    if !manager.is_device_motion_available() {
        eprintln!(
            "headphones motion is unavailable; connect compatible AirPods, put them in your ears, \
             and relaunch the example"
        );
        return;
    }

    println!("tracking AirPods orientation; stop the process to finish");
    let queue = ns::OpQueue::main();
    manager.start_device_motion_updates_to_queue(&queue, move |motion, err| {
        if let Some(err) = err {
            eprintln!("headphone motion error: {err}");
            return;
        }

        let Some(motion) = motion else {
            return;
        };

        let attitude = motion.attitude();
        let quat = attitude.quaternion();
        println!(
            "roll={:7.2}° pitch={:7.2}° yaw={:7.2}°  quat=({:.4}, {:.4}, {:.4}, {:.4})",
            attitude.roll().to_degrees(),
            attitude.pitch().to_degrees(),
            attitude.yaw().to_degrees(),
            quat.x,
            quat.y,
            quat.z,
            quat.w,
        );
    });

    // Keep both the manager and the main operation queue alive while callbacks
    // are delivered.
    ns::RunLoop::main().run();
}
