//! A Rust port of Apple's "Controlling a DockKit accessory using your camera
//! app" sample, without the UI.
//!
//! It follows the same shape as the Swift sample: a capture session runs, the
//! app waits on `accessoryStateChanges`, and once an accessory docks it sets up
//! the accessory subscriptions and then drives the control APIs. Every
//! subscription is an async iterator pulled with `next().await`, so nothing
//! here polls or sleeps.
//!
//! Run this from the Mac and cargo-box will install and launch it on the
//! iPhone. Press Ctrl-C in this terminal to stop it:
//!
//! BOX_ORG_ID=org.cidre.dockkit DEVELOPMENT_TEAM=<TEAM_ID> DEVICE_ID=<DEVICE_ID> \
//!   cargo run -p cidre --target aarch64-apple-ios --example dock-kit \
//!     --no-default-features --features dk,async,av,cm,dispatch,ios_18_0
//!
//! It has to be a physical iPhone: DockKit reports nothing on the simulator,
//! and it only hands an accessory to an app that is actively capturing, so the
//! camera permission prompt has to be accepted on the device once.
//!
//! Useful setup commands:
//!   cargo box teams
//!   cargo box devices

use cidre::{arc, av, cf, cg, ns, spatial, swift::dock_kit as dk};

/// Mirrors the sample's tracking-mode menu.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TrackingMode {
    System,
    Custom,
    Manual,
}

/// Mirrors the sample's framing menu.
#[derive(Clone, Copy, Debug)]
enum Framing {
    Auto,
    Center,
    Left,
    Right,
}

/// Mirrors the sample's manual-control chevrons.
#[derive(Clone, Copy, Debug)]
enum Chevron {
    TiltUp,
    TiltDown,
    PanLeft,
    PanRight,
}

fn main() {
    // The camera permission prompt and DockKit's callbacks both need the app's
    // run loop, so the async work goes to its own thread and main pumps.
    std::thread::spawn(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime")
            .block_on(run());
    });
    cf::RunLoop::run();
}

async fn run() {
    println!("dock-kit: waiting for an accessory to dock");

    // DockKit only hands an accessory to an app that is actively capturing, so
    // a camera session has to be running before anything docks.
    let _session = match start_capture().await {
        Some(session) => session,
        None => {
            println!("no camera session; DockKit will not report an accessory");
            return;
        }
    };

    // Subscribe to accessory state changes.
    let manager = dk::AccessoryManager::shared();
    let state_changes = match manager.accessory_state_changes() {
        Ok(state_changes) => state_changes,
        Err(err) => return report("accessory state changes unavailable", &err),
    };

    let mut state_changes = state_changes.async_iter();
    while let Some(change) = state_changes.next().await {
        println!("state change: {change:?}");

        // Save the DockKit accessory when docked (connected).
        if let Some(accessory) = change.accessory
            && change.state == dk::State::docked()
        {
            setup_accessory_subscriptions(accessory).await;
        }
    }
    println!("state changes ended");
}

/// When an accessory connects, enable system tracking and start the
/// subscriptions, as `setupAccessorySubscriptions(for:)` does in the sample.
///
/// The four subscriptions run concurrently and never finish while the accessory
/// stays docked, so this does not return.
async fn setup_accessory_subscriptions(accessory: arc::R<dk::Accessory>) {
    describe(&accessory);

    // Enable system tracking on the first connection.
    update_tracking_mode(TrackingMode::System).await;

    // One-shot tour of the control APIs the sample drives from its UI.
    demo_controls(&accessory).await;

    tokio::join!(
        subscribe_to_motion_states(&accessory),
        subscribe_to_accessory_events(&accessory),
        subscribe_to_battery_states(&accessory),
        subscribe_to_tracking_states(&accessory),
    );
}

fn describe(accessory: &dk::Accessory) {
    let identifier = accessory.identifier();
    println!("  accessory={}", identifier.name());
    println!("  firmware={:?}", accessory.firmware_version());
    println!("  hardware={:?}", accessory.hardware_model());
    println!("  framing_mode={:?}", accessory.framing_mode());

    match accessory.limits() {
        Ok(limits) => {
            print_limit("yaw", limits.yaw());
            print_limit("pitch", limits.pitch());
            print_limit("roll", limits.roll());
        }
        Err(err) => report("  limits unavailable", &err),
    }
}

fn print_limit(name: &str, limit: Option<dk::Limit>) {
    if let Some(limit) = limit {
        println!(
            "  limit.{name} range={:?} maximum_speed={}",
            limit.position_range(),
            limit.maximum_speed()
        );
    }
}

