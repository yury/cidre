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

    crate::define_cls!(sym CLBeaconIdentityConstraint);

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[inline]
    fn alloc_if_available() -> Option<arc::A<Self>> {
        Some(Self::alloc())
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
