use crate::{define_obj_type, define_opts, ns};

pub type Confidence = f32;
pub type AspectRatio = f32;
pub type Degrees = f32;

define_obj_type!(pub BarcodeSymbology(ns::String));

impl BarcodeSymbology {
    #[doc(alias = "VNBarcodeSymbologyAztec")]
    pub fn aztec() -> &'static Self {
        unsafe { VNBarcodeSymbologyAztec }
    }

    #[doc(alias = "VNBarcodeSymbologyCode39")]
    pub fn code39() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode39 }
    }

    #[doc(alias = "VNBarcodeSymbologyCode39Checksum")]
    pub fn code39_checksum() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode39Checksum }
    }

    #[doc(alias = "VNBarcodeSymbologyCode39FullASCII")]
    pub fn code39_full_ascii() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode39FullASCII }
    }

    #[doc(alias = "VNBarcodeSymbologyCode39FullASCIIChecksum")]
    pub fn code39_full_ascii_checksum() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode39FullASCIIChecksum }
    }

    #[doc(alias = "VNBarcodeSymbologyCode93")]
    pub fn code93() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode93 }
    }

    #[doc(alias = "VNBarcodeSymbologyCode93i")]
    pub fn code93i() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode93i }
    }

    #[doc(alias = "VNBarcodeSymbologyCode128")]
    pub fn code128() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode128 }
    }

    #[doc(alias = "VNBarcodeSymbologyDataMatrix")]
    pub fn data_matrix() -> &'static Self {
        unsafe { VNBarcodeSymbologyDataMatrix }
    }

    #[doc(alias = "VNBarcodeSymbologyEAN8")]
    pub fn ean8() -> &'static Self {
        unsafe { VNBarcodeSymbologyEAN8 }
    }

    #[doc(alias = "VNBarcodeSymbologyEAN13")]
    pub fn ean13() -> &'static Self {
        unsafe { VNBarcodeSymbologyEAN13 }
    }

    #[doc(alias = "VNBarcodeSymbologyI2of5")]
    pub fn i2of5() -> &'static Self {
        unsafe { VNBarcodeSymbologyI2of5 }
    }

    #[doc(alias = "VNBarcodeSymbologyI2of5Checksum")]
    pub fn i2of5_checksum() -> &'static Self {
        unsafe { VNBarcodeSymbologyI2of5Checksum }
    }

    #[doc(alias = "VNBarcodeSymbologyITF14")]
    pub fn itf14() -> &'static Self {
        unsafe { VNBarcodeSymbologyITF14 }
    }

    #[doc(alias = "VNBarcodeSymbologyPDF417")]
    pub fn pdf417() -> &'static Self {
        unsafe { VNBarcodeSymbologyPDF417 }
    }

    #[doc(alias = "VNBarcodeSymbologyQR")]
    pub fn qr() -> &'static Self {
        unsafe { VNBarcodeSymbologyQR }
    }

    #[doc(alias = "VNBarcodeSymbologyUPCE")]
    pub fn upce() -> &'static Self {
        unsafe { VNBarcodeSymbologyUPCE }
    }

    #[doc(alias = "VNBarcodeSymbologyCodabar")]
    pub fn codebar() -> &'static Self {
        unsafe { VNBarcodeSymbologyCodabar }
    }

    #[doc(alias = "VNBarcodeSymbologyGS1DataBar")]
    pub fn gs1_data_bar() -> &'static Self {
        unsafe { VNBarcodeSymbologyGS1DataBar }
    }

    #[doc(alias = "VNBarcodeSymbologyGS1DataBarExpanded")]
    pub fn gs1_data_bar_extended() -> &'static Self {
        unsafe { VNBarcodeSymbologyGS1DataBarExpanded }
    }

    #[doc(alias = "VNBarcodeSymbologyGS1DataBarLimited")]
    pub fn gs1_data_bar_limited() -> &'static Self {
        unsafe { VNBarcodeSymbologyGS1DataBarLimited }
    }

    #[doc(alias = "VNBarcodeSymbologyMicroPDF417")]
    pub fn micro_pdf417() -> &'static Self {
        unsafe { VNBarcodeSymbologyMicroPDF417 }
    }

    #[doc(alias = "VNBarcodeSymbologyMicroQR")]
    pub fn micro_qr() -> &'static Self {
        unsafe { VNBarcodeSymbologyMicroQR }
    }
}

#[link(name = "CoreImage", kind = "framework")]
unsafe extern "C" {
    static VNBarcodeSymbologyAztec: &'static BarcodeSymbology;
    static VNBarcodeSymbologyCode39: &'static BarcodeSymbology;
    static VNBarcodeSymbologyCode39Checksum: &'static BarcodeSymbology;
    static VNBarcodeSymbologyCode39FullASCII: &'static BarcodeSymbology;
    static VNBarcodeSymbologyCode39FullASCIIChecksum: &'static BarcodeSymbology;
    static VNBarcodeSymbologyCode93: &'static BarcodeSymbology;
    static VNBarcodeSymbologyCode93i: &'static BarcodeSymbology;
    static VNBarcodeSymbologyCode128: &'static BarcodeSymbology;
    static VNBarcodeSymbologyDataMatrix: &'static BarcodeSymbology;
    static VNBarcodeSymbologyEAN8: &'static BarcodeSymbology;
    static VNBarcodeSymbologyEAN13: &'static BarcodeSymbology;
    static VNBarcodeSymbologyI2of5: &'static BarcodeSymbology;
    static VNBarcodeSymbologyI2of5Checksum: &'static BarcodeSymbology;
    static VNBarcodeSymbologyITF14: &'static BarcodeSymbology;
    static VNBarcodeSymbologyPDF417: &'static BarcodeSymbology;
    static VNBarcodeSymbologyQR: &'static BarcodeSymbology;
    static VNBarcodeSymbologyUPCE: &'static BarcodeSymbology;
    static VNBarcodeSymbologyCodabar: &'static BarcodeSymbology;
    static VNBarcodeSymbologyGS1DataBar: &'static BarcodeSymbology;
    static VNBarcodeSymbologyGS1DataBarExpanded: &'static BarcodeSymbology;
    static VNBarcodeSymbologyGS1DataBarLimited: &'static BarcodeSymbology;
    static VNBarcodeSymbologyMicroPDF417: &'static BarcodeSymbology;
    static VNBarcodeSymbologyMicroQR: &'static BarcodeSymbology;
}

define_opts!(pub ImageCropAndScaleOpt(usize));

impl ImageCropAndScaleOpt {
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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum ElementType {
    Unknown = 0,
    F32 = 1,
    F64 = 2,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(isize)]
pub enum Chirality {
    Unknonw = 0,
    Left = -1,
    Right = 1,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(isize)]
pub enum PointsClassification {
    Disconnected = 0,
    OpenPath,
    ClosedPath,
}
