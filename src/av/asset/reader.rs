use crate::{arc, av, cf, cm, define_obj_type, msg_send, ns};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    /// Indicates that the status of the asset reader is not currently known.
    Unknown = 0,
    /// Indicates that the asset reader is successfully reading samples from its asset.
    Reading = 1,
    /// Indicates that the asset reader has successfully read all of the samples in its time range.
    Completed = 2,
    /// Indicates that the asset reader can no longer read samples from its asset because of an error. The error is described by the value of the asset reader's error property.
    Failed = 3,
    /// Indicates that the asset reader can no longer read samples because reading was canceled with the cancelReading method.
    Cancelled = 4,
}

define_obj_type!(Reader(ns::Id));

impl Reader {
    pub fn with_asset<'ar>(asset: &av::Asset) -> Result<arc::R<Reader>, &'ar cf::Error> {
        let mut error = None;
        unsafe {
            if let Some(reader) = AVAssetReader_assetReaderWithAsset_error(asset, &mut error) {
                Ok(reader)
            } else {
                Err(error.unwrap_unchecked())
            }
        }
    }

    #[inline]
    pub fn add_output(&mut self, output: &av::AssetReaderOutput) {
        unsafe { AVAssetReader_wsel_addOutput(self, output) }
    }

    /// Prepares the receiver for reading sample buffers from the asset.
    ///
    /// This method validates the entire collection of settings for outputs for tracks, for audio mixing, and for video composition and initiates reading from the receiver's asset.
    /// If this method returns `false`, clients can determine the nature of the failure by checking the value of the status and error properties.
    ///
    /// This method throws an exception if reading has already started (`status` has progressed beyond AVAssetReaderStatusUnknown).
    #[inline]
    pub fn start_reading(&self) -> bool {
        unsafe { rsel_startReading(self) }
    }

    /// Cancels any background work and prevents the receiver's outputs from reading more samples.
    ///
    /// Clients that want to stop reading samples from the receiver before reaching the end of its time range should call this method to stop any background read ahead operations that the may have been in progress.
    ///
    /// This method should not be called concurrently with any calls to -[AVAssetReaderOutput copyNextSampleBuffer].
    pub fn cancel_reading(&self) {
        unsafe { wsel_cancelReading(self) }
    }

    pub fn can_add_output(&self, output: &av::AssetReaderOutput) -> bool {
        unsafe { AVAssetReader_rsel_canAddOutput(self, output) }
    }

    pub fn error(&self) -> Option<&cf::Error> {
        unsafe { rsel_error(self) }
    }

    #[inline]
    pub fn status(&self) -> Status {
        msg_send!("av", self, sel_status)
    }

    pub fn time_range(&mut self) -> cm::TimeRange {
        msg_send!("av", self, sel_timeRange)
    }

    pub fn set_time_range(&mut self, value: cm::TimeRange) {
        msg_send!("av", self, sel_setTimeRange, value)
    }

    pub fn outputs(&self) -> &cf::ArrayOf<av::AssetReaderOutput> {
        unsafe { AVAssetReader_rsel_outputs(self) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVAssetReader_assetReaderWithAsset_error<'ar>(
        asset: &av::Asset,
        error: &mut Option<&'ar cf::Error>,
    ) -> Option<arc::R<Reader>>;
    fn AVAssetReader_wsel_addOutput(reader: &Reader, output: &av::AssetReaderOutput);
    fn AVAssetReader_rsel_canAddOutput(reader: &Reader, output: &av::AssetReaderOutput) -> bool;
    fn rsel_startReading(reader: &Reader) -> bool;
    fn wsel_cancelReading(reader: &Reader);

    fn rsel_error(id: &ns::Id) -> Option<&cf::Error>;
    fn AVAssetReader_rsel_outputs(reader: &Reader) -> &cf::ArrayOf<av::AssetReaderOutput>;
}
