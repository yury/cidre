#[cfg(any(target_os = "ios", target_os = "visionos"))]
use crate::{av, blocks, objc};

#[doc(alias = "AVAudioSessionRouteSelection")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum AudioSessionRouteSelection {
    /// Indicates no route was selected.i
    #[doc(alias = "AVAudioSessionRouteSelectionNone")]
    None = 0,

    /// Indicates that the local device was selected.
    #[doc(alias = "AVAudioSessionRouteSelectionLocal")]
    Local = 1,

    /// Indicates that an external device was selected.
    #[doc(alias = "AVAudioSessionRouteSelectionExternal")]
    External = 2,
}

#[cfg(any(target_os = "ios", target_os = "visionos"))]
impl av::AudioSession {
    #[objc::msg_send(prepareRouteSelectionForPlaybackWithCompletionHandler:)]
    #[objc::available(ios = 13.0, visionos = 1.0)]
    pub fn prepare_route_selection_for_playback_with_ch(
        &self,
        ch: &mut blocks::EscBlock<
            fn(should_start_playback: bool, route_selection: av::AudioSessionRouteSelection),
        >,
    );
}
