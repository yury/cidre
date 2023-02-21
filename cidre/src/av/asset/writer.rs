use std::intrinsics::transmute;

use crate::{arc, av, cm, define_cls, define_obj_type, ns, objc};

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

impl arc::A<Writer> {
    #[objc::msg_send(initWithURL:fileType:error:)]
    pub fn init_with_url_file_type_error<'ar>(
        self,
        url: &ns::URL,
        file_type: &av::FileType,
        error: &mut Option<&'ar ns::Error>,
    ) -> Option<arc::R<Writer>>;
}

impl Writer {
    define_cls!(AV_ASSET_WRITER);

    #[objc::msg_send(shouldOptimizeForNetworkUse)]
    pub fn should_optimize_for_network_use(&self) -> bool;

    #[objc::msg_send(setShouldOptimizeForNetworkUse:)]
    pub fn set_should_optimize_for_network_use(&mut self, value: bool);

    #[objc::msg_send(canAddInput:)]
    pub fn can_add_input(&self, input: &WriterInput) -> bool;

    #[objc::msg_send(addInput:)]
    pub fn add_input_throws(&mut self, input: &WriterInput);

    pub fn add_input<'ar>(&mut self, input: &WriterInput) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| self.add_input_throws(input))
    }

    #[objc::msg_send(startWriting)]
    pub fn start_writing(&mut self);

    #[objc::msg_send(startSessionAtSourceTime:)]
    pub fn start_session_at_source_time(&mut self, start_time: cm::Time);

    #[objc::msg_send(endSessionAtSourceTime:)]
    pub fn end_session_at_source_time(&mut self, start_time: cm::Time);

    #[objc::msg_send(finishWriting)]
    pub fn finish_writing(&mut self);

    #[objc::msg_send(cancelWriting)]
    pub fn cancel_writing(&mut self);

    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<&ns::Error>;

    #[objc::msg_send(inputs)]
    pub fn inputs(&self) -> &ns::Array<WriterInput>;

    /// ```no_run
    /// use cidre::{av, ns};
    /// let url = ns::URL::with_str("file://tmp/bla.mp4").unwrap();
    ///
    /// let writer = av::AssetWriter::with_url_and_file_type(&url, av::FileType::mp4()).unwrap();
    /// assert_eq!(writer.inputs().len(), 0);
    /// ```
    pub fn with_url_and_file_type<'ar>(
        url: &ns::URL,
        file_type: &av::FileType,
    ) -> Result<arc::R<Writer>, &'ar ns::Error> {
        let mut error = None;
        unsafe {
            let res = Self::alloc().init_with_url_file_type_error(url, file_type, &mut error);
            match error {
                None => Ok(transmute(res)),
                Some(e) => Err(e),
            }
        }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_ASSET_WRITER: &'static objc::Class<Writer>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn basics() {
        use crate::{av, ns};
        let url = ns::URL::with_str("file://tmp/bla.mp4").unwrap();

        let writer = av::AssetWriter::with_url_and_file_type(&url, av::FileType::mp4()).unwrap();
        assert_eq!(writer.inputs().len(), 0);
    }
}
