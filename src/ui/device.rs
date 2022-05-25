use crate::{cf, define_obj_type, msg_send, ns};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum Orientation {
    Unknown,
    /// Device oriented vertically, home button on the bottom
    Portrait,
    /// Device oriented vertically, home button on the top
    PortraitUpsideDown,
    /// Device oriented horizontally, home button on the right
    LandscapeLeft,
    /// Device oriented horizontally, home button on the left
    LandscapeRight,
    /// Device oriented flat, face up
    FaceUp,
    /// Device oriented flat, face down
    FaceDown,
}

impl Orientation {
    #[inline]
    pub fn is_portrait(&self) -> bool {
        *self == Self::Portrait || *self == Self::PortraitUpsideDown
    }

    #[inline]
    pub fn is_landscape(&self) -> bool {
        *self == Self::LandscapeLeft || *self == Self::LandscapeRight
    }

    #[inline]
    pub fn is_flat(&self) -> bool {
        *self == Self::FaceUp || *self == Self::FaceDown
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum BatteryState {
    Unknown,
    /// on battery, discharging
    Unplugged,
    /// plugged in, less than 100%
    Charging,
    /// plugged in, at 100%
    Full,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum UserInterfaceIdiom {
    Unspecified = -1,
    /// iPhone and iPod touch style UI
    Phone,
    /// iPad style UI
    Pad,
    /// Apple TV style UI
    TV,
    /// CarPlay style UI
    CarPlay,
    /// Optimized for Mac UI
    Mac = 5,
}

pub mod notifications {
    use crate::cf;

    #[inline]
    pub fn orientation_did_change() -> &'static cf::NotificationName {
        unsafe { UIDeviceOrientationDidChangeNotification }
    }

    #[inline]
    pub fn battery_state_did_change() -> &'static cf::NotificationName {
        unsafe { UIDeviceBatteryStateDidChangeNotification }
    }

    #[inline]
    pub fn battery_level_did_change() -> &'static cf::NotificationName {
        unsafe { UIDeviceBatteryLevelDidChangeNotification }
    }

    #[inline]
    pub fn proximity_state_did_change() -> &'static cf::NotificationName {
        unsafe { UIDeviceProximityStateDidChangeNotification }
    }

    #[link(name = "UIKit", kind = "framework")]
    extern "C" {
        static UIDeviceOrientationDidChangeNotification: &'static cf::NotificationName;
        static UIDeviceBatteryStateDidChangeNotification: &'static cf::NotificationName;
        static UIDeviceBatteryLevelDidChangeNotification: &'static cf::NotificationName;
        static UIDeviceProximityStateDidChangeNotification: &'static cf::NotificationName;
    }
}

define_obj_type!(Device(ns::Id));

impl Device {
    #[inline]
    pub fn user_interface_idiom(&self) -> UserInterfaceIdiom {
        unsafe { rsel_userInterfaceIdiom(self) }
    }

    #[inline]
    pub fn is_multitasking_supported(&self) -> bool {
        unsafe { rsel_isMultitaskingSupported(self) }
    }

    #[inline]
    pub fn proximity_state(&self) -> bool {
        unsafe { rsel_proximityState(self) }
    }

    #[inline]
    pub fn is_proximity_monitoring_enabled(&self) -> bool {
        unsafe { rsel_isProximityMonitoringEnabled(self) }
    }

    #[inline]
    pub fn set_proximity_monitoring_enabled(&self, value: bool) {
        unsafe { wsel_setProximityMonitoringEnabled(self, value) }
    }

    #[inline]
    pub fn battery_level(&self) -> f32 {
        unsafe { rsel_batteryLevel(self) }
    }

    #[inline]
    pub fn battery_state(&self) -> BatteryState {
        unsafe { rsel_batteryState(self) }
    }

    #[inline]
    pub fn is_battery_monitoring_enabled(&self) -> bool {
        unsafe { rsel_isBatteryMonitoringEnabled(self) }
    }

    #[inline]
    pub fn set_battery_monitoring_enabled(&self, value: bool) {
        unsafe { wsel_setBatteryMonitoringEnabled(self, value) }
    }

    #[inline]
    pub fn identifier_for_vendor(&self) -> Option<&cf::UUID> {
        unsafe { rsel_identifierForVendor(self) }
    }

    #[inline]
    pub fn model(&self) -> &cf::String {
        unsafe { rsel_model(self) }
    }

    #[inline]
    pub fn system_name(&self) -> &cf::String {
        unsafe { rsel_UIDevice_systemName(self) }
    }

    #[inline]
    pub fn name(&self) -> &cf::String {
        msg_send!("common", self, sel_name)
    }

    #[inline]
    pub fn system_version(&self) -> &cf::String {
        unsafe { rsel_UIDevice_systemVersion(self) }
    }

    /// Returns current device orientation. This will return Orientation::Unknown
    /// unless device orientation notifications are being generated.
    #[inline]
    pub fn orientation(&self) -> Orientation {
        unsafe { rsel_orientation(self) }
    }
}

#[link(name = "ui", kind = "static")]
extern "C" {
    fn rsel_orientation(device: &Device) -> Orientation;
    fn rsel_UIDevice_systemVersion(device: &Device) -> &cf::String;
    fn rsel_model(device: &Device) -> &cf::String;
    fn rsel_UIDevice_systemName(device: &Device) -> &cf::String;
    fn rsel_userInterfaceIdiom(device: &Device) -> UserInterfaceIdiom;
    fn rsel_isMultitaskingSupported(device: &Device) -> bool;
    fn rsel_proximityState(device: &Device) -> bool;
    fn rsel_isProximityMonitoringEnabled(device: &Device) -> bool;
    fn wsel_setProximityMonitoringEnabled(device: &Device, value: bool);
    fn rsel_batteryLevel(device: &Device) -> f32;
    fn rsel_batteryState(device: &Device) -> BatteryState;
    fn rsel_isBatteryMonitoringEnabled(device: &Device) -> bool;
    fn wsel_setBatteryMonitoringEnabled(device: &Device, value: bool);
    fn rsel_identifierForVendor(device: &Device) -> Option<&cf::UUID>;
}
