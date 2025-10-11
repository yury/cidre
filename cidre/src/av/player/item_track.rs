use crate::{arc, av, define_obj_type, ns, objc};

define_obj_type!(
    pub ItemTrack(ns::Id)
);

impl ItemTrack {
    #[objc::msg_send(assetTrack)]
    pub fn asset_track(&self) -> Option<arc::R<av::AssetTrack>>;

    #[objc::msg_send(isEnabled)]
    pub fn enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    #[objc::msg_send(currentVideoFrameRate)]
    pub fn current_video_frame_rate(&self) -> f32;
}
