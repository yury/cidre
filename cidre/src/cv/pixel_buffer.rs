use crate::{arc, cf, cv, define_opts, four_cc_to_str, os};

#[cfg(feature = "io")]
use crate::io;

pub type PixelBuf = cv::ImageBuf;

impl PixelBuf {
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

    /// Returns the PixelFormat of the PixelBuf.
    #[doc(alias = "CVPixelBufferGetPixelFormatType")]
    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
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
    /// let pixel_buffer = cv::PixelBuf::new(200, 100, cv::PixelFormat::_32_BGRA, None).unwrap();
    ///
    /// assert_eq!(200, pixel_buffer.width());
    /// assert_eq!(100, pixel_buffer.height());
    /// assert_eq!(cv::PixelFormat::_32_BGRA, pixel_buffer.pixel_format());
    /// assert_eq!(0, pixel_buffer.plane_count());
    /// assert_eq!(cv::PixelBuf::type_id(), pixel_buffer.get_type_id());
    ///
    /// ```
    pub fn new(
        width: usize,
        height: usize,
        pixel_format_type: cv::PixelFormat,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
    ) -> Result<arc::R<PixelBuf>, cv::Return> {
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
        pixel_format_type: cv::PixelFormat,
        pixel_buf_attrs: Option<&cf::Dictionary>,
        pixel_buf_out: *mut Option<arc::R<PixelBuf>>,
        allocator: Option<&cf::Allocator>,
    ) -> cv::Return {
        unsafe {
            CVPixelBufferCreate(
                allocator,
                width,
                height,
                pixel_format_type,
                pixel_buf_attrs,
                pixel_buf_out,
            )
        }
    }

    #[doc(alias = "CVPixelBufferLockBaseAddress")]
    #[inline]
    pub unsafe fn lock_base_addr(&self, flags: LockFlags) -> cv::Return {
        CVPixelBufferLockBaseAddress(self, flags)
    }

    #[doc(alias = "CVPixelBufferUnlockBaseAddress")]
    #[inline]
    pub unsafe fn unlock_lock_base_addr(&self, flags: LockFlags) -> cv::Return {
        CVPixelBufferUnlockBaseAddress(self, flags)
    }

    #[inline]
    pub fn base_address_lock(&self, flags: LockFlags) -> Result<BaseAddrLockGuard, cv::Return> {
        unsafe {
            let res = self.lock_base_addr(flags);
            if res.is_ok() {
                Ok(BaseAddrLockGuard(self, flags))
            } else {
                Err(res)
            }
        }
    }

    #[cfg(feature = "io")]
    #[inline]
    pub fn io_surf(&self) -> Option<&io::Surf> {
        unsafe { CVPixelBufferGetIOSurface(self) }
    }

    #[cfg(feature = "io")]
    #[inline]
    pub fn io_surf_mut(&mut self) -> Option<&mut io::Surf> {
        unsafe { std::mem::transmute(CVPixelBufferGetIOSurface(self)) }
    }

    #[cfg(feature = "io")]
    #[inline]
    pub fn with_io_surf(
        surface: &io::Surf,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
    ) -> Result<arc::R<Self>, cv::Return> {
        Self::with_io_surf_in(surface, pixel_buffer_attributes, None)
    }

    /// Call to create a single cv::PixelBuf for a passed-in 'io::Surf'.
    #[cfg(feature = "io")]
    #[inline]
    pub fn with_io_surf_in(
        surface: &io::Surf,
        pixel_buf_attrs: Option<&cf::Dictionary>,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<Self>, cv::Return> {
        let mut buffer = None;
        unsafe {
            let res =
                CVPixelBufferCreateWithIOSurface(allocator, surface, pixel_buf_attrs, &mut buffer);
            if res.is_ok() {
                Ok(buffer.unwrap_unchecked())
            } else {
                Err(res)
            }
        }
    }
}

pub struct BaseAddrLockGuard<'a>(&'a PixelBuf, LockFlags);

