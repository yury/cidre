use crate::cf;

pub type MediaType = cf::String;

impl MediaType {
    #[inline]
    pub fn video() -> &'static Self {
        unsafe { AVMediaTypeVideo }
    }

    #[inline]
    pub fn audio() -> &'static Self {
        unsafe { AVMediaTypeAudio }
    }

    #[inline]
    pub fn text() -> &'static Self {
        unsafe { AVMediaTypeText }
    }

    #[inline]
    pub fn closed_caption() -> &'static Self {
        unsafe { AVMediaTypeClosedCaption }
    }

    #[inline]
    pub fn subtitle() -> &'static Self {
        unsafe { AVMediaTypeSubtitle }
    }

    #[inline]
    pub fn timecode() -> &'static Self {
        unsafe { AVMediaTypeTimecode }
    }

    #[inline]
    pub fn metadata() -> &'static Self {
        unsafe { AVMediaTypeMetadata }
    }

    #[inline]
    pub fn muxed() -> &'static Self {
        unsafe { AVMediaTypeMuxed }
    }

    #[inline]
    #[cfg(not(target_os = "macos"))]
    pub fn metadata_object() -> &'static Self {
        unsafe { AVMediaTypeMetadataObject }
    }

    #[inline]
    pub fn depth_data() -> &'static Self {
        unsafe { AVMediaTypeDepthData }
    }
}

pub type VideoRange = cf::String;

impl VideoRange {
    #[inline]
    pub fn sdr() -> &'static Self {
        unsafe { AVVideoRangeSDR }
    }

    #[inline]
    pub fn hlg() -> &'static Self {
        unsafe { AVVideoRangeHLG }
    }

    #[inline]
    pub fn pq() -> &'static Self {
        unsafe { AVVideoRangePQ }
    }
}

pub type FileType = cf::String;

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVMediaTypeVideo: &'static MediaType;
    static AVMediaTypeAudio: &'static MediaType;
    static AVMediaTypeText: &'static MediaType;
    static AVMediaTypeClosedCaption: &'static MediaType;
    static AVMediaTypeSubtitle: &'static MediaType;
    static AVMediaTypeTimecode: &'static MediaType;
    static AVMediaTypeMetadata: &'static MediaType;
    static AVMediaTypeMuxed: &'static MediaType;

    #[cfg(not(target_os = "macos"))]
    static AVMediaTypeMetadataObject: &'static MediaType;
    static AVMediaTypeDepthData: &'static MediaType;

    static AVVideoRangeSDR: &'static VideoRange;
    static AVVideoRangeHLG: &'static VideoRange;
    static AVVideoRangePQ: &'static VideoRange;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_type() {
        let _s = MediaType::audio();
    }
}
