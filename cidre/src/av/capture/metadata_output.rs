use crate::{arc, av, cg, define_obj_type, dispatch, ns, objc};

use super::Output;

// #[doc(alias = "AVCaptureMetadataOutputObjectsDelegate")]
#[objc::obj_trait]
pub trait MetadataOutputObjsDelegate {
    #[objc::optional]
    #[objc::msg_send(captureOutput:didOutputMetadataObjects:fromConnection:)]
    fn capture_output_did_output_metadata_objs_from_connections(
        &mut self,
        output: &mut Output,
        objects: &ns::Array<av::MetadataObj>,
        connection: &mut av::capture::Connection,
    );
}

define_obj_type!(
    /// A capture output for processing timed metadata produced by a capture session.
    ///
    /// An [`av::CaptureMetadataOutput`] object intercepts metadata objects emitted by
    /// its associated capture connection and forwards them to a delegate object for processing.
    /// You can use instances of this class to process specific types of metadata included with
    /// the input data. You use this class the way you do other output objects, typically by
    /// adding it as an output to an [`av::CaptureSession`] object.
    #[doc(alias = "AVCaptureMetadataOutput")]
    pub MetadataOutput(Output),
    AV_CAPTURE_METADATA_OUTPUT
);

impl MetadataOutput {
    #[objc::msg_send(setMetadataObjectsDelegate:queue:)]
    pub fn set_meta_data_objs_delegate<D: MetadataOutputObjsDelegate>(
        &mut self,
        delegate: Option<&D>,
        queue: Option<&dispatch::Queue>,
    );

    #[objc::msg_send(availableMetadataObjectTypes)]
    pub fn available_metadata_obj_types(&self) -> &ns::Array<av::MetadataObjType>;

    #[objc::msg_send(rectOfInterest)]
    pub fn rect_of_interest(&self) -> cg::Rect;

    #[objc::msg_send(setRectOfInterest:)]
    pub fn set_rect_of_interest(&mut self, value: cg::Rect);
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_METADATA_OUTPUT: &'static objc::Class<MetadataOutput>;
}

#[cfg(test)]
mod tests {
    use crate::{av, cg};

    #[test]
    fn basics() {
        let mut output = av::CaptureMetadataOutput::new();

        let rect = cg::Rect::new(0.0, 0.0, 1.0, 1.0);
        assert_eq!(output.rect_of_interest(), rect);
        let rect = cg::Rect::new(0.0, 0.0, 0.5, 0.5);
        output.set_rect_of_interest(rect);
        assert_eq!(output.rect_of_interest(), rect);
    }
}
