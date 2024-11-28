use crate::{arc, define_obj_type, ns, objc, vn};

impl arc::A<GenImageFeaturePrintRequest> {
    #[objc::msg_send(initWithCompletionHandler:)]
    pub fn init_with_ch(
        self,
        ch: Option<&mut vn::RequestCh<GenImageFeaturePrintRequest>>,
    ) -> arc::R<GenImageFeaturePrintRequest>;
}

define_obj_type!(
    pub GenImageFeaturePrintRequest(vn::ImageBasedRequest),
    VN_GENERATE_IMAGE_FEAUTRE_PRINT_REQUEST
);

impl GenImageFeaturePrintRequest {
    pub const REVISION_1: usize = 1;

    pub fn with_ch(ch: &mut vn::RequestCh<GenImageFeaturePrintRequest>) -> arc::R<Self> {
        Self::alloc().init_with_ch(Some(ch))
    }

    pub fn with(f: impl FnMut(&mut Self, Option<&ns::Error>) + 'static) -> arc::R<Self> {
        let mut block = vn::RequestCh::<Self>::new2(f);
        Self::with_ch(&mut block)
    }

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::FeaturePrintObservation>>>;

    #[objc::msg_send(imageCropAndScaleOption)]
    pub fn image_crop_and_scale_option(&self) -> vn::ImageCropAndScaleOpt;

    #[objc::msg_send(setImageCropAndScaleOption:)]
    pub fn set_image_crop_and_scale_option(&mut self, value: vn::ImageCropAndScaleOpt);
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_GENERATE_IMAGE_FEAUTRE_PRINT_REQUEST:
        &'static objc::Class<GenImageFeaturePrintRequest>;
}
