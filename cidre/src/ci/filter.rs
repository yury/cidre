use crate::{arc, ci, define_obj_type, ns, objc};

define_obj_type!(
    /// An image processor or generator and its input parameters.
    #[doc(alias = "CIFilter")]
    pub Filter(ns::Id),
    sym CIFilter
);

impl Filter {
    #[objc::msg_send(filterWithName:withInputParameters:)]
    pub fn with_name_input_params(
        name: &ns::String,
        params: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> Option<arc::R<Self>>;

    #[objc::msg_send(outputImage)]
    pub fn output_image(&self) -> Option<arc::R<ci::Image>>;
}
