use crate::{av, cf, define_obj_type, ns};

use super::Output;

define_obj_type!(MetadataOutput(Output));

impl MetadataOutput {
    pub fn available_metadata_object_types(&self) -> &cf::ArrayOf<av::MetadataObjectType> {
        unsafe { rsel_availableMetadataObjectTypes(self) }
    }

    /// ```
    /// use cidre::av;
    ///
    /// let output = av::CaptureMetadataOutput::new();
    ///
    /// ```
    pub fn new<'a>() -> cf::Retained<'a, Self> {
        unsafe { AVCaptureMetadataOutput_new() }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_availableMetadataObjectTypes(id: &ns::Id) -> &cf::ArrayOf<av::MetadataObjectType>;
    fn AVCaptureMetadataOutput_new<'a>() -> cf::Retained<'a, MetadataOutput>;
}
