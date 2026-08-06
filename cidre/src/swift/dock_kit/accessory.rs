use std::{hash::Hash, mem::size_of, ptr::NonNull};

use crate::{
    arc, cg, define_swift_tag_enum, ns, spatial, swift,
    swift::{
        FromSwift, SwiftMetadata, SwiftOptional, SwiftType, abi,
        concurrency::{self, define_async_sequence},
        foundation::{Date, Uuid},
        value::{Optional, Storage},
    },
};

crate::define_swift!(
    #[swift::class("DockKit.DockAccessory")]
    pub Accessory
);

pub struct StateChange {
    pub accessory: Option<arc::R<Accessory>>,
    pub state: State,
    pub tracking_button_enabled: bool,
}

impl StateChange {
    /// Reads the three stored properties out of a borrowed Swift
    /// `DockAccessory.StateChange`. The caller still owns the value.
    unsafe fn copy_from_ptr(value: *const ()) -> Self {
        unsafe {
            let mut state_storage = Storage::<StateValue>::new();
            abi::call::value_to_value(
                dock_accessory_state_change_state as *const (),
                value,
                state_storage.as_mut_ptr(),
            );
            let state = State(*(state_storage.as_ptr().cast::<u8>()));
            state_storage.destroy();

            let tracking_button_enabled = abi::call::value_to_bool(
                dock_accessory_state_change_tracking_button_enabled as *const (),
                value,
            );
            let accessory = NonNull::new(abi::call::value_to_object(
                dock_accessory_state_change_accessory as *const (),
                value,
            ))
            .map(|accessory| arc::R::from_raw(accessory.as_ptr().cast()));

            Self {
                accessory,
                state,
                tracking_button_enabled,
            }
        }
    }
}

impl core::fmt::Debug for StateChange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StateChange")
            .field("state", &self.state)
            .field("tracking_button_enabled", &self.tracking_button_enabled)
            .field("accessory", &self.accessory.as_deref())
            .finish()
    }
}

crate::define_swift!(
    #[swift::struct("DockKit.DockAccessory(class).Identifier", size(32), align(8), sendable)]
    pub Identifier
);

crate::define_swift!(
    #[swift::struct("DockKit.DockAccessory(class).MotionState", size(80), align(16), sendable)]
    /// One sample from `DockAccessory.motionStates`.
    pub MotionState
);

crate::define_swift!(
    #[swift::struct("DockKit.DockAccessory(class).BatteryState", size(32), align(8), sendable)]
    /// One sample from `DockAccessory.batteryStates`.
    pub BatteryState
);

crate::define_swift!(
    #[swift::struct("DockKit.DockAccessory(class).Limits", size(96), align(8), trivial, sendable)]
    /// The accessory's mechanical movement limits.
    pub Limits
);

crate::define_swift!(
    #[swift::struct("DockKit.DockAccessory(class).Limits(struct).Limit", size(24), align(8), trivial, sendable)]
    /// Limits for one rotational axis.
    pub Limit
);

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

crate::define_swift!(
    #[swift::struct("DockKit.DockAccessory(class).TrackedPerson", size(96), align(8), trivial, sendable)]
    /// A person currently tracked by DockKit.
    pub TrackedPerson
);

crate::define_swift!(
    #[swift::struct("DockKit.DockAccessory(class).TrackedObject", size(64), align(8), trivial, sendable)]
    /// An object currently tracked by DockKit.
    pub TrackedObject
);

/// A tracked subject and its concrete payload.
pub enum TrackedSubject {
    Person(TrackedPerson),
    Object(TrackedObject),
    Unknown(u32),
}

crate::define_swift!(
    #[swift::struct("DockKit.DockAccessory(class).TrackingState", size(16), align(8), sendable)]
    /// One sample from `DockAccessory.trackingStates`.
    pub TrackingState
);

crate::define_swift!(
    #[swift::struct("DockKit.DockAccessory(class).Observation", size(64), align(8), sendable)]
    /// One subject observation supplied to DockKit tracking.
    pub Observation
);

#[cfg(feature = "av")]
crate::define_swift!(
    #[swift::struct("DockKit.DockAccessory(class).CameraInformation", size(112), align(16), sendable)]
    /// Camera calibration supplied with tracking observations.
    #[cfg(feature = "av")]
    pub CameraInformation
);

#[cfg(feature = "av")]
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

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {

    #[link_name = "$s7DockKit0A9AccessoryC11StateChangeV5stateAC0D0Ovg"]
    fn dock_accessory_state_change_state();

    #[link_name = "$s7DockKit0A9AccessoryC11StateChangeV21trackingButtonEnabledSbvg"]
    fn dock_accessory_state_change_tracking_button_enabled();

    #[link_name = "$s7DockKit0A9AccessoryC11StateChangeV9accessoryACSgvg"]
    fn dock_accessory_state_change_accessory();

    #[link_name = "$s7DockKit0A9AccessoryC10IdentifierV8categoryAC8CategoryOvg"]
    fn dock_accessory_identifier_category();

    #[link_name = "$s7DockKit0A9AccessoryC11MotionStateV5errors5Error_pSgvg"]
    fn dock_accessory_motion_state_error();

    #[link_name = "$s7DockKit0A9AccessoryC12BatteryStateV06chargeE0AC0d6ChargeE0Ovg"]
    fn dock_accessory_battery_state_charge_state();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsV3yawAE5LimitVSgvg"]
    fn dock_accessory_limits_yaw();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsV5pitchAE5LimitVSgvg"]
    fn dock_accessory_limits_pitch();

    #[link_name = "$s7DockKit0A9AccessoryC6LimitsV4rollAE5LimitVSgvg"]
    fn dock_accessory_limits_roll();

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

    #[link_name = "$s7DockKit0A9AccessoryC0C5EventO6buttonyAESi_SbtcAEmFWC"]
    static DOCK_ACCESSORY_EVENT_BUTTON_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC0C5EventO13cameraShutteryA2EmFWC"]
    static DOCK_ACCESSORY_EVENT_CAMERA_SHUTTER_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC0C5EventO10cameraFlipyA2EmFWC"]
    static DOCK_ACCESSORY_EVENT_CAMERA_FLIP_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC0C5EventO10cameraZoomyAESd_tcAEmFWC"]
    static DOCK_ACCESSORY_EVENT_CAMERA_ZOOM_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonV10identifier10Foundation4UUIDVvg"]
    fn dock_accessory_tracked_person_identifier();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonV4rectSo6CGRectVvg"]
    fn dock_accessory_tracked_person_rect();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonV12saliencyRankSiSgvg"]
    fn dock_accessory_tracked_person_saliency_rank();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonV18speakingConfidenceSdSgvg"]
    fn dock_accessory_tracked_person_speaking_confidence();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedPersonV25lookingAtCameraConfidenceSdSgvg"]
    fn dock_accessory_tracked_person_looking_at_camera_confidence();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedObjectV10identifier10Foundation4UUIDVvg"]
    fn dock_accessory_tracked_object_identifier();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedObjectV4rectSo6CGRectVvg"]
    fn dock_accessory_tracked_object_rect();

    #[link_name = "$s7DockKit0A9AccessoryC13TrackedObjectV12saliencyRankSiSgvg"]
    fn dock_accessory_tracked_object_saliency_rank();

    #[link_name = "$s7DockKit0A9AccessoryC18TrackedSubjectTypeO6personyAeC0D6PersonVcAEmFWC"]
    static DOCK_ACCESSORY_TRACKED_SUBJECT_PERSON_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC18TrackedSubjectTypeO6objectyAeC0D6ObjectVcAEmFWC"]
    static DOCK_ACCESSORY_TRACKED_SUBJECT_OBJECT_TAG: u32;

    #[link_name = "$s7DockKit0A9AccessoryC13TrackingStateV15trackedSubjectsSayAC18TrackedSubjectTypeOGvg"]
    fn dock_accessory_tracking_state_subjects();

    #[link_name = "$s7DockKit0A9AccessoryC11ObservationV10identifier4type4rect12faceYawAngleAESi_AE0D4TypeOSo6CGRectV10Foundation11MeasurementVySo06NSUnitJ0CGSgtcfC"]
    fn dock_accessory_observation_init();

    #[link_name = "$s7DockKit0A9AccessoryC11ObservationV4typeAE0D4TypeOvg"]
    fn dock_accessory_observation_type();

    #[link_name = "$s7DockKit0A9AccessoryC11ObservationV4rectSo6CGRectVvg"]
    fn dock_accessory_observation_rect();

    #[cfg(feature = "av")]
    #[cfg(feature = "av")]
    #[link_name = "$s7DockKit0A9AccessoryC17CameraInformationV13captureDevice14cameraPosition11orientation0H10Intrinsics19referenceDimensionsAESo09AVCaptureG4Typea_So0ngI0VAC0D11OrientationOSo13simd_float3x3aSgSo6CGSizeVSgtcfC"]
    fn dock_accessory_camera_information_init();
}

