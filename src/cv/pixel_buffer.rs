use crate::{
    cf::{self, OptionFlags},
    cv, os,
};

pub type PixelBuffer = cv::ImageBuffer;

impl PixelBuffer {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CVPixelBufferGetTypeID() }
    }

    /// Width in pixels.
    #[inline]
    pub fn get_width(&self) -> usize {
        unsafe { CVPixelBufferGetWidth(self) }
    }

    /// Height in pixels.
    #[inline]
    pub fn get_height(&self) -> usize {
        unsafe { CVPixelBufferGetHeight(self) }
    }

    /// Returns the PixelFormatType of the PixelBuffer.
    #[inline]
    pub fn get_pixel_format_type(&self) -> PixelFormatType {
        unsafe { CVPixelBufferGetPixelFormatType(self) }
    }

    #[inline]
    pub fn get_plane_count(&self) -> usize {
        unsafe { CVPixelBufferGetPlaneCount(self) }
    }

    #[inline]
    pub fn get_plane_width(&self, plane_index: usize) -> usize {
        unsafe { CVPixelBufferGetWidthOfPlane(self, plane_index) }
    }

    #[inline]
    pub fn get_plane_height(&self, plane_index: usize) -> usize {
        unsafe { CVPixelBufferGetHeightOfPlane(self, plane_index) }
    }

    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32_BGRA, None).unwrap();
    ///
    /// assert_eq!(200, pixel_buffer.get_width());
    /// assert_eq!(100, pixel_buffer.get_height());
    /// assert_eq!(cv::PixelFormatType::_32_BGRA, pixel_buffer.get_pixel_format_type());
    /// assert_eq!(0, pixel_buffer.get_plane_count());
    /// assert_eq!(cv::PixelBuffer::type_id(), pixel_buffer.get_type_id());
    ///
    /// ```
    pub fn new<'a>(
        width: usize,
        height: usize,
        pixel_format_type: cv::PixelFormatType,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
    ) -> Result<cf::Retained<'a, PixelBuffer>, cv::Return> {
        let mut pixel_buffer_out = None;

        let r = Self::create(
            None,
            width,
            height,
            pixel_format_type,
            pixel_buffer_attributes,
            &mut pixel_buffer_out,
        );

        unsafe { r.to_result(pixel_buffer_out) }
    }

    pub fn create(
        allocator: Option<&cf::Allocator>,
        width: usize,
        height: usize,
        pixel_format_type: cv::PixelFormatType,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: &mut Option<cf::Retained<PixelBuffer>>,
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

    pub unsafe fn lock_base_address(&self, flags: LockFlags) -> cv::Return {
        CVPixelBufferLockBaseAddress(self, flags)
    }

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
    fn drop(&mut self) {
        let res = unsafe { self.0.unlock_lock_base_address(self.1) };
        debug_assert!(res.is_ok());
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct LockFlags(pub OptionFlags);

impl LockFlags {
    pub const DEFAULT: Self = Self(0);
    pub const READ_ONLY: Self = Self(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PixelFormatType(pub os::Type);

// https://developer.apple.com/documentation/technotes/tn3121-selecting-a-pixel-format-for-an-avcapturevideodataoutput

impl PixelFormatType {
    pub const _1_MONOCHROME: Self = Self(0x00000001); /* 1 bit indexed */
    pub const _32_BGRA: Self = Self(os::Type::from_be_bytes(*b"BGRA"));
    pub const _420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE: Self = Self(os::Type::from_be_bytes(*b"420v"));
    pub const _420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE: Self = Self(os::Type::from_be_bytes(*b"420f"));
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

    pub fn from_cf_number(number: &cf::Number) -> Self {
        Self(number.to_i32().unwrap_or(0) as u32)
    }

    pub fn to_description<'a>(&self) -> Option<cf::Retained<'a, cf::Dictionary>> {
        cv::pixel_format_description_create(*self)
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

    //CV_EXPORT CVReturn CVPixelBufferLockBaseAddress( CVPixelBufferRef CV_NONNULL pixelBuffer, CVPixelBufferLockFlags lockFlags ) __OSX_AVAILABLE_STARTING(__MAC_10_4,__IPHONE_4_0);
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
    #[inline]
    pub fn pixel_format_type() -> &'static cf::String {
        unsafe { kCVPixelBufferPixelFormatTypeKey }
    }

    #[inline]
    pub fn width() -> &'static cf::String {
        unsafe { kCVPixelBufferWidthKey }
    }

    #[inline]
    pub fn height() -> &'static cf::String {
        unsafe { kCVPixelBufferHeightKey }
    }

    #[inline]
    pub fn io_surface_properties() -> &'static cf::String {
        unsafe { kCVPixelBufferIOSurfacePropertiesKey }
    }

    pub fn metal_compatability() -> &'static cf::String {
        unsafe { kCVPixelBufferMetalCompatibilityKey }
    }

    extern "C" {
        static kCVPixelBufferPixelFormatTypeKey: &'static cf::String;
        static kCVPixelBufferWidthKey: &'static cf::String;
        static kCVPixelBufferHeightKey: &'static cf::String;
        static kCVPixelBufferIOSurfacePropertiesKey: &'static cf::String;
        static kCVPixelBufferMetalCompatibilityKey: &'static cf::String;
    }
}
