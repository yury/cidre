use crate::define_opts;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct EventType(pub u32);

impl EventType {
    #[doc(alias = "kIOHIDEventTypeNULL")]
    pub const NULL: Self = Self(0);

    #[doc(alias = "kIOHIDEventTypeVendorDefined")]
    pub const VENDOR_DEFINED: Self = Self(1);

    #[doc(alias = "kIOHIDEventTypeButton")]
    pub const BUTTON: Self = Self(2);

    #[doc(alias = "kIOHIDEventTypeKeyboard")]
    pub const KEYBOARD: Self = Self(3);

    #[doc(alias = "kIOHIDEventTypeTranslation")]
    pub const TRANSLATION: Self = Self(4);

    #[doc(alias = "kIOHIDEventTypeRotation")]
    pub const ROTATION: Self = Self(5);

    #[doc(alias = "kIOHIDEventTypeScroll")]
    pub const SCROLL: Self = Self(6);

    #[doc(alias = "kIOHIDEventTypeScale")]
    pub const SCALE: Self = Self(7);

    #[doc(alias = "kIOHIDEventTypeZoom")]
    pub const ZOOM: Self = Self(8);

    #[doc(alias = "kIOHIDEventTypeVelocity")]
    pub const VELOCITY: Self = Self(9);

    #[doc(alias = "kIOHIDEventTypeOrientation")]
    pub const ORIENTATION: Self = Self(10);

    #[doc(alias = "kIOHIDEventTypeDigitizer")]
    pub const DIGITIZER: Self = Self(11);

    #[doc(alias = "kIOHIDEventTypeAmbientLightSensor")]
    pub const AMBIENT_LIGHT_SENSOR: Self = Self(12);

    #[doc(alias = "kIOHIDEventTypeAccelerometer")]
    pub const ACCELEROMETER: Self = Self(13);

    #[doc(alias = "kIOHIDEventTypeProximity")]
    pub const PROXIMITY: Self = Self(14);

    #[doc(alias = "kIOHIDEventTypeTemperature")]
    pub const TEMPERATURE: Self = Self(15);

    #[doc(alias = "kIOHIDEventTypeNavigationSwipe")]
    pub const NAV_SWIPE: Self = Self(16);

    #[doc(alias = "kIOHIDEventTypePointer")]
    pub const POINTER: Self = Self(17);

    #[doc(alias = "kIOHIDEventTypeProgress")]
    pub const PROGRESS: Self = Self(18);

    #[doc(alias = "kIOHIDEventTypeMultiAxisPointer")]
    pub const MULTI_AXIS_POINTER: Self = Self(19);

    #[doc(alias = "kIOHIDEventTypeGyro")]
    pub const GYRO: Self = Self(20);

    #[doc(alias = "kIOHIDEventTypeCompass")]
    pub const COMPASS: Self = Self(21);

    #[doc(alias = "kIOHIDEventTypeZoomToggle")]
    pub const ZOOM_TOGGLE: Self = Self(22);

    // just like kIOHIDEventTypeNavigationSwipe, but intended for consumption by Dock
    #[doc(alias = "kIOHIDEventTypeDockSwipe")]
    pub const DOCK_SWIPE: Self = Self(23);

    #[doc(alias = "kIOHIDEventTypeSymbolicHotKey")]
    pub const SYMBOLIC_HOT_KEY: Self = Self(24);

    #[doc(alias = "kIOHIDEventTypePower")]
    pub const POWER: Self = Self(25);

    #[doc(alias = "kIOHIDEventTypeLED")]
    pub const LED: Self = Self(26);

    // This will eventually superseed Navagation and Dock swipes
    #[doc(alias = "kIOHIDEventTypeFluidTouchGesture")]
    pub const FLUID_TOUCH_GESTURE: Self = Self(27);

    #[doc(alias = "kIOHIDEventTypeBoundaryScroll")]
    pub const BOUNDARY_SCROLL: Self = Self(28);

    #[doc(alias = "kIOHIDEventTypeBiometric")]
    pub const BIOMETRIC: Self = Self(29);

    #[doc(alias = "kIOHIDEventTypeUnicode")]
    pub const UNICODE: Self = Self(30);

    #[doc(alias = "kIOHIDEventTypeAtmosphericPressure")]
    pub const ATMOSPHERIC_PRESSURE: Self = Self(31);

    #[doc(alias = "kIOHIDEventTypeForce")]
    pub const FORCE: Self = Self(32);

    #[doc(alias = "kIOHIDEventTypeMotionActivity")]
    pub const MOTION_ACTIVITY: Self = Self(33);

    #[doc(alias = "kIOHIDEventTypeMotionGesture")]
    pub const MOTION_GESTURE: Self = Self(34);

    #[doc(alias = "kIOHIDEventTypeGameController")]
    pub const GAME_CONTROLLER: Self = Self(35);

