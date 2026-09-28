use std::{hash::Hash, mem::size_of};

use crate::{
    arc, cg, define_swift_tag_enum, ns, spatial, swift,
    swift::{
        FromSwift, SwiftMetadata, abi,
        concurrency::{self, define_async_sequence},
        foundation::{Date, Uuid},
        value::{Optional, Storage, ValueRef},
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
        let value = unsafe { ValueRef::<StateChangeValue>::new(value) };
        Self {
            accessory: value.accessory(),
            state: value.state(),
            tracking_button_enabled: value.tracking_button_enabled(),
        }
    }
}

impl ValueRef<StateChangeValue> {
    #[swift::call(
        "DockKit.DockAccessory(class).StateChange(struct).state: \
         DockKit.DockAccessory(class).State(enum) { get }"
    )]
    fn state(&self) -> State;

    #[swift::call(
        "DockKit.DockAccessory(class).StateChange(struct).trackingButtonEnabled: Bool { get }"
    )]
    fn tracking_button_enabled(&self) -> bool;

    #[swift::call(
        "DockKit.DockAccessory(class).StateChange(struct).accessory: \
         DockKit.DockAccessory(class)? { get }"
    )]
    fn accessory(&self) -> Option<arc::R<Accessory>>;
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
    #[swift::call(
        "DockKit.DockAccessory(class).Identifier(struct).category: \
         DockKit.DockAccessory(class).Category(enum) { get }"
    )]
    pub fn category(&self) -> Category;

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
    #[swift::call(
        "static DockKit.DockAccessory(class).Identifier(struct).==(_: DockKit.DockAccessory(class).Identifier(struct), _: DockKit.DockAccessory(class).Identifier(struct)) -> Bool"
    )]
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

    #[swift::call("DockKit.DockAccessory(class).MotionState(struct).error: Error? { get }")]
    pub fn error(&self) -> Option<arc::R<ns::Error>>;
}

impl BatteryState {
    #[swift::call("DockKit.DockAccessory(class).BatteryState(struct).name: String { get }")]
    pub fn name(&self) -> swift::String;

    #[swift::call("DockKit.DockAccessory(class).BatteryState(struct).batteryLevel: Double { get }")]
    pub fn battery_level(&self) -> f64;

    #[swift::call("DockKit.DockAccessory(class).BatteryState(struct).lowBattery: Bool { get }")]
    pub fn is_low_battery(&self) -> bool;

    #[swift::call(
        "DockKit.DockAccessory(class).BatteryState(struct).chargeState: \
         DockKit.DockAccessory(class).BatteryChargeState(enum) { get }"
    )]
    pub fn charge_state(&self) -> BatteryChargeState;

    #[swift::call("DockKit.DockAccessory(class).BatteryState(struct).hashValue: Int { get }")]
    pub fn hash_value(&self) -> isize;
}

impl BatteryState {
    /// Swift's `==` is a static member taking both operands as arguments
    /// rather than one of them as `self`.
    #[swift::call(
        "static DockKit.DockAccessory(class).BatteryState(struct).==(_: DockKit.DockAccessory(class).BatteryState(struct), _: DockKit.DockAccessory(class).BatteryState(struct)) -> Bool"
    )]
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
        Self::init(
            Storage::from_option(yaw),
            Storage::from_option(pitch),
            Storage::from_option(roll),
        )
    }

    #[swift::call(
        "DockKit.DockAccessory(class).Limits(struct).init(\
         yaw: DockKit.DockAccessory(class).Limits(struct).Limit(struct)?, \
         pitch: DockKit.DockAccessory(class).Limits(struct).Limit(struct)?, \
         roll: DockKit.DockAccessory(class).Limits(struct).Limit(struct)?)"
    )]
    fn init(
        yaw: Storage<Optional<Limit>>,
        pitch: Storage<Optional<Limit>>,
        roll: Storage<Optional<Limit>>,
    ) -> Self;

    #[swift::call(
        "DockKit.DockAccessory(class).Limits(struct).yaw: DockKit.DockAccessory(class).Limits(struct).Limit(struct)? { get }"
    )]
    pub fn yaw(&self) -> Option<Limit>;

    #[swift::call(
        "DockKit.DockAccessory(class).Limits(struct).pitch: DockKit.DockAccessory(class).Limits(struct).Limit(struct)? { get }"
    )]
    pub fn pitch(&self) -> Option<Limit>;

    #[swift::call(
        "DockKit.DockAccessory(class).Limits(struct).roll: DockKit.DockAccessory(class).Limits(struct).Limit(struct)? { get }"
    )]
    pub fn roll(&self) -> Option<Limit>;
}

