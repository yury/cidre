use crate::define_options;

pub type Confidence = f32;
pub type AspectRation = f32;
pub type Degrees = f32;

define_options!(ImageCropAndScaleOption(usize));

impl ImageCropAndScaleOption {
    /// scale image maintaining aspect ratio to fit on the short side and crop centered on the long side
    pub const CENTER_CROP: Self = Self(0);

    /// scale to size required by algorithm while maintaining the original aspect ratio
    pub const SCALE_FIT: Self = Self(1);

    pub const SCALE_FILL: Self = Self(2);

    /// scale image maintaining aspect ratio to fit on the long side but also rotate by 90 degrees
    /// counter clockwise to optimize portrait images to fit into landscape buffers for algorithms
    /// that are rotation agnostic
    pub const SCALE_FIT_ROTATE90_CCW: Self = Self(Self::SCALE_FIT.0 + 0x100);

    /// scale image and rotate by 90 degrees counter clockwise to optimize portrait images to fill
    /// into landscape buffers for algorithms that are rotation agnostic
    pub const SCALE_FILL_ROTATE90_CCW: Self = Self(Self::SCALE_FILL.0 + 0x100);
}

#[repr(usize)]
pub enum ElementType {
    Unknown = 0,
    Float = 1,
    Double = 2,
}

#[repr(isize)]
pub enum Chirality {
    Unknonw = 0,
    Left = -1,
    Right = 1,
}

#[repr(isize)]
pub enum PointsClassification {
    Disconnected = 0,
    OpenPath,
    ClosedPath,
}