impl<'a> Drop for BaseAddrLockGuard<'a> {
    #[inline]
    fn drop(&mut self) {
        let res = unsafe { self.0.unlock_lock_base_addr(self.1) };
        debug_assert!(res.is_ok());
    }
}

define_opts!(pub LockFlags(cv::OptionFlags));

impl LockFlags {
    pub const DEFAULT: Self = Self(0);
    pub const READ_ONLY: Self = Self(1);
}

/// CoreVideo pixel format type constants.
///
/// CoreVideo does not provide support for all of these formats; this list just defines their names.
#[doc(alias = "CVPixelFormatType")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PixelFormat(pub os::Type);

// https://developer.apple.com/documentation/technotes/tn3121-selecting-a-pixel-format-for-an-avcapturevideodataoutput

impl PixelFormat {
    /// 1 bit indexed
    #[doc(alias = "kCVPixelFormatType_1Monochrome")]
    pub const _1_MONOCHROME: Self = Self(0x00000001);

    /// 2 bit indexed
    #[doc(alias = "kCVPixelFormatType_2Indexed")]
    pub const _2_INDEXED: Self = Self(0x00000002);

    /// 4 bit indexed
    #[doc(alias = "kCVPixelFormatType_4Indexed")]
    pub const _4_INDEXED: Self = Self(0x00000004);

    /// 8 bit indexed
    #[doc(alias = "kCVPixelFormatType_8Indexed")]
    pub const _8_INDEXED: Self = Self(0x00000008);

    /// 1 bit indexed gray, white is zero
    #[doc(alias = "kCVPixelFormatType_1IndexedGray_WhiteIsZero")]
    pub const _1_INDEXED_GREY_WHITE_IS_ZERO: Self = Self(0x000000021);

    /// 2 bit indexed gray, white is zero
    #[doc(alias = "kCVPixelFormatType_2IndexedGray_WhiteIsZero")]
    pub const _2_INDEXED_GREY_WHITE_IS_ZERO: Self = Self(0x000000022);

    /// 4 bit indexed gray, white is zero
    #[doc(alias = "kCVPixelFormatType_4IndexedGray_WhiteIsZero")]
    pub const _4_INDEXED_GREY_WHITE_IS_ZERO: Self = Self(0x000000024);

    /// 8 bit indexed gray, white is zero
    #[doc(alias = "kCMPixelFormat_8IndexedGray_WhiteIsZero")]
    #[doc(alias = "kCVPixelFormatType_8IndexedGray_WhiteIsZero")]
    pub const _8_INDEXED_GREY_WHITE_IS_ZERO: Self = Self(0x000000028);

    /// 16 bit BE RGB 555
    #[doc(alias = "kCVPixelFormatType_16BE555")]
    pub const _16_BE_555: Self = Self(0x00000010);

    /// 16 bit LE RGB 555
    #[doc(alias = "kCVPixelFormatType_16LE555")]
    pub const _16_LE_555: Self = Self(os::Type::from_be_bytes(*b"L555"));

    /// 16 bit LE RGB 5551
    #[doc(alias = "kCVPixelFormatType_16LE5551")]
    pub const _16_LE_5551: Self = Self(os::Type::from_be_bytes(*b"5551"));

    /// 16 bit BE RGB 565
    #[doc(alias = "kCVPixelFormatType_16BE565")]
    pub const _16_BE_565: Self = Self(os::Type::from_be_bytes(*b"B565"));

    /// 16 bit LE RGB 565
    #[doc(alias = "kCVPixelFormatType_16LE565")]
    pub const _16_LE_565: Self = Self(os::Type::from_be_bytes(*b"L565"));

    /// 24 bit RGB
    #[doc(alias = "kCVPixelFormatType_24RGB")]
    pub const _24_RGB: Self = Self(0x00000018);

