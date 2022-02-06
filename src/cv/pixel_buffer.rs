use crate::{cf, cv, os};

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
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32BGRA, None).unwrap();
    ///
    /// assert_eq!(200, pixel_buffer.get_width());
    /// assert_eq!(100, pixel_buffer.get_height());
    /// assert_eq!(cv::PixelFormatType::_32BGRA, pixel_buffer.get_pixel_format_type());
    /// assert_eq!(0, pixel_buffer.get_plane_count());
    /// assert_eq!(cv::PixelBuffer::type_id(), pixel_buffer.get_type_id());
    /// 
    /// let display_size = pixel_buffer.get_display_size();
    /// assert_eq!(cg::Size { width: 200.0, height: 100.0}, display_size);
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PixelFormatType(os::Type);

impl PixelFormatType {
    pub const _1_MONOCHROME: Self = Self(0x00000001); /* 1 bit indexed */
    pub const _32BGRA: Self = Self(os::Type::from_be_bytes(*b"BGRA"));
    pub const _420_YP_CB_CR8_BI_PLANAR_VIDEO_RANGE: Self = Self(os::Type::from_be_bytes(*b"420v"));
    pub const _420_YP_CB_CR8_BI_PLANAR_FULL_RANGE: Self = Self(os::Type::from_be_bytes(*b"420f"));
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
}

pub mod buffer_attribute_keys {
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
