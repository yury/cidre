use crate::{av::MediaType, cf, cm, define_obj_type, msg_send, ns};

define_obj_type!(ReaderOutput(ns::Id));
define_obj_type!(ReaderTrackOutput(ReaderOutput));
define_obj_type!(ReaderAudioMixOutput(ReaderOutput));
define_obj_type!(ReaderVideoCompositionOutput(ReaderOutput));
define_obj_type!(ReaderSampleReferenceOutput(ReaderOutput));

define_obj_type!(ReaderOutputMetadataAdaptor(ns::Id));
define_obj_type!(ReaderOutputCaptionAdaptor(ns::Id));

/// Is an abstract class that defines an interface for reading a single collection
/// of samples of a common media type from an av::AssetReader.
///
/// Clients can read the media data of an asset by adding one or more concrete
/// instances of AVAssetReaderOutput to an AVAssetReader using the -[AVAssetReader addOutput:] method.
///
/// MPORTANT PERFORMANCE NOTE: Make sure to set the alwaysCopiesSampleData property to false
/// if you do not need to modify the sample data in-place, to avoid unnecessary and inefficient copying.
impl ReaderOutput {
    pub fn media_type(&self) -> &MediaType {
        unsafe { rsel_mediaType(self) }
    }

    /// Indicates whether or not the data in buffers gets copied before being vended to the client.
    ///
    /// When the value of this property is YES, the AVAssetReaderOutput will always vend a buffer
    /// with copied data to the client.  Data in such buffers can be freely modified by the client.
    /// When the value of this property is NO, the buffers vended to the client may not be copied.
    /// Such buffers may still be referenced by other entities. The result of modifying a buffer
    /// whose data hasn't been copied is undefined. Requesting buffers whose data hasn't been copied
    /// when possible can lead to performance improvements.
    ///
    /// Default value is true
    pub fn always_copies_sample_data(&self) -> bool {
        unsafe { rsel_alwaysCopiesSampleData(self) }
    }

    pub fn set_always_copies_sample_data(&self, value: bool) {
        unsafe { wsel_setAlwaysCopiesSampleData(self, value) }
    }

    /// Copies the next sample buffer for the output synchronously.
    ///
    /// The client is responsible for calling cf::Release on the returned cm::SampleBuffer
    /// object when finished with it.
    /// This method will return NULL if there are no more sample buffers available for
    /// the receiver within the time range specified by its av::AssetReader's time_range property,
    /// or if there is an error that prevents the AVAssetReader from reading more media data.
    /// When this method returns NULL, clients should check the value of the associated AVAssetReader's
    /// status property to determine why no more samples could be read.
    ///
    /// This method throws an exception if this output is not added to an instance of av::AssetReader
    /// (using -addOutput:) and -startReading is not called on that asset reader.
    pub fn copy_next_sample_buffer(&self) -> Option<cf::Retained<cm::SampleBuffer>> {
        msg_send!("av", self, sel_copyNextSampleBuffer)
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_mediaType(id: &ns::Id) -> &MediaType;

    fn rsel_alwaysCopiesSampleData(id: &ns::Id) -> bool;
    fn wsel_setAlwaysCopiesSampleData(id: &ns::Id, value: bool);
}