/// Exercises each control the sample exposes through its UI once, so a run
/// without a UI still covers the same API surface.
async fn demo_controls(accessory: &dk::Accessory) {
    for framing in [
        Framing::Center,
        Framing::Left,
        Framing::Right,
        Framing::Auto,
    ] {
        update_framing(accessory, framing).await;
    }

    set_region_of_interest(accessory, cg::Rect::new(0.25, 0.25, 0.5, 0.5)).await;

    // Tap-to-track, then clear the selection again.
    select_subject(accessory, Some(cg::Point::new(0.5, 0.5))).await;
    select_subject(accessory, None).await;

    // Manual mode drives the accessory directly, so system tracking goes off
    // first. Each velocity supersedes the one before it the moment it lands, so
    // this is an API tour rather than a motion sequence, and a zero velocity
    // leaves the accessory still.
    update_tracking_mode(TrackingMode::Manual).await;
    for chevron in [
        Chevron::PanLeft,
        Chevron::PanRight,
        Chevron::TiltUp,
        Chevron::TiltDown,
    ] {
        handle_chevron_tapped(accessory, chevron, 0.2).await;
    }
    handle_chevron_tapped(accessory, Chevron::PanRight, 0.0).await;

    run_animation(accessory, dk::Animation::yes()).await;

    // Back to the default the sample starts in.
    update_tracking_mode(TrackingMode::System).await;
}

/// `updateTrackingMode(to:)`: system tracking is on for `.system` and off for
/// the custom and manual modes, which drive the accessory themselves.
async fn update_tracking_mode(mode: TrackingMode) {
    let enabled = mode == TrackingMode::System;
    match dk::AccessoryManager::shared()
        .set_system_tracking_enabled(enabled)
        .await
    {
        Ok(()) => println!("tracking mode={mode:?} (system tracking={enabled})"),
        Err(err) => report("set system tracking", &err),
    }
}

/// `selectSubject(at:)`: `Some(point)` selects the subject there, `None` clears
/// the selection.
async fn select_subject(accessory: &dk::Accessory, point: Option<cg::Point>) {
    let result = match point {
        Some(point) => accessory.select_subject(point).await,
        None => accessory.select_subjects(&[]).await,
    };
    match result {
        Ok(()) => println!("select subject at={point:?}"),
        Err(err) => report("select subject", &err),
    }
}

/// `setRegionOfInterest(to:)`: the accessory keeps subjects framed in `region`.
async fn set_region_of_interest(accessory: &dk::Accessory, region: cg::Rect) {
    match accessory.set_region_of_interest(region).await {
        Ok(()) => println!("region of interest={region:?}"),
        Err(err) => report("set region of interest", &err),
    }
}

/// `updateFraming(to:)`.
async fn update_framing(accessory: &dk::Accessory, framing: Framing) {
    match accessory.set_framing_mode(framing_mode(framing)).await {
        Ok(()) => println!("framing={framing:?}"),
        Err(err) => report("set framing mode", &err),
    }
}

/// Maps the local menu enum onto `DockAccessory.FramingMode`, as the sample's
/// `dockKitFramingMode(from:)` does.
fn framing_mode(framing: Framing) -> dk::FramingMode {
    match framing {
        Framing::Auto => dk::FramingMode::automatic(),
        Framing::Center => dk::FramingMode::center(),
        Framing::Left => dk::FramingMode::left(),
        Framing::Right => dk::FramingMode::right(),
    }
}

/// `handleChevronTapped(chevronType:speed:)`: manual control through
/// `setAngularVelocity`.
async fn handle_chevron_tapped(accessory: &dk::Accessory, chevron: Chevron, speed: f64) {
    let velocity = match chevron {
        Chevron::TiltUp => spatial::Vector3D::new(-speed, 0.0, 0.0),
        Chevron::TiltDown => spatial::Vector3D::new(speed, 0.0, 0.0),
        Chevron::PanLeft => spatial::Vector3D::new(0.0, -speed, 0.0),
        Chevron::PanRight => spatial::Vector3D::new(0.0, speed, 0.0),
    };
    match accessory.set_angular_velocity(velocity).await {
        Ok(()) => println!("chevron={chevron:?} speed={speed}"),
        Err(err) => report("set angular velocity", &err),
    }
}

/// Runs one of the accessory's built-in animations. The sample disables system
/// tracking first and restores it afterwards, since the two would fight.
async fn run_animation(accessory: &dk::Accessory, animation: dk::Animation) {
    update_tracking_mode(TrackingMode::Custom).await;

    match accessory.animate(animation).await {
        // `Progress` reports the animation as it runs. The sample polls it on a
        // timer; there is nothing to wait for here, so this just reports it.
        Ok(progress) => println!(
            "animation started: finished={} cancelled={}",
            progress.is_finished(),
            progress.is_cancelled()
        ),
        Err(err) => report("animate", &err),
    }

    update_tracking_mode(TrackingMode::System).await;
}

/// `motionStates`: the accessory's orientation and how fast it is turning.
async fn subscribe_to_motion_states(accessory: &dk::Accessory) {
    let states = match accessory.motion_states() {
        Ok(states) => states,
        Err(err) => return report("orientation unavailable", &err),
    };

    let mut states = states.async_iter();
    while let Some(state) = states.next().await {
        let [yaw, pitch, roll] = angles(state.angular_positions());
        let [vyaw, vpitch, vroll] = angles(state.angular_velocities());
        println!(
            "orientation t={:.3} yaw={yaw:.3} pitch={pitch:.3} roll={roll:.3} \
             velocity=({vyaw:.3}, {vpitch:.3}, {vroll:.3}){}",
            state.timestamp(),
            match state.error() {
                Some(error) => format!(" error={}", error.localized_desc()),
                None => std::string::String::new(),
            }
        );
    }
    println!("orientation stream ended");
}

