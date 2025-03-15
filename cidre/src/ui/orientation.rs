use crate::define_opts;

#[doc(alias = "UIDeviceOrientation")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum DeviceOrientation {
    #[doc(alias = "UIDeviceOrientationUnknown")]
    Unknown,

    /// Device oriented vertically, home button on the bottom
    #[doc(alias = "UIDeviceOrientationPortrait")]
    Portrait,

    /// Device oriented vertically, home button on the top
    #[doc(alias = "UIDeviceOrientationPortraitUpsideDown")]
    PortraitUpsideDown,

    /// Device oriented horizontally, home button on the right
    #[doc(alias = "UIDeviceOrientationLandscapeLeft")]
    LandscapeLeft,

    /// Device oriented horizontally, home button on the left
    #[doc(alias = "UIDeviceOrientationLandscapeRight")]
    LandscapeRight,

    /// Device oriented flat, face up
    #[doc(alias = "UIDeviceOrientationFaceUp")]
    FaceUp,

    /// Device oriented flat, face down
    #[doc(alias = "UIDeviceOrientationFaceDown")]
    FaceDown,
}

impl DeviceOrientation {
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

#[doc(alias = "UIInterfaceOrientation")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum Orientation {
    #[doc(alias = "UIInterfaceOrientationUnknown")]
    Unknown = DeviceOrientation::Unknown as isize,

    #[doc(alias = "UIInterfaceOrientationPortrait")]
    Portrait = DeviceOrientation::Portrait as isize,

    #[doc(alias = "UIInterfaceOrientationPortraitUpsideDown")]
    PortraitUpsideDown = DeviceOrientation::PortraitUpsideDown as isize,

    #[doc(alias = "UIInterfaceOrientationLandscapeLeft")]
    LandscapeLeft = DeviceOrientation::LandscapeRight as isize,

    #[doc(alias = "UIInterfaceOrientationLandscapeRight")]
    LandscapeRight = DeviceOrientation::LandscapeLeft as isize,
}

impl Orientation {
    #[inline]
    #[doc(alias = "UIInterfaceOrientationIsPortrait")]
    pub fn is_portrait(&self) -> bool {
        *self == Self::Portrait || *self == Self::PortraitUpsideDown
    }

    #[inline]
    #[doc(alias = "UIInterfaceOrientationIsLandscape")]
    pub fn is_landscape(&self) -> bool {
        *self == Self::LandscapeLeft || *self == Self::LandscapeRight
    }
}

define_opts!(
    #[doc(alias = "UIInterfaceOrientationMask")]
    pub OrientationMask(usize)
);

impl OrientationMask {
    pub const PORTRAIT: Self = Self(1 << Orientation::Portrait as usize);
    pub const LANDSCAPE_LEFT: Self = Self(1 << Orientation::LandscapeLeft as usize);
    pub const LANDSCAPE_RIGHT: Self = Self(1 << Orientation::LandscapeRight as usize);
    pub const PORTRAIT_UPSIDE_DOWN: Self = Self(1 << Orientation::PortraitUpsideDown as usize);
    pub const LANDSCAPE: Self = Self(Self::LANDSCAPE_LEFT.0 | Self::LANDSCAPE_RIGHT.0);
    pub const ALL: Self = Self(
        Self::PORTRAIT.0
            | Self::LANDSCAPE_LEFT.0
            | Self::LANDSCAPE_RIGHT.0
            | Self::PORTRAIT_UPSIDE_DOWN.0,
    );

    pub const ALL_BUT_UPSIDE_DOWN: Self =
        Self(Self::PORTRAIT.0 | Self::LANDSCAPE_LEFT.0 | Self::LANDSCAPE_RIGHT.0);
}
