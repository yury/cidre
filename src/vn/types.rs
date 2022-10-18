use crate::{cf, define_cf_type, define_options};

pub type Confidence = f32;
pub type AspectRation = f32;
pub type Degrees = f32;

define_cf_type!(BarcodeSymbology(cf::String));

impl BarcodeSymbology {
    pub fn aztec() -> &'static Self {
        unsafe { VNBarcodeSymbologyAztec }
    }

    pub fn code39() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode39 }
    }

    pub fn code39_checksum() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode39Checksum }
    }

    pub fn code39_full_ascii() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode39FullASCII }
    }

    pub fn code39_full_ascii_checksum() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode39FullASCIIChecksum }
    }

    pub fn code93() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode93 }
    }

    pub fn code93i() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode93i }
    }

    pub fn code128() -> &'static Self {
        unsafe { VNBarcodeSymbologyCode128 }
    }

    pub fn data_matrix() -> &'static Self {
        unsafe { VNBarcodeSymbologyDataMatrix }
    }

    pub fn ean8() -> &'static Self {
        unsafe { VNBarcodeSymbologyEAN8 }
    }

    pub fn ean13() -> &'static Self {
        unsafe { VNBarcodeSymbologyEAN13 }
    }

    pub fn i2of5() -> &'static Self {
        unsafe { VNBarcodeSymbologyI2of5 }
    }

    pub fn i2of5_checksum() -> &'static Self {
        unsafe { VNBarcodeSymbologyI2of5Checksum }
    }

    pub fn itf14() -> &'static Self {
        unsafe { VNBarcodeSymbologyITF14 }
    }

    pub fn pdf417() -> &'static Self {
        unsafe { VNBarcodeSymbologyPDF417 }
    }

    pub fn qr() -> &'static Self {
        unsafe { VNBarcodeSymbologyQR }
    }

    pub fn upce() -> &'static Self {
        unsafe { VNBarcodeSymbologyUPCE }
    }

    pub fn codebar() -> &'static Self {
        unsafe { VNBarcodeSymbologyCodabar }
    }

    pub fn gs1_data_bar() -> &'static Self {
        unsafe { VNBarcodeSymbologyGS1DataBar }
    }

    pub fn gs1_data_bar_extended() -> &'static Self {
        unsafe { VNBarcodeSymbologyGS1DataBarExpanded }
    }

    pub fn gs1_data_bar_limited() -> &'static Self {
        unsafe { VNBarcodeSymbologyGS1DataBarLimited }
    }

    pub fn micro_pdf417() -> &'static Self {
        unsafe { VNBarcodeSymbologyMicroPDF417 }
    }

    pub fn micro_qr() -> &'static Self {
        unsafe { VNBarcodeSymbologyMicroQR }
    }
}

#[link(name = "CoreImage", kind = "framework")]
extern "C" {
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