    /// 24 bit BGR
    #[doc(alias = "kCVPixelFormatType_24BGR")]
    pub const _24_BGR: Self = Self(os::Type::from_be_bytes(*b"24BG"));

    /// 32 bit ARGB
    #[doc(alias = "kCVPixelFormatType_32ARGB")]
    pub const _32_ARGB: Self = Self(0x00000020);

    /// 32 bit BGRA
    #[doc(alias = "kCMPixelFormat_32BGRA")]
    #[doc(alias = "kCVPixelFormatType_32BGRA")]
    pub const _32_BGRA: Self = Self(os::Type::from_be_bytes(*b"BGRA"));

    /// 32 bit ABGR
    #[doc(alias = "kCVPixelFormatType_32ABGR")]
    pub const _32_ABGR: Self = Self(os::Type::from_be_bytes(*b"ABGR"));

    /// 32 bit RGBA
    #[doc(alias = "kCVPixelFormatType_32RGBA")]
    pub const _32_RGBA: Self = Self(os::Type::from_be_bytes(*b"RGBA"));

    /// 64 bit ARGB, 16-bit big-endian samples
    #[doc(alias = "kCVPixelFormatType_64ARGB")]
    pub const _64_ARGB: Self = Self(os::Type::from_be_bytes(*b"b64a"));

    /// 64 bit RGBA, 16-bit little-endian full-range (0-65535) samples
    #[doc(alias = "kCVPixelFormatType_64RGBALE")]
    pub const _64_RGBALE: Self = Self(os::Type::from_be_bytes(*b"l64r"));

    /// 30 bit RGB, 10-bit big-endian samples, 2 unused padding bits (at least significant end).
    #[doc(alias = "kCVPixelFormatType_30RGB")]
    pub const _30_RGB: Self = Self(os::Type::from_be_bytes(*b"R10k"));

    /// 30 bit RGB, 10-bit big-endian samples, 2 unused padding bits (at most significant end), video-range (64-940).
    #[doc(alias = "kCVPixelFormatType_30RGB_r210")]
    pub const _30_RGB_R210: Self = Self(os::Type::from_be_bytes(*b"r210"));

    /// Component Y'CbCr 8-bit 4:2:2, ordered Cb Y'0 Cr Y'1
    #[doc(alias = "kCVPixelFormatType_422YpCbCr8")]
    pub const _422_YP_CB_CR_8: Self = Self(os::Type::from_be_bytes(*b"2vuy"));
    #[doc(alias = "kCVPixelFormatType_422YpCbCr8")]
    pub const _2VUY: Self = Self::_422_YP_CB_CR_8;

    /// Component Y'CbCrA 8-bit 4:4:4:4, ordered Cb Y' Cr A
    #[doc(alias = "kCVPixelFormatType_4444YpCbCrA8")]
    pub const _4444_YP_CB_CR_A_8: Self = Self(os::Type::from_be_bytes(*b"v408"));

    /// Component Y'CbCrA 8-bit 4:4:4:4, rendering format. full range alpha, zero biased YUV, ordered A Y' Cb Cr
    #[doc(alias = "kCVPixelFormatType_4444YpCbCrA8R")]
    pub const _4444_YP_CB_CR_A_8_R: Self = Self(os::Type::from_be_bytes(*b"r408"));

    /// Component Y'CbCrA 8-bit 4:4:4:4, ordered A Y' Cb Cr, full range alpha, video range Y'CbCr.
    #[doc(alias = "kCVPixelFormatType_4444AYpCbCr8")]
    pub const _4444_A_YP_CB_CR_8: Self = Self(os::Type::from_be_bytes(*b"y408"));

    /// Component Y'CbCrA 16-bit 4:4:4:4, ordered A Y' Cb Cr, full range alpha, video range Y'CbCr, 16-bit little-endian samples.
    #[doc(alias = "kCVPixelFormatType_4444AYpCbCr16")]
    pub const _4444_A_YP_CB_CR_16: Self = Self(os::Type::from_be_bytes(*b"y416"));