crate::define_swift!(#[swift::struct("DockKit.DockAccessory(class).StateChange")] pub(crate) StateChangeValue);

unsafe impl SwiftMetadata for StateChange {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        StateChangeValue::metadata()
    }
}

unsafe impl crate::swift::FromSwift for StateChange {
    #[inline]
    unsafe fn copy_swift(value: *const ()) -> Self {
        unsafe { Self::copy_from_ptr(value) }
    }
}

crate::define_swift_marker!(pub(crate) StateValue = mangled "7DockKit0A9AccessoryC5StateO");

crate::define_swift!(#[swift::enum("DockKit.DockAccessory(class).AccessoryEvent")] pub(crate) AccessoryEventValue);

unsafe impl SwiftMetadata for AccessoryEvent {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        AccessoryEventValue::metadata()
    }
}

unsafe impl crate::swift::FromSwift for AccessoryEvent {
    #[inline]
    unsafe fn copy_swift(value: *const ()) -> Self {
        unsafe { Self::copy_from_ptr(value) }
    }
}
crate::define_swift!(#[swift::enum("DockKit.DockAccessory(class).TrackedSubjectType")] pub(crate) TrackedSubjectValue);
crate::define_swift_marker!(pub(crate) MeasurementAngleValue = mangled "10Foundation11MeasurementVySo11NSUnitAngleCG");

#[cfg(feature = "av")]
#[cfg(feature = "av")]
crate::define_swift_marker!(pub(crate) CameraIntrinsicsValue = mangled "So13simd_float3x3a");
#[cfg(feature = "av")]
crate::define_swift_marker!(pub(crate) ReferenceDimensionsValue = mangled "So6CGSizeV");

impl Identifier {
    pub fn category(&self) -> Category {
        let mut value = Category::tracking_stand();
        unsafe {
            abi::call::value_to_value(
                dock_accessory_identifier_category as *const (),
                self.as_ptr(),
                (&mut value as *mut Category).cast(),
            );
        }
        value
    }

    #[swift::call("DockKit.DockAccessory(class).Identifier(struct).name: String { get }")]
    pub fn name(&self) -> swift::String;

    #[swift::call(
        "DockKit.DockAccessory(class).Identifier(struct).uuid: Foundation.UUID(struct) { get }"
    )]
    pub fn uuid(&self) -> Uuid;

    #[swift::call(
        "DockKit.DockAccessory(class).Identifier(struct).debugDescription: String { get }"
    )]
    pub fn debug_desc(&self) -> swift::String;

    #[swift::call("DockKit.DockAccessory(class).Identifier(struct).hashValue: Int { get }")]
    pub fn hash_value(&self) -> isize;
}

impl Identifier {
    /// Swift's `==` is a static member taking both operands as arguments
    /// rather than one of them as `self`.
    #[swift::call(sym = "$s7DockKit0A9AccessoryC10IdentifierV2eeoiySbAE_AEtFZ")]
    fn swift_eq(lhs: &Self, rhs: &Self) -> bool;
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        Self::swift_eq(self, other)
    }
}

impl Eq for Identifier {}

impl std::hash::Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.hash_value(), state)
    }
}

impl MotionState {
    /// An `SPVector3D` comes back in `d0`-`d2` rather than through an
    /// indirect result.
    #[swift::call(
        "DockKit.DockAccessory(class).MotionState(struct).angularVelocities: __C.SPVector3D { get }"
    )]
    pub fn angular_velocities(&self) -> spatial::Vector3D;

    #[swift::call(
        "DockKit.DockAccessory(class).MotionState(struct).angularPositions: __C.SPVector3D { get }"
    )]
    pub fn angular_positions(&self) -> spatial::Vector3D;

    #[swift::call("DockKit.DockAccessory(class).MotionState(struct).timestamp: Double { get }")]
    pub fn timestamp(&self) -> f64;

    pub fn error(&self) -> Option<arc::R<ns::Error>> {
        unsafe {
            NonNull::new(abi::call::value_to_object(
                dock_accessory_motion_state_error as *const (),
                self.as_ptr(),
            ))
            .map(|error| arc::R::from_raw(abi::error_as_ns_error(error.as_ptr()).cast()))
        }
    }
}

impl BatteryState {
    #[swift::call("DockKit.DockAccessory(class).BatteryState(struct).name: String { get }")]
    pub fn name(&self) -> swift::String;

    #[swift::call("DockKit.DockAccessory(class).BatteryState(struct).batteryLevel: Double { get }")]
    pub fn battery_level(&self) -> f64;

