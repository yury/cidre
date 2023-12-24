use std::ffi::c_void;

use crate::define_cls;
use crate::{arc, av, blocks, define_obj_type, ns, objc};

pub mod cache;
pub use cache::Cache as AssetCache;

pub mod reader;
pub use reader::Reader;
pub use reader::Status as ReaderStatus;

pub mod reader_output;
pub use reader_output::ReaderAudioMixOutput;
pub use reader_output::ReaderOutput;
pub use reader_output::ReaderOutputCaptionAdaptor;
pub use reader_output::ReaderOutputMetadataAdaptor;
pub use reader_output::ReaderSampleReferenceOutput;
pub use reader_output::ReaderTrackOutput;
pub use reader_output::ReaderVideoCompositionOutput;

pub mod writer;
pub use writer::Delegate as AssetWriterDelegate;
pub use writer::DelegateImpl as AssetWriterDelegateImpl;
pub use writer::Status as WriterStatus;
pub use writer::Writer as AssetWriter;

pub mod writer_input;
pub use writer_input::WriterInput;

pub mod track;
pub use track::FragmentedTrack;
pub use track::Track;

pub mod segment_report;
pub use segment_report::SampleInfo as SegmentReportSampleInfo;
pub use segment_report::SegmentReport;
pub use segment_report::SegmentType;
pub use segment_report::TrackReport as SegmentTrackReport;

define_obj_type!(
    #[doc(alias = "AVAsset")]
    pub Asset(ns::Id)
);

define_obj_type!(
    #[doc(alias = "AVURLAsset")]
    pub UrlAsset(Asset)
);

define_obj_type!(
    #[doc(alias = "AVFragmentedAsset")]
    pub FragmentedAsset(UrlAsset)
);

define_obj_type!(
    #[doc(alias = "AVFragmentedAssetMinder")]
    pub FragmentedAssetMinder(ns::Id)
);

impl arc::A<UrlAsset> {
    #[objc::msg_send(initWithURL:options:)]
    pub fn init_with_url_options(
        self,
        url: &ns::Url,
        options: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> Option<arc::R<UrlAsset>>;
}

impl UrlAsset {
    define_cls!(AV_URL_ASSET);

    #[inline]
    pub fn with_url(
        url: &ns::Url,
        options: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_url_options(url, options)
    }

    #[objc::msg_send(loadTracksWithMediaType:completionHandler:)]
    pub fn load_tracks_with_media_type_ch(
        &self,
        media_type: &av::MediaType,
        completion: *mut c_void,
    );

    pub fn load_tracks_with_media_type_once<'ar, F>(&self, media_type: &av::MediaType, block: F)
    where
        F: FnOnce(Option<&'ar ns::Array<av::asset::Track>>, Option<&'ar ns::Error>)
            + 'static
            + Sync,
    {
        let block = blocks::once2(block);
        self.load_tracks_with_media_type_ch(media_type, block.escape().as_mut_ptr())
    }

    pub async fn load_tracks_with_media_type(
        &self,
        media_type: &av::MediaType,
    ) -> Result<arc::R<ns::Array<av::asset::Track>>, arc::R<ns::Error>> {
        let (future, block) = blocks::result();
        self.load_tracks_with_media_type_ch(media_type, block.escape().as_mut_ptr());
        future.await
    }

    /// A session identifier that the asset sends in HTTP requests that it makes.
    ///
    /// The asset uses this value to set as the X-Playback-Session-Id header of HTTP requests that it creates.
    #[objc::msg_send(httpSessionIdentifier)]
    pub fn http_session_identifier(&self) -> ns::Uuid;

    #[objc::cls_msg_send(audiovisualTypes)]
    pub fn av_types_ar() -> arc::Rar<ns::Array<av::FileType>>;

    #[objc::cls_rar_retain]
    pub fn av_types() -> arc::R<ns::Array<av::FileType>>;

    #[objc::cls_msg_send(audiovisualMIMETypes)]
    pub fn av_mime_types_ar() -> arc::Rar<ns::Array<ns::String>>;

    #[objc::cls_rar_retain]
    pub fn av_mime_types() -> arc::R<ns::Array<ns::String>>;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_URL_ASSET: &'static objc::Class<UrlAsset>;
}

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        let types = av::UrlAsset::av_types();
        assert!(!types.is_empty());
        let types = av::UrlAsset::av_mime_types();
        assert!(!types.is_empty());
    }
}