    /// Component AY'CbCr single precision floating-point 4:4:4:4
    #[doc(alias = "kCVPixelFormatType_4444AYpCbCrFloat")]
    pub const _4444_A_YP_CB_CR_FLOAT: Self = Self(os::Type::from_be_bytes(*b"r4fl"));

    /// Component Y'CbCr 8-bit 4:4:4, ordered Cr Y' Cb, video range Y'CbCr
    #[doc(alias = "kCVPixelFormatType_444YpCbCr8")]
    pub const _444_YP_CB_CR_8: Self = Self(os::Type::from_be_bytes(*b"v308"));

    /// Component Y'CbCr 10,12,14,16-bit 4:2:2
    #[doc(alias = "kCVPixelFormatType_422YpCbCr16")]
    pub const _422_YP_CB_CR_16: Self = Self(os::Type::from_be_bytes(*b"v216"));

    /// Component Y'CbCr 10-bit 4:2:2
    #[doc(alias = "kCVPixelFormatType_422YpCbCr10")]
    pub const _422_YP_CB_CR_10: Self = Self(os::Type::from_be_bytes(*b"v210"));

    /// Component Y'CbCr 10-bit 4:4:4
    #[doc(alias = "kCVPixelFormatType_444YpCbCr10")]
    pub const _444_YP_CB_CR_10: Self = Self(os::Type::from_be_bytes(*b"v410"));

    /// Planar Component Y'CbCr 8-bit 4:2:0, full range.  baseAddr points to a big-endian CVPlanarPixelBufferInfo_YCbCrPlanar struct
    #[doc(alias = "kCVPixelFormatType_420YpCbCr8PlanarFullRange")]
    pub const _420_YP_CB_CR_8_PLANAR_FULL_RANGE: Self = Self(os::Type::from_be_bytes(*b"f420"));

    /// First plane: Video-range Component Y'CbCr 8-bit 4:2:2, ordered Cb Y'0 Cr Y'1; second plane: alpha 8-bit 0-255
    #[doc(alias = "kCVPixelFormatType_422YpCbCr_4A_8BiPlanar")]
    pub const _422_YP_CB_CR_4_A_8_BI_PLANAR: Self = Self(os::Type::from_be_bytes(*b"a2vy"));

    /// Bi-Planar Component Y'CbCr 8-bit 4:2:0, video-range (luma=\[16,235\] chroma=\[16,240\]).
    #[doc(alias = "kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange")]
    pub const _420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE: Self = Self(os::Type::from_be_bytes(*b"420v"));
    #[doc(alias = "kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange")]
    pub const _420V: Self = Self::_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE;

    /// Bi-Planar Component Y'CbCr 8-bit 4:2:0, full-range (luma=\[0,255\] chroma=\[1,255\]).  baseAddr points to a big-endian
    #[doc(alias = "kCVPixelFormatType_420YpCbCr8BiPlanarFullRange")]
    pub const _420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE: Self = Self(os::Type::from_be_bytes(*b"420f"));
    #[doc(alias = "kCVPixelFormatType_420YpCbCr8BiPlanarFullRange")]
    pub const _420F: Self = Self::_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE;