impl Limit {
    /// Swift traps on a range whose lower bound is above its upper one, so
    /// that is checked on the way in.
    #[swift::call(
        "DockKit.DockAccessory(class).Limits(struct).Limit(struct).init(\
         positionRange: Range<Double>, maximumSpeed: Double) throws"
    )]
    pub fn new(
        position_range: std::ops::Range<f64>,
        maximum_speed: f64,
    ) -> Result<Self, arc::R<ns::Error>>;

    #[swift::call(
        "DockKit.DockAccessory(class).Limits(struct).Limit(struct).positionRange: \
         Range<Double> { get }"
    )]
    pub fn position_range(&self) -> std::ops::Range<f64>;

    #[swift::call(
        "DockKit.DockAccessory(class).Limits(struct).Limit(struct).maximumSpeed: Double { get }"
    )]
    pub fn maximum_speed(&self) -> f64;
}

/// The tag an enum value carries for a case, read from the case's descriptor.
#[inline]
fn case_tag(descriptor: *const u8) -> u32 {
    unsafe { descriptor.cast::<u32>().read() }
}

impl AccessoryEvent {
    fn button_tag() -> u32 {
        case_tag(swift::enum_case!(
            "DockKit.DockAccessory(class).AccessoryEvent(enum).button(Int, Bool)"
        ))
    }

    fn camera_shutter_tag() -> u32 {
        case_tag(swift::enum_case!(
            "DockKit.DockAccessory(class).AccessoryEvent(enum).cameraShutter"
        ))
    }

    fn camera_flip_tag() -> u32 {
        case_tag(swift::enum_case!(
            "DockKit.DockAccessory(class).AccessoryEvent(enum).cameraFlip"
        ))
    }

    fn camera_zoom_tag() -> u32 {
        case_tag(swift::enum_case!(
            "DockKit.DockAccessory(class).AccessoryEvent(enum).cameraZoom(factor: Double)"
        ))
    }

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
            if tag == Self::button_tag() {
                abi::destructive_project_enum_data(
                    storage.as_mut_ptr(),
                    AccessoryEventValue::metadata(),
                );
                let id = storage.as_ptr().cast::<isize>().read();
                let pressed = storage.as_ptr().cast::<u8>().add(size_of::<isize>()).read() != 0;
                Self::Button { id, pressed }
            } else if tag == Self::camera_zoom_tag() {
                abi::destructive_project_enum_data(
                    storage.as_mut_ptr(),
                    AccessoryEventValue::metadata(),
                );
                let factor = storage.as_ptr().cast::<f64>().read();
                Self::CameraZoom { factor }
            } else if tag == Self::camera_shutter_tag() {
                storage.destroy();
                Self::CameraShutter
            } else if tag == Self::camera_flip_tag() {
                storage.destroy();
                Self::CameraFlip
            } else {
                storage.destroy();
                Self::Unknown(tag)
            }
        }
    }
}

impl TrackedPerson {
    #[swift::call(
        "DockKit.DockAccessory(class).TrackedPerson(struct).identifier: Foundation.UUID(struct) { get }"
    )]
    pub fn identifier(&self) -> Uuid;

    #[swift::call(
        "DockKit.DockAccessory(class).TrackedPerson(struct).rect: __C.CGRect(struct) { get }"
    )]
    pub fn rect(&self) -> cg::Rect;

    #[swift::call("DockKit.DockAccessory(class).TrackedPerson(struct).saliencyRank: Int? { get }")]
    pub fn saliency_rank(&self) -> Option<isize>;

    #[swift::call(
        "DockKit.DockAccessory(class).TrackedPerson(struct).speakingConfidence: Double? { get }"
    )]
    pub fn speaking_confidence(&self) -> Option<f64>;

    #[swift::call(
        "DockKit.DockAccessory(class).TrackedPerson(struct).lookingAtCameraConfidence: \
         Double? { get }"
    )]
    pub fn looking_at_camera_confidence(&self) -> Option<f64>;
}

impl TrackedPerson {
    /// Swift's `==` is a static member taking both operands as arguments
    /// rather than one of them as `self`.
    #[swift::call(
        "static DockKit.DockAccessory(class).TrackedPerson(struct).==(_: DockKit.DockAccessory(class).TrackedPerson(struct), _: DockKit.DockAccessory(class).TrackedPerson(struct)) -> Bool"
    )]
    fn swift_eq(lhs: &Self, rhs: &Self) -> bool;
}

impl PartialEq for TrackedPerson {
    fn eq(&self, other: &Self) -> bool {
        Self::swift_eq(self, other)
    }
}

