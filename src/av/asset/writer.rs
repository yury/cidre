use std::intrinsics::transmute;

use crate::{av, cf, cm, define_obj_type, ns};

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
    pub fn should_optimize_for_network_use(&self) -> bool {
        unsafe { rsel_shouldOptimizeForNetworkUse(self) }
    }

    pub fn set_should_optimize_for_network_use(&self, value: bool) {
        unsafe { wsel_setShouldOptimizeForNetworkUse(self, value) }
    }

    pub fn can_add_input(&self, input: &WriterInput) -> bool {
        unsafe { AVAssetWriter_rsel_canAddInput(self, input) }
    }

    pub fn add_input(&self, input: &WriterInput) {
        unsafe { AVAssetWriter_wsel_addInput(self, input) }
    }

    pub fn start_writing(&self) {
        unsafe { wsel_startWriting(self) }
    }

    pub fn start_session_at_source_time(&self, start_time: cm::Time) {
        unsafe { wsel_startSessionAtSourceTime(self, start_time) }
    }

    pub fn end_session_at_source_time(&self, start_time: cm::Time) {
        unsafe { wsel_endSessionAtSourceTime(self, start_time) }
    }

    pub fn finish_writing(&self) {
        unsafe { wsel_finishWriting(self) }
    }

    pub fn cancel_writing(&self) {
        unsafe { wsel_cancelWriting(self) }
    }

    pub fn error(&self) -> Option<&cf::Error> {
        unsafe { rsel_error(self) }
    }

    pub fn inputs(&self) -> &cf::ArrayOf<WriterInput> {
        unsafe { AVAssetWriter_rsel_inputs(self) }
    }

    /// ```
    /// use cidre::{av, cf};
    /// let url = cf::URL::from_str("file://tmp/bla.mp4").unwrap();
    ///
    /// let writer = av::AssetWriter::with_url_and_file_type(&url, av::FileType::mp4()).unwrap();
    /// assert_eq!(writer.inputs().len(), 0);
    /// ```
    pub fn with_url_and_file_type<'a>(
        url: &cf::URL,
        file_type: &av::FileType,
    ) -> Result<cf::Retained<Writer>, cf::Retained<cf::Error>> {
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
    fn rsel_shouldOptimizeForNetworkUse(id: &ns::Id) -> bool;
    fn wsel_setShouldOptimizeForNetworkUse(id: &ns::Id, value: bool);

    fn AVAssetWriter_rsel_canAddInput(writer: &Writer, input: &WriterInput) -> bool;
    fn AVAssetWriter_wsel_addInput(writer: &Writer, input: &WriterInput);

    fn wsel_startWriting(id: &ns::Id);
    fn wsel_startSessionAtSourceTime(id: &ns::Id, start_time: cm::Time);
    fn wsel_endSessionAtSourceTime(id: &ns::Id, start_time: cm::Time);
    fn wsel_finishWriting(id: &ns::Id);
    fn wsel_cancelWriting(id: &ns::Id);
    fn rsel_error(id: &ns::Id) -> Option<&cf::Error>;

    fn AVAssetWriter_rsel_inputs(id: &ns::Id) -> &cf::ArrayOf<WriterInput>;

    fn AVAssetWriter_assetWriterWithURL_fileType_error<'a>(
        url: &cf::URL,
        file_type: &av::FileType,
        error: &mut Option<cf::Retained<cf::Error>>,
    ) -> Option<cf::Retained<Writer>>;

    //csel_ab(, AVAssetWriterInput, assetWriterInputWithMediaType, AVMediaType, outputSettings, NSDictionary * _Nullable, AVAssetWriterInput *)
    // fn

    // wsel_a(, id, finishWritingWithCompletionHandler, VoidBlock)
    // fn wsel_finishWritingWithCompletionHandler(id: &ns::Id, block: &DispatchBlock);
}
