use crate::{cf, cg, cv};

pub type ImageBuffer = cv::Buffer;

impl ImageBuffer {
    /// Returns the full encoded dimensions of a cv::ImageBuffer.  For example, for an NTSC DV frame this would be 720x480
    ///
    /// Example:
    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32_BGRA, None).unwrap();
    ///
    /// let size = pixel_buffer.encoded_size();
    /// assert_eq!(cg::Size { width: 200.0, height: 100.0 }, size);
    /// ```
    #[inline]
    pub fn encoded_size(&self) -> cg::Size {
        unsafe { CVImageBufferGetEncodedSize(self) }
    }

    /// Returns the nominal output display size, in square pixels, of a Core Video image buffer.
    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32_BGRA, None).unwrap();
    ///
    /// let display_size = pixel_buffer.display_size();
    /// assert_eq!(cg::Size { width: 200.0, height: 100.0}, display_size);
    /// ```
    #[inline]
    pub fn display_size(&self) -> cg::Size {
        unsafe { CVImageBufferGetDisplaySize(self) }
    }

    /// Returns the source rectangle of a Core Video image buffer that represents the clean aperture of the buffer in encoded pixels.
    ///
    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32_BGRA, None).unwrap();
    ///
    /// let rect = pixel_buffer.clean_rect();
    /// assert_eq!(cg::Rect { origin: cg::Point::zero(), size: cg::Size { width: 200.0, height: 100.0 }}, rect);
    /// ```
    #[inline]
    pub fn clean_rect(&self) -> cg::Rect {
        unsafe { CVImageBufferGetCleanRect(self) }
    }

    /// Returns a Boolean value indicating whether the image is flipped vertically.
    ///
    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32_BGRA, None).unwrap();
    ///
    /// assert_eq!(true, pixel_buffer.is_flipped());
    /// ```
    #[inline]
    pub fn is_flipped(&self) -> bool {
        unsafe { CVImageBufferIsFlipped(self) }
    }

    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32_BGRA, None).unwrap();
    ///
    /// ```

    #[inline]
    pub fn color_space(&self) -> Option<&cg::ColorSpace> {
        unsafe { CVImageBufferGetColorSpace(self) }
    }

    #[inline]
    pub fn create_color_space_form_attachments(
        attachments: &cf::Dictionary,
    ) -> Option<cf::Retained<cg::ColorSpace>> {
        unsafe { CVImageBufferCreateColorSpaceFromAttachments(attachments) }
    }
}

extern "C" {
    fn CVImageBufferGetColorSpace(image_buffer: &ImageBuffer) -> Option<&cg::ColorSpace>;
    fn CVImageBufferGetEncodedSize(image_buffer: &ImageBuffer) -> cg::Size;
    fn CVImageBufferGetDisplaySize(image_buffer: &ImageBuffer) -> cg::Size;
    fn CVImageBufferGetCleanRect(image_buffer: &ImageBuffer) -> cg::Rect;
    fn CVImageBufferIsFlipped(image_buffer: &ImageBuffer) -> bool;
    fn CVImageBufferCreateColorSpaceFromAttachments(
        attachments: &cf::Dictionary,
    ) -> Option<cf::Retained<cg::ColorSpace>>;
}

pub mod attachment {

    pub mod keys {
        use crate::cf;

        #[inline]
        pub fn color_space() -> &'static cf::String {
            unsafe { kCVImageBufferCGColorSpaceKey }
        }

