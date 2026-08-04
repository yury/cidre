use std::{
    hash::Hash,
    mem::size_of,
    panic::{AssertUnwindSafe, catch_unwind},
    ptr::NonNull,
};

use crate::{
    arc, cg, ns, spatial, swift,
    swift::{
        SwiftMetadata, abi,
        concurrency::{
            self, swift_async_epilogue, swift_async_function_pointer, swift_async_load_parent,
            swift_async_load_resume, swift_async_prologue, swift_async_store_parent,
            swift_async_store_resume, swift_async_task_descriptor, swift_task_alloc,
            swift_task_dealloc, swift_task_switch,
        },
        foundation::{Date, DateValue, Uuid, UuidValue},
        value::{Optional, Storage, Value},
    },
};

crate::define_swift_class!(pub Accessory);

pub struct StateChanges(Value<StateChangesValue>);

pub struct StateChangesIter(Value<StateChangesIteratorValue>);

pub struct StateChange {
    pub accessory: Option<arc::R<Accessory>>,
    pub state: State,
    pub tracking_button_enabled: bool,
}

/// `DockAccessory.Identifier`.
pub struct Identifier(Value<IdentifierValue>);

/// One sample from `DockAccessory.motionStates`.
pub struct MotionState(Value<MotionStateValue>);

/// One sample from `DockAccessory.batteryStates`.
pub struct BatteryState(Value<BatteryStateValue>);

/// The accessory's mechanical movement limits.
pub struct Limits(Value<LimitsValue>);

/// Limits for one rotational axis.
pub struct Limit(Value<LimitValue>);

/// A physical event reported by the dock accessory.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccessoryEvent {
    Button { id: isize, pressed: bool },
    CameraShutter,
    CameraFlip,
    CameraZoom { factor: f64 },
    Unknown(u32),
}

impl std::hash::Hash for AccessoryEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Button { id, pressed } => (id, pressed).hash(state),
            Self::CameraZoom { factor } => {
                // Rust and Swift both compare -0.0 equal to 0.0.
                let bits = if *factor == 0.0 { 0 } else { factor.to_bits() };
                bits.hash(state);
            }
            Self::Unknown(tag) => tag.hash(state),
            Self::CameraShutter | Self::CameraFlip => {}
        }
    }
}

/// A person currently tracked by DockKit.
pub struct TrackedPerson(Value<TrackedPersonValue>);

/// An object currently tracked by DockKit.
pub struct TrackedObject(Value<TrackedObjectValue>);

/// A tracked subject and its concrete payload.
pub enum TrackedSubject {
    Person(TrackedPerson),
    Object(TrackedObject),
    Unknown(u32),
}

/// One sample from `DockAccessory.trackingStates`.
pub struct TrackingState(Value<TrackingStateValue>);

/// The native Swift array returned by `TrackingState.trackedSubjects`.
pub struct TrackedSubjects {
    raw: *mut (),
}

unsafe impl Send for TrackedSubjects {}
unsafe impl Sync for TrackedSubjects {}

/// One subject observation supplied to DockKit tracking.
pub struct Observation(Value<ObservationValue>);

/// A native Swift `[DockAccessory.Observation]`.
pub struct Observations {
    raw: *mut (),
}

unsafe impl Send for Observations {}
unsafe impl Sync for Observations {}

/// Camera calibration supplied with tracking observations.
#[cfg(feature = "av")]
pub struct CameraInformation(Value<CameraInformationValue>);

/// Native layout of `simd_float3x3`, stored as three padded column vectors.
#[cfg(feature = "av")]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C, align(16))]
pub struct CameraIntrinsics {
    pub columns: [[f32; 4]; 3],
}

#[cfg(feature = "av")]
impl CameraIntrinsics {
    pub const fn from_columns(columns: [[f32; 3]; 3]) -> Self {
        Self {
            columns: [
                [columns[0][0], columns[0][1], columns[0][2], 0.0],
                [columns[1][0], columns[1][1], columns[1][2], 0.0],
                [columns[2][0], columns[2][1], columns[2][2], 0.0],
            ],
        }
    }

    pub const fn columns(&self) -> [[f32; 3]; 3] {
        [
            [self.columns[0][0], self.columns[0][1], self.columns[0][2]],
            [self.columns[1][0], self.columns[1][1], self.columns[1][2]],
            [self.columns[2][0], self.columns[2][1], self.columns[2][2]],
        ]
    }
}

struct StateChangeNextTask {
    state_changes: Option<StateChanges>,
    iter: Option<StateChangesIter>,
    result: Option<NonNull<u8>>,
    callback: Box<dyn FnMut(Option<StateChange>) -> bool + Send>,
}

swift_async_task_descriptor!(
    #[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
    cidre_dk_state_changes_next_task_descriptor,
    entry: state_changes_next_task_entry,
    context_size: "64",
);

/// Allocates the callee context, wires up the resume symbol, and tail-calls
/// `Accessory.StateChanges.Iterator.next()`.
///
/// Shared by the first iteration and by every subsequent one, which is why the
/// context slots are re-read rather than kept in registers.
macro_rules! state_changes_next_call {
    () => {
        concat!(
            // Word 1 of the async function pointer is the callee's context size.
            "adrp x8, {next_async}@GOTPAGE\n",
            "ldr x8, [x8, {next_async}@GOTPAGEOFF]\n",
            "ldr w0, [x8, #4]\n",
            "bl {task_alloc}\n",
            "mov x9, x0\n",
            "str x9, [x22, #48]\n",
            $crate::swift::concurrency::swift_async_store_parent!(), "\n",
            $crate::swift::concurrency::swift_async_store_resume!("{resume}"), "\n",
            "ldr x0, [x22, #40]\n",
            "ldr x20, [x22, #32]\n",
            "mov x22, x9\n",
            "mov x21, #0\n",
            $crate::swift::concurrency::swift_async_epilogue!(frame: "32", fp: "16"), "\n",
            "b {next}",
        )
    };
}

/// Allocates the reusable result buffer from Swift's task allocator, records it
/// in the Rust context, then hops to a plain frame for the remaining setup.
#[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
#[unsafe(naked)]
unsafe extern "C" fn state_changes_next_task_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "bl {result_size}",
        "bl {task_alloc}",
        "mov x1, x0",
        "str x1, [x22, #40]",
        "ldr x0, [x22, #24]",
        "bl {set_task_result}",
        swift_async_function_pointer!("{run}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {task_switch}",
        result_size = sym cidre_dk_state_changes_next_result_size,
        set_task_result = sym cidre_dk_state_changes_next_set_task_result,
        task_alloc = sym swift_task_alloc,
        task_switch = sym swift_task_switch,
        run = sym state_changes_next_task_run,
    );
}

/// Makes the async iterator on the Rust side, then starts iterating.
#[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
#[unsafe(naked)]
unsafe extern "C" fn state_changes_next_task_run() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {prepare}",
        "ldr x0, [x22, #24]",
        "bl {iter_ptr}",
        "str x0, [x22, #32]",
        "str x22, [x22, #16]",
        state_changes_next_call!(),
        prepare = sym cidre_dk_state_changes_next_prepare,
        iter_ptr = sym cidre_dk_state_changes_next_iter_ptr,
        next_async = sym STATE_CHANGES_ITERATOR_NEXT_ASYNC_FN,
        next = sym state_changes_iterator_next,
        task_alloc = sym swift_task_alloc,
        resume = sym state_changes_next_task_resume,
    );
}

/// Resumed once per element. Hops to a plain frame before calling into Rust.
#[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
#[unsafe(naked)]
unsafe extern "C" fn state_changes_next_task_resume() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        swift_async_load_parent!(),
        "str x9, [sp]",
        "str x9, [x9, #16]",
        "ldr x0, [x9, #48]",
        "bl {task_dealloc}",
        "ldr x22, [sp]",
        swift_async_function_pointer!("{complete}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {task_switch}",
        task_dealloc = sym swift_task_dealloc,
        task_switch = sym swift_task_switch,
        complete = sym state_changes_next_task_complete,
    );
}

/// Delivers one state change to Rust, then either asks for the next one or
/// returns to whoever created the task.
#[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
#[unsafe(naked)]
unsafe extern "C" fn state_changes_next_task_complete() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {process}",
        "ldr x22, [sp, #8]",
        "cbnz x0, 0f",
        "ldr x0, [x22, #24]",
        "bl {take_result}",
        "bl {task_dealloc}",
        "ldr x0, [x22, #24]",
        "bl {drop_task}",
        "ldr x9, [x22, #16]",
        swift_async_load_resume!(),
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        "0:",
        "str x22, [x22, #16]",
        state_changes_next_call!(),
        process = sym cidre_dk_state_changes_next_process,
        take_result = sym cidre_dk_state_changes_next_take_result,
        drop_task = sym cidre_dk_state_changes_next_drop,
        task_dealloc = sym swift_task_dealloc,
        next_async = sym STATE_CHANGES_ITERATOR_NEXT_ASYNC_FN,
        next = sym state_changes_iterator_next,
        task_alloc = sym swift_task_alloc,
        resume = sym state_changes_next_task_resume,
    );
}

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaF"]
    fn state_changes_iterator_next();

    #[link_name = "$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaFTu"]
    static STATE_CHANGES_ITERATOR_NEXT_ASYNC_FN: u8;

    #[link_name = "$s7DockKit0A9AccessoryC12StateChangesV8IteratorVMa"]
    fn dock_accessory_state_changes_iterator_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC15firmwareVersionSSSgvg"]
    fn dock_accessory_firmware_version();

    #[link_name = "$s7DockKit0A9AccessoryC2eeoiySbAC_ACtFZ"]
    fn dock_accessory_equal();

    #[link_name = "$s7DockKit0A9AccessoryC9hashValueSivg"]
    fn dock_accessory_hash_value();

    #[link_name = "$s7DockKit0A9AccessoryC13hardwareModelSSSgvg"]
    fn dock_accessory_hardware_model();

    #[link_name = "$s7DockKit0A9AccessoryC12StateChangesVMa"]
    fn dock_accessory_state_changes_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC12StateChangesV17makeAsyncIteratorAE0H0VyF"]
    fn dock_accessory_state_changes_make_async_iterator();

    #[link_name = "$s7DockKit0A9AccessoryC11StateChangeVMa"]
    fn dock_accessory_state_change_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC11StateChangeV5stateAC0D0Ovg"]
    fn dock_accessory_state_change_state();

    #[link_name = "$s7DockKit0A9AccessoryC11StateChangeV21trackingButtonEnabledSbvg"]
    fn dock_accessory_state_change_tracking_button_enabled();

    #[link_name = "$s7DockKit0A9AccessoryC11StateChangeV9accessoryACSgvg"]
    fn dock_accessory_state_change_accessory();

    #[link_name = "$s7DockKit0A9AccessoryC10IdentifierVMa"]
    fn dock_accessory_identifier_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC10IdentifierV8categoryAC8CategoryOvg"]
    fn dock_accessory_identifier_category();

    #[link_name = "$s7DockKit0A9AccessoryC10IdentifierV4nameSSvg"]
    fn dock_accessory_identifier_name();

    #[link_name = "$s7DockKit0A9AccessoryC10IdentifierV4uuid10Foundation4UUIDVvg"]
    fn dock_accessory_identifier_uuid();

    #[link_name = "$s7DockKit0A9AccessoryC10IdentifierV16debugDescriptionSSvg"]
    fn dock_accessory_identifier_debug_description();

    #[link_name = "$s7DockKit0A9AccessoryC10IdentifierV9hashValueSivg"]
    fn dock_accessory_identifier_hash_value();

    #[link_name = "$s7DockKit0A9AccessoryC10IdentifierV2eeoiySbAE_AEtFZ"]
    fn dock_accessory_identifier_equal();

    #[link_name = "$s7DockKit0A9AccessoryC11MotionStateVMa"]
    fn dock_accessory_motion_state_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC11MotionStateV17angularVelocitiesSo10SPVector3Davg"]
    fn dock_accessory_motion_state_angular_velocities();

    #[link_name = "$s7DockKit0A9AccessoryC11MotionStateV16angularPositionsSo10SPVector3Davg"]
    fn dock_accessory_motion_state_angular_positions();

    #[link_name = "$s7DockKit0A9AccessoryC11MotionStateV9timestampSdvg"]
    fn dock_accessory_motion_state_timestamp();

    #[link_name = "$s7DockKit0A9AccessoryC11MotionStateV5errors5Error_pSgvg"]
    fn dock_accessory_motion_state_error();

    #[link_name = "$s7DockKit0A9AccessoryC12BatteryStateVMa"]
    fn dock_accessory_battery_state_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC12BatteryStateV4nameSSvg"]
    fn dock_accessory_battery_state_name();

    #[link_name = "$s7DockKit0A9AccessoryC12BatteryStateV12batteryLevelSdvg"]
    fn dock_accessory_battery_state_level();

    #[link_name = "$s7DockKit0A9AccessoryC12BatteryStateV03lowD0Sbvg"]
    fn dock_accessory_battery_state_low();

    #[link_name = "$s7DockKit0A9AccessoryC12BatteryStateV06chargeE0AC0d6ChargeE0Ovg"]
    fn dock_accessory_battery_state_charge_state();

    #[link_name = "$s7DockKit0A9AccessoryC12BatteryStateV9hashValueSivg"]
    fn dock_accessory_battery_state_hash_value();

    #[link_name = "$s7DockKit0A9AccessoryC12BatteryStateV2eeoiySbAE_AEtFZ"]
    fn dock_accessory_battery_state_equal();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsVMa"]
    fn dock_accessory_limits_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsV5LimitVMa"]
    fn dock_accessory_limit_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsV3yawAE5LimitVSgvg"]
    fn dock_accessory_limits_yaw();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsV5pitchAE5LimitVSgvg"]
    fn dock_accessory_limits_pitch();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsV4rollAE5LimitVSgvg"]
    fn dock_accessory_limits_roll();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsV5LimitV12maximumSpeedSdvg"]
    fn dock_accessory_limit_maximum_speed();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsV5LimitV13positionRangeSnySdGvg"]
    fn dock_accessory_limit_position_range();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsV5LimitV13positionRange12maximumSpeedAGSnySdG_SdtKcfC"]
    fn dock_accessory_limit_init();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsV3yaw5pitch4rollA2E5LimitVSg_A2KtcfC"]
    fn dock_accessory_limits_init();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC17CameraInformationV13captureDeviceSo09AVCaptureG4Typeavg"]
    fn dock_accessory_camera_information_capture_device();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC17CameraInformationV14cameraPositionSo015AVCaptureDeviceG0Vvg"]
    fn dock_accessory_camera_information_camera_position();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC17CameraInformationV11orientationAC0D11OrientationOvg"]
    fn dock_accessory_camera_information_orientation();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC17CameraInformationV16cameraIntrinsicsSo13simd_float3x3aSgvg"]
    fn dock_accessory_camera_information_intrinsics();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC17CameraInformationV19referenceDimensionsSo6CGSizeVSgvg"]
    fn dock_accessory_camera_information_reference_dimensions();

    #[link_name = "$s7DockKit0A9AccessoryC0C5EventOMa"]
    fn dock_accessory_event_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC0C5EventO6buttonyAESi_SbtcAEmFWC"]
    static DOCK_ACCESSORY_EVENT_BUTTON_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC0C5EventO13cameraShutteryA2EmFWC"]
    static DOCK_ACCESSORY_EVENT_CAMERA_SHUTTER_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC0C5EventO10cameraFlipyA2EmFWC"]
    static DOCK_ACCESSORY_EVENT_CAMERA_FLIP_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC0C5EventO10cameraZoomyAESd_tcAEmFWC"]
    static DOCK_ACCESSORY_EVENT_CAMERA_ZOOM_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonVMa"]
    fn dock_accessory_tracked_person_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonV10identifier10Foundation4UUIDVvg"]
    fn dock_accessory_tracked_person_identifier();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonV2eeoiySbAE_AEtFZ"]
    fn dock_accessory_tracked_person_equal();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonV4rectSo6CGRectVvg"]
    fn dock_accessory_tracked_person_rect();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonV12saliencyRankSiSgvg"]
    fn dock_accessory_tracked_person_saliency_rank();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonV18speakingConfidenceSdSgvg"]
    fn dock_accessory_tracked_person_speaking_confidence();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonV25lookingAtCameraConfidenceSdSgvg"]
    fn dock_accessory_tracked_person_looking_at_camera_confidence();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedObjectVMa"]
    fn dock_accessory_tracked_object_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedObjectV10identifier10Foundation4UUIDVvg"]
    fn dock_accessory_tracked_object_identifier();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedObjectV2eeoiySbAE_AEtFZ"]
    fn dock_accessory_tracked_object_equal();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedObjectV4rectSo6CGRectVvg"]
    fn dock_accessory_tracked_object_rect();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedObjectV12saliencyRankSiSgvg"]
    fn dock_accessory_tracked_object_saliency_rank();

    #[link_name = "$s7DockKit0A9AccessoryC18TrackedSubjectTypeOMa"]
    fn dock_accessory_tracked_subject_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC18TrackedSubjectTypeO6personyAeC0D6PersonVcAEmFWC"]
    static DOCK_ACCESSORY_TRACKED_SUBJECT_PERSON_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC18TrackedSubjectTypeO6objectyAeC0D6ObjectVcAEmFWC"]
    static DOCK_ACCESSORY_TRACKED_SUBJECT_OBJECT_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC13TrackingStateVMa"]
    fn dock_accessory_tracking_state_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackingStateV4time10Foundation4DateVvg"]
    fn dock_accessory_tracking_state_time();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackingStateV15trackedSubjectsSayAC18TrackedSubjectTypeOGvg"]
    fn dock_accessory_tracking_state_subjects();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackingStateV11descriptionSSvg"]
    fn dock_accessory_tracking_state_description();

    #[link_name = "$s7DockKit0A9AccessoryC11ObservationVMa"]
    fn dock_accessory_observation_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC11ObservationV10identifier4type4rect12faceYawAngleAESi_AE0D4TypeOSo6CGRectV10Foundation11MeasurementVySo06NSUnitJ0CGSgtcfC"]
    fn dock_accessory_observation_init();

    #[link_name = "$s7DockKit0A9AccessoryC11ObservationV10identifierSivg"]
    fn dock_accessory_observation_identifier();

    #[link_name = "$s7DockKit0A9AccessoryC11ObservationV4typeAE0D4TypeOvg"]
    fn dock_accessory_observation_type();

    #[link_name = "$s7DockKit0A9AccessoryC11ObservationV4rectSo6CGRectVvg"]
    fn dock_accessory_observation_rect();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC17CameraInformationVMa"]
    fn dock_accessory_camera_information_metadata();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC17CameraInformationV13captureDevice14cameraPosition11orientation0H10Intrinsics19referenceDimensionsAESo09AVCaptureG4Typea_So0ngI0VAC0D11OrientationOSo13simd_float3x3aSgSo6CGSizeVSgtcfC"]
    fn dock_accessory_camera_information_init();
}