    #[swift::call("DockKit.DockAccessory(class).BatteryState(struct).lowBattery: Bool { get }")]
    pub fn is_low_battery(&self) -> bool;

    pub fn charge_state(&self) -> BatteryChargeState {
        let mut value = BatteryChargeState::not_charging();
        unsafe {
            abi::call::value_to_value(
                dock_accessory_battery_state_charge_state as *const (),
                self.as_ptr(),
                (&mut value as *mut BatteryChargeState).cast(),
            );
        }
        value
    }

    #[swift::call("DockKit.DockAccessory(class).BatteryState(struct).hashValue: Int { get }")]
    pub fn hash_value(&self) -> isize;
}

impl BatteryState {
    /// Swift's `==` is a static member taking both operands as arguments
    /// rather than one of them as `self`.
    #[swift::call(sym = "$s7DockKit0A9AccessoryC12BatteryStateV2eeoiySbAE_AEtFZ")]
    fn swift_eq(lhs: &Self, rhs: &Self) -> bool;
}

impl PartialEq for BatteryState {
    fn eq(&self, other: &Self) -> bool {
        Self::swift_eq(self, other)
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
            fn optional(value: Option<&Limit>) -> Storage<Optional<Limit>> {
                match value {
                    None => Storage::none(),
                    Some(value) => unsafe {
                        let mut storage = Storage::<Optional<Limit>>::new();
                        abi::initialize_with_copy(
                            storage.as_mut_ptr().cast(),
                            value.as_ptr(),
                            <Limit as SwiftMetadata>::metadata(),
                        );
                        abi::store_enum_tag_single_payload(
                            storage.as_mut_ptr().cast(),
                            0,
                            1,
                            <Limit as SwiftMetadata>::metadata(),
                        );
                        storage
                    },
                }
            }

            let mut yaw = optional(yaw);
            let mut pitch = optional(pitch);
            let mut roll = optional(roll);
            let mut storage = core::mem::MaybeUninit::<Limits>::uninit();
            abi::call::values3_to_value(
                dock_accessory_limits_init as *const (),
                yaw.as_mut_ptr(),
                pitch.as_mut_ptr(),
                roll.as_mut_ptr(),
                storage.as_mut_ptr().cast(),
            );
            storage.assume_init()
        }
    }

    unsafe fn optional_limit(&self, getter: *const ()) -> Option<Limit> {
        unsafe {
            let mut storage = Storage::<Optional<Limit>>::new();
            abi::call::value_to_value(getter, self.as_ptr(), storage.as_mut_ptr().cast());
            storage.take()
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
            let mut storage = core::mem::MaybeUninit::<Limit>::uninit();
            let error = abi::call::doubles3_to_throwing_value(
                dock_accessory_limit_init as *const (),
                (position_range.start, position_range.end, maximum_speed),
                storage.as_mut_ptr().cast(),
            );
            if error.is_null() {
                Ok(storage.assume_init())
            } else {
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    pub fn position_range(&self) -> std::ops::Range<f64> {
        let (start, end) = unsafe {
            abi::call::value_to_doubles2(
                dock_accessory_limit_position_range as *const (),
                self.as_ptr(),
            )
        };
        start..end
    }

    #[swift::call(
        "DockKit.DockAccessory(class).Limits(struct).Limit(struct).maximumSpeed: Double { get }"
    )]
    pub fn maximum_speed(&self) -> f64;
}

impl AccessoryEvent {
    unsafe fn copy_from_ptr(value: *const ()) -> Self {
        unsafe {
            let mut storage = Storage::<AccessoryEventValue>::new();
            abi::initialize_with_copy(
                storage.as_mut_ptr().cast(),
                value,
                AccessoryEventValue::metadata(),
            );
            let tag = abi::get_enum_tag(storage.as_ptr(), AccessoryEventValue::metadata());

            // Projecting a case is destructive, so the payload is read out and
            // nothing is left to destroy. A case that is not projected still
            // holds the whole value, which is destroyed through its witness.
            if tag == DOCK_ACCESSORY_EVENT_BUTTON_TAG {
                abi::destructive_project_enum_data(
                    storage.as_mut_ptr(),
                    AccessoryEventValue::metadata(),
                );
                let id = storage.as_ptr().cast::<isize>().read();
                let pressed = storage.as_ptr().cast::<u8>().add(size_of::<isize>()).read() != 0;
                Self::Button { id, pressed }
            } else if tag == DOCK_ACCESSORY_EVENT_CAMERA_ZOOM_TAG {
                abi::destructive_project_enum_data(
                    storage.as_mut_ptr(),
                    AccessoryEventValue::metadata(),
                );
                let factor = storage.as_ptr().cast::<f64>().read();
                Self::CameraZoom { factor }
            } else if tag == DOCK_ACCESSORY_EVENT_CAMERA_SHUTTER_TAG {
                storage.destroy();
                Self::CameraShutter
            } else if tag == DOCK_ACCESSORY_EVENT_CAMERA_FLIP_TAG {
                storage.destroy();
                Self::CameraFlip
            } else {
                storage.destroy();
                Self::Unknown(tag)
            }
        }
    }
}

unsafe fn optional_primitive<T: SwiftType + SwiftOptional + FromSwift + Copy>(
    owner: *const (),
    getter: *const (),
) -> Option<T> {
    unsafe {
        let mut storage = Storage::<Optional<T>>::new();
        abi::call::value_to_value(getter, owner, storage.as_mut_ptr().cast());
        storage.take()
    }
}

unsafe fn uuid_property(owner: *const (), getter: *const ()) -> Uuid {
    unsafe {
        let mut storage = core::mem::MaybeUninit::<Uuid>::uninit();
        abi::call::value_to_value(getter, owner, storage.as_mut_ptr().cast());
        storage.assume_init()
    }
}

