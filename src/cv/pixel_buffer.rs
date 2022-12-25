use crate::{cf, cv, define_options, os};

pub type PixelBuffer = cv::ImageBuffer;

impl PixelBuffer {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CVPixelBufferGetTypeID() }
    }

    /// Width in pixels.
    #[doc(alias = "CVPixelBufferGetWidth")]
    #[inline]
    pub fn width(&self) -> usize {
        unsafe { CVPixelBufferGetWidth(self) }
    }

    /// Height in pixels.
    #[doc(alias = "CVPixelBufferGetHeight")]
    #[inline]
    pub fn height(&self) -> usize {
        unsafe { CVPixelBufferGetHeight(self) }
    }

    /// Returns the PixelFormatType of the PixelBuffer.
    #[doc(alias = "CVPixelBufferGetPixelFormatType")]
    #[inline]
    pub fn pixel_format_type(&self) -> PixelFormatType {
        unsafe { CVPixelBufferGetPixelFormatType(self) }
    }

    #[doc(alias = "CVPixelBufferGetPlaneCount")]
    #[inline]
    pub fn plane_count(&self) -> usize {
        unsafe { CVPixelBufferGetPlaneCount(self) }
    }

    #[doc(alias = "CVPixelBufferGetWidthOfPlane")]
    #[inline]
    pub fn plane_width(&self, plane_index: usize) -> usize {
        unsafe { CVPixelBufferGetWidthOfPlane(self, plane_index) }
    }

    #[doc(alias = "CVPixelBufferGetHeightOfPlane")]
    #[inline]
    pub fn plane_height(&self, plane_index: usize) -> usize {
        unsafe { CVPixelBufferGetHeightOfPlane(self, plane_index) }
    }

    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32_BGRA, None).unwrap();
    ///
    /// assert_eq!(200, pixel_buffer.width());
    /// assert_eq!(100, pixel_buffer.height());
    /// assert_eq!(cv::PixelFormatType::_32_BGRA, pixel_buffer.pixel_format_type());
    /// assert_eq!(0, pixel_buffer.plane_count());
    /// assert_eq!(cv::PixelBuffer::type_id(), pixel_buffer.get_type_id());
    ///
    /// ```
    pub fn new(
        width: usize,
        height: usize,
        pixel_format_type: cv::PixelFormatType,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
    ) -> Result<cf::Retained<PixelBuffer>, cv::Return> {
        let mut pixel_buffer_out = None;

        let r = Self::create_in(
            width,
            height,
            pixel_format_type,
            pixel_buffer_attributes,
            &mut pixel_buffer_out,
            None,
        );

        unsafe { r.to_result_unchecked(pixel_buffer_out) }
    }

    pub fn create_in(
        width: usize,
        height: usize,
        pixel_format_type: cv::PixelFormatType,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: &mut Option<cf::Retained<PixelBuffer>>,
        allocator: Option<&cf::Allocator>,
    ) -> cv::Return {
        unsafe {
            CVPixelBufferCreate(
                allocator,
                width,
                height,
                pixel_format_type,
                pixel_buffer_attributes,
                pixel_buffer_out,
            )
        }
    }

    #[doc(alias = "CVPixelBufferLockBaseAddress")]
    #[inline]
    pub unsafe fn lock_base_address(&self, flags: LockFlags) -> cv::Return {
        CVPixelBufferLockBaseAddress(self, flags)
    }

    #[doc(alias = "CVPixelBufferUnlockBaseAddress")]
    #[inline]
    pub unsafe fn unlock_lock_base_address(&self, flags: LockFlags) -> cv::Return {
        CVPixelBufferUnlockBaseAddress(self, flags)
    }

    pub fn base_address_lock(&self, flags: LockFlags) -> Result<BaseAddressLockGuard, cv::Return> {
        unsafe {
            let res = self.lock_base_address(flags);
            if res.is_ok() {
                Ok(BaseAddressLockGuard(self, flags))
            } else {
                Err(res)
            }
        }
    }
}

pub struct BaseAddressLockGuard<'a>(&'a PixelBuffer, LockFlags);

impl<'a> Drop for BaseAddressLockGuard<'a> {
    #[inline]
    fn drop(&mut self) {
        let res = unsafe { self.0.unlock_lock_base_address(self.1) };
        debug_assert!(res.is_ok());
    }
}

// #[derive(Clone, Copy, PartialEq, Eq)]
// #[repr(transparent)]
// pub struct LockFlags(pub cv::OptionFlags);

define_options!(LockFlags(cv::OptionFlags));

impl LockFlags {
    pub const DEFAULT: Self = Self(0);
    pub const READ_ONLY: Self = Self(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PixelFormatType(pub os::Type);

// https://developer.apple.com/documentation/technotes/tn3121-selecting-a-pixel-format-for-an-avcapturevideodataoutput

impl PixelFormatType {
    #[doc(alias = "kCVPixelFormatType_1Monochrome")]
    pub const _1_MONOCHROME: Self = Self(0x00000001); /* 1 bit indexed */

    #[doc(alias = "kCMPixelFormat_32BGRA")]
    #[doc(alias = "kCVPixelFormatType_32BGRA")]
    pub const _32_BGRA: Self = Self(os::Type::from_be_bytes(*b"BGRA"));

    pub const _420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE: Self = Self(os::Type::from_be_bytes(*b"420v"));
    pub const _420V: Self = Self(os::Type::from_be_bytes(*b"420v"));
    // TODO: how we can optimize that agly long consts?
    pub const _420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE: Self = Self(os::Type::from_be_bytes(*b"420f"));
    pub const _420F: Self = Self(os::Type::from_be_bytes(*b"420f"));
    pub const LOSSY_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-8v0"));
    pub const LOSSY_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-8f0"));
    pub const _420_YP_CB_CR_10_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"x420"));
    pub const LOSSY_420_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-xv0"));
    pub const LOSSY_422_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-xv2"));