crate::define_swift_marker!(
    pub(crate) StateChangesValue = accessor dock_accessory_state_changes_metadata
);

crate::define_swift_marker!(
    StateChangesIteratorValue = accessor dock_accessory_state_changes_iterator_metadata
);

crate::define_swift_marker!(StateChangeValue = accessor dock_accessory_state_change_metadata);

crate::define_swift_marker!(StateValue = mangled "7DockKit0A9AccessoryC5StateO");

crate::define_swift_marker!(IdentifierValue = accessor dock_accessory_identifier_metadata);
crate::define_swift_marker!(MotionStateValue = accessor dock_accessory_motion_state_metadata);
crate::define_swift_marker!(BatteryStateValue = accessor dock_accessory_battery_state_metadata);
crate::define_swift_marker!(LimitsValue = accessor dock_accessory_limits_metadata);
crate::define_swift_marker!(LimitValue = accessor dock_accessory_limit_metadata);
crate::define_swift_marker!(AccessoryEventValue = accessor dock_accessory_event_metadata);
crate::define_swift_marker!(TrackedPersonValue = accessor dock_accessory_tracked_person_metadata);
crate::define_swift_marker!(TrackedObjectValue = accessor dock_accessory_tracked_object_metadata);
crate::define_swift_marker!(TrackedSubjectValue = accessor dock_accessory_tracked_subject_metadata);
crate::define_swift_marker!(TrackingStateValue = accessor dock_accessory_tracking_state_metadata);
crate::define_swift_marker!(ObservationValue = accessor dock_accessory_observation_metadata);
crate::define_swift_marker!(MeasurementAngleValue = mangled "10Foundation11MeasurementVySo11NSUnitAngleCG");

#[cfg(feature = "av")]
crate::define_swift_marker!(CameraInformationValue = accessor dock_accessory_camera_information_metadata);
#[cfg(feature = "av")]
crate::define_swift_marker!(CameraIntrinsicsValue = mangled "So13simd_float3x3a");
#[cfg(feature = "av")]
crate::define_swift_marker!(ReferenceDimensionsValue = mangled "So6CGSizeV");

macro_rules! impl_value_wrapper {
    ($ty:ident, $marker:ident) => {
        unsafe impl Send for $ty {}
        unsafe impl Sync for $ty {}

        impl Clone for $ty {
            fn clone(&self) -> Self {
                unsafe { Self::copy_from_ptr(self.0.as_ptr()) }
            }
        }

        impl $ty {
            unsafe fn copy_from_ptr(value: *const ()) -> Self {
                unsafe {
                    let mut storage = Storage::<$marker>::new();
                    abi::initialize_with_copy(storage.as_mut_ptr(), value, $marker::metadata());
                    Self(storage.assume_init())
                }
            }

            #[inline]
            fn as_ptr(&self) -> *const () {
                self.0.as_ptr()
            }
        }
    };
}

impl_value_wrapper!(Identifier, IdentifierValue);
impl_value_wrapper!(MotionState, MotionStateValue);
impl_value_wrapper!(BatteryState, BatteryStateValue);
impl_value_wrapper!(Limits, LimitsValue);
impl_value_wrapper!(Limit, LimitValue);
impl_value_wrapper!(TrackedPerson, TrackedPersonValue);
impl_value_wrapper!(TrackedObject, TrackedObjectValue);
impl_value_wrapper!(TrackingState, TrackingStateValue);
impl_value_wrapper!(Observation, ObservationValue);
#[cfg(feature = "av")]
impl_value_wrapper!(CameraInformation, CameraInformationValue);

impl Identifier {
    pub fn category(&self) -> Category {
        let mut value = Category::tracking_stand();
        unsafe {
            abi::call_value_to_value(
                dock_accessory_identifier_category as *const (),
                self.as_ptr(),
                (&mut value as *mut Category).cast(),
            );
        }
        value
    }

    pub fn name(&self) -> swift::String {
        unsafe {
            swift::String::from_raw(abi::call_value_to_string(
                dock_accessory_identifier_name as *const (),
                self.as_ptr(),
            ))
        }
    }

    pub fn uuid(&self) -> Uuid {
        unsafe {
            let mut storage = Storage::<UuidValue>::new();
            abi::call_value_to_value(
                dock_accessory_identifier_uuid as *const (),
                self.as_ptr(),
                storage.as_mut_ptr(),
            );
            Uuid::from_value(storage.assume_init())
        }
    }

    pub fn debug_desc(&self) -> swift::String {
        unsafe {
            swift::String::from_raw(abi::call_value_to_string(
                dock_accessory_identifier_debug_description as *const (),
                self.as_ptr(),
            ))
        }
    }

    pub fn hash_value(&self) -> isize {
        unsafe {
            abi::call_value_to_int(
                dock_accessory_identifier_hash_value as *const (),
                self.as_ptr(),
            )
        }
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            abi::call_objects_to_bool(
                dock_accessory_identifier_equal as *const (),
                self.as_ptr(),
                other.as_ptr(),
            )
        }
    }
}

impl Eq for Identifier {}

impl std::hash::Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.hash_value(), state)
    }
}

impl MotionState {
    pub fn angular_velocities(&self) -> spatial::Vector3D {
        unsafe {
            let mut storage = Storage::<spatial::Vector3D>::new();
            abi::call_value_to_value(
                dock_accessory_motion_state_angular_velocities as *const (),
                self.as_ptr(),
                storage.as_mut_ptr(),
            );
            let value = storage.assume_init();
            let result = value.as_ptr().cast::<spatial::Vector3D>().read();
            value.assume_consumed();
            result
        }
    }

    pub fn angular_positions(&self) -> spatial::Vector3D {
        unsafe {
            let mut storage = Storage::<spatial::Vector3D>::new();
            abi::call_value_to_value(
                dock_accessory_motion_state_angular_positions as *const (),
                self.as_ptr(),
                storage.as_mut_ptr(),
            );
            let value = storage.assume_init();
            let result = value.as_ptr().cast::<spatial::Vector3D>().read();
            value.assume_consumed();
            result
        }
    }

    pub fn timestamp(&self) -> f64 {
        unsafe {
            abi::call_value_to_double(
                dock_accessory_motion_state_timestamp as *const (),
                self.as_ptr(),
            )
        }
    }

    pub fn error(&self) -> Option<arc::R<ns::Error>> {
        unsafe {
            NonNull::new(abi::call_value_to_object(
                dock_accessory_motion_state_error as *const (),
                self.as_ptr(),
            ))
            .map(|error| arc::R::from_raw(abi::error_as_ns_error(error.as_ptr()).cast()))
        }
    }
}

impl BatteryState {
    pub fn name(&self) -> swift::String {
        unsafe {
            swift::String::from_raw(abi::call_value_to_string(
                dock_accessory_battery_state_name as *const (),
                self.as_ptr(),
            ))
        }
    }

    pub fn battery_level(&self) -> f64 {
        unsafe {
            abi::call_value_to_double(
                dock_accessory_battery_state_level as *const (),
                self.as_ptr(),
            )
        }
    }

    pub fn is_low_battery(&self) -> bool {
        unsafe {
            abi::call_value_to_bool(dock_accessory_battery_state_low as *const (), self.as_ptr())
        }
    }

    pub fn charge_state(&self) -> BatteryChargeState {
        let mut value = BatteryChargeState::not_charging();
        unsafe {
            abi::call_value_to_value(
                dock_accessory_battery_state_charge_state as *const (),
                self.as_ptr(),
                (&mut value as *mut BatteryChargeState).cast(),
            );
        }
        value
    }

    pub fn hash_value(&self) -> isize {
        unsafe {
            abi::call_value_to_int(
                dock_accessory_battery_state_hash_value as *const (),
                self.as_ptr(),
            )
        }
    }
}

impl PartialEq for BatteryState {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            abi::call_objects_to_bool(
                dock_accessory_battery_state_equal as *const (),
                self.as_ptr(),
                other.as_ptr(),
            )
        }
    }
}

impl std::hash::Hash for BatteryState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.hash_value(), state)
    }
}

impl Limits {
    pub fn new(yaw: Option<&Limit>, pitch: Option<&Limit>, roll: Option<&Limit>) -> Self {
        unsafe {
            fn optional(value: Option<&Limit>) -> Value<Optional<LimitValue>> {
                match value {
                    None => Value::none(),
                    Some(value) => unsafe {
                        let mut storage = Storage::<Optional<LimitValue>>::new();
                        abi::initialize_with_copy(
                            storage.as_mut_ptr(),
                            value.as_ptr(),
                            LimitValue::metadata(),
                        );
                        abi::store_enum_tag_single_payload(
                            storage.as_mut_ptr(),
                            0,
                            1,
                            LimitValue::metadata(),
                        );
                        storage.assume_init()
                    },
                }
            }

            let mut yaw = optional(yaw);
            let mut pitch = optional(pitch);
            let mut roll = optional(roll);
            let mut storage = Storage::<LimitsValue>::new();
            abi::call_values3_to_value(
                dock_accessory_limits_init as *const (),
                yaw.as_mut_ptr(),
                pitch.as_mut_ptr(),
                roll.as_mut_ptr(),
                storage.as_mut_ptr(),
            );
            yaw.assume_consumed();
            pitch.assume_consumed();
            roll.assume_consumed();
            Self(storage.assume_init())
        }
    }