    /// 2 plane YCbCr10 4:2:0, each 10 bits in the MSBs of 16bits, video-range (luma=\[64,940\] chroma=\[64,960\])
    #[doc(alias = "kCVPixelFormatType_420YpCbCr10BiPlanarVideoRange")]
    pub const _420_YP_CB_CR_10_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"x420"));

    /// 2 plane YCbCr10 4:2:2, each 10 bits in the MSBs of 16bits, video-range (luma=\[64,940\] chroma=\[64,960\])
    #[doc(alias = "kCVPixelFormatType_422YpCbCr10BiPlanarVideoRange")]
    pub const _422_YP_CB_CR_10_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"x422"));

    /// 2 plane YCbCr10 4:4:4, each 10 bits in the MSBs of 16bits, video-range (luma=\[64,940\] chroma=\[64,960\])
    #[doc(alias = "kCVPixelFormatType_444YpCbCr10BiPlanarVideoRange")]
    pub const _444_YP_CB_CR_10_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"x444"));

    /// 2 plane YCbCr10 4:2:0, each 10 bits in the MSBs of 16bits, full-range (Y range 0-1023)
    #[doc(alias = "kCVPixelFormatType_420YpCbCr10BiPlanarFullRange")]
    pub const _420_YP_CB_CR_10_BI_PLANAR_FULL_RANGE: Self = Self(os::Type::from_be_bytes(*b"xf20"));

    /// 2 plane YCbCr10 4:2:2, each 10 bits in the MSBs of 16bits, full-range (Y range 0-1023)
    #[doc(alias = "kCVPixelFormatType_422YpCbCr10BiPlanarFullRange")]
    pub const _422_YP_CB_CR_10_BI_PLANAR_FULL_RANGE: Self = Self(os::Type::from_be_bytes(*b"xf22"));

    /// 2 plane YCbCr10 4:4:4, each 10 bits in the MSBs of 16bits, full-range (Y range 0-1023)
    #[doc(alias = "kCVPixelFormatType_444YpCbCr10BiPlanarFullRange")]
    pub const _444_YP_CB_CR_10_BI_PLANAR_FULL_RANGE: Self = Self(os::Type::from_be_bytes(*b"xf44"));

    /// little-endian ARGB2101010 full-range ARGB
    #[doc(alias = "kCVPixelFormatType_ARGB2101010LEPacked")]
    pub const ARGB_2101010_LE_PACKED: Self = Self(os::Type::from_be_bytes(*b"l10r"));

    #[doc(alias = "kCVPixelFormatType_OneComponent8")]
    pub const ONE_COMPONENT_8: Self = Self(os::Type::from_be_bytes(*b"L008"));

    #[doc(alias = "kCVPixelFormatType_OneComponent16Half")]
    pub const ONE_COMPONENT_16_HALF: Self = Self(os::Type::from_be_bytes(*b"L00h"));

    #[doc(alias = "kCVPixelFormatType_OneComponent32Float")]
    pub const ONE_COMPONENT_32_FLOAT: Self = Self(os::Type::from_be_bytes(*b"L00f"));

    /// 16 bit two component IEEE half-precision float, 16-bit little-endian samples
    #[doc(alias = "kCVPixelFormatType_TwoComponent16Half")]
    pub const TWO_COMPONENT_16_HALF: Self = Self(os::Type::from_be_bytes(*b"2C0h"));

    /// 32 bit two component IEEE float, 32-bit little-endian samples
    #[doc(alias = "kCVPixelFormatType_TwoComponent32Float")]
    pub const TWO_COMPONENT_32_FLOAT: Self = Self(os::Type::from_be_bytes(*b"2C0f"));

    /// 64 bit RGBA IEEE half-precision float, 16-bit little-endian samples
    #[doc(alias = "kCVPixelFormatType_64RGBAHalf")]
    pub const _64_RGBA_HALF: Self = Self(os::Type::from_be_bytes(*b"RGhA"));

    /// 128 bit RGBA IEEE float, 32-bit little-endian samples
    #[doc(alias = "kCVPixelFormatType_128RGBAFloat")]
    pub const _128_RGBA_FLOAT: Self = Self(os::Type::from_be_bytes(*b"RGfA"));

    pub fn from_cf_number(number: &cf::Number) -> Self {
        Self(number.to_i32().unwrap_or(0) as u32)
    }

    pub fn to_desc(&self) -> Option<arc::R<cf::Dictionary>> {
        cv::pixel_format_desc_create(*self)
    }

    #[inline]
    pub fn to_cf_number(&self) -> &'static cf::Number {
        cf::Number::from_four_char_code(self.0)
    }

    /// Checks if a compressed pixel format is supported on the current platform.
    #[doc(alias = "CVIsCompressedPixelFormatAvailable")]
    #[inline]
    pub fn is_compressed_avaliable(&self) -> bool {
        unsafe { CVIsCompressedPixelFormatAvailable(*self) }
    }
}

