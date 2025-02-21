use crate::{arc, cl, define_obj_type, ns, objc};

pub type Degrees = std::ffi::c_double;
pub type Accuracy = std::ffi::c_double;
pub type Speed = std::ffi::c_double;
pub type SpeedAccuracy = std::ffi::c_double;
pub type Direction = std::ffi::c_double;
pub type DirectionAccuracy = std::ffi::c_double;
pub type Distance = std::ffi::c_double;

#[doc(alias = "CLLocationCoordinate2D")]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Coordinate2d {
    pub lat: Degrees,
    pub lon: Degrees,
}

impl Coordinate2d {
    #[inline]
    pub fn invalid() -> Self {
        unsafe { kCLLocationCoordinate2DInvalid }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { CLLocationCoordinate2DIsValid(*self) }
    }
}

define_obj_type!(
    #[doc(alias = "CLLocation")]
    pub Location(ns::Id),
    CL_LOCATION
);

unsafe impl Send for Location {}
unsafe impl Sync for Location {}

impl arc::A<Location> {
    #[objc::msg_send(initWithLatitude:longitude:)]
    pub fn init_with_lat_lon(
        self,
        lat: cl::LocationDegrees,
        lon: cl::LocationDegrees,
    ) -> arc::R<Location>;
}

impl Location {
    #[inline]
    pub fn with_lat_lon(lat: cl::LocationDegrees, lon: cl::LocationDegrees) -> arc::R<Self> {
        Self::alloc().init_with_lat_lon(lat, lon)
    }

    #[objc::msg_send(coordinate)]
    pub fn coordinate(&self) -> cl::LocationCoordinate2d;

    #[objc::msg_send(altitude)]
    pub fn altitude(&self) -> cl::LocationDistance;

    #[objc::msg_send(ellipsoidalAltitude)]
    pub fn ellipsoidal_altitude(&self) -> cl::LocationDistance;

    #[objc::msg_send(horizontalAccuracy)]
    pub fn horizontal_accuracy(&self) -> cl::LocationAccuracy;

    #[objc::msg_send(verticalAccuracy)]
    pub fn vertical_accuracy(&self) -> cl::LocationAccuracy;

    #[objc::msg_send(course)]
    pub fn course(&self) -> cl::LocationDirection;

    #[objc::msg_send(courseAccuracy)]
    pub fn course_accuracy(&self) -> cl::LocationDirectionAccuracy;

    #[objc::msg_send(speed)]
    pub fn speed(&self) -> cl::LocationSpeed;

    #[objc::msg_send(speedAccuracy)]
    pub fn speed_accuracy(&self) -> cl::LocationSpeedAccuracy;

    #[objc::msg_send(timestamp)]
    pub fn timestamp(&self) -> arc::R<ns::Date>;

    #[objc::msg_send(floor)]
    pub fn floor(&self) -> Option<arc::R<cl::Floor>>;

    #[objc::msg_send(sourceInformation)]
    pub fn src_info(&self) -> Option<arc::R<cl::LocationSrcInfo>>;

    #[objc::msg_send(distanceFromLocation:)]
    pub fn distance_from_location(&self, location: &cl::Location) -> cl::LocationDistance;
}

#[link(name = "cl", kind = "static")]
unsafe extern "C" {
    static CL_LOCATION: &'static objc::Class<Location>;
}

#[link(name = "CoreLocation", kind = "framework")]
unsafe extern "C" {
    static kCLLocationCoordinate2DInvalid: Coordinate2d;
    fn CLLocationCoordinate2DIsValid(coord: Coordinate2d) -> bool;
}

pub mod accuracy {
    use crate::cl;

    /// The highest possible accuracy that uses additional sensor data to facilitate navigation apps.
    #[inline]
    pub fn best_for_naviation() -> cl::LocationAccuracy {
        unsafe { kCLLocationAccuracyBestForNavigation }
    }

    /// The best level of accuracy available.
    #[inline]
    pub fn best() -> cl::LocationAccuracy {
        unsafe { kCLLocationAccuracyBest }
    }

    /// Accurate to within ten meters of the desired target.
    #[inline]
    pub fn nearest_ten_meters() -> cl::LocationAccuracy {
        unsafe { kCLLocationAccuracyNearestTenMeters }
    }

    /// Accurate to within one hundred meters.
    #[inline]
    pub fn hundred_meters() -> cl::LocationAccuracy {
        unsafe { kCLLocationAccuracyHundredMeters }
    }

    /// Accurate to the nearest kilometer.
    #[inline]
    pub fn kilometer() -> cl::LocationAccuracy {
        unsafe { kCLLocationAccuracyKilometer }
    }

    /// Accurate to the nearest three kilometers.
    #[inline]
    pub fn three_kilometers() -> cl::LocationAccuracy {
        unsafe { kCLLocationAccuracyThreeKilometers }
    }

    /// The level of accuracy used when an app isn’t authorized for full accuracy location data.
    #[inline]
    pub fn reduced() -> cl::LocationAccuracy {
        unsafe { kCLLocationAccuracyReduced }
    }

    #[link(name = "CoreLocation", kind = "framework")]
    unsafe extern "C" {
        static kCLLocationAccuracyBestForNavigation: cl::LocationAccuracy;
        static kCLLocationAccuracyBest: cl::LocationAccuracy;
        static kCLLocationAccuracyNearestTenMeters: cl::LocationAccuracy;
        static kCLLocationAccuracyHundredMeters: cl::LocationAccuracy;
        static kCLLocationAccuracyKilometer: cl::LocationAccuracy;
        static kCLLocationAccuracyThreeKilometers: cl::LocationAccuracy;
        static kCLLocationAccuracyReduced: cl::LocationAccuracy;
    }
}

define_obj_type!(
    #[doc(alias = "CLFloor")]
    pub Floor(ns::Id)
);

impl Floor {
    #[objc::msg_send(level)]
    pub fn level(&self) -> isize;
}

define_obj_type!(
    #[doc(alias = "CLLocationSourceInformation")]
    pub SrcInfo(ns::Id)
);

impl SrcInfo {
    #[objc::msg_send(isSimulatedBySoftware)]
    pub fn is_simulated_by_software(&self) -> bool;

    #[objc::msg_send(isProducedByAccessory)]
    pub fn is_produced_by_accessory(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::cl;

    #[test]
    fn basics() {
        let loc = cl::LocationCoordinate2d::invalid();
        assert!(!loc.is_valid());
    }
}