    unsafe fn optional_limit(&self, getter: *const ()) -> Option<Limit> {
        unsafe {
            let mut storage = Storage::<Optional<LimitValue>>::new();
            abi::call_value_to_value(getter, self.as_ptr(), storage.as_mut_ptr());
            let value = storage.assume_init();
            value
                .is_some()
                .then(|| Limit::copy_from_ptr(value.as_ptr()))
        }
    }

    pub fn yaw(&self) -> Option<Limit> {
        unsafe { self.optional_limit(dock_accessory_limits_yaw as *const ()) }
    }

    pub fn pitch(&self) -> Option<Limit> {
        unsafe { self.optional_limit(dock_accessory_limits_pitch as *const ()) }
    }

    pub fn roll(&self) -> Option<Limit> {
        unsafe { self.optional_limit(dock_accessory_limits_roll as *const ()) }
    }
}

impl Limit {
    pub fn new(
        position_range: std::ops::Range<f64>,
        maximum_speed: f64,
    ) -> Result<Self, arc::R<ns::Error>> {
        assert!(
            position_range.start <= position_range.end,
            "Swift Range requires lowerBound <= upperBound"
        );
        unsafe {
            let mut storage = Storage::<LimitValue>::new();
            let error = abi::call_doubles3_to_throwing_value(
                dock_accessory_limit_init as *const (),
                (position_range.start, position_range.end, maximum_speed),
                storage.as_mut_ptr(),
            );
            if error.is_null() {
                Ok(Self(storage.assume_init()))
            } else {
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    pub fn position_range(&self) -> std::ops::Range<f64> {
        let (start, end) = unsafe {
            abi::call_value_to_doubles2(
                dock_accessory_limit_position_range as *const (),
                self.as_ptr(),
            )
        };
        start..end
    }

    pub fn maximum_speed(&self) -> f64 {
        unsafe {
            abi::call_value_to_double(
                dock_accessory_limit_maximum_speed as *const (),
                self.as_ptr(),
            )
        }
    }
}

impl AccessoryEvent {
    unsafe fn copy_from_ptr(value: *const ()) -> Self {
        unsafe {
            let mut storage = Storage::<AccessoryEventValue>::new();
            abi::initialize_with_copy(storage.as_mut_ptr(), value, AccessoryEventValue::metadata());
            let mut value = storage.assume_init();
            let tag = abi::get_enum_tag(value.as_ptr(), AccessoryEventValue::metadata());

            if tag == DOCK_ACCESSORY_EVENT_BUTTON_TAG {
                abi::destructive_project_enum_data(
                    value.as_mut_ptr(),
                    AccessoryEventValue::metadata(),
                );
                let id = value.as_ptr().cast::<isize>().read();
                let pressed = value.as_ptr().cast::<u8>().add(size_of::<isize>()).read() != 0;
                value.assume_consumed();
                Self::Button { id, pressed }
            } else if tag == DOCK_ACCESSORY_EVENT_CAMERA_ZOOM_TAG {
                abi::destructive_project_enum_data(
                    value.as_mut_ptr(),
                    AccessoryEventValue::metadata(),
                );
                let factor = value.as_ptr().cast::<f64>().read();
                value.assume_consumed();
                Self::CameraZoom { factor }
            } else if tag == DOCK_ACCESSORY_EVENT_CAMERA_SHUTTER_TAG {
                Self::CameraShutter
            } else if tag == DOCK_ACCESSORY_EVENT_CAMERA_FLIP_TAG {
                Self::CameraFlip
            } else {
                Self::Unknown(tag)
            }
        }
    }
}

unsafe fn optional_primitive<T: SwiftMetadata + Copy>(
    owner: *const (),
    getter: *const (),
) -> Option<T> {
    unsafe {
        let mut storage = Storage::<Optional<T>>::new();
        abi::call_value_to_value(getter, owner, storage.as_mut_ptr());
        let value = storage.assume_init();
        value.is_some().then(|| value.as_ptr().cast::<T>().read())
    }
}

unsafe fn uuid_property(owner: *const (), getter: *const ()) -> Uuid {
    unsafe {
        let mut storage = Storage::<UuidValue>::new();
        abi::call_value_to_value(getter, owner, storage.as_mut_ptr());
        Uuid::from_value(storage.assume_init())
    }
}

unsafe fn rect_property(owner: *const (), getter: *const ()) -> cg::Rect {
    let (x, y, width, height) = unsafe { abi::call_value_to_rect(getter, owner) };
    cg::Rect {
        origin: cg::Point { x, y },
        size: cg::Size { width, height },
    }
}

impl TrackedPerson {
    pub fn identifier(&self) -> Uuid {
        unsafe {
            uuid_property(
                self.as_ptr(),
                dock_accessory_tracked_person_identifier as *const (),
            )
        }
    }

    pub fn rect(&self) -> cg::Rect {
        unsafe {
            rect_property(
                self.as_ptr(),
                dock_accessory_tracked_person_rect as *const (),
            )
        }
    }

    pub fn saliency_rank(&self) -> Option<isize> {
        unsafe {
            optional_primitive(
                self.as_ptr(),
                dock_accessory_tracked_person_saliency_rank as *const (),
            )
        }
    }

    pub fn speaking_confidence(&self) -> Option<f64> {
        unsafe {
            optional_primitive(
                self.as_ptr(),
                dock_accessory_tracked_person_speaking_confidence as *const (),
            )
        }
    }

    pub fn looking_at_camera_confidence(&self) -> Option<f64> {
        unsafe {
            optional_primitive(
                self.as_ptr(),
                dock_accessory_tracked_person_looking_at_camera_confidence as *const (),
            )
        }
    }
}

impl PartialEq for TrackedPerson {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            abi::call_objects_to_bool(
                dock_accessory_tracked_person_equal as *const (),
                self.as_ptr(),
                other.as_ptr(),
            )
        }
    }
}

impl TrackedObject {
    pub fn identifier(&self) -> Uuid {
        unsafe {
            uuid_property(
                self.as_ptr(),
                dock_accessory_tracked_object_identifier as *const (),
            )
        }
    }

    pub fn rect(&self) -> cg::Rect {
        unsafe {
            rect_property(
                self.as_ptr(),
                dock_accessory_tracked_object_rect as *const (),
            )
        }
    }

    pub fn saliency_rank(&self) -> Option<isize> {
        unsafe {
            optional_primitive(
                self.as_ptr(),
                dock_accessory_tracked_object_saliency_rank as *const (),
            )
        }
    }
}

impl PartialEq for TrackedObject {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            abi::call_objects_to_bool(
                dock_accessory_tracked_object_equal as *const (),
                self.as_ptr(),
                other.as_ptr(),
            )
        }
    }
}

impl TrackedSubject {
    unsafe fn copy_from_ptr(value: *const ()) -> Self {
        unsafe {
            let mut storage = Storage::<TrackedSubjectValue>::new();
            abi::initialize_with_copy(storage.as_mut_ptr(), value, TrackedSubjectValue::metadata());
            let mut value = storage.assume_init();
            let tag = abi::get_enum_tag(value.as_ptr(), TrackedSubjectValue::metadata());

            if tag == DOCK_ACCESSORY_TRACKED_SUBJECT_PERSON_TAG {
                abi::destructive_project_enum_data(
                    value.as_mut_ptr(),
                    TrackedSubjectValue::metadata(),
                );
                let person = TrackedPerson::copy_from_ptr(value.as_ptr());
                abi::destroy_value(value.as_mut_ptr(), TrackedPersonValue::metadata());
                value.assume_consumed();
                Self::Person(person)
            } else if tag == DOCK_ACCESSORY_TRACKED_SUBJECT_OBJECT_TAG {
                abi::destructive_project_enum_data(
                    value.as_mut_ptr(),
                    TrackedSubjectValue::metadata(),
                );
                let object = TrackedObject::copy_from_ptr(value.as_ptr());
                abi::destroy_value(value.as_mut_ptr(), TrackedObjectValue::metadata());
                value.assume_consumed();
                Self::Object(object)
            } else {
                Self::Unknown(tag)
            }
        }
    }
}

impl TrackedSubjects {
    unsafe fn from_raw(raw: *mut ()) -> Self {
        Self { raw }
    }

