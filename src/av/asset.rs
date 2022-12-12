use std::ffi::c_void;

use crate::{av, blocks, cf, define_obj_type, ns};

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
pub use writer::Status as WriterStatus;
pub use writer::Writer as AssetWriter;

pub mod writer_input;
pub use writer_input::WriterInput;

pub mod track;
pub use track::FragmentedTrack;
pub use track::Track;

define_obj_type!(Asset(ns::Id));
define_obj_type!(URLAsset(Asset));
define_obj_type!(FragmentedAsset(URLAsset));
define_obj_type!(FragmentedAssetMinder(ns::Id));

impl URLAsset {
    #[inline]
    pub fn with_url(url: &cf::URL, options: Option<&cf::Dictionary>) -> cf::Retained<URLAsset> {
        unsafe { AVURLAsset_URLAssetWithURL_options(url, options) }
    }

    pub fn load_tracks_with_media_type_completion<'ar, F>(
        &self,
        media_type: &av::MediaType,
        completion: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(Option<&'ar cf::ArrayOf<av::asset::Track>>, Option<&'ar cf::Error>),
    {
        unsafe {
            wsel_loadTracksWithMediaType_completionHandler(self, media_type, completion.as_ptr())
        }
    }

    pub fn load_tracks_with_media_type_once<'ar, F>(&self, media_type: &av::MediaType, block: F)
    where
        F: FnOnce(Option<&'ar cf::ArrayOf<av::asset::Track>>, Option<&'ar cf::Error>)
            + 'static
            + Sync,
    {
        let block = blocks::once2(block);
        self.load_tracks_with_media_type_completion(media_type, block.escape())
    }

    pub async fn load_tracks_with_media_type(
        &self,
        media_type: &av::MediaType,
    ) -> Result<cf::Retained<cf::ArrayOf<av::asset::Track>>, cf::Retained<cf::Error>> {
        let (future, block) = blocks::result();
        self.load_tracks_with_media_type_completion(media_type, block.escape());
        future.await
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVURLAsset_URLAssetWithURL_options(
        url: &cf::URL,
        options: Option<&cf::Dictionary>,
    ) -> cf::Retained<URLAsset>;

    fn wsel_loadTracksWithMediaType_completionHandler(
        id: &ns::Id,
        media_type: &av::MediaType,
        callback: *mut c_void,
    );

}