/// Lossless-Compressed Pixel Formats
///
/// The following pixel formats can be used to reduce the memory bandwidth involved
/// in large-scale pixel data flow, which can have benefits for battery life and
/// thermal efficiency.
///
/// They work by dividing pixel buffers into fixed-width, fixed-height, fixed-byte-size
/// blocks.  Hardware units (video codecs, GPU, ISP, etc.) attempt to write a compressed
/// encoding for each block using a lossless algorithm.  If a block of pixels is successfully
/// encoded using fewer bytes than the uncompressed pixel data, the hardware unit does not need
/// to write as many bytes for that pixel block.  If the encoding is unsuccessful,
/// the uncompressed pixel data is written, filling the whole pixel block.  Each compressed
/// pixel buffer has a separate area of metadata recording the encoding choices for each pixel
/// block.
///
/// Padding bits are eliminated, so for example, 10-bit-per-component lossless-compressed pixel
/// buffers are slightly smaller than their uncompressed equivalents. For pixel formats with
/// no padding, the lossless-compressed pixel buffers are slightly larger due to the metadata.
///
/// # Important caveats:
///
/// Some devices do not support these pixel formats at all.
/// Before using one of these pixel formats, call [`Self::is_compressed_avaliable()`] to check that it
/// is available on the current device.
///
/// On different devices, the concrete details of these formats may be different.
///
/// On different devices, the degree and details of support by hardware units (video codecs, GPU, ISP, etc.)
/// may be different.
///
/// Do not ship code that reads the contents of lossless-compressed pixel buffers directly
/// with the CPU, or which saves or transfers it to other devices, as this code will break
/// with future hardware.
///
/// The bandwidth benefits of these formats are generally outweighed by the cost of buffer
/// copies to convert to uncompressed pixel formats, so if you find that you need to perform
/// a buffer copy to covert for CPU usage, it's likely that you would have been better served
/// by using the equivalent uncompressed pixel formats in the first place.
impl PixelFormat {
    /// Lossless-compressed form of 'cv::PixelFormat::_32BGRA'
    #[doc(alias = "kCVPixelFormatType_Lossless_32BGRA")]
    pub const LOSSLESS_32_BGRA: Self = Self(os::Type::from_be_bytes(*b"&BGA"));

    /// Lossless-compressed form of 'cv::PixelFormat::_64_RGBA_HALF'. No CVPlanarPixelBufferInfo struct.
    #[doc(alias = "kCVPixelFormatType_Lossless_64RGBAHalf")]
    pub const LOSSLESS_64_RGBA_HALF: Self = Self(os::Type::from_be_bytes(*b"&RhA"));