unsafe fn rect_property(owner: *const (), getter: *const ()) -> cg::Rect {
    let (x, y, width, height) = unsafe { abi::call::value_to_rect(getter, owner) };
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

impl TrackedPerson {
    /// Swift's `==` is a static member taking both operands as arguments
    /// rather than one of them as `self`.
    #[swift::call(sym = "$s7DockKit0A9AccessoryC13TrackedPersonV2eeoiySbAE_AEtFZ")]
    fn swift_eq(lhs: &Self, rhs: &Self) -> bool;
}

impl PartialEq for TrackedPerson {
    fn eq(&self, other: &Self) -> bool {
        Self::swift_eq(self, other)
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

impl TrackedObject {
    /// Swift's `==` is a static member taking both operands as arguments
    /// rather than one of them as `self`.
    #[swift::call(sym = "$s7DockKit0A9AccessoryC13TrackedObjectV2eeoiySbAE_AEtFZ")]
    fn swift_eq(lhs: &Self, rhs: &Self) -> bool;
}

impl PartialEq for TrackedObject {
    fn eq(&self, other: &Self) -> bool {
        Self::swift_eq(self, other)
    }
}

unsafe impl SwiftMetadata for TrackedSubject {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        TrackedSubjectValue::metadata()
    }
}

unsafe impl FromSwift for TrackedSubject {
    unsafe fn copy_swift(value: *const ()) -> Self {
        unsafe {
            let mut storage = Storage::<TrackedSubjectValue>::new();
            abi::initialize_with_copy(
                storage.as_mut_ptr().cast(),
                value,
                TrackedSubjectValue::metadata(),
            );
            let tag = abi::get_enum_tag(storage.as_ptr(), TrackedSubjectValue::metadata());

            if tag == DOCK_ACCESSORY_TRACKED_SUBJECT_PERSON_TAG {
                abi::destructive_project_enum_data(
                    storage.as_mut_ptr(),
                    TrackedSubjectValue::metadata(),
                );
                let person = TrackedPerson::copy_swift(storage.as_ptr());
                abi::destroy_value(
                    storage.as_mut_ptr(),
                    <TrackedPerson as SwiftMetadata>::metadata(),
                );
                Self::Person(person)
            } else if tag == DOCK_ACCESSORY_TRACKED_SUBJECT_OBJECT_TAG {
                abi::destructive_project_enum_data(
                    storage.as_mut_ptr(),
                    TrackedSubjectValue::metadata(),
                );
                let object = TrackedObject::copy_swift(storage.as_ptr());
                abi::destroy_value(
                    storage.as_mut_ptr(),
                    <TrackedObject as SwiftMetadata>::metadata(),
                );
                Self::Object(object)
            } else {
                storage.destroy();
                Self::Unknown(tag)
            }
        }
    }
}

impl TrackingState {
    #[swift::call(
        "DockKit.DockAccessory(class).TrackingState(struct).time: Foundation.Date(struct) { get }"
    )]
    pub fn time(&self) -> Date;

    pub fn tracked_subjects(&self) -> swift::Array<TrackedSubject> {
        unsafe {
            swift::Array::from_raw(abi::call::value_to_object(
                dock_accessory_tracking_state_subjects as *const (),
                self.as_ptr(),
            ))
        }
    }

    #[swift::call("DockKit.DockAccessory(class).TrackingState(struct).description: String { get }")]
    pub fn description(&self) -> swift::String;
}

impl Observation {
    /// Creates an observation without a face-yaw measurement.
    pub fn new(identifier: isize, ty: ObservationType, rect: cg::Rect) -> Self {
        unsafe {
            let face_yaw = Storage::<Optional<MeasurementAngleValue>>::none();
            let mut storage = core::mem::MaybeUninit::<Observation>::uninit();
            abi::call::int_value_rect_value_to_value(
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
                storage.as_mut_ptr().cast(),
            );
            storage.assume_init()
        }
    }

    #[swift::call("DockKit.DockAccessory(class).Observation(struct).identifier: Int { get }")]
    pub fn identifier(&self) -> isize;

    pub fn ty(&self) -> ObservationType {
        let mut value = ObservationType::human_face();
        unsafe {
            abi::call::value_to_value(
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
                None => Storage::<Optional<CameraIntrinsicsValue>>::none(),
                Some(value) => {
                    let mut storage = Storage::<Optional<CameraIntrinsicsValue>>::new();
                    abi::initialize_with_copy(
                        storage.as_mut_ptr().cast(),
                        (&raw const value).cast(),
                        CameraIntrinsicsValue::metadata(),
                    );
                    abi::store_enum_tag_single_payload(
                        storage.as_mut_ptr().cast(),
                        0,
                        1,
                        CameraIntrinsicsValue::metadata(),
                    );
                    storage
                }
            };
            let dimensions = match reference_dimensions {
                None => Storage::<Optional<ReferenceDimensionsValue>>::none(),
                Some(value) => {
                    let mut storage = Storage::<Optional<ReferenceDimensionsValue>>::new();
                    abi::initialize_with_copy(
                        storage.as_mut_ptr().cast(),
                        (&raw const value).cast(),
                        ReferenceDimensionsValue::metadata(),
                    );
                    abi::store_enum_tag_single_payload(
                        storage.as_mut_ptr().cast(),
                        0,
                        1,
                        ReferenceDimensionsValue::metadata(),
                    );
                    storage
                }
            };
            let words = dimensions.as_ptr().cast::<u64>();
            let mut storage = core::mem::MaybeUninit::<CameraInformation>::uninit();
            abi::call::camera_information_init(
                dock_accessory_camera_information_init as *const (),
                (device_type as *const crate::av::CaptureDeviceType).cast(),
                position as isize,
                orientation.as_abi_ptr(),
                intrinsics.as_ptr(),
                (words.read(), words.add(1).read(), words.add(2).read()),
                storage.as_mut_ptr().cast(),
            );
            storage.assume_init()
        }
    }

    pub fn capture_device(&self) -> arc::R<crate::av::CaptureDeviceType> {
        unsafe {
            arc::R::from_raw(
                abi::call::value_to_object(
                    dock_accessory_camera_information_capture_device as *const (),
                    self.as_ptr(),
                )
                .cast(),
            )
        }
    }

    pub fn camera_position(&self) -> crate::av::CaptureDevicePos {
        unsafe {
            std::mem::transmute(abi::call::value_to_int(
                dock_accessory_camera_information_camera_position as *const (),
                self.as_ptr(),
            ))
        }
    }

    pub fn orientation(&self) -> CameraOrientation {
        let mut value = CameraOrientation::portrait();
        unsafe {
            abi::call::value_to_value(
                dock_accessory_camera_information_orientation as *const (),
                self.as_ptr(),
                (&mut value as *mut CameraOrientation).cast(),
            );
        }
        value
    }

    pub fn camera_intrinsics(&self) -> Option<CameraIntrinsics> {
        unsafe {
            let mut storage = Storage::<Optional<CameraIntrinsicsValue>>::new();
            abi::call::value_to_value(
                dock_accessory_camera_information_intrinsics as *const (),
                self.as_ptr(),
                storage.as_mut_ptr().cast(),
            );
            storage
                .is_some()
                .then(|| storage.as_ptr().cast::<CameraIntrinsics>().read())
        }
    }