    #[doc(alias = "kIOHIDEventTypeHumidity")]
    pub const HUMIDITY: Self = Self(36);

    #[doc(alias = "kIOHIDEventTypeCollection")]
    pub const COLLECTION: Self = Self(37);

    #[doc(alias = "kIOHIDEventTypeBrightness")]
    pub const BRIGHTNESS: Self = Self(38);

    #[doc(alias = "kIOHIDEventTypeGenericGesture")]
    pub const GENERIC_GESTURE: Self = Self(39);

    #[doc(alias = "kIOHIDEventTypeDynamicButton")]
    pub const DYNAMIC_BUTTON: Self = Self(40);

    #[doc(alias = "kIOHIDEventTypeForceStage")]
    pub const FORCE_STAGE: Self = Self(41);

    #[doc(alias = "kIOHIDEventTypeTouchSensitiveButton")]
    pub const TOUCH_SENSITIVE_BUTTON: Self = Self(42);

    #[doc(alias = "kIOHIDEventTypeHeartRate")]
    pub const HEART_RATE: Self = Self(43);
}

define_opts!(pub DigitizerEventFlags(u64));

impl DigitizerEventFlags {
    #[doc(alias = "kIOHIDDigitizerEventRange")]
    pub const RANGE: Self = Self(1 << 0);

    #[doc(alias = "kIOHIDDigitizerEventTouch")]
    pub const TOUCH: Self = Self(1 << 1);

    #[doc(alias = "kIOHIDDigitizerEventPosition")]
    pub const POSITION: Self = Self(1 << 2);

    #[doc(alias = "kIOHIDDigitizerEventStop")]
    pub const STOP: Self = Self(1 << 3);

    #[doc(alias = "kIOHIDDigitizerEventPeak")]
    pub const PEAK: Self = Self(1 << 4);

    #[doc(alias = "kIOHIDDigitizerEventIdentity")]
    pub const IDENTITY: Self = Self(1 << 5);

    #[doc(alias = "kIOHIDDigitizerEventAttribute")]
    pub const ATTRIBUTE: Self = Self(1 << 6);

    #[doc(alias = "kIOHIDDigitizerEventCancel")]
    pub const CANCEL: Self = Self(1 << 7);

    #[doc(alias = "kIOHIDDigitizerEventStart")]
    pub const START: Self = Self(1 << 8);

    #[doc(alias = "kIOHIDDigitizerEventResting")]
    pub const RESTING: Self = Self(1 << 9);

    #[doc(alias = "kIOHIDDigitizerEventFromEdgeFlat")]
    pub const FROM_EDGE_FLAT: Self = Self(1 << 10);

    #[doc(alias = "kIOHIDDigitizerEventFromEdgeTip")]
    pub const FROM_EDGE_TIP: Self = Self(1 << 11);

    #[doc(alias = "kIOHIDDigitizerEventFromCorner")]
    pub const FROM_CORNER: Self = Self(1 << 12);

    #[doc(alias = "kIOHIDDigitizerEventSwipePending")]
    pub const SWIPE_PENDING: Self = Self(1 << 13);

    #[doc(alias = "kIOHIDDigitizerEventFromEdgeForcePending")]
    pub const FROM_EDGE_FORCE_PENDING: Self = Self(1 << 14);

    #[doc(alias = "kIOHIDDigitizerEventFromEdgeForceActive")]
    pub const FROM_EDGE_FORCE_ACTIVE: Self = Self(1 << 15);

    #[doc(alias = "kIOHIDDigitizerEventForcePopped")]
    pub const FORCE_POPPED: Self = Self(1 << 16);

    #[doc(alias = "kIOHIDDigitizerEventSwipeUp")]
    pub const SWIPE_UP: Self = Self(1 << 24);

    #[doc(alias = "kIOHIDDigitizerEventSwipeDown")]
    pub const SWIPE_DOWN: Self = Self(1 << 25);

    #[doc(alias = "kIOHIDDigitizerEventSwipeLeft")]
    pub const SWIPE_LEFT: Self = Self(1 << 26);

    #[doc(alias = "kIOHIDDigitizerEventSwipeRight")]
    pub const SWIPE_RIGHT: Self = Self(1 << 27);

    #[doc(alias = "kIOHIDDigitizerEventEstimatedAltitude")]
    pub const ESTIMATED_ALTITUDE: Self = Self(1 << 28);

    #[doc(alias = "kIOHIDDigitizerEventEstimatedAzimuth")]
    pub const ESTIMATED_AZIMUTH: Self = Self(1 << 29);

    #[doc(alias = "kIOHIDDigitizerEventEstimatedPressure")]
    pub const ESTIMATED_PRESSURE: Self = Self(1 << 30);

    #[doc(alias = "kIOHIDDigitizerEventSwipeMask")]
    pub const SWIPE_MASK: Self = Self(0xF << 24);
}
