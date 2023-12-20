use crate::{av, cm, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVAssetTrack")]
    pub Track(ns::Id)
);

define_obj_type!(
    #[doc(alias = "AVFragmentedTrack")]
    pub FragmentedTrack(Track)
);

impl Track {
    /// A reference to the [`av::Asset`] of which the [`av::AssetTrack`] is a part.
    #[objc::msg_send(asset)]
    pub fn asset(&self) -> &av::Asset;

    /// The persistent unique identifier for this track of the asset.
    #[objc::msg_send(trackID)]
    pub fn track_id(&self) -> cm::PersistentTrackId;
}

/// AVAssetTrackBasicPropertiesAndCharacteristics
impl Track {
    #[objc::msg_send(mediaType)]
    pub fn media_type(&self) -> &av::MediaType;

    #[objc::msg_send(isPlayable)]
    pub fn is_playable(&self) -> bool;

    #[objc::msg_send(isDecodable)]
    pub fn is_decodable(&self) -> bool;

    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(isSelfContained)]
    pub fn is_self_contained(&self) -> bool;

    #[objc::msg_send(totalSampleDataLength)]
    pub fn total_sample_data_length(&self) -> isize;

    #[objc::msg_send(hasMediaCharacteristic:)]
    pub fn has_media_characterisitc(&self, val: av::MediaCharacteristic) -> bool;
}