fn angles(vector: spatial::Vector3D) -> [f64; 3] {
    [vector.x, vector.y, vector.z]
}

/// `accessoryEvents`: physical input on the accessory. The sample turns these
/// into camera operations; with no UI this reports them and applies the zoom.
async fn subscribe_to_accessory_events(accessory: &dk::Accessory) {
    let events = match accessory.accessory_events() {
        Ok(events) => events,
        Err(err) => return report("accessory events unavailable", &err),
    };

    let mut events = events.async_iter();
    while let Some(event) = events.next().await {
        match event {
            dk::AccessoryEvent::Button { id, pressed } => {
                println!("event: button id={id} pressed={pressed}")
            }
            dk::AccessoryEvent::CameraZoom { factor } => {
                let direction = if factor > 0.0 { "increase" } else { "decrease" };
                println!("event: camera zoom {direction} factor={factor}")
            }
            dk::AccessoryEvent::CameraShutter => println!("event: camera shutter"),
            dk::AccessoryEvent::CameraFlip => println!("event: camera flip"),
            dk::AccessoryEvent::Unknown(tag) => println!("event: unknown tag={tag}"),
        }
    }
    println!("accessory events ended");
}

/// `batteryStates`: battery level and charging state.
async fn subscribe_to_battery_states(accessory: &dk::Accessory) {
    let states = match accessory.battery_states() {
        Ok(states) => states,
        Err(err) => return report("battery states unavailable", &err),
    };

    let mut states = states.async_iter();
    while let Some(state) = states.next().await {
        println!(
            "battery {} level={} charging={:?} low={}",
            state.name(),
            state.battery_level(),
            state.charge_state(),
            state.is_low_battery()
        );
    }
    println!("battery states ended");
}

/// `trackingStates`: the subjects the accessory is following, with saliency and
/// speaking confidence. The sample maps each rect into view space to draw an
/// overlay; the normalized rect is reported here instead.
async fn subscribe_to_tracking_states(accessory: &dk::Accessory) {
    let states = match accessory.tracking_states() {
        Ok(states) => states,
        Err(err) => return report("tracking states unavailable", &err),
    };

    let mut states = states.async_iter();
    while let Some(state) = states.next().await {
        for subject in state.tracked_subjects().iter() {
            match subject {
                dk::TrackedSubject::Person(person) => println!(
                    "tracking person {} rect={:?} saliency={:?} speaking={:?} looking={:?}",
                    person.identifier(),
                    person.rect(),
                    person.saliency_rank(),
                    person.speaking_confidence(),
                    person.looking_at_camera_confidence()
                ),
                dk::TrackedSubject::Object(object) => println!(
                    "tracking object {} rect={:?} saliency={:?}",
                    object.identifier(),
                    object.rect(),
                    object.saliency_rank()
                ),
                dk::TrackedSubject::Unknown(tag) => println!("tracking unknown tag={tag}"),
            }
        }
    }
    println!("tracking states ended");
}

fn report(what: &str, err: &ns::Error) {
    if err.domain().as_ref() != dk::err::domain() {
        println!("{what}: {}", err.localized_desc());
        return;
    }

    use dk::err::code;
    let reason = match err.code() {
        code::NOT_SUPPORTED => "not supported on this platform",
        code::NOT_CONNECTED => "no accessory is connected",
        code::NOT_SUPPORTED_BY_DEVICE => "not supported by this accessory",
        code::INVALID_PARAMETER => "invalid parameter",
        code::NO_SUBJECT_FOUND => "no subject found",
        code::FRAME_RATE_TOO_LOW => "camera frame rate too low",
        code::CAMERA_TCC_MISSING => "camera access has not been granted",
        code::FRAME_RATE_TOO_HIGH => "camera frame rate too high",
        other => {
            println!("{what}: DockKit error {other}");
            return;
        }
    };
    println!("{what}: {reason}");
}

/// Starts a capture session, which is what makes DockKit consider this app the
/// one an accessory should dock to.
async fn start_capture() -> Option<arc::R<av::CaptureSession>> {
    let video = av::MediaType::video();
    match av::CaptureDevice::request_access_for_media_type(video).await {
        Ok(true) => {}
        Ok(false) => {
            println!("camera access denied");
            return None;
        }
        Err(err) => {
            println!("camera access request failed: {err:?}");
            return None;
        }
    }

    let device = av::CaptureDevice::with_type_media_and_pos(
        av::CaptureDeviceType::built_in_wide_angle_camera(),
        Some(video),
        av::CaptureDevicePos::Back,
    )?;
    println!("capturing with {}", device.localized_name());

    let input = av::CaptureDeviceInput::with_device(&device).ok()?;
    let mut session = av::CaptureSession::new();
    session.configure(|s| {
        if s.can_add_input(&input) {
            s.add_input(&input);
        }
    });
    session.start_running();
    Some(session)
}