impl TrackedObject {
    #[swift::call(
        "DockKit.DockAccessory(class).TrackedObject(struct).identifier: Foundation.UUID(struct) { get }"
    )]
    pub fn identifier(&self) -> Uuid;

    #[swift::call(
        "DockKit.DockAccessory(class).TrackedObject(struct).rect: __C.CGRect(struct) { get }"
    )]
    pub fn rect(&self) -> cg::Rect;

    #[swift::call("DockKit.DockAccessory(class).TrackedObject(struct).saliencyRank: Int? { get }")]
    pub fn saliency_rank(&self) -> Option<isize>;
}

impl TrackedObject {
    /// Swift's `==` is a static member taking both operands as arguments
    /// rather than one of them as `self`.
    #[swift::call(
        "static DockKit.DockAccessory(class).TrackedObject(struct).==(_: DockKit.DockAccessory(class).TrackedObject(struct), _: DockKit.DockAccessory(class).TrackedObject(struct)) -> Bool"
    )]
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

            if tag
                == case_tag(swift::enum_case!(
                    "DockKit.DockAccessory(class).TrackedSubjectType(enum).person(\
                 DockKit.DockAccessory(class).TrackedPerson(struct))"
                ))
            {
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
            } else if tag
                == case_tag(swift::enum_case!(
                    "DockKit.DockAccessory(class).TrackedSubjectType(enum).object(\
                 DockKit.DockAccessory(class).TrackedObject(struct))"
                ))
            {
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

    #[swift::call(
        "DockKit.DockAccessory(class).TrackingState(struct).trackedSubjects: \
         [DockKit.DockAccessory(class).TrackedSubjectType(enum)] { get }"
    )]
    pub fn tracked_subjects(&self) -> swift::Array<TrackedSubject>;

    #[swift::call("DockKit.DockAccessory(class).TrackingState(struct).description: String { get }")]
    pub fn description(&self) -> swift::String;
}

impl Observation {
    /// Creates an observation without a face-yaw measurement.
    pub fn new(identifier: isize, ty: ObservationType, rect: cg::Rect) -> Self {
        Self::init(identifier, ty, rect, Storage::none())
    }