    pub const ARGB_2101010_LE_PACKED: Self = Self(os::Type::from_be_bytes(*b"l10r")); /* little-endian ARGB2101010 full-range ARGB */

    /// kCVPixelFormatType_OneComponent8
    pub const ONE_COMPONENT_8: Self = Self(os::Type::from_be_bytes(*b"L008"));

    /// kCVPixelFormatType_OneComponent16Half  = 'L00h'
    pub const ONE_COMPONENT_16_HALF: Self = Self(os::Type::from_be_bytes(*b"L00h"));

    ///  kCVPixelFormatType_OneComponent32Float = 'L00f'
    pub const ONE_COMPONENT_32_FLOAT: Self = Self(os::Type::from_be_bytes(*b"L00f"));

    pub fn from_cf_number(number: &cf::Number) -> Self {
        Self(number.to_i32().unwrap_or(0) as u32)
    }

    pub fn to_description(&self) -> Option<cf::Retained<cf::Dictionary>> {
        cv::pixel_format_description_create(*self)
    }

    #[inline]
    pub fn to_cf_number(&self) -> cf::Retained<cf::Number> {
        cf::Number::from_i32(self.0 as _)
    }
}

extern "C" {
    fn CVPixelBufferGetTypeID() -> cf::TypeId;
    fn CVPixelBufferCreate(
        allocator: Option<&cf::Allocator>,
        width: usize,
        height: usize,
        pixel_format_type: cv::PixelFormatType,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: &mut Option<cf::Retained<PixelBuffer>>,
    ) -> cv::Return;

    fn CVPixelBufferGetWidth(pixel_buffer: &PixelBuffer) -> usize;
    fn CVPixelBufferGetHeight(pixel_buffer: &PixelBuffer) -> usize;
    fn CVPixelBufferGetPixelFormatType(pixel_buffer: &PixelBuffer) -> PixelFormatType;
    fn CVPixelBufferGetPlaneCount(pixel_buffer: &PixelBuffer) -> usize;
    fn CVPixelBufferGetWidthOfPlane(pixel_buffer: &PixelBuffer, plane_index: usize) -> usize;
    fn CVPixelBufferGetHeightOfPlane(pixel_buffer: &PixelBuffer, plane_index: usize) -> usize;

    fn CVPixelBufferLockBaseAddress(
        pixel_buffer: &PixelBuffer,
        lock_flags: LockFlags,
    ) -> cv::Return;
    fn CVPixelBufferUnlockBaseAddress(
        pixel_buffer: &PixelBuffer,
        lock_flags: LockFlags,
    ) -> cv::Return;
}

pub mod keys {
    use crate::cf;

    /// A single cf::Number or a cf::Array of cf::Numbers (os::Types)
    #[doc(alias = "kCVPixelBufferPixelFormatTypeKey")]
    #[inline]
    pub fn pixel_format_type() -> &'static cf::String {
        unsafe { kCVPixelBufferPixelFormatTypeKey }
    }

    #[doc(alias = "kCVPixelBufferWidthKey")]
    #[inline]
    pub fn width() -> &'static cf::String {
        unsafe { kCVPixelBufferWidthKey }
    }

    #[doc(alias = "kCVPixelBufferHeightKey")]
    #[inline]
    pub fn height() -> &'static cf::String {
        unsafe { kCVPixelBufferHeightKey }
    }

    #[doc(alias = "kCVPixelBufferIOSurfacePropertiesKey")]
    #[inline]
    pub fn io_surface_properties() -> &'static cf::String {
        unsafe { kCVPixelBufferIOSurfacePropertiesKey }
    }

    #[doc(alias = "kCVPixelBufferMetalCompatibilityKey")]
    #[inline]
    pub fn metal_compatability() -> &'static cf::String {
        unsafe { kCVPixelBufferMetalCompatibilityKey }
    }

    #[doc(alias = "kCVPixelBufferPlaneAlignmentKey")]
    #[inline]
    pub fn plane_aligment() -> &'static cf::String {
        unsafe { kCVPixelBufferPlaneAlignmentKey }
    }

    #[doc(alias = "kCVPixelBufferExtendedPixelsBottomKey")]
    #[inline]
    pub fn extended_pixels_bottom() -> &'static cf::String {
        unsafe { kCVPixelBufferExtendedPixelsBottomKey }
    }

    #[doc(alias = "kCVPixelBufferCGImageCompatibilityKey")]
    #[inline]
    pub fn cg_image_comaptibility() -> &'static cf::String {
        unsafe { kCVPixelBufferCGImageCompatibilityKey }
    }

    extern "C" {
        static kCVPixelBufferPixelFormatTypeKey: &'static cf::String;
        static kCVPixelBufferWidthKey: &'static cf::String;
        static kCVPixelBufferHeightKey: &'static cf::String;
        static kCVPixelBufferIOSurfacePropertiesKey: &'static cf::String;
        static kCVPixelBufferMetalCompatibilityKey: &'static cf::String;
        static kCVPixelBufferPlaneAlignmentKey: &'static cf::String;
        static kCVPixelBufferExtendedPixelsBottomKey: &'static cf::String;
        static kCVPixelBufferCGImageCompatibilityKey: &'static cf::String;
    }
}