    pub fn reference_dimensions(&self) -> Option<cg::Size> {
        unsafe {
            let words = abi::call::value_to_words3(
                dock_accessory_camera_information_reference_dimensions as *const (),
                self.as_ptr(),
            );
            let mut storage = Storage::<Optional<ReferenceDimensionsValue>>::new();
            let ptr = storage.as_mut_ptr().cast::<u64>();
            ptr.write(words.0);
            ptr.add(1).write(words.1);
            ptr.add(2).write(words.2);
            storage
                .is_some()
                .then(|| storage.as_ptr().cast::<cg::Size>().read())
        }
    }
}

define_async_sequence! {
    /// `DockAccessory.StateChanges`.
    StateChanges, StateChangesValue, StateChangesIteratorValue,
    framework = "DockKit",
    element = StateChange,
    sequence_metadata = dock_accessory_state_changes_metadata => "$s7DockKit0A9AccessoryC12StateChangesVMa",
    iterator_metadata = dock_accessory_state_changes_iterator_metadata => "$s7DockKit0A9AccessoryC12StateChangesV8IteratorVMa",
    make_iterator = dock_accessory_state_changes_make_iterator => "$s7DockKit0A9AccessoryC12StateChangesV17makeAsyncIteratorAE0H0VyF",
    next = dock_accessory_state_changes_next => "$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaF",
    next_async = DOCK_ACCESSORY_STATE_CHANGES_NEXT_ASYNC => "$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaFTu",
    async_iter = StateChangesAsyncIter,
}

define_async_sequence! {
    /// `DockAccessory.MotionStates`.
    MotionStates, MotionStatesValue, MotionStatesIteratorValue,
    framework = "DockKit",
    element = MotionState,
    sequence_metadata = dock_accessory_motion_states_metadata => "$s7DockKit0A9AccessoryC12MotionStatesVMa",
    iterator_metadata = dock_accessory_motion_states_iterator_metadata => "$s7DockKit0A9AccessoryC12MotionStatesV8IteratorVMa",
    make_iterator = dock_accessory_motion_states_make_iterator => "$s7DockKit0A9AccessoryC12MotionStatesV17makeAsyncIteratorAE0H0VyF",
    next = dock_accessory_motion_states_next => "$s7DockKit0A9AccessoryC12MotionStatesV8IteratorV4nextAC0D5StateVSgyYaF",
    next_async = DOCK_ACCESSORY_MOTION_STATES_NEXT_ASYNC => "$s7DockKit0A9AccessoryC12MotionStatesV8IteratorV4nextAC0D5StateVSgyYaFTu",
    async_iter = MotionStatesAsyncIter,
}

define_async_sequence! {
    /// `DockAccessory.AccessoryEvents`.
    AccessoryEvents, AccessoryEventsValue, AccessoryEventsIteratorValue,
    framework = "DockKit",
    element = AccessoryEvent,
    sequence_metadata = dock_accessory_events_metadata => "$s7DockKit0A9AccessoryC0C6EventsVMa",
    iterator_metadata = dock_accessory_events_iterator_metadata => "$s7DockKit0A9AccessoryC0C6EventsV8IteratorVMa",
    make_iterator = dock_accessory_events_make_iterator => "$s7DockKit0A9AccessoryC0C6EventsV17makeAsyncIteratorAE0G0VyF",
    next = dock_accessory_events_next => "$s7DockKit0A9AccessoryC0C6EventsV8IteratorV4nextAC0C5EventOSgyYaF",
    next_async = DOCK_ACCESSORY_EVENTS_NEXT_ASYNC => "$s7DockKit0A9AccessoryC0C6EventsV8IteratorV4nextAC0C5EventOSgyYaFTu",
    async_iter = AccessoryEventsAsyncIter,
}

define_async_sequence! {
    /// `DockAccessory.TrackingStates`.
    TrackingStates, TrackingStatesValue, TrackingStatesIteratorValue,
    framework = "DockKit",
    element = TrackingState,
    sequence_metadata = dock_accessory_tracking_states_metadata => "$s7DockKit0A9AccessoryC14TrackingStatesVMa",
    iterator_metadata = dock_accessory_tracking_states_iterator_metadata => "$s7DockKit0A9AccessoryC14TrackingStatesV8IteratorVMa",
    make_iterator = dock_accessory_tracking_states_make_iterator => "$s7DockKit0A9AccessoryC14TrackingStatesV17makeAsyncIteratorAE0H0VyF",
    next = dock_accessory_tracking_states_next => "$s7DockKit0A9AccessoryC14TrackingStatesV8IteratorV4nextAC0D5StateVSgyYaF",
    next_async = DOCK_ACCESSORY_TRACKING_STATES_NEXT_ASYNC => "$s7DockKit0A9AccessoryC14TrackingStatesV8IteratorV4nextAC0D5StateVSgyYaFTu",
    async_iter = TrackingStatesAsyncIter,
}

define_async_sequence! {
    /// `DockAccessory.BatteryStates`.
    BatteryStates, BatteryStatesValue, BatteryStatesIteratorValue,
    framework = "DockKit",
    element = BatteryState,
    sequence_metadata = dock_accessory_battery_states_metadata => "$s7DockKit0A9AccessoryC13BatteryStatesVMa",
    iterator_metadata = dock_accessory_battery_states_iterator_metadata => "$s7DockKit0A9AccessoryC13BatteryStatesV8IteratorVMa",
    make_iterator = dock_accessory_battery_states_make_iterator => "$s7DockKit0A9AccessoryC13BatteryStatesV17makeAsyncIteratorAE0H0VyF",
    next = dock_accessory_battery_states_next => "$s7DockKit0A9AccessoryC13BatteryStatesV8IteratorV4nextAC0D5StateVSgyYaF",
    next_async = DOCK_ACCESSORY_BATTERY_STATES_NEXT_ASYNC => "$s7DockKit0A9AccessoryC13BatteryStatesV8IteratorV4nextAC0D5StateVSgyYaFTu",
    async_iter = BatteryStatesAsyncIter,
}

