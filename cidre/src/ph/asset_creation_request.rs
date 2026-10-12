use crate::{arc, define_cls, define_obj_type, ns, objc};

#[doc(alias = "PHAssetResourceType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum AssetResourceType {
    Photo = 1,
    Video = 2,
    Audio = 3,
    AlternatePhoto = 4,
    FullSizePhoto = 5,
    FullSizeVideo = 6,
    AdjustmentData = 7,
    AdjustmentBasePhoto = 8,
    PairedVideo = 9,
    FullSizePairedVideo = 10,
    AdjustmentBasePairedVideo = 11,
    AdjustmentBaseVideo = 12,
    PhotoProxy = 19,
}

define_obj_type!(
    #[doc(alias = "PHAssetCreationRequest")]
    pub AssetCreationRequest(ns::Id)
);

impl AssetCreationRequest {
    define_cls!(sym PHAssetCreationRequest);

    /// A new asset, filled with [`Self::add_resource_with_data`]; only inside
    /// [`crate::ph::PhotoLib::perform_changes_ch`]'s changes.
    #[objc::msg_send(creationRequestForAsset)]
    pub fn for_asset() -> arc::R<Self>;

    /// Adds `data` (an encoded image, for a photo) as the asset's resource of type `ty`.
    #[objc::msg_send(addResourceWithType:data:options:)]
    pub fn add_resource_with_data(
        &mut self,
        ty: AssetResourceType,
        data: &ns::Data,
        options: Option<&ns::Id>,
    );
}