    #[swift::call(
        "DockKit.DockAccessory(class).Observation(struct).init(\
         identifier: Int, \
         type: DockKit.DockAccessory(class).Observation(struct).ObservationType(enum), \
         rect: __C.CGRect(struct), \
         faceYawAngle: Foundation.Measurement(struct)<__C.NSUnitAngle(class)>?)"
    )]
    fn init(
        identifier: isize,
        ty: ObservationType,
        rect: cg::Rect,
        face_yaw_angle: Storage<Optional<MeasurementAngleValue>>,
    ) -> Self;

    #[swift::call("DockKit.DockAccessory(class).Observation(struct).identifier: Int { get }")]
    pub fn identifier(&self) -> isize;

    #[swift::call(
        "DockKit.DockAccessory(class).Observation(struct).type: \
         DockKit.DockAccessory(class).Observation(struct).ObservationType(enum) { get }"
    )]
    pub fn ty(&self) -> ObservationType;

    #[swift::call(
        "DockKit.DockAccessory(class).Observation(struct).rect: __C.CGRect(struct) { get }"
    )]
    pub fn rect(&self) -> cg::Rect;
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
            // Written out rather than generated: the optional size travels as
            // three words in registers, which a generated call cannot pass yet.
            // The initializer takes the device type at `+1`, so it gets a
            // reference of its own.
            abi::call::camera_information_init(
                swift::symbol!(
                    "DockKit.DockAccessory(class).CameraInformation(struct).init(\
                     captureDevice: __C.AVCaptureDeviceType, \
                     cameraPosition: __C.AVCaptureDevicePosition(struct), \
                     orientation: DockKit.DockAccessory(class).CameraOrientation(enum), \
                     cameraIntrinsics: __C.simd_float3x3?, \
                     referenceDimensions: __C.CGSize(struct)?)"
                ),
                arc::Retain::retained(device_type).into_raw().cast(),
                position as isize,
                orientation.as_abi_ptr(),
                intrinsics.as_ptr(),
                (words.read(), words.add(1).read(), words.add(2).read()),
                storage.as_mut_ptr().cast(),
            );
            storage.assume_init()
        }
    }

    #[swift::call(
        "DockKit.DockAccessory(class).CameraInformation(struct).captureDevice: \
         __C.AVCaptureDeviceType { get }"
    )]
    pub fn capture_device(&self) -> arc::R<crate::av::CaptureDeviceType>;

    pub fn camera_position(&self) -> crate::av::CaptureDevicePos {
        unsafe { std::mem::transmute(self.camera_position_raw()) }
    }

    /// An `NS_ENUM`, which Swift returns as its raw integer.
    #[swift::call(
        "DockKit.DockAccessory(class).CameraInformation(struct).cameraPosition: \
         __C.AVCaptureDevicePosition(struct) { get }"
    )]
    fn camera_position_raw(&self) -> isize;

    #[swift::call(
        "DockKit.DockAccessory(class).CameraInformation(struct).orientation: \
         DockKit.DockAccessory(class).CameraOrientation(enum) { get }"
    )]
    pub fn orientation(&self) -> CameraOrientation;

    pub fn camera_intrinsics(&self) -> Option<CameraIntrinsics> {
        unsafe {
            let mut storage = Storage::<Optional<CameraIntrinsicsValue>>::new();
            abi::call::value_to_value(
                swift::symbol!(
                    "DockKit.DockAccessory(class).CameraInformation(struct).cameraIntrinsics: \
                     __C.simd_float3x3? { get }"
                ),
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
            // Three words: the size's two doubles and the optional's tag.
            let words = abi::call::value_to_words3(
                swift::symbol!(
                    "DockKit.DockAccessory(class).CameraInformation(struct).referenceDimensions: \
                     __C.CGSize(struct)? { get }"
                ),
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
    sequence = "DockKit.DockAccessory(class).StateChanges",
    element = StateChange = "DockKit.DockAccessory(class).StateChange(struct)",
    async_iter = StateChangesAsyncIter,
}

define_async_sequence! {
    /// `DockAccessory.MotionStates`.
    MotionStates, MotionStatesValue, MotionStatesIteratorValue,
    sequence = "DockKit.DockAccessory(class).MotionStates",
    element = MotionState = "DockKit.DockAccessory(class).MotionState(struct)",
    async_iter = MotionStatesAsyncIter,
}

define_async_sequence! {
    /// `DockAccessory.AccessoryEvents`.
    AccessoryEvents, AccessoryEventsValue, AccessoryEventsIteratorValue,
    sequence = "DockKit.DockAccessory(class).AccessoryEvents",
    element = AccessoryEvent = "DockKit.DockAccessory(class).AccessoryEvent(enum)",
    async_iter = AccessoryEventsAsyncIter,
}

define_async_sequence! {
    /// `DockAccessory.TrackingStates`.
    TrackingStates, TrackingStatesValue, TrackingStatesIteratorValue,
    sequence = "DockKit.DockAccessory(class).TrackingStates",
    element = TrackingState = "DockKit.DockAccessory(class).TrackingState(struct)",
    async_iter = TrackingStatesAsyncIter,
}

define_async_sequence! {
    /// `DockAccessory.BatteryStates`.
    BatteryStates, BatteryStatesValue, BatteryStatesIteratorValue,
    sequence = "DockKit.DockAccessory(class).BatteryStates",
    element = BatteryState = "DockKit.DockAccessory(class).BatteryState(struct)",
    async_iter = BatteryStatesAsyncIter,
}

define_swift_tag_enum!(
    /// DockKit `DockAccessory.State`.
    #[doc(alias = "DockAccessory.State")]
    pub State = swift "DockKit.DockAccessory(class).State" {
        debug,
        cases {
            undocked = "undocked",
            docked = "docked",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.Category`.
    #[doc(alias = "DockAccessory.Category")]
    pub Category = swift "DockKit.DockAccessory(class).Category" {
        debug,
        cases {
            tracking_stand = "trackingStand",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.CameraOrientation`.
    #[doc(alias = "DockAccessory.CameraOrientation")]
    pub CameraOrientation = swift "DockKit.DockAccessory(class).CameraOrientation" {
        cases {
            unknown = "unknown",
            portrait = "portrait",
            portrait_upside_down = "portraitUpsideDown",
            landscape_right = "landscapeRight",
            landscape_left = "landscapeLeft",
            face_up = "faceUp",
            face_down = "faceDown",
            corrected = "corrected",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.Observation.ObservationType`.
    #[doc(alias = "DockAccessory.Observation.ObservationType")]
    pub ObservationType = swift "DockKit.DockAccessory(class).Observation(struct).ObservationType" {
        cases {
            human_face = "humanFace",
            human_body = "humanBody",
            object = "object",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.BatteryChargeState`.
    #[doc(alias = "DockAccessory.BatteryChargeState")]
    pub BatteryChargeState = swift "DockKit.DockAccessory(class).BatteryChargeState" {
        cases {
            not_charging = "notCharging",
            charging = "charging",
            not_chargeable = "notChargeable",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.FramingMode`.
    #[doc(alias = "DockAccessory.FramingMode")]
    pub FramingMode = swift "DockKit.DockAccessory(class).FramingMode" {
        cases {
            automatic = "automatic",
            center = "center",
            left = "left",
            right = "right",
        }
    }
);

define_swift_tag_enum!(
    /// DockKit `DockAccessory.Animation`.
    #[doc(alias = "DockAccessory.Animation")]
    pub Animation = swift "DockKit.DockAccessory(class).Animation" {
        cases {
            wakeup = "wakeup",
            yes = "yes",
            no = "no",
            kapow = "kapow",
        }
    }
);

impl Accessory {
    #[swift::call("DockKit.DockAccessory(class).hashValue: Int { get }")]
    pub fn hash_value(&self) -> isize;

    #[swift::call(
        "DockKit.DockAccessory(class).identifier: DockKit.DockAccessory(class).Identifier(struct) { get }"
    )]
    #[doc(alias = "DockAccessory.identifier")]
    pub fn identifier(&self) -> Identifier;

    #[swift::call("DockKit.DockAccessory(class).debugDescription: String { get }")]
    pub fn debug_desc(&self) -> swift::String;

    #[doc(alias = "DockAccessory.framingMode")]
    #[swift::call(
        "DockKit.DockAccessory(class).framingMode: \
         DockKit.DockAccessory(class).FramingMode(enum) { get }"
    )]
    pub fn framing_mode(&self) -> FramingMode;

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

    #[swift::call(
        "DockKit.DockAccessory(class).limits: DockKit.DockAccessory(class).Limits(struct) { get }"
    )]
    #[doc(alias = "DockAccessory.limits")]
    pub fn limits(&self) -> Result<Limits, arc::R<ns::Error>>;

    #[swift::call(
        "DockKit.DockAccessory(class).motionStates: \
         DockKit.DockAccessory(class).MotionStates(struct) { get }"
    )]
    #[doc(alias = "DockAccessory.motionStates")]
    pub fn motion_states(&self) -> Result<MotionStates, arc::R<ns::Error>>;

    #[crate::api::available(macos = 15.0, ios = 18.0)]
    #[swift::call(
        "DockKit.DockAccessory(class).batteryStates: \
         DockKit.DockAccessory(class).BatteryStates(struct) { get }"
    )]
    #[doc(alias = "DockAccessory.batteryStates")]
    pub fn battery_states(&self) -> Result<BatteryStates, arc::R<ns::Error>>;

    #[doc(alias = "DockAccessory.accessoryEvents")]
    #[crate::api::available(macos = 14.4, ios = 17.4)]
    #[swift::call(
        "DockKit.DockAccessory(class).accessoryEvents: \
         DockKit.DockAccessory(class).AccessoryEvents(struct) { get }"
    )]
    pub fn accessory_events(&self) -> Result<AccessoryEvents, arc::R<ns::Error>>;

    #[doc(alias = "DockAccessory.trackingStates")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    #[swift::call(
        "DockKit.DockAccessory(class).trackingStates: \
         DockKit.DockAccessory(class).TrackingStates(struct) { get }"
    )]
    pub fn tracking_states(&self) -> Result<TrackingStates, arc::R<ns::Error>>;

    #[doc(alias = "DockAccessory.setLimits(_:)")]
    #[swift::call(
        "DockKit.DockAccessory(class).setLimits(_: DockKit.DockAccessory(class).Limits(struct)) throws"
    )]
    pub fn set_limits(&self, limits: &Limits) -> Result<(), arc::R<ns::Error>>;

    /// Deprecated synchronous DockKit orientation API.
    #[doc(alias = "DockAccessory.setOrientation(_:duration:relative:)")]
    #[deprecated = "use the async set_orientation method on iOS 18 or macOS 15"]
    pub fn set_orientation_sync(
        &self,
        rotation: spatial::Vector3D,
        duration: std::time::Duration,
        relative: bool,
    ) -> Result<arc::R<ns::Progress>, arc::R<ns::Error>> {
        self.set_vector_orientation_sync(rotation, duration.into(), relative)
    }

    #[swift::call(
        "DockKit.DockAccessory(class).setOrientation(_: __C.SPVector3D, \
         duration: Swift.Duration(struct), relative: Bool) \
         throws -> __C.NSProgress(class)"
    )]
    fn set_vector_orientation_sync(
        &self,
        rotation: spatial::Vector3D,
        duration: swift::Duration,
        relative: bool,
    ) -> Result<arc::R<ns::Progress>, arc::R<ns::Error>>;

    /// Deprecated synchronous DockKit rotation API.
    #[doc(alias = "DockAccessory.setOrientation(_:duration:relative:)")]
    #[deprecated = "use the async set_rotation method on iOS 18 or macOS 15"]
    pub fn set_rotation_sync(
        &self,
        rotation: spatial::Rotation3D,
        duration: std::time::Duration,
        relative: bool,
    ) -> Result<arc::R<ns::Progress>, arc::R<ns::Error>> {
        self.set_rotation_orientation_sync(rotation, duration.into(), relative)
    }

    /// Passed directly, the quaternion takes `d0`-`d3`.
    #[swift::call(
        "DockKit.DockAccessory(class).setOrientation(_: __C.SPRotation3D, \
         duration: Swift.Duration(struct), relative: Bool) \
         throws -> __C.NSProgress(class)"
    )]
    fn set_rotation_orientation_sync(
        &self,
        rotation: spatial::Rotation3D,
        duration: swift::Duration,
        relative: bool,
    ) -> Result<arc::R<ns::Progress>, arc::R<ns::Error>>;

    /// Awaits one of the accessory's `Void`-returning methods.
    ///
    /// They differ only in which registers their arguments go in, so the call
    /// itself is `owned` — whatever has to stay alive — plus where it goes.
    #[cfg(feature = "av")]
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

    /// The future sibling of [`Self::call_void`], which builds its awaiting
    /// state on the task's own allocation rather than beside it.
    #[cfg(all(feature = "async", feature = "av"))]
    fn call_void_future<O>(
        &self,
        function: *const (),
        async_fn: *const u8,
        owned: O,
        args: impl FnOnce(&mut O) -> concurrency::AsyncCallArgs,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>>
    where
        O: Send + 'static,
    {
        unsafe {
            concurrency::call_async_future(
                function,
                async_fn,
                (arc::Retain::retained(self), owned),
                |(accessory, owned)| args(owned).swift_self(accessory.as_ptr().cast()),
                |_, _| (),
            )
        }
    }

    /// The same for [`Self::call_progress`].
    #[cfg(feature = "async")]
    fn call_progress_future<O>(
        &self,
        function: *const (),
        async_fn: *const u8,
        owned: O,
        args: impl FnOnce(&mut O) -> concurrency::AsyncCallArgs,
    ) -> impl std::future::Future<Output = Result<arc::R<ns::Progress>, arc::R<ns::Error>>>
    where
        O: Send + 'static,
    {
        unsafe {
            concurrency::call_async_future(
                function,
                async_fn,
                (arc::Retain::retained(self), owned),
                |(accessory, owned)| args(owned).swift_self(accessory.as_ptr().cast()),
                |_, progress| arc::R::from_raw(progress.cast()),
            )
        }
    }

    #[swift::call(
        "DockKit.DockAccessory(class).setAngularVelocity(_: __C.SPVector3D) async throws"
    )]
    pub fn set_angular_velocity(
        &self,
        velocity: spatial::Vector3D,
    ) -> Result<(), arc::R<ns::Error>>;

    #[swift::call(
        "DockKit.DockAccessory(class).selectSubject(at: __C.CGPoint(struct)) async throws"
    )]
    pub fn select_subject(&self, point: cg::Point) -> Result<(), arc::R<ns::Error>>;

    #[doc(alias = "DockAccessory.setFramingMode(_:)")]
    #[swift::call(
        "DockKit.DockAccessory(class).setFramingMode(\
         _: DockKit.DockAccessory(class).FramingMode(enum)) async throws"
    )]
    pub fn set_framing_mode(&self, mode: FramingMode) -> Result<(), arc::R<ns::Error>>;

    #[swift::call(
        "DockKit.DockAccessory(class).setRegionOfInterest(_: __C.CGRect(struct)) async throws"
    )]
    pub fn set_region_of_interest(&self, rect: cg::Rect) -> Result<(), arc::R<ns::Error>>;

    #[swift::call(
        "DockKit.DockAccessory(class).selectSubjects(_: [Foundation.UUID(struct)]) async throws"
    )]
    fn select_subjects_array(&self, ids: swift::Array<Uuid>) -> Result<(), arc::R<ns::Error>>;

    #[doc(alias = "DockAccessory.selectSubjects(_:)")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    pub fn select_subjects_handler<F>(&self, ids: &[Uuid], callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        self.select_subjects_array_handler(swift::Array::from_slice(ids), callback);
    }

    /// The four `track` overloads differ only in what they hand over as the
    /// observations and whether they carry an image, so each is declared
    /// against its own symbol rather than routed through one call.
    #[cfg(feature = "av")]
    #[doc(alias = "DockAccessory.track(_:cameraInformation:)")]
    #[swift::call(
        "DockKit.DockAccessory(class).track(\
         _: [DockKit.DockAccessory(class).Observation(struct)], \
         cameraInformation: DockKit.DockAccessory(class).CameraInformation(struct)) async throws"
    )]
    pub fn track(
        &self,
        observations: swift::Array<Observation>,
        camera: CameraInformation,
    ) -> Result<(), arc::R<ns::Error>>;

    #[cfg(feature = "av")]
    #[swift::call(
        "DockKit.DockAccessory(class).track(\
         _: [__C.AVMetadataObject(class)], \
         cameraInformation: DockKit.DockAccessory(class).CameraInformation(struct)) async throws"
    )]
    fn track_metadata_array(
        &self,
        metadata: swift::Array<MetadataObjRef>,
        camera: CameraInformation,
    ) -> Result<(), arc::R<ns::Error>>;

    /// The two `track` overloads that carry an image, which are still written
    /// out: `arc::R<cv::PixelBuf>` is deliberately not `Send`, and a generated
    /// call requires everything it keeps alive to be, so what says the buffer
    /// may cross is [`TrackArgs`] rather than the pixel buffer's own type.
    #[cfg(feature = "av")]
    fn track_image<F>(
        &self,
        (function, async_fn): (*const (), *const u8),
        data: TrackData,
        camera: CameraInformation,
        image: arc::R<crate::cv::PixelBuf>,
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
                image: Some(image),
            },
            track_args,
            callback,
        );
    }

    /// The future sibling of [`Self::track_image`].
    #[cfg(all(feature = "async", feature = "av"))]
    fn track_image_future(
        &self,
        (function, async_fn): (*const (), *const u8),
        data: TrackData,
        camera: CameraInformation,
        image: arc::R<crate::cv::PixelBuf>,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        self.call_void_future(
            function,
            async_fn,
            TrackArgs {
                data,
                camera,
                image: Some(image),
            },
            track_args,
        )
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
        self.track_image(
            track_observations_with_image(),
            TrackData::Observations(observations),
            camera,
            arc::Retain::retained(image),
            callback,
        );
    }

    #[cfg(all(feature = "async", feature = "av"))]
    pub fn track_with_image(
        &self,
        observations: swift::Array<Observation>,
        camera: CameraInformation,
        image: &crate::cv::PixelBuf,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        self.track_image_future(
            track_observations_with_image(),
            TrackData::Observations(observations),
            camera,
            arc::Retain::retained(image),
        )
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
        self.track_metadata_array_handler(metadata_objects(metadata), camera, callback);
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
        self.track_image(
            track_metadata_with_image(),
            TrackData::Metadata(metadata_objects(metadata)),
            camera,
            arc::Retain::retained(image),
            callback,
        );
    }

    #[cfg(feature = "async")]
    #[crate::api::available(macos = 15.0, ios = 18.0)]
    pub fn select_subjects(
        &self,
        ids: &[Uuid],
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        self.select_subjects_array(swift::Array::from_slice(ids))
    }

    #[cfg(all(feature = "async", feature = "av"))]
    pub fn track_metadata(
        &self,
        metadata: &[&crate::av::MetadataObj],
        camera: CameraInformation,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        self.track_metadata_array(metadata_objects(metadata), camera)
    }

    #[cfg(all(feature = "async", feature = "av"))]
    pub fn track_metadata_with_image(
        &self,
        metadata: &[&crate::av::MetadataObj],
        camera: CameraInformation,
        image: &crate::cv::PixelBuf,
    ) -> impl std::future::Future<Output = Result<(), arc::R<ns::Error>>> {
        self.track_image_future(
            track_metadata_with_image(),
            TrackData::Metadata(metadata_objects(metadata)),
            camera,
            arc::Retain::retained(image),
        )
    }

    #[doc(alias = "DockAccessory.animate(motion:)")]
    #[swift::call(
        "DockKit.DockAccessory(class).animate(\
         motion: DockKit.DockAccessory(class).Animation(enum)) async throws -> __C.NSProgress(class)"
    )]
    pub fn animate(&self, animation: Animation) -> Result<arc::R<ns::Progress>, arc::R<ns::Error>>;

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
        let duration = duration.into();
        let vector_orientation = vector_orientation();
        self.call_progress(
            vector_orientation.0,
            vector_orientation.1,
            (),
            move |_| vector_orientation_args(rotation, duration, relative),
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
        let duration = duration.into();
        let rotation_orientation = rotation_orientation();
        self.call_progress(
            rotation_orientation.0,
            rotation_orientation.1,
            (),
            move |_| rotation_orientation_args(rotation, duration, relative),
            callback,
        );
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
        let duration = duration.into();
        let vector_orientation = vector_orientation();
        self.call_progress_future(vector_orientation.0, vector_orientation.1, (), move |_| {
            vector_orientation_args(rotation, duration, relative)
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
        let duration = duration.into();
        let rotation_orientation = rotation_orientation();
        self.call_progress_future(
            rotation_orientation.0,
            rotation_orientation.1,
            (),
            move |_| rotation_orientation_args(rotation, duration, relative),
        )
    }
}

impl Accessory {
    /// Swift's `==` is a static member taking both operands as arguments
    /// rather than one of them as `self`.
    #[swift::call(
        "static DockKit.DockAccessory(class).==(_: DockKit.DockAccessory(class), \
                   _: DockKit.DockAccessory(class)) -> Bool"
    )]
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

/// The registers one `track` call goes out in.
///
/// A free function rather than a closure at each call site, since the handler
/// and the future halves have to agree on them exactly.
#[cfg(feature = "av")]
fn track_args(track: &mut TrackArgs) -> concurrency::AsyncCallArgs {
    let args = concurrency::AsyncCallArgs::new()
        .arg(0, track.data.as_raw())
        .arg(1, track.camera.as_ptr().cast_mut());
    match &track.image {
        Some(image) => args.arg(2, image.as_ptr().cast()),
        None => args,
    }
}

/// `setOrientation(_:duration:relative:)` with a vector: the entry point and
/// the async function pointer that sizes its context.
fn vector_orientation() -> (*const (), *const u8) {
    swift::async_symbols!(
        "DockKit.DockAccessory(class).setOrientation(_: __C.SPVector3D, \
         duration: Swift.Duration(struct), relative: Bool) \
         async throws -> __C.NSProgress(class)"
    )
}

/// The same, with a rotation.
fn rotation_orientation() -> (*const (), *const u8) {
    swift::async_symbols!(
        "DockKit.DockAccessory(class).setOrientation(_: __C.SPRotation3D, \
         duration: Swift.Duration(struct), relative: Bool) \
         async throws -> __C.NSProgress(class)"
    )
}

/// `track(_:cameraInformation:image:)` with observations.
#[cfg(feature = "av")]
fn track_observations_with_image() -> (*const (), *const u8) {
    swift::async_symbols!(
        "DockKit.DockAccessory(class).track(\
         _: [DockKit.DockAccessory(class).Observation(struct)], \
         cameraInformation: DockKit.DockAccessory(class).CameraInformation(struct), \
         image: __C.CVBufferRef) async throws"
    )
}

/// `track(_:cameraInformation:image:)` with metadata objects.
#[cfg(feature = "av")]
fn track_metadata_with_image() -> (*const (), *const u8) {
    swift::async_symbols!(
        "DockKit.DockAccessory(class).track(\
         _: [__C.AVMetadataObject(class)], \
         cameraInformation: DockKit.DockAccessory(class).CameraInformation(struct), \
         image: __C.CVBufferRef) async throws"
    )
}

/// A `Swift.Duration`'s two words, as the argument registers take them.
fn duration_words(duration: swift::Duration) -> [usize; 2] {
    unsafe { core::mem::transmute(duration) }
}

/// `setOrientation(_:duration:relative:)` with a vector, whose three doubles
/// each take a register of their own.
fn vector_orientation_args(
    rotation: spatial::Vector3D,
    duration: swift::Duration,
    relative: bool,
) -> concurrency::AsyncCallArgs {
    let [low, high] = duration_words(duration);
    concurrency::AsyncCallArgs::new()
        .float(0, rotation.x)
        .float(1, rotation.y)
        .float(2, rotation.z)
        .arg(0, low as *mut ())
        .arg(1, high as *mut ())
        .arg(2, relative as usize as *mut ())
}

/// The same with a rotation, which is a four-`Double` vector Swift passes as
/// two of them rather than as four scalars.
fn rotation_orientation_args(
    rotation: spatial::Rotation3D,
    duration: swift::Duration,
    relative: bool,
) -> concurrency::AsyncCallArgs {
    let [low, high] = duration_words(duration);
    concurrency::AsyncCallArgs::new()
        .vector2(0, [rotation.x, rotation.y])
        .vector2(1, [rotation.z, rotation.w])
        .arg(0, low as *mut ())
        .arg(1, high as *mut ())
        .arg(2, relative as usize as *mut ())
}

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
            let shutter = event(AccessoryEvent::camera_shutter_tag());
            assert_eq!(
                AccessoryEvent::CameraShutter,
                AccessoryEvent::copy_from_ptr(shutter.as_ptr())
            );

            let flip = event(AccessoryEvent::camera_flip_tag());
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
