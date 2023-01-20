use std::intrinsics::transmute;

use crate::{arc, av, cf, cm, define_obj_type, ns, objc};

use super::WriterInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    /// Indicates that the status of the asset writer is not currently known.
    Unknown = 0,
    /// Indicates that the asset writer is successfully writing samples to its output file.
    Writing = 1,
    /// Indicates that the asset writer has successfully written all samples following a call to finishWriting.
    Completed = 2,
    /// Indicates that the asset writer can no longer write samples to its output file because of an error. The error is described by the value of the asset writer's error property.
    Failed = 3,
    /// Indicates that the asset writer can no longer write samples because writing was canceled with the cancelWriting method.
    Cancelled = 4,
}

define_obj_type!(Writer(ns::Id));

impl Writer {
    #[objc::msg_send2(shouldOptimizeForNetworkUse)]
    pub fn should_optimize_for_network_use(&self) -> bool;

    #[objc::msg_send2(setShouldOptimizeForNetworkUse:)]
    pub fn set_should_optimize_for_network_use(&self, value: bool);

    #[objc::msg_send2(canAddInput:)]
    pub fn can_add_input(&self, input: &WriterInput) -> bool;

    #[objc::msg_send2(addInput:)]
    pub fn add_input(&self, input: &WriterInput);

    #[objc::msg_send2(startWriting)]
    pub fn start_writing(&self);

    #[objc::msg_send2(startSessionAtSourceTime:)]
    pub fn start_session_at_source_time(&self, start_time: cm::Time);

    #[objc::msg_send2(endSessionAtSourceTime:)]
    pub fn end_session_at_source_time(&self, start_time: cm::Time);

    #[objc::msg_send2(finishWriting)]
    pub fn finish_writing(&self);

    #[objc::msg_send2(cancelWriting)]
    pub fn cancel_writing(&self);

    #[objc::msg_send2(error)]
    pub fn error(&self) -> Option<&ns::Error>;

    #[objc::msg_send2(inputes)]
    pub fn inputs(&self) -> &ns::Array<WriterInput>;

    /// ```
    /// use cidre::{av, cf};
    /// let url = cf::URL::from_str("file://tmp/bla.mp4").unwrap();
    ///
    /// let writer = av::AssetWriter::with_url_and_file_type(&url, av::FileType::mp4()).unwrap();
    /// assert_eq!(writer.inputs().len(), 0);
    /// ```
    pub fn with_url_and_file_type(
        url: &cf::URL,
        file_type: &av::FileType,
    ) -> Result<arc::R<Writer>, arc::R<cf::Error>> {
        let mut error = None;
        unsafe {
            let res = AVAssetWriter_assetWriterWithURL_fileType_error(url, file_type, &mut error);
            match error {
                None => Ok(transmute(res)),
                Some(e) => Err(e),
            }
        }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVAssetWriter_assetWriterWithURL_fileType_error<'a>(
        url: &cf::URL,
        file_type: &av::FileType,
        error: &mut Option<arc::R<cf::Error>>,
    ) -> Option<arc::R<Writer>>;
}
