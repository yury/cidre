use crate::{arc, av, cf, cg, define_obj_type, ns};

use super::Output;

define_obj_type!(MetadataOutput(Output));

impl MetadataOutput {
    /// ```
    /// use cidre::{av, cg};
    ///
    /// let mut output = av::CaptureMetadataOutput::new();
    ///
    /// let rect = cg::Rect::new(0.0, 0.0, 1.0, 1.0);
    /// assert_eq!(output.rect_of_intereset(), rect);
    /// let rect = cg::Rect::new(0.0, 0.0, 0.5, 0.5);
    /// output.set_rect_of_interest(rect);
    /// assert_eq!(output.rect_of_intereset(), rect);
    /// ```
    pub fn new() -> arc::R<Self> {
        unsafe { AVCaptureMetadataOutput_new() }
    }

    pub fn available_metadata_object_types(&self) -> &cf::ArrayOf<av::MetadataObjectType> {
        unsafe { rsel_availableMetadataObjectTypes(self) }
    }

    pub fn rect_of_intereset(&self) -> cg::Rect {
        unsafe { rsel_rectOfInterest(self) }
    }

    pub fn set_rect_of_interest(&mut self, value: cg::Rect) {
        unsafe { wsel_setRectOfInterest(self, value) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_availableMetadataObjectTypes(id: &ns::Id) -> &cf::ArrayOf<av::MetadataObjectType>;
    fn rsel_rectOfInterest(id: &ns::Id) -> cg::Rect;
    fn wsel_setRectOfInterest(id: &ns::Id, value: cg::Rect);
    fn AVCaptureMetadataOutput_new() -> arc::R<MetadataOutput>;
}