    /// Lossless-compressed form of 'cv::PixelFormat::_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE'.
    #[doc(alias = "kCVPixelFormatType_Lossless_420YpCbCr8BiPlanarVideoRange")]
    pub const LOSSLESS_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"&8v0"));
    #[doc(alias = "kCVPixelFormatType_Lossless_420YpCbCr8BiPlanarVideoRange")]
    pub const LOSSLESS_420V: Self = Self::LOSSLESS_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE;

    /// Lossless-compressed form of 'cv::PixelFormat::_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE'
    #[doc(alias = "kCVPixelFormatType_Lossless_420YpCbCr8BiPlanarFullRange")]
    pub const LOSSLESS_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"&8f0"));
    #[doc(alias = "kCVPixelFormatType_Lossless_420YpCbCr8BiPlanarFullRange")]
    pub const LOSSLESS_420F: Self = Self::LOSSLESS_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE;

    /// Lossless-compressed-packed form of 'cv::PixelFormat::_420_YP_CB_CR_10_BI_PLANAR_VIDEO_RANGE'.
    /// Format is compressed-packed with no padding bits between pixels.
    #[doc(alias = "kCVPixelFormatType_Lossless_420YpCbCr10PackedBiPlanarVideoRange")]
    pub const LOSSLESS_420_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"&xv0"));
    /// Lossless-compressed form of 'cv::PixelFormat::_422_YP_CB_CR_10_BI_PLANAR_VIDEO_RANGE'.

    /// Format is compressed-packed with no padding bits between pixels.
    #[doc(alias = "kCVPixelFormatType_Lossless_422YpCbCr10PackedBiPlanarVideoRange")]
    pub const LOSSLESS_422_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"&xv2"));
}

/// Lossy-Compressed Pixel Formats
///
/// The following pixel formats can be used to reduce memory bandwidth and memory footprint
/// involved in large-scale pixel data flow, which can have benefits for battery life
/// and thermal efficiency.
///
/// Similar to lossless pixel formats, they work by dividing pixel buffers into fixed-width,
/// fixed-height, fixed-byte-size blocks. Pixel buffers allocated using lossy formats have
/// reduced memory footprint than their lossless equivalents; this reduced footprint may or
/// may not result in loss of quality depending on the content of the individual block.
/// Hardware units (video codecs, GPU, ISP, etc.) attempt to write a compressed encoding
/// for each block using either a lossless or lossy algorithm. If a block of pixels is
/// successfully encoded within its pre-defined memory footprint, then the lossless alogrithm
/// is applied; if the encoded block of pixels exceeds the pre-defined memory footprint then
/// the lossy algorithm is applied. Each compressed pixel buffer has a separate area of
/// metadata recording the encoding choices for each pixel block.
///
/// Usefull links:
/// - <https://developer.apple.com/documentation/technotes/tn3104-recording-video-in-apple-prores>
/// - <https://developer.apple.com/documentation/technotes/tn3121-selecting-a-pixel-format-for-an-avcapturevideodataoutput>
impl PixelFormat {
    /// Lossy-compressed form of `cv::PixelFormat::_32_BGRA`.
    #[doc(alias = "kCVPixelFormatType_Lossy_32BGRA")]
    pub const LOSSY_32_BGRA: Self = Self(os::Type::from_be_bytes(*b"-BGA"));

    /// Lossy-compressed form of `cv::PixelFormat::_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE`.
    #[doc(alias = "kCVPixelFormatType_Lossy_420YpCbCr8BiPlanarVideoRange")]
    pub const LOSSY_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-8v0"));

    #[doc(alias = "kCVPixelFormatType_Lossy_420YpCbCr8BiPlanarVideoRange")]
    pub const LOSSY_420V: Self = Self::LOSSY_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE;

    /// Lossy-compressed form of `cv::PixelFormat::_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE`.
    #[doc(alias = "kCVPixelFormatType_Lossy_420YpCbCr8BiPlanarFullRange")]
    pub const LOSSY_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-8f0"));

    #[doc(alias = "kCVPixelFormatType_Lossy_420YpCbCr8BiPlanarFullRange")]
    pub const LOSSY_420F: Self = Self::LOSSY_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE;

    /// Lossy-compressed form of `cv::PixelFormat::_420_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE`.
    #[doc(alias = "kCVPixelFormatType_Lossy_420YpCbCr10PackedBiPlanarVideoRange")]
    pub const LOSSY_420_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-xv0"));

    /// Lossy-compressed form of `cv::PixelFormat::_422_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE`.
    /// Format is compressed-packed with no padding bits between pixels.
    #[doc(alias = "kCVPixelFormatType_Lossy_422YpCbCr10PackedBiPlanarVideoRange")]
    pub const LOSSY_422_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-xv2"));
}

