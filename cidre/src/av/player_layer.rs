use crate::{arc, av, ca, cg, cv, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVPlayerLayer")]
    pub PlayerLayer(ca::Layer),
    AV_PLAYER_LAYER
);

impl PlayerLayer {
    /// Creates a layer object to present the visual contents of a player’s current item.
    #[objc::msg_send(playerLayerWithPlayer:)]
    pub fn with_player(player: Option<&av::Player>) -> arc::R<Self>;

    /// The player whose visual content the layer displays.
    #[objc::msg_send(player)]
    pub fn player(&self) -> Option<arc::R<av::Player>>;

    #[objc::msg_send(setPlayer:)]
    pub fn set_player(&mut self, val: Option<&av::Player>);

    /// A value that specifies how the layer displays the player’s visual content within the layer’s bounds.
    #[objc::msg_send(videoGravity)]
    pub fn video_gravity(&self) -> arc::R<av::LayerVideoGravity>;

    #[objc::msg_send(setVideoGravity:)]
    pub fn set_video_gravity(&mut self, val: &av::LayerVideoGravity);

    /// A Boolean value that indicates whether the first video frame of the player’s current item is ready for display.
    #[objc::msg_send(isReadyForDisplay)]
    pub fn is_ready_for_display(&self) -> bool;

    /// The current size and position of the video image as displayed within the receiver's bounds.
    #[objc::msg_send(videoRect)]
    pub fn video_rect(&self) -> cg::Rect;

    /// The attributes of the visual output that displays in the player layer during playback.
    #[objc::msg_send(pixelBufferAttributes)]
    pub fn pixel_buf_attrs(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

    /// Returns the pixel buffer that the player layer currently displays.
    #[objc::msg_send(copyDisplayedPixelBuffer)]
    pub fn displayed_pixel_buf(&self) -> Option<arc::Retained<cv::PixelBuf>>;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_PLAYER_LAYER: &'static objc::Class<PlayerLayer>;
}

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        let layer = av::PlayerLayer::new();
        assert!(layer.player().is_none());
        assert!(layer.pixel_buf_attrs().is_none());
        assert!(layer.displayed_pixel_buf().is_none());
        assert!(!layer.is_ready_for_display());
        assert_eq!(
            &layer.video_gravity(),
            av::LayerVideoGravity::resize_aspect()
        )
    }
}
