use crate::{av, cf, define_obj_type, ns, cg};

use super::Output;

define_obj_type!(MetadataOutput(Output));

impl MetadataOutput {
    /// ```
    /// use cidre::av;
    ///
    /// let output = av::CaptureMetadataOutput::new();
    ///
    /// ```
    pub fn new<'a>() -> cf::Retained<'a, Self> {
        unsafe { AVCaptureMetadataOutput_new() }
    }

    pub fn available_metadata_object_types(&self) -> &cf::ArrayOf<av::MetadataObjectType> {
        unsafe { rsel_availableMetadataObjectTypes(self) }
    }

    pub fn rect_of_intereset(&self) -> cg::Rect {
        unsafe { rsel_rectOfInterest(self) }
    }

    pub fn set_rect_of_interst(&self, value: cg::Rect) {
        unsafe { wsel_setRectOfInterest(self, value) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_availableMetadataObjectTypes(id: &ns::Id) -> &cf::ArrayOf<av::MetadataObjectType>;
    fn rsel_rectOfInterest(id: &ns::Id) -> cg::Rect;
    fn wsel_setRectOfInterest(id: &ns::Id, value: cg::Rect);
    fn AVCaptureMetadataOutput_new<'a>() -> cf::Retained<'a, MetadataOutput>;
}