impl std::fmt::Debug for PixelFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fcc = self.0.to_be_bytes();
        f.debug_struct("cv::PixelFormat")
            .field("raw", &self.0)
            .field("fcc", &four_cc_to_str(&mut fcc))
            .finish()
    }
}

#[link(name = "CoreVideo", kind = "framework")]
extern "C-unwind" {
    fn CVPixelBufferGetTypeID() -> cf::TypeId;
    fn CVPixelBufferCreate(
        allocator: Option<&cf::Allocator>,
        width: usize,
        height: usize,
        pixel_format_type: PixelFormat,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: *mut Option<arc::R<PixelBuf>>,
    ) -> cv::Return;

    fn CVPixelBufferGetWidth(pixel_buffer: &PixelBuf) -> usize;
    fn CVPixelBufferGetHeight(pixel_buffer: &PixelBuf) -> usize;
    fn CVPixelBufferGetPixelFormatType(pixel_buffer: &PixelBuf) -> PixelFormat;
    fn CVPixelBufferGetPlaneCount(pixel_buffer: &PixelBuf) -> usize;
    fn CVPixelBufferGetWidthOfPlane(pixel_buffer: &PixelBuf, plane_index: usize) -> usize;
    fn CVPixelBufferGetHeightOfPlane(pixel_buffer: &PixelBuf, plane_index: usize) -> usize;

    fn CVPixelBufferLockBaseAddress(pixel_buffer: &PixelBuf, lock_flags: LockFlags) -> cv::Return;
    fn CVPixelBufferUnlockBaseAddress(pixel_buffer: &PixelBuf, lock_flags: LockFlags)
        -> cv::Return;

    #[cfg(feature = "io")]
    fn CVPixelBufferGetIOSurface(pixel_buffer: &PixelBuf) -> Option<&io::Surf>;

    #[cfg(feature = "io")]
    fn CVPixelBufferCreateWithIOSurface(
        allocator: Option<&cf::Allocator>,
        surface: &io::Surf,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: *mut Option<arc::R<cv::PixelBuf>>,
    ) -> cv::Return;

    fn CVIsCompressedPixelFormatAvailable(pixel_format: PixelFormat) -> bool;
}

pub mod keys {
    use crate::cf;

    /// A single cf::Number or a cf::Array of cf::Numbers (os::Types)
    #[doc(alias = "kCVPixelBufferPixelFormatTypeKey")]
    #[inline]
    pub fn pixel_format() -> &'static cf::String {
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
    pub fn io_surf_props() -> &'static cf::String {
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

#[cfg(test)]
mod tests {
    use crate::cv::PixelFormat;

    #[test]
    fn basics() {
        let number = PixelFormat::_1_MONOCHROME.to_cf_number();
        assert!(number.is_tagged_ptr());
        number.show();
    }

    #[test]
    fn compressed() {
        assert!(PixelFormat::LOSSY_32_BGRA.is_compressed_avaliable());
        assert!(PixelFormat::LOSSY_420V.is_compressed_avaliable());
        assert!(PixelFormat::LOSSY_420F.is_compressed_avaliable());
        assert!(
            PixelFormat::LOSSY_420_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE
                .is_compressed_avaliable()
        );

        assert!(PixelFormat::LOSSLESS_32_BGRA.is_compressed_avaliable());
        assert!(PixelFormat::LOSSLESS_420V.is_compressed_avaliable());
        assert!(PixelFormat::LOSSLESS_420F.is_compressed_avaliable());
        assert!(
            PixelFormat::LOSSLESS_420_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE
                .is_compressed_avaliable()
        );
        assert!(
            PixelFormat::LOSSLESS_422_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE
                .is_compressed_avaliable()
        );
    }
}
