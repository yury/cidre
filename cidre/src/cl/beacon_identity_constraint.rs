use crate::{arc, cl, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CLBeaconIdentityConstraint")]
    pub BeaconIdentityConstraint(cl::BeaconIdentityCondition)
);

impl BeaconIdentityConstraint {
    #[objc::init(initWithUUID:)]
    pub fn init_with_uuid(self, uuid: &ns::Uuid) -> arc::R<BeaconIdentityConstraint>;

    #[objc::init(initWithUUID:major:)]
    pub fn init_with_uuid_major(
        self,
        uuid: &ns::Uuid,
        major: cl::BeaconMajorValue,
    ) -> arc::R<BeaconIdentityConstraint>;

    #[objc::init(initWithUUID:major:minor:)]
    pub fn init_with_uuid_major_minor(
        self,
        uuid: &ns::Uuid,
        major: cl::BeaconMajorValue,
        minor: cl::BeaconMinorValue,
    ) -> arc::R<BeaconIdentityConstraint>;

    #[objc::available(macos = 10.15, ios = 13.0)]
    crate::define_cls!(CL_BEACON_IDENTITY_CONSTRAINT);

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[inline]
    fn alloc_if_available() -> Option<arc::A<Self>> {
        #[cfg(any(
            all(target_os = "macos", feature = "macos_10_15"),
            all(target_os = "ios", feature = "ios_13_0")
        ))]
        {
            Some(Self::alloc())
        }

        #[cfg(not(any(
            all(target_os = "macos", feature = "macos_10_15"),
            all(target_os = "ios", feature = "ios_13_0")
        )))]
        Self::alloc()
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[inline]
    pub fn with_uuid(uuid: &ns::Uuid) -> Option<arc::R<Self>> {
        Self::alloc_if_available().map(|obj| obj.init_with_uuid(uuid))
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[inline]
    pub fn with_uuid_major(uuid: &ns::Uuid, major: cl::BeaconMajorValue) -> Option<arc::R<Self>> {
        Self::alloc_if_available().map(|obj| obj.init_with_uuid_major(uuid, major))
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[inline]
    pub fn with_uuid_major_minor(
        uuid: &ns::Uuid,
        major: cl::BeaconMajorValue,
        minor: cl::BeaconMinorValue,
    ) -> Option<arc::R<Self>> {
        Self::alloc_if_available().map(|obj| obj.init_with_uuid_major_minor(uuid, major, minor))
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe extern "C" {
    static CL_BEACON_IDENTITY_CONSTRAINT: &'static objc::Class<BeaconIdentityConstraint>;
}
