use crate::{arc, av, cg, define_obj_type, ns, objc};

use super::Output;

define_obj_type!(MetadataOutput(Output), AV_CAPTURE_METADATA_OUTPUT);

impl MetadataOutput {
    #[objc::msg_send(availableMetadataObjectTypes)]
    pub fn available_metadata_object_types(&self) -> &ns::Array<av::MetadataObjectType>;

    #[objc::msg_send(rectOfInterest)]
    pub fn rect_of_intereset(&self) -> cg::Rect;

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
        assert_eq!(output.rect_of_intereset(), rect);
        let rect = cg::Rect::new(0.0, 0.0, 0.5, 0.5);
        output.set_rect_of_interest(rect);
        assert_eq!(output.rect_of_intereset(), rect);
    }
}