        #[inline]
        pub fn clean_aperture() -> &'static cf::String {
            unsafe { kCVImageBufferCleanApertureKey }
        }

        #[inline]
        pub fn preferred_clean_aperture() -> &'static cf::String {
            unsafe { kCVImageBufferPreferredCleanApertureKey }
        }

        #[inline]
        pub fn field_count() -> &'static cf::String {
            unsafe { kCVImageBufferFieldCountKey }
        }

        #[inline]
        pub fn field_detail() -> &'static cf::String {
            unsafe { kCVImageBufferFieldDetailKey }
        }

        #[inline]
        pub fn aspect_ratio() -> &'static cf::String {
            unsafe { kCVImageBufferPixelAspectRatioKey }
        }

        #[inline]
        pub fn display_dimensions() -> &'static cf::String {
            unsafe { kCVImageBufferDisplayDimensionsKey }
        }

        #[inline]
        pub fn gamma_level() -> &'static cf::String {
            unsafe { kCVImageBufferGammaLevelKey }
        }

        #[inline]
        pub fn iic_profile() -> &'static cf::String {
            unsafe { kCVImageBufferICCProfileKey }
        }

        #[inline]
        pub fn y_cb_cr_matrix() -> &'static cf::String {
            unsafe { kCVImageBufferYCbCrMatrixKey }
        }

        #[inline]
        pub fn color_primaries() -> &'static cf::String {
            unsafe { kCVImageBufferColorPrimariesKey }
        }

        #[inline]
        pub fn transfer_function() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunctionKey }
        }

        extern "C" {
            static kCVImageBufferCGColorSpaceKey: &'static cf::String;
            static kCVImageBufferCleanApertureKey: &'static cf::String;

            static kCVImageBufferPreferredCleanApertureKey: &'static cf::String;
            static kCVImageBufferFieldCountKey: &'static cf::String;
            static kCVImageBufferFieldDetailKey: &'static cf::String;
            static kCVImageBufferPixelAspectRatioKey: &'static cf::String;
            static kCVImageBufferDisplayDimensionsKey: &'static cf::String;
            static kCVImageBufferGammaLevelKey: &'static cf::String;
            static kCVImageBufferICCProfileKey: &'static cf::String;
            static kCVImageBufferYCbCrMatrixKey: &'static cf::String;
            static kCVImageBufferColorPrimariesKey: &'static cf::String;
            static kCVImageBufferTransferFunctionKey: &'static cf::String;
        }
    }

    pub mod clean_aperture_keys {
        use crate::cf;

        #[inline]
        pub fn width() -> &'static cf::String {
            unsafe { kCVImageBufferCleanApertureWidthKey }
        }
        #[inline]
        pub fn height() -> &'static cf::String {
            unsafe { kCVImageBufferCleanApertureHeightKey }
        }
        #[inline]
        pub fn horizontal_offset() -> &'static cf::String {
            unsafe { kCVImageBufferCleanApertureHorizontalOffsetKey }
        }

        #[inline]
        pub fn vertical_offset() -> &'static cf::String {
            unsafe { kCVImageBufferCleanApertureVerticalOffsetKey }
        }

        extern "C" {
            static kCVImageBufferCleanApertureWidthKey: &'static cf::String;
            static kCVImageBufferCleanApertureHeightKey: &'static cf::String;
            static kCVImageBufferCleanApertureHorizontalOffsetKey: &'static cf::String;
            static kCVImageBufferCleanApertureVerticalOffsetKey: &'static cf::String;
        }
    }

    pub mod field_detail {
        use crate::cf;

        pub fn termporal_top_first() -> &'static cf::String {
            unsafe { kCVImageBufferFieldDetailTemporalTopFirst }
        }
        pub fn termporal_bottom_first() -> &'static cf::String {
            unsafe { kCVImageBufferFieldDetailTemporalBottomFirst }
        }
        pub fn spatial_first_line_early() -> &'static cf::String {
            unsafe { kCVImageBufferFieldDetailSpatialFirstLineEarly }
        }
        pub fn spatial_first_line_late() -> &'static cf::String {
            unsafe { kCVImageBufferFieldDetailSpatialFirstLineLate }
        }
        extern "C" {
            static kCVImageBufferFieldDetailTemporalTopFirst: &'static cf::String;
            static kCVImageBufferFieldDetailTemporalBottomFirst: &'static cf::String;
            static kCVImageBufferFieldDetailSpatialFirstLineEarly: &'static cf::String;
            static kCVImageBufferFieldDetailSpatialFirstLineLate: &'static cf::String;
        }
    }

    pub mod aspect_ratio_keys {
        use crate::cf;

        pub fn horizontal_spacing() -> &'static cf::String {
            unsafe { kCVImageBufferPixelAspectRatioHorizontalSpacingKey }
        }

        pub fn vertical_spacing() -> &'static cf::String {
            unsafe { kCVImageBufferPixelAspectRatioVerticalSpacingKey }
        }

        extern "C" {
            static kCVImageBufferPixelAspectRatioHorizontalSpacingKey: &'static cf::String;
            static kCVImageBufferPixelAspectRatioVerticalSpacingKey: &'static cf::String;
        }
    }

    pub mod display_keys {
        use crate::cf;

        pub fn width() -> &'static cf::String {
            unsafe { kCVImageBufferDisplayWidthKey }
        }

        pub fn height() -> &'static cf::String {
            unsafe { kCVImageBufferDisplayHeightKey }
        }

        extern "C" {
            static kCVImageBufferDisplayWidthKey: &'static cf::String;
            static kCVImageBufferDisplayHeightKey: &'static cf::String;
        }
    }

    pub mod y_cb_cr_matrix {
        use crate::cf;

        pub fn itu_r_709_2() -> &'static cf::String {
            unsafe { kCVImageBufferYCbCrMatrix_ITU_R_709_2 }
        }
        pub fn itu_r_601_4() -> &'static cf::String {
            unsafe { kCVImageBufferYCbCrMatrix_ITU_R_601_4 }
        }
        pub fn smpte_240m_1995() -> &'static cf::String {
            unsafe { kCVImageBufferYCbCrMatrix_SMPTE_240M_1995 }
        }
        pub fn itu_r_2020() -> &'static cf::String {
            unsafe { kCVImageBufferYCbCrMatrix_ITU_R_2020 }
        }
        extern "C" {
            static kCVImageBufferYCbCrMatrix_ITU_R_709_2: &'static cf::String;
            static kCVImageBufferYCbCrMatrix_ITU_R_601_4: &'static cf::String;
            static kCVImageBufferYCbCrMatrix_SMPTE_240M_1995: &'static cf::String;
            static kCVImageBufferYCbCrMatrix_ITU_R_2020: &'static cf::String;
        }
    }

    pub mod color_primaries {
        use crate::cf;

        pub fn itu_r_709_2() -> &'static cf::String {
            unsafe { kCVImageBufferColorPrimaries_ITU_R_709_2 }
        }
        pub fn ebu_3213() -> &'static cf::String {
            unsafe { kCVImageBufferColorPrimaries_EBU_3213 }
        }
        pub fn smpte_c() -> &'static cf::String {
            unsafe { kCVImageBufferColorPrimaries_SMPTE_C }
        }
        pub fn p22() -> &'static cf::String {
            unsafe { kCVImageBufferColorPrimaries_P22 }
        }
        pub fn dci_p3() -> &'static cf::String {
            unsafe { kCVImageBufferColorPrimaries_DCI_P3 }
        }
        pub fn p3_d65() -> &'static cf::String {
            unsafe { kCVImageBufferColorPrimaries_P3_D65 }
        }

        pub fn itu_r_2020() -> &'static cf::String {
            unsafe { kCVImageBufferColorPrimaries_ITU_R_2020 }
        }
        extern "C" {
            static kCVImageBufferColorPrimaries_ITU_R_709_2: &'static cf::String;
            static kCVImageBufferColorPrimaries_EBU_3213: &'static cf::String;
            static kCVImageBufferColorPrimaries_SMPTE_C: &'static cf::String;
            static kCVImageBufferColorPrimaries_P22: &'static cf::String;
            static kCVImageBufferColorPrimaries_DCI_P3: &'static cf::String;
            static kCVImageBufferColorPrimaries_P3_D65: &'static cf::String;
            static kCVImageBufferColorPrimaries_ITU_R_2020: &'static cf::String;
        }
    }

    pub mod transfer_function {
        use crate::cf;

        pub fn itu_r_709_2() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunction_ITU_R_709_2 }
        }
        pub fn smpte_240m_1995() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunction_SMPTE_240M_1995 }
        }
        pub fn use_gamma() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunction_UseGamma }
        }
        pub fn ebu_3213() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunction_EBU_3213 }
        }
        pub fn smpte_c() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunction_SMPTE_C }
        }
        pub fn srgb() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunction_sRGB }
        }
        pub fn itu_r_2020() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunction_ITU_R_2020 }
        }
        pub fn smpte_st_428_1() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunction_SMPTE_ST_428_1 }
        }
        pub fn smpte_st_2084_pq() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ }
        }
        pub fn itu_r_2100_hlg() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunction_ITU_R_2100_HLG }
        }
        pub fn linear() -> &'static cf::String {
            unsafe { kCVImageBufferTransferFunction_Linear }
        }

        extern "C" {
            static kCVImageBufferTransferFunction_ITU_R_709_2: &'static cf::String;
            static kCVImageBufferTransferFunction_SMPTE_240M_1995: &'static cf::String;
            static kCVImageBufferTransferFunction_UseGamma: &'static cf::String;
            static kCVImageBufferTransferFunction_EBU_3213: &'static cf::String;
            static kCVImageBufferTransferFunction_SMPTE_C: &'static cf::String;
            static kCVImageBufferTransferFunction_sRGB: &'static cf::String;
            static kCVImageBufferTransferFunction_ITU_R_2020: &'static cf::String;
            static kCVImageBufferTransferFunction_SMPTE_ST_428_1: &'static cf::String;
            static kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ: &'static cf::String;
            static kCVImageBufferTransferFunction_ITU_R_2100_HLG: &'static cf::String;
            static kCVImageBufferTransferFunction_Linear: &'static cf::String;
        }
    }
}