    pub fn len(&self) -> usize {
        unsafe { abi::array_count(self.raw.cast_const(), TrackedSubjectValue::metadata()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<TrackedSubject> {
        if index >= self.len() {
            return None;
        }
        unsafe {
            let mut storage = Storage::<TrackedSubjectValue>::new();
            abi::array_get(
                self.raw.cast_const(),
                index as isize,
                storage.as_mut_ptr(),
                TrackedSubjectValue::metadata(),
            );
            let value = storage.assume_init();
            let result = TrackedSubject::copy_from_ptr(value.as_ptr());
            drop(value);
            Some(result)
        }
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = TrackedSubject> + '_ {
        (0..self.len()).map(|index| unsafe { self.get(index).unwrap_unchecked() })
    }
}

impl Clone for TrackedSubjects {
    fn clone(&self) -> Self {
        unsafe { abi::bridge_object_retain(self.raw as usize) };
        Self { raw: self.raw }
    }
}

impl Drop for TrackedSubjects {
    fn drop(&mut self) {
        unsafe { abi::bridge_object_release(self.raw as usize) }
    }
}

impl TrackingState {
    pub fn time(&self) -> Date {
        unsafe {
            let mut storage = Storage::<DateValue>::new();
            abi::call_value_to_value(
                dock_accessory_tracking_state_time as *const (),
                self.as_ptr(),
                storage.as_mut_ptr(),
            );
            Date::from_value(storage.assume_init())
        }
    }

    pub fn tracked_subjects(&self) -> TrackedSubjects {
        unsafe {
            TrackedSubjects::from_raw(abi::call_value_to_object(
                dock_accessory_tracking_state_subjects as *const (),
                self.as_ptr(),
            ))
        }
    }

    pub fn description(&self) -> swift::String {
        unsafe {
            swift::String::from_raw(abi::call_value_to_string(
                dock_accessory_tracking_state_description as *const (),
                self.as_ptr(),
            ))
        }
    }
}

impl Observation {
    /// Creates an observation without a face-yaw measurement.
    pub fn new(identifier: isize, ty: ObservationType, rect: cg::Rect) -> Self {
        unsafe {
            let face_yaw = Value::<Optional<MeasurementAngleValue>>::none();
            let mut storage = Storage::<ObservationValue>::new();
            abi::call_int_value_rect_value_to_value(
                dock_accessory_observation_init as *const (),
                identifier,
                ty.as_abi_ptr(),
                (
                    rect.origin.x,
                    rect.origin.y,
                    rect.size.width,
                    rect.size.height,
                ),
                face_yaw.as_ptr(),
                storage.as_mut_ptr(),
            );
            Self(storage.assume_init())
        }
    }

    pub fn identifier(&self) -> isize {
        unsafe {
            abi::call_value_to_int(
                dock_accessory_observation_identifier as *const (),
                self.as_ptr(),
            )
        }
    }

    pub fn ty(&self) -> ObservationType {
        let mut value = ObservationType::human_face();
        unsafe {
            abi::call_value_to_value(
                dock_accessory_observation_type as *const (),
                self.as_ptr(),
                (&mut value as *mut ObservationType).cast(),
            );
        }
        value
    }

    pub fn rect(&self) -> cg::Rect {
        unsafe { rect_property(self.as_ptr(), dock_accessory_observation_rect as *const ()) }
    }
}

impl Observations {
    pub fn from_slice(values: &[Observation]) -> Self {
        unsafe {
            let metadata = ObservationValue::metadata();
            let (raw, elements) = abi::allocate_uninitialized_array(values.len(), metadata);
            let stride = abi::value_layout(metadata).stride;
            for (index, value) in values.iter().enumerate() {
                abi::initialize_with_copy(
                    elements.cast::<u8>().add(index * stride).cast(),
                    value.as_ptr(),
                    metadata,
                );
            }
            Self { raw }
        }
    }

    pub fn len(&self) -> usize {
        unsafe { abi::array_count(self.raw.cast_const(), ObservationValue::metadata()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[cfg(feature = "av")]
    fn as_raw(&self) -> *mut () {
        self.raw
    }
}

impl Clone for Observations {
    fn clone(&self) -> Self {
        unsafe { abi::bridge_object_retain(self.raw as usize) };
        Self { raw: self.raw }
    }
}

impl Drop for Observations {
    fn drop(&mut self) {
        unsafe { abi::bridge_object_release(self.raw as usize) }
    }
}

#[cfg(feature = "av")]
impl CameraInformation {
    /// Creates camera information without optional intrinsics or reference dimensions.
    pub fn new(
        device_type: &crate::av::CaptureDeviceType,
        position: crate::av::CaptureDevicePos,
        orientation: CameraOrientation,
    ) -> Self {
        Self::with_calibration(device_type, position, orientation, None, None)
    }

    pub fn with_calibration(
        device_type: &crate::av::CaptureDeviceType,
        position: crate::av::CaptureDevicePos,
        orientation: CameraOrientation,
        intrinsics: Option<CameraIntrinsics>,
        reference_dimensions: Option<cg::Size>,
    ) -> Self {
        unsafe {
            let intrinsics = match intrinsics {
                None => Value::<Optional<CameraIntrinsicsValue>>::none(),
                Some(value) => {
                    let mut storage = Storage::<Optional<CameraIntrinsicsValue>>::new();
                    abi::initialize_with_copy(
                        storage.as_mut_ptr(),
                        (&raw const value).cast(),
                        CameraIntrinsicsValue::metadata(),
                    );
                    abi::store_enum_tag_single_payload(
                        storage.as_mut_ptr(),
                        0,
                        1,
                        CameraIntrinsicsValue::metadata(),
                    );
                    storage.assume_init()
                }
            };
            let dimensions = match reference_dimensions {
                None => Value::<Optional<ReferenceDimensionsValue>>::none(),
                Some(value) => {
                    let mut storage = Storage::<Optional<ReferenceDimensionsValue>>::new();
                    abi::initialize_with_copy(
                        storage.as_mut_ptr(),
                        (&raw const value).cast(),
                        ReferenceDimensionsValue::metadata(),
                    );
                    abi::store_enum_tag_single_payload(
                        storage.as_mut_ptr(),
                        0,
                        1,
                        ReferenceDimensionsValue::metadata(),
                    );
                    storage.assume_init()
                }
            };
            let words = dimensions.as_ptr().cast::<u64>();
            let mut storage = Storage::<CameraInformationValue>::new();
            abi::call_camera_information_init(
                dock_accessory_camera_information_init as *const (),
                (device_type as *const crate::av::CaptureDeviceType).cast(),
                position as isize,
                orientation.as_abi_ptr(),
                intrinsics.as_ptr(),
                (words.read(), words.add(1).read(), words.add(2).read()),
                storage.as_mut_ptr(),
            );
            Self(storage.assume_init())
        }
    }

    pub fn capture_device(&self) -> arc::R<crate::av::CaptureDeviceType> {
        unsafe {
            arc::R::from_raw(
                abi::call_value_to_object(
                    dock_accessory_camera_information_capture_device as *const (),
                    self.0.as_ptr(),
                )
                .cast(),
            )
        }
    }

    pub fn camera_position(&self) -> crate::av::CaptureDevicePos {
        unsafe {
            std::mem::transmute(abi::call_value_to_int(
                dock_accessory_camera_information_camera_position as *const (),
                self.0.as_ptr(),
            ))
        }
    }

    pub fn orientation(&self) -> CameraOrientation {
        let mut value = CameraOrientation::portrait();
        unsafe {
            abi::call_value_to_value(
                dock_accessory_camera_information_orientation as *const (),
                self.0.as_ptr(),
                (&mut value as *mut CameraOrientation).cast(),
            );
        }
        value
    }

    pub fn camera_intrinsics(&self) -> Option<CameraIntrinsics> {
        unsafe {
            let mut storage = Storage::<Optional<CameraIntrinsicsValue>>::new();
            abi::call_value_to_value(
                dock_accessory_camera_information_intrinsics as *const (),
                self.0.as_ptr(),
                storage.as_mut_ptr(),
            );
            let value = storage.assume_init();
            value
                .is_some()
                .then(|| value.as_ptr().cast::<CameraIntrinsics>().read())
        }
    }

    pub fn reference_dimensions(&self) -> Option<cg::Size> {
        unsafe {
            let words = abi::call_value_to_words3(
                dock_accessory_camera_information_reference_dimensions as *const (),
                self.0.as_ptr(),
            );
            let mut storage = Storage::<Optional<ReferenceDimensionsValue>>::new();
            let ptr = storage.as_mut_ptr().cast::<u64>();
            ptr.write(words.0);
            ptr.add(1).write(words.1);
            ptr.add(2).write(words.2);
            let value = storage.assume_init();
            value
                .is_some()
                .then(|| value.as_ptr().cast::<cg::Size>().read())
        }
    }
}

macro_rules! define_async_sequence {
    (
        $(#[$meta:meta])*
        $sequence:ident, $sequence_value:ident, $iterator_value:ident,
        element = $element:ident, $element_value:ident,
        sequence_metadata = $sequence_metadata:ident => $sequence_metadata_link:literal,
        iterator_metadata = $iterator_metadata:ident => $iterator_metadata_link:literal,
        make_iterator = $make_iterator:ident => $make_iterator_link:literal,
        next = $next:ident => $next_link:literal,
        next_async = $next_async:ident => $next_async_link:literal $(,)?
    ) => {
        $(#[$meta])*
        pub struct $sequence(Value<$sequence_value>);

        #[link(name = "DockKit", kind = "framework")]
        unsafe extern "C" {
            #[link_name = $sequence_metadata_link]
            fn $sequence_metadata();

            #[link_name = $iterator_metadata_link]
            fn $iterator_metadata();

            #[link_name = $make_iterator_link]
            fn $make_iterator();

            #[link_name = $next_link]
            fn $next();

            #[link_name = $next_async_link]
            static $next_async: u8;
        }

        crate::define_swift_marker!($sequence_value = accessor $sequence_metadata);
        crate::define_swift_marker!($iterator_value = accessor $iterator_metadata);

        impl $sequence {
            unsafe fn from_storage(storage: Storage<$sequence_value>) -> Self {
                Self(unsafe { storage.assume_init() })
            }

            pub fn for_each_while<F>(self, mut callback: F)
            where
                F: FnMut(Option<$element>) -> bool + Send + 'static,
            {
                concurrency::iterate_async_sequence(
                    self.0,
                    concurrency::AsyncSequenceSymbols {
                        iterator_metadata: $iterator_value::metadata(),
                        element_metadata: $element_value::metadata(),
                        make_iterator: $make_iterator as *const (),
                        next: $next as *const (),
                        next_async_fn: (&raw const $next_async).cast(),
                    },
                    move |value| match value {
                        Some(value) => callback(Some(unsafe { $element::copy_from_ptr(value) })),
                        None => {
                            callback(None);
                            false
                        }
                    },
                );
            }

            pub fn for_each<F>(self, mut callback: F)
            where
                F: FnMut(Option<$element>) + Send + 'static,
            {
                self.for_each_while(move |value| {
                    let has_value = value.is_some();
                    callback(value);
                    has_value
                });
            }

            /// Compatibility spelling for [`Self::for_each`].
            pub fn next<F>(self, mut callback: F)
            where
                F: FnMut(Option<$element>) + Send + 'static,
            {
                self.for_each(move |value| callback(value));
            }
        }
    };
}

define_async_sequence! {
    /// `DockAccessory.MotionStates`.
    MotionStates, MotionStatesValue, MotionStatesIteratorValue,
    element = MotionState, MotionStateValue,
    sequence_metadata = dock_accessory_motion_states_metadata => "$s7DockKit0A9AccessoryC12MotionStatesVMa",
    iterator_metadata = dock_accessory_motion_states_iterator_metadata => "$s7DockKit0A9AccessoryC12MotionStatesV8IteratorVMa",
    make_iterator = dock_accessory_motion_states_make_iterator => "$s7DockKit0A9AccessoryC12MotionStatesV17makeAsyncIteratorAE0H0VyF",
    next = dock_accessory_motion_states_next => "$s7DockKit0A9AccessoryC12MotionStatesV8IteratorV4nextAC0D5StateVSgyYaF",
    next_async = DOCK_ACCESSORY_MOTION_STATES_NEXT_ASYNC => "$s7DockKit0A9AccessoryC12MotionStatesV8IteratorV4nextAC0D5StateVSgyYaFTu",
}

define_async_sequence! {
    /// `DockAccessory.AccessoryEvents`.
    AccessoryEvents, AccessoryEventsValue, AccessoryEventsIteratorValue,
    element = AccessoryEvent, AccessoryEventValue,
    sequence_metadata = dock_accessory_events_metadata => "$s7DockKit0A9AccessoryC0C6EventsVMa",
    iterator_metadata = dock_accessory_events_iterator_metadata => "$s7DockKit0A9AccessoryC0C6EventsV8IteratorVMa",
    make_iterator = dock_accessory_events_make_iterator => "$s7DockKit0A9AccessoryC0C6EventsV17makeAsyncIteratorAE0G0VyF",
    next = dock_accessory_events_next => "$s7DockKit0A9AccessoryC0C6EventsV8IteratorV4nextAC0C5EventOSgyYaF",
    next_async = DOCK_ACCESSORY_EVENTS_NEXT_ASYNC => "$s7DockKit0A9AccessoryC0C6EventsV8IteratorV4nextAC0C5EventOSgyYaFTu",
}

define_async_sequence! {
    /// `DockAccessory.TrackingStates`.
    TrackingStates, TrackingStatesValue, TrackingStatesIteratorValue,
    element = TrackingState, TrackingStateValue,
    sequence_metadata = dock_accessory_tracking_states_metadata => "$s7DockKit0A9AccessoryC14TrackingStatesVMa",
    iterator_metadata = dock_accessory_tracking_states_iterator_metadata => "$s7DockKit0A9AccessoryC14TrackingStatesV8IteratorVMa",
    make_iterator = dock_accessory_tracking_states_make_iterator => "$s7DockKit0A9AccessoryC14TrackingStatesV17makeAsyncIteratorAE0H0VyF",
    next = dock_accessory_tracking_states_next => "$s7DockKit0A9AccessoryC14TrackingStatesV8IteratorV4nextAC0D5StateVSgyYaF",
    next_async = DOCK_ACCESSORY_TRACKING_STATES_NEXT_ASYNC => "$s7DockKit0A9AccessoryC14TrackingStatesV8IteratorV4nextAC0D5StateVSgyYaFTu",
}

define_async_sequence! {
    /// `DockAccessory.BatteryStates`.
    BatteryStates, BatteryStatesValue, BatteryStatesIteratorValue,
    element = BatteryState, BatteryStateValue,
    sequence_metadata = dock_accessory_battery_states_metadata => "$s7DockKit0A9AccessoryC13BatteryStatesVMa",
    iterator_metadata = dock_accessory_battery_states_iterator_metadata => "$s7DockKit0A9AccessoryC13BatteryStatesV8IteratorVMa",
    make_iterator = dock_accessory_battery_states_make_iterator => "$s7DockKit0A9AccessoryC13BatteryStatesV17makeAsyncIteratorAE0H0VyF",
    next = dock_accessory_battery_states_next => "$s7DockKit0A9AccessoryC13BatteryStatesV8IteratorV4nextAC0D5StateVSgyYaF",
    next_async = DOCK_ACCESSORY_BATTERY_STATES_NEXT_ASYNC => "$s7DockKit0A9AccessoryC13BatteryStatesV8IteratorV4nextAC0D5StateVSgyYaFTu",
}

impl StateChanges {
    pub(crate) fn storage() -> Storage<StateChangesValue> {
        Storage::new()
    }

    #[inline]
    pub(crate) unsafe fn from_storage(storage: Storage<StateChangesValue>) -> Self {
        Self(unsafe { storage.assume_init() })
    }

    #[inline]
    pub fn make_async_iter(&self) -> StateChangesIter {
        unsafe {
            let mut storage = Storage::<StateChangesIteratorValue>::new();
            abi::call_value_to_value(
                dock_accessory_state_changes_make_async_iterator as *const (),
                self.0.as_ptr(),
                storage.as_mut_ptr(),
            );
            StateChangesIter(storage.assume_init())
        }
    }

    #[inline]
    pub fn for_each_while<F>(self, callback: F)
    where
        F: FnMut(Option<StateChange>) -> bool + Send + 'static,
    {
        StateChangeNextTask::start(self, callback);
    }

    #[inline]
    pub fn for_each<F>(self, mut callback: F)
    where
        F: FnMut(Option<StateChange>) + Send + 'static,
    {
        self.for_each_while(move |value| {
            let has_value = value.is_some();
            callback(value);
            has_value
        });
    }

    /// Compatibility spelling for [`Self::for_each`].
    #[inline]
    pub fn next<F>(self, mut callback: F)
    where
        F: FnMut(Option<StateChange>) + Send + 'static,
    {
        self.for_each(move |value| callback(value));
    }
}

impl StateChangesIter {
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut () {
        self.0.as_mut_ptr()
    }
}

impl StateChangeNextTask {
    fn start<F>(state_changes: StateChanges, callback: F)
    where
        F: FnMut(Option<StateChange>) -> bool + Send + 'static,
    {
        unsafe {
            let task = Box::new(Self {
                state_changes: Some(state_changes),
                iter: None,
                result: None,
                callback: Box::new(callback),
            });
            let context: *mut () = Box::into_raw(task).cast();
            let (_task, _) = concurrency::task_create(
                concurrency::ENQUEUED_DISCARDING_TASK_FLAGS,
                core::ptr::null(),
                (&raw const cidre_dk_state_changes_next_task_descriptor).cast(),
                context,
            );
        }
    }
}

extern "C" fn cidre_dk_state_changes_next_result_size(_: *mut StateChangeNextTask) -> usize {
    unsafe {
        abi::value_layout(Optional::<StateChangeValue>::metadata())
            .stride
            .max(1)
    }
}

extern "C" fn cidre_dk_state_changes_next_set_task_result(
    task: *mut StateChangeNextTask,
    result: *mut u8,
) {
    unsafe {
        let task = &mut *task;
        let result = NonNull::new(result).expect("swift task result allocation failed");
        assert!(
            task.result.replace(result).is_none(),
            "Swift task result already set"
        );
    }
}

extern "C" fn cidre_dk_state_changes_next_prepare(task: *mut StateChangeNextTask) {
    unsafe {
        let task = &mut *task;
        if task.iter.is_some() {
            return;
        }
        let state_changes = task
            .state_changes
            .take()
            .expect("state changes sequence already prepared");
        task.iter = Some(state_changes.make_async_iter());
    }
}

extern "C" fn cidre_dk_state_changes_next_iter_ptr(task: *mut StateChangeNextTask) -> *mut () {
    unsafe {
        let task = &mut *task;
        let iter = task.iter.as_mut().expect("state changes iterator missing");
        iter.as_mut_ptr().cast()
    }
}

extern "C" fn cidre_dk_state_changes_next_process(task: *mut StateChangeNextTask) -> bool {
    match catch_unwind(AssertUnwindSafe(|| unsafe {
        let task = &mut *task;
        let result = task.result.expect("Swift task result missing");
        let payload_metadata = StateChangeValue::metadata();
        let result_metadata = Optional::<StateChangeValue>::metadata();
        let tag = abi::get_enum_tag_single_payload(result.as_ptr().cast(), 1, payload_metadata);
        if tag == 1 {
            abi::destroy_value(result.as_ptr().cast(), result_metadata);
            let _ = (task.callback)(None);
            return false;
        }

        let mut state_storage = Storage::<StateValue>::new();
        abi::call_value_to_value(
            dock_accessory_state_change_state as *const (),
            result.as_ptr().cast_const().cast(),
            state_storage.as_mut_ptr(),
        );
        let state_value = state_storage.assume_init();
        let state = State(*(state_value.as_ptr().cast::<u8>()));
        drop(state_value);

        let tracking_button_enabled = abi::call_value_to_bool(
            dock_accessory_state_change_tracking_button_enabled as *const (),
            result.as_ptr().cast_const().cast(),
        );
        let accessory = NonNull::new(abi::call_value_to_object(
            dock_accessory_state_change_accessory as *const (),
            result.as_ptr().cast_const().cast(),
        ))
        .map(|accessory| arc::R::from_raw(accessory.as_ptr().cast()));
        abi::destroy_value(result.as_ptr().cast(), payload_metadata);
        (task.callback)(Some(StateChange {
            accessory,
            state,
            tracking_button_enabled,
        }))
    })) {
        Ok(keep_going) => keep_going,
        Err(_) => false,
    }
}

extern "C" fn cidre_dk_state_changes_next_take_result(task: *mut StateChangeNextTask) -> *mut () {
    unsafe {
        (*task)
            .result
            .take()
            .expect("Swift task result missing")
            .as_ptr()
            .cast()
    }
}

extern "C" fn cidre_dk_state_changes_next_drop(task: *mut StateChangeNextTask) {
    unsafe { drop(Box::from_raw(task)) }
}

macro_rules! resilient_enum {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            hash = $hash_fn:ident = $hash_link:literal,
            debug = $debug_fn:ident = $debug_link:literal,
            cases { $($case:ident => $symbol:ident = $link:literal),+ $(,)? }
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[repr(transparent)]
        $vis struct $name(u8);

        #[link(name = "DockKit", kind = "framework")]
        unsafe extern "C" {
            $(
                #[link_name = $link]
                static $symbol: u8;
            )+

            #[link_name = $hash_link]
            fn $hash_fn();

            #[link_name = $debug_link]
            fn $debug_fn();
        }

        impl $name {
            $(
                #[inline]
                pub fn $case() -> Self {
                    unsafe { Self($symbol) }
                }
            )+

            #[inline]
            pub fn as_abi_ptr(&self) -> *const () {
                (self as *const Self).cast()
            }

            #[inline]
            pub fn hash_value(&self) -> isize {
                unsafe { abi::call_value_to_int($hash_fn as *const (), self.as_abi_ptr()) }
            }

            #[inline]
            pub fn debug_desc(&self) -> swift::String {
                unsafe {
                    swift::String::from_raw(abi::call_value_to_string(
                        $debug_fn as *const (),
                        self.as_abi_ptr(),
                    ))
                }
            }
        }
    };
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            hash = $hash_fn:ident = $hash_link:literal,
            cases { $($case:ident => $symbol:ident = $link:literal),+ $(,)? }
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #[repr(transparent)]
        $vis struct $name(u8);

        #[link(name = "DockKit", kind = "framework")]
        unsafe extern "C" {
            $(
                #[link_name = $link]
                static $symbol: u8;
            )+

            #[link_name = $hash_link]
            fn $hash_fn();
        }

        impl $name {
            $(
                #[inline]
                pub fn $case() -> Self {
                    unsafe { Self($symbol) }
                }
            )+

            #[inline]
            pub fn as_abi_ptr(&self) -> *const () {
                (self as *const Self).cast()
            }

            #[inline]
            pub fn hash_value(&self) -> isize {
                unsafe { abi::call_value_to_int($hash_fn as *const (), self.as_abi_ptr()) }
            }
        }
    };
}

resilient_enum! {
    /// DockKit `DockAccessory.State`.
    #[doc(alias = "DockAccessory.State")]
    pub struct State {
        hash = dock_accessory_state_hash_value = "$s7DockKit0A9AccessoryC5StateO9hashValueSivg",
        debug = dock_accessory_state_debug_description = "$s7DockKit0A9AccessoryC5StateO16debugDescriptionSSvg",
        cases {
            undocked => DOCK_ACCESSORY_STATE_UNDOCKED = "$s7DockKit0A9AccessoryC5StateO8undockedyA2EmFWC",
            docked => DOCK_ACCESSORY_STATE_DOCKED = "$s7DockKit0A9AccessoryC5StateO6dockedyA2EmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.Category`.
    #[doc(alias = "DockAccessory.Category")]
    pub struct Category {
        hash = dock_accessory_category_hash_value = "$s7DockKit0A9AccessoryC8CategoryO9hashValueSivg",
        debug = dock_accessory_category_debug_description = "$s7DockKit0A9AccessoryC8CategoryO16debugDescriptionSSvg",
        cases {
            tracking_stand => DOCK_ACCESSORY_CATEGORY_TRACKING_STAND = "$s7DockKit0A9AccessoryC8CategoryO13trackingStandyA2EmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.CameraOrientation`.
    #[doc(alias = "DockAccessory.CameraOrientation")]
    pub struct CameraOrientation {
        hash = dock_accessory_camera_orientation_hash_value = "$s7DockKit0A9AccessoryC17CameraOrientationO9hashValueSivg",
        cases {
            unknown => DOCK_ACCESSORY_CAMERA_ORIENTATION_UNKNOWN = "$s7DockKit0A9AccessoryC17CameraOrientationO7unknownyA2EmFWC",
            portrait => DOCK_ACCESSORY_CAMERA_ORIENTATION_PORTRAIT = "$s7DockKit0A9AccessoryC17CameraOrientationO8portraityA2EmFWC",
            portrait_upside_down => DOCK_ACCESSORY_CAMERA_ORIENTATION_PORTRAIT_UPSIDE_DOWN = "$s7DockKit0A9AccessoryC17CameraOrientationO18portraitUpsideDownyA2EmFWC",
            landscape_right => DOCK_ACCESSORY_CAMERA_ORIENTATION_LANDSCAPE_RIGHT = "$s7DockKit0A9AccessoryC17CameraOrientationO14landscapeRightyA2EmFWC",
            landscape_left => DOCK_ACCESSORY_CAMERA_ORIENTATION_LANDSCAPE_LEFT = "$s7DockKit0A9AccessoryC17CameraOrientationO13landscapeLeftyA2EmFWC",
            face_up => DOCK_ACCESSORY_CAMERA_ORIENTATION_FACE_UP = "$s7DockKit0A9AccessoryC17CameraOrientationO6faceUpyA2EmFWC",
            face_down => DOCK_ACCESSORY_CAMERA_ORIENTATION_FACE_DOWN = "$s7DockKit0A9AccessoryC17CameraOrientationO8faceDownyA2EmFWC",
            corrected => DOCK_ACCESSORY_CAMERA_ORIENTATION_CORRECTED = "$s7DockKit0A9AccessoryC17CameraOrientationO9correctedyA2EmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.Observation.ObservationType`.
    #[doc(alias = "DockAccessory.Observation.ObservationType")]
    pub struct ObservationType {
        hash = dock_accessory_observation_type_hash_value = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO9hashValueSivg",
        cases {
            human_face => DOCK_ACCESSORY_OBSERVATION_TYPE_HUMAN_FACE = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO9humanFaceyA2GmFWC",
            human_body => DOCK_ACCESSORY_OBSERVATION_TYPE_HUMAN_BODY = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO9humanBodyyA2GmFWC",
            object => DOCK_ACCESSORY_OBSERVATION_TYPE_OBJECT = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO6objectyA2GmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.BatteryChargeState`.
    #[doc(alias = "DockAccessory.BatteryChargeState")]
    pub struct BatteryChargeState {
        hash = dock_accessory_battery_charge_state_hash_value = "$s7DockKit0A9AccessoryC18BatteryChargeStateO9hashValueSivg",
        cases {
            not_charging => DOCK_ACCESSORY_BATTERY_CHARGE_STATE_NOT_CHARGING = "$s7DockKit0A9AccessoryC18BatteryChargeStateO11notChargingyA2EmFWC",
            charging => DOCK_ACCESSORY_BATTERY_CHARGE_STATE_CHARGING = "$s7DockKit0A9AccessoryC18BatteryChargeStateO8chargingyA2EmFWC",
            not_chargeable => DOCK_ACCESSORY_BATTERY_CHARGE_STATE_NOT_CHARGEABLE = "$s7DockKit0A9AccessoryC18BatteryChargeStateO13notChargeableyA2EmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.FramingMode`.
    #[doc(alias = "DockAccessory.FramingMode")]
    pub struct FramingMode {
        hash = dock_accessory_framing_mode_hash_value = "$s7DockKit0A9AccessoryC11FramingModeO9hashValueSivg",
        cases {
            automatic => DOCK_ACCESSORY_FRAMING_MODE_AUTOMATIC = "$s7DockKit0A9AccessoryC11FramingModeO9automaticyA2EmFWC",
            center => DOCK_ACCESSORY_FRAMING_MODE_CENTER = "$s7DockKit0A9AccessoryC11FramingModeO6centeryA2EmFWC",
            left => DOCK_ACCESSORY_FRAMING_MODE_LEFT = "$s7DockKit0A9AccessoryC11FramingModeO4leftyA2EmFWC",
            right => DOCK_ACCESSORY_FRAMING_MODE_RIGHT = "$s7DockKit0A9AccessoryC11FramingModeO5rightyA2EmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.Animation`.
    #[doc(alias = "DockAccessory.Animation")]
    pub struct Animation {
        hash = dock_accessory_animation_hash_value = "$s7DockKit0A9AccessoryC9AnimationO9hashValueSivg",
        cases {
            wakeup => DOCK_ACCESSORY_ANIMATION_WAKEUP = "$s7DockKit0A9AccessoryC9AnimationO6wakeupyA2EmFWC",
            yes => DOCK_ACCESSORY_ANIMATION_YES = "$s7DockKit0A9AccessoryC9AnimationO3yesyA2EmFWC",
            no => DOCK_ACCESSORY_ANIMATION_NO = "$s7DockKit0A9AccessoryC9AnimationO2noyA2EmFWC",
            kapow => DOCK_ACCESSORY_ANIMATION_KAPOW = "$s7DockKit0A9AccessoryC9AnimationO5kapowyA2EmFWC",
        }
    }
}

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s7DockKit0A9AccessoryC16debugDescriptionSSvg"]
    fn dock_accessory_debug_description();

    #[link_name = "$s7DockKit0A9AccessoryC11framingModeAC07FramingE0Ovg"]
    fn dock_accessory_framing_mode();

    #[link_name = "$s7DockKit0A9AccessoryC16regionOfInterestSo6CGRectVvg"]
    fn dock_accessory_region_of_interest();

    #[link_name = "$s7DockKit0A9AccessoryC10identifierAC10IdentifierVvg"]
    fn dock_accessory_identifier();

    #[link_name = "$s7DockKit0A9AccessoryC6limitsAC6LimitsVvg"]
    fn dock_accessory_limits();

    #[link_name = "$s7DockKit0A9AccessoryC12motionStatesAC06MotionE0Vvg"]
    fn dock_accessory_motion_states();

    #[link_name = "$s7DockKit0A9AccessoryC13batteryStatesAC07BatteryE0Vvg"]
    fn dock_accessory_battery_states();

    #[link_name = "$s7DockKit0A9AccessoryC15accessoryEventsAC0cE0Vvg"]
    fn dock_accessory_events();

    #[link_name = "$s7DockKit0A9AccessoryC14trackingStatesAC08TrackingE0Vvg"]
    fn dock_accessory_tracking_states();

    #[link_name = "$s7DockKit0A9AccessoryC9setLimitsyyAC0E0VKF"]
    fn dock_accessory_set_limits();

    #[link_name = "$s7DockKit0A9AccessoryC18setAngularVelocityyySo10SPVector3DaYaKF"]
    fn dock_accessory_set_angular_velocity();

    #[link_name = "$s7DockKit0A9AccessoryC18setAngularVelocityyySo10SPVector3DaYaKFTu"]
    static DOCK_ACCESSORY_SET_ANGULAR_VELOCITY_ASYNC: u8;

    #[link_name = "$s7DockKit0A9AccessoryC13selectSubject2atySo7CGPointV_tYaKF"]
    fn dock_accessory_select_subject();

    #[link_name = "$s7DockKit0A9AccessoryC13selectSubject2atySo7CGPointV_tYaKFTu"]
    static DOCK_ACCESSORY_SELECT_SUBJECT_ASYNC: u8;

    #[link_name = "$s7DockKit0A9AccessoryC14setFramingModeyyAC0eF0OYaKF"]
    fn dock_accessory_set_framing_mode();

    #[link_name = "$s7DockKit0A9AccessoryC14setFramingModeyyAC0eF0OYaKFTu"]
    static DOCK_ACCESSORY_SET_FRAMING_MODE_ASYNC: u8;

    #[link_name = "$s7DockKit0A9AccessoryC19setRegionOfInterestyySo6CGRectVYaKF"]
    fn dock_accessory_set_region_of_interest();

    #[link_name = "$s7DockKit0A9AccessoryC19setRegionOfInterestyySo6CGRectVYaKFTu"]
    static DOCK_ACCESSORY_SET_REGION_OF_INTEREST_ASYNC: u8;

    #[link_name = "$s7DockKit0A9AccessoryC14selectSubjectsyySay10Foundation4UUIDVGYaKF"]
    fn dock_accessory_select_subjects();

    #[link_name = "$s7DockKit0A9AccessoryC14selectSubjectsyySay10Foundation4UUIDVGYaKFTu"]
    static DOCK_ACCESSORY_SELECT_SUBJECTS_ASYNC: u8;

    #[link_name = "$s7DockKit0A9AccessoryC7animate6motionSo10NSProgressCAC9AnimationO_tYaKF"]
    fn dock_accessory_animate();

    #[link_name = "$s7DockKit0A9AccessoryC7animate6motionSo10NSProgressCAC9AnimationO_tYaKFTu"]
    static DOCK_ACCESSORY_ANIMATE_ASYNC: u8;

    #[link_name = "$s7DockKit0A9AccessoryC14setOrientation_8duration8relativeSo10NSProgressCSo10SPVector3Da_s8DurationVSbtYaKF"]
    fn dock_accessory_set_vector_orientation();

    #[link_name = "$s7DockKit0A9AccessoryC14setOrientation_8duration8relativeSo10NSProgressCSo10SPVector3Da_s8DurationVSbtKF"]
    fn dock_accessory_set_vector_orientation_sync();

    #[link_name = "$s7DockKit0A9AccessoryC14setOrientation_8duration8relativeSo10NSProgressCSo10SPVector3Da_s8DurationVSbtYaKFTu"]
    static DOCK_ACCESSORY_SET_VECTOR_ORIENTATION_ASYNC: u8;

    #[link_name = "$s7DockKit0A9AccessoryC14setOrientation_8duration8relativeSo10NSProgressCSo12SPRotation3Da_s8DurationVSbtYaKF"]
    fn dock_accessory_set_rotation_orientation();

    #[link_name = "$s7DockKit0A9AccessoryC14setOrientation_8duration8relativeSo10NSProgressCSo12SPRotation3Da_s8DurationVSbtKF"]
    fn dock_accessory_set_rotation_orientation_sync();

    #[link_name = "$s7DockKit0A9AccessoryC14setOrientation_8duration8relativeSo10NSProgressCSo12SPRotation3Da_s8DurationVSbtYaKFTu"]
    static DOCK_ACCESSORY_SET_ROTATION_ORIENTATION_ASYNC: u8;

    #[link_name = "$ss8DurationV7secondsyABSdFZ"]
    fn swift_duration_seconds();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC5track_17cameraInformationySayAC11ObservationVG_AC06CameraF0VtYaKF"]
    fn dock_accessory_track_observations();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC5track_17cameraInformationySayAC11ObservationVG_AC06CameraF0VtYaKFTu"]
    static DOCK_ACCESSORY_TRACK_OBSERVATIONS_ASYNC: u8;

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC5track_17cameraInformation5imageySayAC11ObservationVG_AC06CameraF0VSo11CVBufferRefatYaKF"]
    fn dock_accessory_track_observations_with_image();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC5track_17cameraInformation5imageySayAC11ObservationVG_AC06CameraF0VSo11CVBufferRefatYaKFTu"]
    static DOCK_ACCESSORY_TRACK_OBSERVATIONS_WITH_IMAGE_ASYNC: u8;

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC5track_17cameraInformationySaySo16AVMetadataObjectCG_AC06CameraF0VtYaKF"]
    fn dock_accessory_track_metadata();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC5track_17cameraInformationySaySo16AVMetadataObjectCG_AC06CameraF0VtYaKFTu"]
    static DOCK_ACCESSORY_TRACK_METADATA_ASYNC: u8;

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC5track_17cameraInformation5imageySaySo16AVMetadataObjectCG_AC06CameraF0VSo11CVBufferRefatYaKF"]
    fn dock_accessory_track_metadata_with_image();

    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC5track_17cameraInformation5imageySaySo16AVMetadataObjectCG_AC06CameraF0VSo11CVBufferRefatYaKFTu"]
    static DOCK_ACCESSORY_TRACK_METADATA_WITH_IMAGE_ASYNC: u8;
}

impl Accessory {
    #[doc(alias = "DockAccessory.hashValue")]
    pub fn hash_value(&self) -> isize {
        unsafe {
            abi::call_object_to_int(
                dock_accessory_hash_value as *const (),
                (self as *const Self).cast(),
            )
        }
    }

    #[doc(alias = "DockAccessory.identifier")]
    pub fn identifier(&self) -> Identifier {
        unsafe {
            let mut storage = Storage::<IdentifierValue>::new();
            abi::call_object_to_value(
                dock_accessory_identifier as *const (),
                (self as *const Self).cast(),
                storage.as_mut_ptr(),
            );
            Identifier(storage.assume_init())
        }
    }

    #[doc(alias = "DockAccessory.debugDescription")]
    #[inline]
    pub fn debug_desc(&self) -> swift::String {
        unsafe {
            swift::String::from_raw(abi::call_object_to_string(
                dock_accessory_debug_description as *const (),
                (self as *const Self).cast(),
            ))
        }
    }

    #[doc(alias = "DockAccessory.framingMode")]
    #[inline]
    pub fn framing_mode(&self) -> FramingMode {
        let mut value = FramingMode::automatic();
        unsafe {
            abi::call_object_to_value(
                dock_accessory_framing_mode as *const (),
                (self as *const Self).cast(),
                (&mut value as *mut FramingMode).cast(),
            );
        }
        value
    }

    /// `DockAccessory.firmwareVersion`.
    #[doc(alias = "DockAccessory.firmwareVersion")]
    pub fn firmware_version(&self) -> Option<swift::String> {
        unsafe { self.optional_string(dock_accessory_firmware_version as *const ()) }
    }

    /// `DockAccessory.hardwareModel`.
    #[doc(alias = "DockAccessory.hardwareModel")]
    pub fn hardware_model(&self) -> Option<swift::String> {
        unsafe { self.optional_string(dock_accessory_hardware_model as *const ()) }
    }

    /// Reads a `String?` getter, whose two words are the string itself; Swift
    /// spells the empty case as a string with a null discriminator word.
    unsafe fn optional_string(&self, getter: *const ()) -> Option<swift::String> {
        unsafe {
            let raw = abi::call_object_to_string(getter, (self as *const Self).cast());
            (raw.word0 != 0 || raw.word1 != 0).then(|| swift::String::from_raw(raw))
        }
    }

    pub fn region_of_interest(&self) -> cg::Rect {
        let (x, y, width, height) = unsafe {
            abi::call_object_to_rect(
                dock_accessory_region_of_interest as *const (),
                (self as *const Self).cast(),
            )
        };
        cg::Rect {
            origin: cg::Point { x, y },
            size: cg::Size { width, height },
        }
    }

    #[doc(alias = "DockAccessory.limits")]
    pub fn limits(&self) -> Result<Limits, arc::R<ns::Error>> {
        unsafe {
            let mut storage = Storage::<LimitsValue>::new();
            let error = abi::call_object_to_throwing_value(
                dock_accessory_limits as *const (),
                (self as *const Self).cast(),
                storage.as_mut_ptr(),
            );
            if error.is_null() {
                Ok(Limits(storage.assume_init()))
            } else {
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    #[doc(alias = "DockAccessory.motionStates")]
    pub fn motion_states(&self) -> Result<MotionStates, arc::R<ns::Error>> {
        unsafe {
            let mut storage = Storage::<MotionStatesValue>::new();
            let error = abi::call_object_to_throwing_value(
                dock_accessory_motion_states as *const (),
                (self as *const Self).cast(),
                storage.as_mut_ptr(),
            );
            if error.is_null() {
                Ok(MotionStates::from_storage(storage))
            } else {
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    #[doc(alias = "DockAccessory.batteryStates")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    pub fn battery_states(&self) -> Result<BatteryStates, arc::R<ns::Error>> {
        unsafe {
            let mut storage = Storage::<BatteryStatesValue>::new();
            let error = abi::call_object_to_throwing_value(
                dock_accessory_battery_states as *const (),
                (self as *const Self).cast(),
                storage.as_mut_ptr(),
            );
            if error.is_null() {
                Ok(BatteryStates::from_storage(storage))
            } else {
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    #[doc(alias = "DockAccessory.accessoryEvents")]
    #[crate::api::available(macos = 14.4, ios = 17.4)]
    pub fn accessory_events(&self) -> Result<AccessoryEvents, arc::R<ns::Error>> {
        unsafe {
            let mut storage = Storage::<AccessoryEventsValue>::new();
            let error = abi::call_object_to_throwing_value(
                dock_accessory_events as *const (),
                (self as *const Self).cast(),
                storage.as_mut_ptr(),
            );
            if error.is_null() {
                Ok(AccessoryEvents::from_storage(storage))
            } else {
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    #[doc(alias = "DockAccessory.trackingStates")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    pub fn tracking_states(&self) -> Result<TrackingStates, arc::R<ns::Error>> {
        unsafe {
            let mut storage = Storage::<TrackingStatesValue>::new();
            let error = abi::call_object_to_throwing_value(
                dock_accessory_tracking_states as *const (),
                (self as *const Self).cast(),
                storage.as_mut_ptr(),
            );
            if error.is_null() {
                Ok(TrackingStates::from_storage(storage))
            } else {
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    #[doc(alias = "DockAccessory.setLimits(_:)")]
    pub fn set_limits(&self, limits: &Limits) -> Result<(), arc::R<ns::Error>> {
        unsafe {
            let error = abi::call_value_object_to_throwing_void(
                dock_accessory_set_limits as *const (),
                limits.as_ptr(),
                (self as *const Self).cast(),
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    /// Deprecated synchronous DockKit orientation API.
    #[doc(alias = "DockAccessory.setOrientation(_:duration:relative:)")]
    #[deprecated = "use the async set_orientation method on iOS 18 or macOS 15"]
    pub fn set_orientation_sync(
        &self,
        rotation: spatial::Vector3D,
        duration: std::time::Duration,
        relative: bool,
    ) -> Result<arc::R<ns::Progress>, arc::R<ns::Error>> {
        unsafe {
            let duration = abi::call_double_to_words2(
                swift_duration_seconds as *const (),
                duration.as_secs_f64(),
            );
            let (result, error) = abi::call_vector_duration_bool_object(
                dock_accessory_set_vector_orientation_sync as *const (),
                (rotation.x, rotation.y, rotation.z),
                duration,
                relative,
                (self as *const Self).cast(),
            );
            if error.is_null() {
                Ok(arc::R::from_raw(result.cast()))
            } else {
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    /// Deprecated synchronous DockKit rotation API.
    #[doc(alias = "DockAccessory.setOrientation(_:duration:relative:)")]
    #[deprecated = "use the async set_rotation method on iOS 18 or macOS 15"]
    pub fn set_rotation_sync(
        &self,
        rotation: spatial::Rotation3D,
        duration: std::time::Duration,
        relative: bool,
    ) -> Result<arc::R<ns::Progress>, arc::R<ns::Error>> {
        unsafe {
            let duration = abi::call_double_to_words2(
                swift_duration_seconds as *const (),
                duration.as_secs_f64(),
            );
            let (result, error) = abi::call_rotation_duration_bool_object(
                dock_accessory_set_rotation_orientation_sync as *const (),
                (rotation.x, rotation.y, rotation.z, rotation.w),
                duration,
                relative,
                (self as *const Self).cast(),
            );
            if error.is_null() {
                Ok(arc::R::from_raw(result.cast()))
            } else {
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    #[doc(alias = "DockAccessory.setAngularVelocity(_:)")]
    pub fn set_angular_velocity_handler<F>(&self, velocity: spatial::Vector3D, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        AsyncVoidTask::start(
            self,
            AsyncVoidArg::Vector(velocity),
            dock_accessory_set_angular_velocity as *const (),
            (&raw const DOCK_ACCESSORY_SET_ANGULAR_VELOCITY_ASYNC).cast(),
            callback,
        );
    }

    #[doc(alias = "DockAccessory.selectSubject(at:)")]
    pub fn select_subject_handler<F>(&self, point: cg::Point, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        AsyncVoidTask::start(
            self,
            AsyncVoidArg::Point(point),
            dock_accessory_select_subject as *const (),
            (&raw const DOCK_ACCESSORY_SELECT_SUBJECT_ASYNC).cast(),
            callback,
        );
    }

    #[doc(alias = "DockAccessory.setFramingMode(_:)")]
    pub fn set_framing_mode_handler<F>(&self, mode: FramingMode, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        AsyncVoidTask::start(
            self,
            AsyncVoidArg::FramingMode(mode),
            dock_accessory_set_framing_mode as *const (),
            (&raw const DOCK_ACCESSORY_SET_FRAMING_MODE_ASYNC).cast(),
            callback,
        );
    }

    #[doc(alias = "DockAccessory.setRegionOfInterest(_:)")]
    pub fn set_region_of_interest_handler<F>(&self, rect: cg::Rect, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        AsyncVoidTask::start(
            self,
            AsyncVoidArg::Rect(rect),
            dock_accessory_set_region_of_interest as *const (),
            (&raw const DOCK_ACCESSORY_SET_REGION_OF_INTEREST_ASYNC).cast(),
            callback,
        );
    }

    #[doc(alias = "DockAccessory.selectSubjects(_:)")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    pub fn select_subjects_handler<F>(&self, ids: &[Uuid], callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        AsyncVoidTask::start(
            self,
            AsyncVoidArg::Subjects(UuidArray::from_slice(ids)),
            dock_accessory_select_subjects as *const (),
            (&raw const DOCK_ACCESSORY_SELECT_SUBJECTS_ASYNC).cast(),
            callback,
        );
    }

    #[cfg(feature = "av")]
    #[doc(alias = "DockAccessory.track(_:cameraInformation:)")]
    pub fn track_handler<F>(
        &self,
        observations: Observations,
        camera: CameraInformation,
        callback: F,
    ) where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        AsyncVoidTask::start(
            self,
            AsyncVoidArg::Track {
                data: TrackData::Observations(observations),
                camera,
                image: None,
            },
            dock_accessory_track_observations as *const (),
            (&raw const DOCK_ACCESSORY_TRACK_OBSERVATIONS_ASYNC).cast(),
            callback,
        );
    }

    #[cfg(feature = "av")]
    #[doc(alias = "DockAccessory.track(_:cameraInformation:image:)")]
    pub fn track_with_image_handler<F>(
        &self,
        observations: Observations,
        camera: CameraInformation,
        image: &crate::cv::PixelBuf,
        callback: F,
    ) where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        AsyncVoidTask::start(
            self,
            AsyncVoidArg::Track {
                data: TrackData::Observations(observations),
                camera,
                image: Some(arc::Retain::retained(image)),
            },
            dock_accessory_track_observations_with_image as *const (),
            (&raw const DOCK_ACCESSORY_TRACK_OBSERVATIONS_WITH_IMAGE_ASYNC).cast(),
            callback,
        );
    }

    #[cfg(feature = "av")]
    #[doc(alias = "DockAccessory.track(_:cameraInformation:)")]
    pub fn track_metadata_handler<F>(
        &self,
        metadata: &[&crate::av::MetadataObj],
        camera: CameraInformation,
        callback: F,
    ) where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        AsyncVoidTask::start(
            self,
            AsyncVoidArg::Track {
                data: TrackData::Metadata(MetadataObjects::from_slice(metadata)),
                camera,
                image: None,
            },
            dock_accessory_track_metadata as *const (),
            (&raw const DOCK_ACCESSORY_TRACK_METADATA_ASYNC).cast(),
            callback,
        );
    }

    #[cfg(feature = "av")]
    #[doc(alias = "DockAccessory.track(_:cameraInformation:image:)")]
    pub fn track_metadata_with_image_handler<F>(
        &self,
        metadata: &[&crate::av::MetadataObj],
        camera: CameraInformation,
        image: &crate::cv::PixelBuf,
        callback: F,
    ) where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        AsyncVoidTask::start(
            self,
            AsyncVoidArg::Track {
                data: TrackData::Metadata(MetadataObjects::from_slice(metadata)),
                camera,
                image: Some(arc::Retain::retained(image)),
            },
            dock_accessory_track_metadata_with_image as *const (),
            (&raw const DOCK_ACCESSORY_TRACK_METADATA_WITH_IMAGE_ASYNC).cast(),
            callback,
        );
    }

    #[cfg(feature = "async")]
    pub fn set_angular_velocity(
        &self,
        velocity: spatial::Vector3D,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        self.async_void(move |accessory, callback| {
            accessory.set_angular_velocity_handler(velocity, callback)
        })
    }

    #[cfg(feature = "async")]
    pub fn select_subject(
        &self,
        point: cg::Point,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        self.async_void(move |accessory, callback| {
            accessory.select_subject_handler(point, callback)
        })
    }

    #[cfg(feature = "async")]
    pub fn set_framing_mode(
        &self,
        mode: FramingMode,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        self.async_void(move |accessory, callback| {
            accessory.set_framing_mode_handler(mode, callback)
        })
    }

    #[cfg(feature = "async")]
    pub fn set_region_of_interest(
        &self,
        rect: cg::Rect,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        self.async_void(move |accessory, callback| {
            accessory.set_region_of_interest_handler(rect, callback)
        })
    }

    #[cfg(feature = "async")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    pub fn select_subjects(
        &self,
        ids: &[Uuid],
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        let ids = UuidArray::from_slice(ids);
        self.async_void(move |accessory, callback| {
            AsyncVoidTask::start(
                accessory,
                AsyncVoidArg::Subjects(ids),
                dock_accessory_select_subjects as *const (),
                (&raw const DOCK_ACCESSORY_SELECT_SUBJECTS_ASYNC).cast(),
                callback,
            )
        })
    }

    #[cfg(all(feature = "async", feature = "av"))]
    pub fn track(
        &self,
        observations: Observations,
        camera: CameraInformation,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        self.async_void(move |accessory, callback| {
            accessory.track_handler(observations, camera, callback)
        })
    }

    #[cfg(all(feature = "async", feature = "av"))]
    pub fn track_with_image(
        &self,
        observations: Observations,
        camera: CameraInformation,
        image: &crate::cv::PixelBuf,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        let image = arc::Retain::retained(image);
        self.async_void(move |accessory, callback| {
            accessory.track_with_image_handler(observations, camera, &image, callback)
        })
    }

    #[cfg(all(feature = "async", feature = "av"))]
    pub fn track_metadata(
        &self,
        metadata: &[&crate::av::MetadataObj],
        camera: CameraInformation,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        let metadata = MetadataObjects::from_slice(metadata);
        self.async_void(move |accessory, callback| {
            AsyncVoidTask::start(
                accessory,
                AsyncVoidArg::Track {
                    data: TrackData::Metadata(metadata),
                    camera,
                    image: None,
                },
                dock_accessory_track_metadata as *const (),
                (&raw const DOCK_ACCESSORY_TRACK_METADATA_ASYNC).cast(),
                callback,
            )
        })
    }

    #[cfg(all(feature = "async", feature = "av"))]
    pub fn track_metadata_with_image(
        &self,
        metadata: &[&crate::av::MetadataObj],
        camera: CameraInformation,
        image: &crate::cv::PixelBuf,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        let metadata = MetadataObjects::from_slice(metadata);
        let image = arc::Retain::retained(image);
        self.async_void(move |accessory, callback| {
            AsyncVoidTask::start(
                accessory,
                AsyncVoidArg::Track {
                    data: TrackData::Metadata(metadata),
                    camera,
                    image: Some(image),
                },
                dock_accessory_track_metadata_with_image as *const (),
                (&raw const DOCK_ACCESSORY_TRACK_METADATA_WITH_IMAGE_ASYNC).cast(),
                callback,
            )
        })
    }

    #[cfg(feature = "async")]
    fn async_void<F>(&self, start: F) -> crate::blocks::Completion<Result<(), arc::R<ns::Error>>>
    where
        F: FnOnce(&Accessory, Box<dyn FnOnce(Result<(), arc::R<ns::Error>>) + Send>),
    {
        let shared = crate::blocks::Shared::new();
        let comp = crate::blocks::Completion(shared.clone());
        start(self, Box::new(move |result| shared.lock().ready(result)));
        comp
    }

    #[doc(alias = "DockAccessory.animate(motion:)")]
    pub fn animate_handler<F>(&self, animation: Animation, callback: F)
    where
        F: FnOnce(Result<arc::R<ns::Progress>, arc::R<ns::Error>>) + Send + 'static,
    {
        AsyncProgressTask::start(
            self,
            AsyncProgressArg::Animation(animation),
            dock_accessory_animate as *const (),
            (&raw const DOCK_ACCESSORY_ANIMATE_ASYNC).cast(),
            callback,
        );
    }

    #[doc(alias = "DockAccessory.setOrientation(_:duration:relative:)")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    pub fn set_orientation_handler<F>(
        &self,
        rotation: spatial::Vector3D,
        duration: std::time::Duration,
        relative: bool,
        callback: F,
    ) where
        F: FnOnce(Result<arc::R<ns::Progress>, arc::R<ns::Error>>) + Send + 'static,
    {
        let duration = unsafe {
            abi::call_double_to_words2(swift_duration_seconds as *const (), duration.as_secs_f64())
        };
        AsyncProgressTask::start(
            self,
            AsyncProgressArg::Vector {
                rotation,
                duration,
                relative,
            },
            dock_accessory_set_vector_orientation as *const (),
            (&raw const DOCK_ACCESSORY_SET_VECTOR_ORIENTATION_ASYNC).cast(),
            callback,
        );
    }

    #[doc(alias = "DockAccessory.setOrientation(_:duration:relative:)")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    pub fn set_rotation_handler<F>(
        &self,
        rotation: spatial::Rotation3D,
        duration: std::time::Duration,
        relative: bool,
        callback: F,
    ) where
        F: FnOnce(Result<arc::R<ns::Progress>, arc::R<ns::Error>>) + Send + 'static,
    {
        let duration = unsafe {
            abi::call_double_to_words2(swift_duration_seconds as *const (), duration.as_secs_f64())
        };
        AsyncProgressTask::start(
            self,
            AsyncProgressArg::Rotation {
                rotation,
                duration,
                relative,
            },
            dock_accessory_set_rotation_orientation as *const (),
            (&raw const DOCK_ACCESSORY_SET_ROTATION_ORIENTATION_ASYNC).cast(),
            callback,
        );
    }

    #[cfg(feature = "async")]
    pub fn animate(
        &self,
        animation: Animation,
    ) -> impl std::future::Future<Output = Result<arc::R<ns::Progress>, arc::R<ns::Error>>> {
        self.async_progress(move |accessory, callback| {
            accessory.animate_handler(animation, callback)
        })
    }

    #[cfg(feature = "async")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    #[allow(unused_unsafe)]
    pub fn set_orientation(
        &self,
        rotation: spatial::Vector3D,
        duration: std::time::Duration,
        relative: bool,
    ) -> impl std::future::Future<Output = Result<arc::R<ns::Progress>, arc::R<ns::Error>>> {
        self.async_progress(move |accessory, callback| unsafe {
            accessory.set_orientation_handler(rotation, duration, relative, callback)
        })
    }

    #[cfg(feature = "async")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    #[allow(unused_unsafe)]
    pub fn set_rotation(
        &self,
        rotation: spatial::Rotation3D,
        duration: std::time::Duration,
        relative: bool,
    ) -> impl std::future::Future<Output = Result<arc::R<ns::Progress>, arc::R<ns::Error>>> {
        self.async_progress(move |accessory, callback| unsafe {
            accessory.set_rotation_handler(rotation, duration, relative, callback)
        })
    }

    #[cfg(feature = "async")]
    fn async_progress<F>(
        &self,
        start: F,
    ) -> crate::blocks::Completion<Result<arc::R<ns::Progress>, arc::R<ns::Error>>>
    where
        F: FnOnce(
            &Accessory,
            Box<dyn FnOnce(Result<arc::R<ns::Progress>, arc::R<ns::Error>>) + Send>,
        ),
    {
        let shared = crate::blocks::Shared::new();
        let comp = crate::blocks::Completion(shared.clone());
        start(self, Box::new(move |result| shared.lock().ready(result)));
        comp
    }
}

impl PartialEq for Accessory {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            abi::call_objects_to_bool(
                dock_accessory_equal as *const (),
                (self as *const Self).cast(),
                (other as *const Self).cast(),
            )
        }
    }
}

impl Eq for Accessory {}

impl std::hash::Hash for Accessory {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.hash_value(), state)
    }
}

enum AsyncVoidArg {
    Vector(spatial::Vector3D),
    Point(cg::Point),
    Rect(cg::Rect),
    FramingMode(FramingMode),
    Subjects(UuidArray),
    #[cfg(feature = "av")]
    Track {
        data: TrackData,
        camera: CameraInformation,
        image: Option<arc::R<crate::cv::PixelBuf>>,
    },
}

#[cfg(feature = "av")]
enum TrackData {
    Observations(Observations),
    Metadata(MetadataObjects),
}

#[cfg(feature = "av")]
impl TrackData {
    fn as_raw(&self) -> *mut () {
        match self {
            Self::Observations(value) => value.as_raw(),
            Self::Metadata(value) => value.raw,
        }
    }
}

struct AsyncVoidTask {
    accessory: arc::R<Accessory>,
    arg: AsyncVoidArg,
    function: *const (),
    async_fn: *const u8,
    error: *mut (),
    callback: Option<Box<dyn FnOnce(Result<(), arc::R<ns::Error>>) + Send>>,
}

unsafe impl Send for AsyncVoidTask {}

impl AsyncVoidTask {
    fn start<F>(
        accessory: &Accessory,
        arg: AsyncVoidArg,
        function: *const (),
        async_fn: *const u8,
        callback: F,
    ) where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            let task = Box::new(Self {
                accessory: arc::Retain::retained(accessory),
                arg,
                function,
                async_fn,
                error: core::ptr::null_mut(),
                callback: Some(Box::new(callback)),
            });
            let context = Box::into_raw(task).cast();
            let _ = concurrency::task_create(
                concurrency::ENQUEUED_DISCARDING_TASK_FLAGS,
                core::ptr::null(),
                (&raw const CIDRE_DK_ASYNC_VOID_TASK_DESCRIPTOR).cast(),
                context,
            );
        }
    }
}

swift_async_task_descriptor!(
    CIDRE_DK_ASYNC_VOID_TASK_DESCRIPTOR,
    entry: dock_accessory_async_void_entry,
    context_size: "112",
);

#[unsafe(naked)]
unsafe extern "C" fn dock_accessory_async_void_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "mov x1, x22",
        "bl {prepare}",
        "ldr x8, [x22, #48]",
        "ldr w0, [x8, #4]",
        "bl {task_alloc}",
        "mov x9, x0",
        "str x9, [x22, #56]",
        swift_async_store_parent!(),
        swift_async_store_resume!("{resume}"),
        "ldr x10, [x22, #96]",
        "cmp x10, #0",
        "b.eq 0f",
        "cmp x10, #1",
        "b.eq 1f",
        "cmp x10, #2",
        "b.eq 2f",
        "cmp x10, #3",
        "b.ne 4f",
        "ldr x0, [x22, #64]",
        "b 3f",
        "0:",
        "ldr d0, [x22, #64]",
        "ldr d1, [x22, #72]",
        "ldr d2, [x22, #80]",
        "b 3f",
        "1:",
        "ldr d0, [x22, #64]",
        "ldr d1, [x22, #72]",
        "b 3f",
        "2:",
        "ldr d0, [x22, #64]",
        "ldr d1, [x22, #72]",
        "ldr d2, [x22, #80]",
        "ldr d3, [x22, #88]",
        "b 3f",
        "4:",
        "cmp x10, #6",
        "b.ne 5f",
        "ldr x0, [x22, #64]",
        "b 3f",
        "5:",
        "ldr x0, [x22, #64]",
        "ldr x1, [x22, #72]",
        "cmp x10, #4",
        "b.eq 3f",
        "ldr x2, [x22, #80]",
        "3:",
        "ldr x20, [x22, #32]",
        "ldr x16, [x22, #40]",
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        prepare = sym cidre_dk_async_void_prepare,
        task_alloc = sym swift_task_alloc,
        resume = sym dock_accessory_async_void_resume,
    );
}

#[unsafe(naked)]
unsafe extern "C" fn dock_accessory_async_void_resume() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        swift_async_load_parent!(),
        "str x9, [sp]",
        "mov x0, x22",
        "bl {task_dealloc}",
        "ldr x9, [sp]",
        "ldr x0, [x9, #24]",
        "mov x1, x20",
        "bl {set_error}",
        "ldr x22, [sp]",
        swift_async_function_pointer!("{finish}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {task_switch}",
        task_dealloc = sym swift_task_dealloc,
        set_error = sym cidre_dk_async_void_set_error,
        finish = sym dock_accessory_async_void_finish,
        task_switch = sym swift_task_switch,
    );
}

#[unsafe(naked)]
unsafe extern "C" fn dock_accessory_async_void_finish() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {complete}",
        "ldr x22, [sp, #8]",
        "ldr x9, [x22, #16]",
        swift_async_load_resume!(),
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        complete = sym cidre_dk_async_void_complete,
    );
}

extern "C" fn cidre_dk_async_void_prepare(task: *mut AsyncVoidTask, context: *mut u64) {
    unsafe {
        let task = &mut *task;
        context
            .add(4)
            .write(task.accessory.as_ptr().cast::<()>() as u64);
        context.add(5).write(task.function as u64);
        context.add(6).write(task.async_fn as u64);
        match &task.arg {
            AsyncVoidArg::Vector(value) => {
                context.add(8).write(value.x.to_bits());
                context.add(9).write(value.y.to_bits());
                context.add(10).write(value.z.to_bits());
                context.add(12).write(0);
            }
            AsyncVoidArg::Point(value) => {
                context.add(8).write(value.x.to_bits());
                context.add(9).write(value.y.to_bits());
                context.add(12).write(1);
            }
            AsyncVoidArg::Rect(value) => {
                context.add(8).write(value.origin.x.to_bits());
                context.add(9).write(value.origin.y.to_bits());
                context.add(10).write(value.size.width.to_bits());
                context.add(11).write(value.size.height.to_bits());
                context.add(12).write(2);
            }
            AsyncVoidArg::FramingMode(value) => {
                context.add(8).write((value as *const FramingMode) as u64);
                context.add(12).write(3);
            }
            AsyncVoidArg::Subjects(ids) => {
                context.add(8).write(ids.raw as u64);
                context.add(12).write(6);
            }
            #[cfg(feature = "av")]
            AsyncVoidArg::Track {
                data,
                camera,
                image,
            } => {
                context.add(8).write(data.as_raw() as u64);
                context.add(9).write(camera.as_ptr() as u64);
                context.add(10).write(
                    image
                        .as_ref()
                        .map_or(core::ptr::null_mut(), |image| image.as_ptr())
                        .cast::<()>() as u64,
                );
                context.add(12).write(if image.is_some() { 5 } else { 4 });
            }
        }
    }
}

struct UuidArray {
    raw: *mut (),
}

unsafe impl Send for UuidArray {}

impl UuidArray {
    fn from_slice(values: &[Uuid]) -> Self {
        unsafe {
            let metadata = UuidValue::metadata();
            let (raw, elements) = abi::allocate_uninitialized_array(values.len(), metadata);
            let stride = abi::value_layout(metadata).stride;
            for (index, value) in values.iter().enumerate() {
                abi::initialize_with_copy(
                    elements.cast::<u8>().add(index * stride).cast(),
                    value.as_ptr(),
                    metadata,
                );
            }
            Self { raw }
        }
    }
}

impl Drop for UuidArray {
    fn drop(&mut self) {
        unsafe { abi::bridge_object_release(self.raw as usize) }
    }
}

#[cfg(feature = "av")]
struct MetadataObjects {
    raw: *mut (),
}

#[cfg(feature = "av")]
unsafe impl Send for MetadataObjects {}

#[cfg(feature = "av")]
impl MetadataObjects {
    fn from_slice(values: &[&crate::av::MetadataObj]) -> Self {
        unsafe {
            let class = crate::objc::objc_getClass(c"AVMetadataObject".as_ptr().cast())
                .expect("AVMetadataObject class missing");
            let metadata = abi::objc_class_metadata(
                (class as *const crate::objc::Class<crate::objc::Id>).cast(),
            );
            let (raw, elements) = abi::allocate_uninitialized_array(values.len(), metadata);
            let stride = abi::value_layout(metadata).stride;
            for (index, value) in values.iter().enumerate() {
                let object = (*value as *const crate::av::MetadataObj)
                    .cast_mut()
                    .cast::<()>();
                abi::initialize_with_copy(
                    elements.cast::<u8>().add(index * stride).cast(),
                    (&raw const object).cast(),
                    metadata,
                );
            }
            Self { raw }
        }
    }
}

#[cfg(feature = "av")]
impl Drop for MetadataObjects {
    fn drop(&mut self) {
        unsafe { abi::bridge_object_release(self.raw as usize) }
    }
}

extern "C" fn cidre_dk_async_void_set_error(task: *mut AsyncVoidTask, error: *mut ()) {
    unsafe { (*task).error = error }
}

extern "C" fn cidre_dk_async_void_complete(task: *mut AsyncVoidTask) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        let mut task = Box::from_raw(task);
        let callback = task.callback.take().expect("DockKit callback missing");
        if task.error.is_null() {
            callback(Ok(()));
        } else {
            callback(Err(arc::R::from_raw(
                abi::error_as_ns_error(task.error).cast(),
            )));
        }
    }));
}

enum AsyncProgressArg {
    Animation(Animation),
    Vector {
        rotation: spatial::Vector3D,
        duration: (u64, u64),
        relative: bool,
    },
    Rotation {
        rotation: spatial::Rotation3D,
        duration: (u64, u64),
        relative: bool,
    },
}

struct AsyncProgressTask {
    accessory: arc::R<Accessory>,
    arg: AsyncProgressArg,
    function: *const (),
    async_fn: *const u8,
    result: *mut (),
    error: *mut (),
    callback: Option<Box<dyn FnOnce(Result<arc::R<ns::Progress>, arc::R<ns::Error>>) + Send>>,
}

unsafe impl Send for AsyncProgressTask {}

impl AsyncProgressTask {
    fn start<F>(
        accessory: &Accessory,
        arg: AsyncProgressArg,
        function: *const (),
        async_fn: *const u8,
        callback: F,
    ) where
        F: FnOnce(Result<arc::R<ns::Progress>, arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            let task = Box::new(Self {
                accessory: arc::Retain::retained(accessory),
                arg,
                function,
                async_fn,
                result: core::ptr::null_mut(),
                error: core::ptr::null_mut(),
                callback: Some(Box::new(callback)),
            });
            let context = Box::into_raw(task).cast();
            let _ = concurrency::task_create(
                concurrency::ENQUEUED_DISCARDING_TASK_FLAGS,
                core::ptr::null(),
                (&raw const CIDRE_DK_ASYNC_PROGRESS_TASK_DESCRIPTOR).cast(),
                context,
            );
        }
    }
}

swift_async_task_descriptor!(
    CIDRE_DK_ASYNC_PROGRESS_TASK_DESCRIPTOR,
    entry: dock_accessory_async_progress_entry,
    context_size: "128",
);

#[unsafe(naked)]
unsafe extern "C" fn dock_accessory_async_progress_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "mov x1, x22",
        "bl {prepare}",
        "ldr x8, [x22, #48]",
        "ldr w0, [x8, #4]",
        "bl {task_alloc}",
        "mov x9, x0",
        "str x9, [x22, #56]",
        swift_async_store_parent!(),
        swift_async_store_resume!("{resume}"),
        "ldr x10, [x22, #120]",
        "cbnz x10, 0f",
        "ldr x0, [x22, #64]",
        "b 2f",
        "0:",
        "cmp x10, #1",
        "b.ne 1f",
        "ldr d0, [x22, #64]",
        "ldr d1, [x22, #72]",
        "ldr d2, [x22, #80]",
        "b 3f",
        "1:",
        "ldr q0, [x22, #64]",
        "ldr q1, [x22, #80]",
        "3:",
        "ldr x0, [x22, #96]",
        "ldr x1, [x22, #104]",
        "ldr x2, [x22, #112]",
        "2:",
        "ldr x20, [x22, #32]",
        "ldr x16, [x22, #40]",
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        prepare = sym cidre_dk_async_progress_prepare,
        task_alloc = sym swift_task_alloc,
        resume = sym dock_accessory_async_progress_resume,
    );
}

#[unsafe(naked)]
unsafe extern "C" fn dock_accessory_async_progress_resume() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        swift_async_load_parent!(),
        "str x9, [sp]",
        "mov x1, x0",
        "ldr x0, [x9, #24]",
        "mov x2, x20",
        "bl {set_result}",
        "mov x0, x22",
        "bl {task_dealloc}",
        "ldr x22, [sp]",
        swift_async_function_pointer!("{finish}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {task_switch}",
        set_result = sym cidre_dk_async_progress_set_result,
        task_dealloc = sym swift_task_dealloc,
        finish = sym dock_accessory_async_progress_finish,
        task_switch = sym swift_task_switch,
    );
}

#[unsafe(naked)]
unsafe extern "C" fn dock_accessory_async_progress_finish() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {complete}",
        "ldr x22, [sp, #8]",
        "ldr x9, [x22, #16]",
        swift_async_load_resume!(),
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        complete = sym cidre_dk_async_progress_complete,
    );
}

extern "C" fn cidre_dk_async_progress_prepare(task: *mut AsyncProgressTask, context: *mut u64) {
    unsafe {
        let task = &mut *task;
        context
            .add(4)
            .write(task.accessory.as_ptr().cast::<()>() as u64);
        context.add(5).write(task.function as u64);
        context.add(6).write(task.async_fn as u64);
        match &task.arg {
            AsyncProgressArg::Animation(value) => {
                context.add(8).write((value as *const Animation) as u64);
                context.add(15).write(0);
            }
            AsyncProgressArg::Vector {
                rotation,
                duration,
                relative,
            } => {
                context.add(8).write(rotation.x.to_bits());
                context.add(9).write(rotation.y.to_bits());
                context.add(10).write(rotation.z.to_bits());
                context.add(12).write(duration.0);
                context.add(13).write(duration.1);
                context.add(14).write(*relative as u64);
                context.add(15).write(1);
            }
            AsyncProgressArg::Rotation {
                rotation,
                duration,
                relative,
            } => {
                context.add(8).write(rotation.x.to_bits());
                context.add(9).write(rotation.y.to_bits());
                context.add(10).write(rotation.z.to_bits());
                context.add(11).write(rotation.w.to_bits());
                context.add(12).write(duration.0);
                context.add(13).write(duration.1);
                context.add(14).write(*relative as u64);
                context.add(15).write(2);
            }
        }
    }
}

extern "C" fn cidre_dk_async_progress_set_result(
    task: *mut AsyncProgressTask,
    result: *mut (),
    error: *mut (),
) {
    unsafe {
        (*task).result = result;
        (*task).error = error;
    }
}

extern "C" fn cidre_dk_async_progress_complete(task: *mut AsyncProgressTask) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        let mut task = Box::from_raw(task);
        let callback = task.callback.take().expect("DockKit callback missing");
        if task.error.is_null() {
            callback(Ok(arc::R::from_raw(task.result.cast())));
        } else {
            callback(Err(arc::R::from_raw(
                abi::error_as_ns_error(task.error).cast(),
            )));
        }
    }));
}

#[cfg(test)]
mod abi_tests {
    use super::*;

    #[test]
    fn observation_round_trips_rust_primitives() {
        let rect = cg::Rect {
            origin: cg::Point { x: 0.1, y: 0.2 },
            size: cg::Size {
                width: 0.3,
                height: 0.4,
            },
        };
        let observation = Observation::new(42, ObservationType::human_face(), rect);
        assert_eq!(42, observation.identifier());
        assert_eq!(ObservationType::human_face(), observation.ty());
        assert_eq!(rect, observation.rect());

        let observations = Observations::from_slice(&[observation]);
        assert_eq!(1, observations.len());
    }

    #[test]
    fn limits_round_trip_through_swift_initializers() {
        let limit = Limit::new(-1.0..1.5, 2.0).expect("valid DockKit limit");
        assert_eq!(-1.0..1.5, limit.position_range());
        assert_eq!(2.0, limit.maximum_speed());

        let limits = Limits::new(Some(&limit), None, Some(&limit));
        assert!(limits.yaw().is_some());
        assert!(limits.pitch().is_none());
        assert!(limits.roll().is_some());
    }

    #[test]
    fn accessory_event_tags_use_enum_value_witnesses() {
        unsafe fn event(tag: u32) -> Value<AccessoryEventValue> {
            unsafe {
                let mut storage = Storage::<AccessoryEventValue>::new();
                abi::destructive_inject_enum_tag(
                    storage.as_mut_ptr(),
                    tag,
                    AccessoryEventValue::metadata(),
                );
                storage.assume_init()
            }
        }

        unsafe {
            let shutter = event(DOCK_ACCESSORY_EVENT_CAMERA_SHUTTER_TAG);
            assert_eq!(
                AccessoryEvent::CameraShutter,
                AccessoryEvent::copy_from_ptr(shutter.as_ptr())
            );

            let flip = event(DOCK_ACCESSORY_EVENT_CAMERA_FLIP_TAG);
            assert_eq!(
                AccessoryEvent::CameraFlip,
                AccessoryEvent::copy_from_ptr(flip.as_ptr())
            );
        }
    }

    #[cfg(feature = "av")]
    #[test]
    fn camera_information_constructs_without_optional_calibration() {
        let device_type = crate::av::CaptureDeviceType::built_in_wide_angle_camera();
        let info = CameraInformation::new(
            device_type,
            crate::av::CaptureDevicePos::Front,
            CameraOrientation::portrait(),
        );
        assert_eq!(device_type, info.capture_device().as_ref());
        assert_eq!(crate::av::CaptureDevicePos::Front, info.camera_position());
        assert_eq!(CameraOrientation::portrait(), info.orientation());
        assert_eq!(None, info.camera_intrinsics());
        assert_eq!(None, info.reference_dimensions());

        let intrinsics = CameraIntrinsics::from_columns([
            [1200.0, 0.0, 0.0],
            [0.0, 1200.0, 0.0],
            [640.0, 360.0, 1.0],
        ]);
        let dimensions = cg::Size {
            width: 1280.0,
            height: 720.0,
        };
        let calibrated = CameraInformation::with_calibration(
            device_type,
            crate::av::CaptureDevicePos::Back,
            CameraOrientation::landscape_right(),
            Some(intrinsics),
            Some(dimensions),
        );
        assert_eq!(Some(intrinsics), calibrated.camera_intrinsics());
        assert_eq!(Some(dimensions), calibrated.reference_dimensions());

        let _ = MetadataObjects::from_slice(&[]);
    }
}