define_swift_tag_enum!(
    /// DockKit `DockAccessory.State`.
    #[doc(alias = "DockAccessory.State")]
    pub State in "DockKit" {
        hash = "$s7DockKit0A9AccessoryC5StateO9hashValueSivg",
        debug = "$s7DockKit0A9AccessoryC5StateO16debugDescriptionSSvg",
        cases {
            undocked = "$s7DockKit0A9AccessoryC5StateO8undockedyA2EmFWC",
            docked = "$s7DockKit0A9AccessoryC5StateO6dockedyA2EmFWC",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.Category`.
    #[doc(alias = "DockAccessory.Category")]
    pub Category in "DockKit" {
        hash = "$s7DockKit0A9AccessoryC8CategoryO9hashValueSivg",
        debug = "$s7DockKit0A9AccessoryC8CategoryO16debugDescriptionSSvg",
        cases {
            tracking_stand = "$s7DockKit0A9AccessoryC8CategoryO13trackingStandyA2EmFWC",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.CameraOrientation`.
    #[doc(alias = "DockAccessory.CameraOrientation")]
    pub CameraOrientation in "DockKit" {
        hash = "$s7DockKit0A9AccessoryC17CameraOrientationO9hashValueSivg",
        cases {
            unknown = "$s7DockKit0A9AccessoryC17CameraOrientationO7unknownyA2EmFWC",
            portrait = "$s7DockKit0A9AccessoryC17CameraOrientationO8portraityA2EmFWC",
            portrait_upside_down = "$s7DockKit0A9AccessoryC17CameraOrientationO18portraitUpsideDownyA2EmFWC",
            landscape_right = "$s7DockKit0A9AccessoryC17CameraOrientationO14landscapeRightyA2EmFWC",
            landscape_left = "$s7DockKit0A9AccessoryC17CameraOrientationO13landscapeLeftyA2EmFWC",
            face_up = "$s7DockKit0A9AccessoryC17CameraOrientationO6faceUpyA2EmFWC",
            face_down = "$s7DockKit0A9AccessoryC17CameraOrientationO8faceDownyA2EmFWC",
            corrected = "$s7DockKit0A9AccessoryC17CameraOrientationO9correctedyA2EmFWC",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.Observation.ObservationType`.
    #[doc(alias = "DockAccessory.Observation.ObservationType")]
    pub ObservationType in "DockKit" {
        hash = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO9hashValueSivg",
        cases {
            human_face = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO9humanFaceyA2GmFWC",
            human_body = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO9humanBodyyA2GmFWC",
            object = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO6objectyA2GmFWC",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.BatteryChargeState`.
    #[doc(alias = "DockAccessory.BatteryChargeState")]
    pub BatteryChargeState in "DockKit" {
        hash = "$s7DockKit0A9AccessoryC18BatteryChargeStateO9hashValueSivg",
        cases {
            not_charging = "$s7DockKit0A9AccessoryC18BatteryChargeStateO11notChargingyA2EmFWC",
            charging = "$s7DockKit0A9AccessoryC18BatteryChargeStateO8chargingyA2EmFWC",
            not_chargeable = "$s7DockKit0A9AccessoryC18BatteryChargeStateO13notChargeableyA2EmFWC",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.FramingMode`.
    #[doc(alias = "DockAccessory.FramingMode")]
    pub FramingMode in "DockKit" {
        hash = "$s7DockKit0A9AccessoryC11FramingModeO9hashValueSivg",
        cases {
            automatic = "$s7DockKit0A9AccessoryC11FramingModeO9automaticyA2EmFWC",
            center = "$s7DockKit0A9AccessoryC11FramingModeO6centeryA2EmFWC",
            left = "$s7DockKit0A9AccessoryC11FramingModeO4leftyA2EmFWC",
            right = "$s7DockKit0A9AccessoryC11FramingModeO5rightyA2EmFWC",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.Animation`.
    #[doc(alias = "DockAccessory.Animation")]
    pub Animation in "DockKit" {
        hash = "$s7DockKit0A9AccessoryC9AnimationO9hashValueSivg",
        cases {
            wakeup = "$s7DockKit0A9AccessoryC9AnimationO6wakeupyA2EmFWC",
            yes = "$s7DockKit0A9AccessoryC9AnimationO3yesyA2EmFWC",
            no = "$s7DockKit0A9AccessoryC9AnimationO2noyA2EmFWC",
            kapow = "$s7DockKit0A9AccessoryC9AnimationO5kapowyA2EmFWC",
        }
    }
);

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {

    #[link_name = "$s7DockKit0A9AccessoryC11framingModeAC07FramingE0Ovg"]
    fn dock_accessory_framing_mode();

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
    #[swift::call("DockKit.DockAccessory(class).hashValue: Int { get }")]
    pub fn hash_value(&self) -> isize;

    #[swift::call(sym = "$s7DockKit0A9AccessoryC10identifierAC10IdentifierVvg")]
    #[doc(alias = "DockAccessory.identifier")]
    pub fn identifier(&self) -> Identifier;

    #[swift::call("DockKit.DockAccessory(class).debugDescription: String { get }")]
    pub fn debug_desc(&self) -> swift::String;

    #[doc(alias = "DockAccessory.framingMode")]
    #[inline]
    pub fn framing_mode(&self) -> FramingMode {
        let mut value = FramingMode::automatic();
        unsafe {
            abi::call::object_to_value(
                dock_accessory_framing_mode as *const (),
                (self as *const Self).cast(),
                (&mut value as *mut FramingMode).cast(),
            );
        }
        value
    }

    /// A `String?` getter hands back the string's own two words, and Swift
    /// spells the empty case as a null word pair.
    #[swift::call("DockKit.DockAccessory(class).firmwareVersion: String? { get }")]
    pub fn firmware_version(&self) -> Option<swift::String>;

    #[swift::call("DockKit.DockAccessory(class).hardwareModel: String? { get }")]
    pub fn hardware_model(&self) -> Option<swift::String>;

    /// A `CGRect` comes back in `d0`-`d3` rather than through an indirect
    /// result.
    #[swift::call("DockKit.DockAccessory(class).regionOfInterest: __C.CGRect(struct) { get }")]
    pub fn region_of_interest(&self) -> cg::Rect;

    #[swift::call(sym = "$s7DockKit0A9AccessoryC6limitsAC6LimitsVvg")]
    #[doc(alias = "DockAccessory.limits")]
    pub fn limits(&self) -> Result<Limits, arc::R<ns::Error>>;

    #[swift::call(sym = "$s7DockKit0A9AccessoryC12motionStatesAC06MotionE0Vvg")]
    #[doc(alias = "DockAccessory.motionStates")]
    pub fn motion_states(&self) -> Result<MotionStates, arc::R<ns::Error>>;

    #[crate::api::available(macos = 15.0, ios = 18.0)]
    #[swift::call(sym = "$s7DockKit0A9AccessoryC13batteryStatesAC07BatteryE0Vvg")]
    #[doc(alias = "DockAccessory.batteryStates")]
    pub fn battery_states(&self) -> Result<BatteryStates, arc::R<ns::Error>>;

    #[doc(alias = "DockAccessory.accessoryEvents")]
    #[crate::api::available(macos = 14.4, ios = 17.4)]
    pub fn accessory_events(&self) -> Result<AccessoryEvents, arc::R<ns::Error>> {
        unsafe {
            let mut storage = <AccessoryEvents as crate::swift::value::SwiftOut>::out_buf();
            let error = abi::call::object_to_throwing_value(
                dock_accessory_events as *const (),
                (self as *const Self).cast(),
                storage.as_mut_ptr().cast(),
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
            let mut storage = <TrackingStates as crate::swift::value::SwiftOut>::out_buf();
            let error = abi::call::object_to_throwing_value(
                dock_accessory_tracking_states as *const (),
                (self as *const Self).cast(),
                storage.as_mut_ptr().cast(),
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
            let error = abi::call::value_object_to_throwing_void(
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
            let duration = abi::call::double_to_words2(
                swift_duration_seconds as *const (),
                duration.as_secs_f64(),
            );
            let (result, error) = abi::call::vector_duration_bool_object(
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
            let duration = abi::call::double_to_words2(
                swift_duration_seconds as *const (),
                duration.as_secs_f64(),
            );
            let (result, error) = abi::call::rotation_duration_bool_object(
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

    /// Awaits one of the accessory's `Void`-returning methods.
    ///
    /// They differ only in which registers their arguments go in, so the call
    /// itself is `owned` — whatever has to stay alive — plus where it goes.
    fn call_void<O, F>(
        &self,
        function: *const (),
        async_fn: *const u8,
        owned: O,
        args: impl FnOnce(&mut O) -> concurrency::AsyncCallArgs,
        callback: F,
    ) where
        O: Send + 'static,
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            concurrency::call_async_result(
                function,
                async_fn,
                (arc::Retain::retained(self), owned),
                |(accessory, owned)| args(owned).swift_self(accessory.as_ptr().cast()),
                |_, _| (),
                callback,
            );
        }
    }

    /// The same for the ones that return an `ns::Progress`.
    fn call_progress<O, F>(
        &self,
        function: *const (),
        async_fn: *const u8,
        owned: O,
        args: impl FnOnce(&mut O) -> concurrency::AsyncCallArgs,
        callback: F,
    ) where
        O: Send + 'static,
        F: FnOnce(Result<arc::R<ns::Progress>, arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            concurrency::call_async_result(
                function,
                async_fn,
                (arc::Retain::retained(self), owned),
                |(accessory, owned)| args(owned).swift_self(accessory.as_ptr().cast()),
                |_, progress| arc::R::from_raw(progress.cast()),
                callback,
            );
        }
    }

    #[doc(alias = "DockAccessory.setAngularVelocity(_:)")]
    pub fn set_angular_velocity_handler<F>(&self, velocity: spatial::Vector3D, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        self.call_void(
            dock_accessory_set_angular_velocity as *const (),
            (&raw const DOCK_ACCESSORY_SET_ANGULAR_VELOCITY_ASYNC).cast(),
            (),
            |_| {
                concurrency::AsyncCallArgs::new()
                    .float(0, velocity.x)
                    .float(1, velocity.y)
                    .float(2, velocity.z)
            },
            callback,
        );
    }

    #[doc(alias = "DockAccessory.selectSubject(at:)")]
    pub fn select_subject_handler<F>(&self, point: cg::Point, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        self.call_void(
            dock_accessory_select_subject as *const (),
            (&raw const DOCK_ACCESSORY_SELECT_SUBJECT_ASYNC).cast(),
            (),
            |_| {
                concurrency::AsyncCallArgs::new()
                    .float(0, point.x)
                    .float(1, point.y)
            },
            callback,
        );
    }

    #[doc(alias = "DockAccessory.setFramingMode(_:)")]
    pub fn set_framing_mode_handler<F>(&self, mode: FramingMode, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        self.call_void(
            dock_accessory_set_framing_mode as *const (),
            (&raw const DOCK_ACCESSORY_SET_FRAMING_MODE_ASYNC).cast(),
            // A resilient enum is passed indirectly, so the call needs
            // somewhere stable to point at.
            mode,
            |mode| concurrency::AsyncCallArgs::new().arg(0, mode.as_abi_ptr().cast_mut()),
            callback,
        );
    }

    #[doc(alias = "DockAccessory.setRegionOfInterest(_:)")]
    pub fn set_region_of_interest_handler<F>(&self, rect: cg::Rect, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        self.call_void(
            dock_accessory_set_region_of_interest as *const (),
            (&raw const DOCK_ACCESSORY_SET_REGION_OF_INTEREST_ASYNC).cast(),
            (),
            |_| {
                concurrency::AsyncCallArgs::new()
                    .float(0, rect.origin.x)
                    .float(1, rect.origin.y)
                    .float(2, rect.size.width)
                    .float(3, rect.size.height)
            },
            callback,
        );
    }

    #[doc(alias = "DockAccessory.selectSubjects(_:)")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    pub fn select_subjects_handler<F>(&self, ids: &[Uuid], callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        self.select_subjects_array(swift::Array::from_slice(ids), callback);
    }

    fn select_subjects_array<F>(&self, ids: swift::Array<Uuid>, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        self.call_void(
            dock_accessory_select_subjects as *const (),
            (&raw const DOCK_ACCESSORY_SELECT_SUBJECTS_ASYNC).cast(),
            ids,
            |ids| concurrency::AsyncCallArgs::new().arg(0, ids.as_raw()),
            callback,
        );
    }

    /// The four `track` overloads differ only in what they hand over as the
    /// observations and whether they carry an image.
    #[cfg(feature = "av")]
    fn track_data<F>(
        &self,
        function: *const (),
        async_fn: *const u8,
        data: TrackData,
        camera: CameraInformation,
        image: Option<arc::R<crate::cv::PixelBuf>>,
        callback: F,
    ) where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        self.call_void(
            function,
            async_fn,
            TrackArgs {
                data,
                camera,
                image,
            },
            |track| {
                let args = concurrency::AsyncCallArgs::new()
                    .arg(0, track.data.as_raw())
                    .arg(1, track.camera.as_ptr().cast_mut());
                match &track.image {
                    Some(image) => args.arg(2, image.as_ptr().cast()),
                    None => args,
                }
            },
            callback,
        );
    }

    #[cfg(feature = "av")]
    #[doc(alias = "DockAccessory.track(_:cameraInformation:)")]
    pub fn track_handler<F>(
        &self,
        observations: swift::Array<Observation>,
        camera: CameraInformation,
        callback: F,
    ) where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        self.track_data(
            dock_accessory_track_observations as *const (),
            (&raw const DOCK_ACCESSORY_TRACK_OBSERVATIONS_ASYNC).cast(),
            TrackData::Observations(observations),
            camera,
            None,
            callback,
        );
    }

    #[cfg(feature = "av")]
    #[doc(alias = "DockAccessory.track(_:cameraInformation:image:)")]
    pub fn track_with_image_handler<F>(
        &self,
        observations: swift::Array<Observation>,
        camera: CameraInformation,
        image: &crate::cv::PixelBuf,
        callback: F,
    ) where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        self.track_data(
            dock_accessory_track_observations_with_image as *const (),
            (&raw const DOCK_ACCESSORY_TRACK_OBSERVATIONS_WITH_IMAGE_ASYNC).cast(),
            TrackData::Observations(observations),
            camera,
            Some(arc::Retain::retained(image)),
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
        self.track_data(
            dock_accessory_track_metadata as *const (),
            (&raw const DOCK_ACCESSORY_TRACK_METADATA_ASYNC).cast(),
            TrackData::Metadata(metadata_objects(metadata)),
            camera,
            None,
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
        self.track_data(
            dock_accessory_track_metadata_with_image as *const (),
            (&raw const DOCK_ACCESSORY_TRACK_METADATA_WITH_IMAGE_ASYNC).cast(),
            TrackData::Metadata(metadata_objects(metadata)),
            camera,
            Some(arc::Retain::retained(image)),
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
        let ids = swift::Array::from_slice(ids);
        self.async_void(move |accessory, callback| accessory.select_subjects_array(ids, callback))
    }

    #[cfg(all(feature = "async", feature = "av"))]
    pub fn track(
        &self,
        observations: swift::Array<Observation>,
        camera: CameraInformation,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        self.async_void(move |accessory, callback| {
            accessory.track_handler(observations, camera, callback)
        })
    }

    #[cfg(all(feature = "async", feature = "av"))]
    pub fn track_with_image(
        &self,
        observations: swift::Array<Observation>,
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
        let data = TrackData::Metadata(metadata_objects(metadata));
        self.async_void(move |accessory, callback| {
            accessory.track_data(
                dock_accessory_track_metadata as *const (),
                (&raw const DOCK_ACCESSORY_TRACK_METADATA_ASYNC).cast(),
                data,
                camera,
                None,
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
        let data = TrackData::Metadata(metadata_objects(metadata));
        let image = arc::Retain::retained(image);
        self.async_void(move |accessory, callback| {
            accessory.track_data(
                dock_accessory_track_metadata_with_image as *const (),
                (&raw const DOCK_ACCESSORY_TRACK_METADATA_WITH_IMAGE_ASYNC).cast(),
                data,
                camera,
                Some(image),
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
        self.call_progress(
            dock_accessory_animate as *const (),
            (&raw const DOCK_ACCESSORY_ANIMATE_ASYNC).cast(),
            // A resilient enum is passed indirectly, so the call needs
            // somewhere stable to point at.
            animation,
            |animation| concurrency::AsyncCallArgs::new().arg(0, animation.as_abi_ptr().cast_mut()),
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
            abi::call::double_to_words2(swift_duration_seconds as *const (), duration.as_secs_f64())
        };
        self.call_progress(
            dock_accessory_set_vector_orientation as *const (),
            (&raw const DOCK_ACCESSORY_SET_VECTOR_ORIENTATION_ASYNC).cast(),
            (),
            |_| {
                concurrency::AsyncCallArgs::new()
                    .float(0, rotation.x)
                    .float(1, rotation.y)
                    .float(2, rotation.z)
                    .arg(0, duration.0 as *mut ())
                    .arg(1, duration.1 as *mut ())
                    .arg(2, relative as usize as *mut ())
            },
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
            abi::call::double_to_words2(swift_duration_seconds as *const (), duration.as_secs_f64())
        };
        self.call_progress(
            dock_accessory_set_rotation_orientation as *const (),
            (&raw const DOCK_ACCESSORY_SET_ROTATION_ORIENTATION_ASYNC).cast(),
            (),
            |_| {
                // A `Rotation3D` is a four-`Double` vector, which Swift passes
                // as two of them rather than as four scalars.
                concurrency::AsyncCallArgs::new()
                    .vector2(0, [rotation.x, rotation.y])
                    .vector2(1, [rotation.z, rotation.w])
                    .arg(0, duration.0 as *mut ())
                    .arg(1, duration.1 as *mut ())
                    .arg(2, relative as usize as *mut ())
            },
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

impl Accessory {
    /// Swift's `==` is a static member taking both operands as arguments
    /// rather than one of them as `self`.
    #[swift::call(sym = "$s7DockKit0A9AccessoryC2eeoiySbAC_ACtFZ")]
    fn swift_eq(lhs: &Self, rhs: &Self) -> bool;
}

impl PartialEq for Accessory {
    fn eq(&self, other: &Self) -> bool {
        Self::swift_eq(self, other)
    }
}

impl Eq for Accessory {}

impl std::hash::Hash for Accessory {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.hash_value(), state)
    }
}

#[cfg(feature = "av")]
crate::define_swift_objc_ref!(
    /// One `AVMetadataObject` as DockKit's tracking API takes it.
    pub(crate) MetadataObjRef(crate::av::MetadataObj) = class "AVMetadataObject"
);

/// Retains the borrowed metadata objects into a Swift array of them.
#[cfg(feature = "av")]
fn metadata_objects(values: &[&crate::av::MetadataObj]) -> swift::Array<MetadataObjRef> {
    swift::Array::from_iter(values.iter().map(|value| MetadataObjRef(value.retained())))
}

#[cfg(feature = "av")]
enum TrackData {
    Observations(swift::Array<Observation>),
    Metadata(swift::Array<MetadataObjRef>),
}

/// What one `track` call keeps alive while Swift runs it.
///
/// `arc::R<cv::PixelBuf>` is not `Send` — a CoreFoundation type carries no such
/// promise in these bindings — but a pixel buffer is reference-counted
/// atomically and DockKit's own API takes one across the same boundary, so
/// handing this to the task is what the framework already expects.
#[cfg(feature = "av")]
struct TrackArgs {
    data: TrackData,
    camera: CameraInformation,
    image: Option<arc::R<crate::cv::PixelBuf>>,
}

#[cfg(feature = "av")]
unsafe impl Send for TrackArgs {}

#[cfg(feature = "av")]
impl TrackData {
    fn as_raw(&self) -> *mut () {
        match self {
            Self::Observations(value) => value.as_raw(),
            Self::Metadata(value) => value.as_raw(),
        }
    }
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

        let observations = swift::Array::from_slice(&[observation]);
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
        // A no-payload case fully initializes the value, so the storage needs
        // nothing destroyed when it goes.
        unsafe fn event(tag: u32) -> Storage<AccessoryEventValue> {
            unsafe {
                let mut storage = Storage::<AccessoryEventValue>::new();
                abi::destructive_inject_enum_tag(
                    storage.as_mut_ptr().cast(),
                    tag,
                    AccessoryEventValue::metadata(),
                );
                storage
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

        let _ = metadata_objects(&[]);
    }
}

impl core::fmt::Debug for Accessory {
    /// Renders Swift's `DockAccessory.debugDescription`.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.debug_desc().to_string())
    }
}
