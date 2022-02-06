use crate::{cv, os};

pub type PixelBuffer = cv::ImageBuffer;

pub struct PixelFormat(os::Type);

impl PixelFormat {
  const _1_MONOCHROME: os::Type =  0x00000001; /* 1 bit indexed */
  const _32BGRA: os::Type = os::Type::from_be_bytes(*b"BGRA");
  const _420_YP_CB_CR8_BI_PLANAR_VIDEO_RANGE: os::Type = os::Type::from_be_bytes(*b"420v");
  const _420_YP_CB_CR8_BI_PLANAR_FULL_RANGE: os::Type = os::Type::from_be_bytes(*b"420f");
}

pub mod buffer_attribute_keys {
  use crate::cf;

  /// A single cf::Number or a cf::Array of cf::Numbers (os::Types)
  #[inline]
  pub fn pixel_format_type() -> &'static cf::String {
    unsafe {
      kCVPixelBufferPixelFormatTypeKey 
    }
  }

  #[inline]
  pub fn width_key() -> &'static cf::String {
    unsafe {
      kCVPixelBufferWidthKey
    }
  }

  #[inline]
  pub fn height_key() -> &'static cf::String {
    unsafe {
      kCVPixelBufferHeightKey
    }
  }

  extern "C" {
    static kCVPixelBufferPixelFormatTypeKey: &'static cf::String;
    static kCVPixelBufferWidthKey: &'static cf::String;
    static kCVPixelBufferHeightKey: &'static cf::String;
    
  }
}
