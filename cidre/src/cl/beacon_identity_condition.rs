use crate::{arc, cl, define_obj_type, ns, objc};

#[doc(alias = "CLBeaconMajorValue")]
pub type BeaconMajorValue = u16;
#[doc(alias = "CLBeaconMinorValue")]
pub type BeaconMinorValue = u16;

define_obj_type!(
    #[doc(alias = "CLBeaconIdentityCondition")]
    pub BeaconIdentityCondition(cl::Condition)
);

impl arc::A<BeaconIdentityCondition> {
    #[objc::msg_send(initWithUUID:)]
    pub fn init_with_uuid(self, uuid: &ns::Uuid) -> arc::R<BeaconIdentityCondition>;

    #[objc::msg_send(initWithUUID:major:)]
    pub fn init_with_uuid_major(
        self,
        uuid: &ns::Uuid,
        major: BeaconMajorValue,
    ) -> arc::R<BeaconIdentityCondition>;

    #[objc::msg_send(initWithUUID:major:minor:)]
    pub fn init_with_uuid_major_minor(
        self,
        uuid: &ns::Uuid,
        major: BeaconMajorValue,
        minor: BeaconMinorValue,
    ) -> arc::R<BeaconIdentityCondition>;
}

impl BeaconIdentityCondition {
    #[objc::available(macos = 14.0, ios = 17.0)]
    crate::define_cls!(CL_BEACON_IDENTITY_CONDITION);

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[inline]
    fn alloc_if_available() -> Option<arc::A<Self>> {
        #[cfg(any(
            all(target_os = "macos", feature = "macos_14_0"),
            all(target_os = "ios", feature = "ios_17_0")
        ))]
        {
            Some(Self::alloc())
        }

        #[cfg(not(any(
            all(target_os = "macos", feature = "macos_14_0"),
            all(target_os = "ios", feature = "ios_17_0")
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
    pub fn with_uuid_major(uuid: &ns::Uuid, major: BeaconMajorValue) -> Option<arc::R<Self>> {
        Self::alloc_if_available().map(|obj| obj.init_with_uuid_major(uuid, major))
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[inline]
    pub fn with_uuid_major_minor(
        uuid: &ns::Uuid,
        major: BeaconMajorValue,
        minor: BeaconMinorValue,
    ) -> Option<arc::R<Self>> {
        Self::alloc_if_available().map(|obj| obj.init_with_uuid_major_minor(uuid, major, minor))
    }

    #[objc::msg_send(UUID)]
    pub fn uuid(&self) -> arc::R<ns::Uuid>;

    #[objc::msg_send(major)]
    pub fn major(&self) -> Option<arc::R<ns::Number>>;

    #[objc::msg_send(minor)]
    pub fn minor(&self) -> Option<arc::R<ns::Number>>;
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe extern "C" {
    static CL_BEACON_IDENTITY_CONDITION: &'static objc::Class<BeaconIdentityCondition>;
}
