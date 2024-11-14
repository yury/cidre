use crate::{arc, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "NSTouchPhase")]
    pub TouchPhase(usize)
);

impl TouchPhase {
    #[doc(alias = "NSTouchPhaseBegan")]
    pub const BEGAN: Self = Self(1usize << 0);

    #[doc(alias = "NSTouchPhaseMoved")]
    pub const MOVED: Self = Self(1usize << 1);

    #[doc(alias = "NSTouchPhaseStationary")]
    pub const STATIONARY: Self = Self(1usize << 2);

    #[doc(alias = "NSTouchPhaseEnded")]
    pub const ENDED: Self = Self(1usize << 3);

    #[doc(alias = "NSTouchPhaseCancelled")]
    pub const CANCELLED: Self = Self(1usize << 4);

    #[doc(alias = "NSTouchPhaseTouching")]
    pub const TOUCHING: Self = Self(Self::BEGAN.0 | Self::MOVED.0 | Self::STATIONARY.0);

    #[doc(alias = "NSTouchPhaseAny")]
    pub const ANY: Self = Self(usize::MAX);
}

#[doc(alias = "NSTouchType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TouchType {
    /// A direct touch from a finger (on a screen)
    #[doc(alias = "NSTouchTypeDirect")]
    Direct,

    /// An indirect touch (not a screen)
    #[doc(alias = "NSTouchTypeIndirect")]
    Indirect,
}

impl TouchType {
    #[doc(alias = "NSTouchTypeMaskFromType")]
    pub const fn as_mask(self) -> TouchTypeMask {
        TouchTypeMask(1usize << self as usize)
    }
}

define_opts!(
    #[doc(alias = "NSTouchTypeMask")]
    pub TouchTypeMask(usize)
);

impl TouchTypeMask {
    pub const DIRECT: Self = TouchType::Direct.as_mask();
    pub const INDIRECT: Self = TouchType::Indirect.as_mask();
}

define_obj_type!(
    #[doc(alias = "NSTouch")]
    pub Touch(ns::Id)
);

unsafe impl Send for Touch {}

impl Touch {
    #[objc::msg_send(identity)]
    pub fn identity(&self) -> arc::R<ns::Id>;

    #[objc::msg_send(phase)]
    pub fn phase(&self) -> ns::TouchPhase;

    #[objc::msg_send(normalizedPosition)]
    pub fn normalized_position(&self) -> ns::Point;

    #[objc::msg_send(isResting)]
    pub fn is_resting(&self) -> bool;

    #[objc::msg_send(device)]
    pub fn device(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(deviceSize)]
    pub fn device_size(&self) -> ns::Size;
}
