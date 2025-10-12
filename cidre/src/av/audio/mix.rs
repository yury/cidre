use crate::{
    arc, av, cm, define_obj_type,
    ns::{self, Copying, CopyingMut},
    objc,
};

#[cfg(feature = "mt")]
use crate::mt;

define_obj_type!(
    /// Allows custom audio processing to be performed on audio tracks during playback or other operations.
    #[doc(alias = "AVAudioMix")]
    pub Mix(ns::Id),
    AV_AUDIO_MIX
);

impl ns::Copying for Mix {}
impl ns::CopyingMut for Mix {}

impl Mix {
    pub fn copy(&self) -> Option<arc::R<Mix>> {
        unsafe { std::mem::transmute(self.copy_with_zone(std::ptr::null_mut())) }
    }

    pub fn copy_mut(&self) -> Option<arc::R<MixMut>> {
        unsafe { std::mem::transmute(self.copy_with_zone_mut(std::ptr::null_mut())) }
    }

    #[objc::msg_send(inputParameters)]
    pub fn input_params(&self) -> arc::R<ns::Array<InputParams>>;
}

define_obj_type!(
    #[doc(alias = "AVMutableAudioMix")]
    pub MixMut(Mix),
    AV_MUTABLE_AUDIO_MIX
);

impl MixMut {
    #[objc::msg_send(setInputParameters:)]
    pub fn set_input_params(&mut self, val: &ns::Array<InputParams>);

    #[objc::msg_send(setInputParameters:)]
    pub fn set_input_params_mut(&mut self, val: &ns::Array<InputParamsMut>);
}

define_obj_type!(
    #[doc(alias = "AVAudioMixInputParameters")]
    pub InputParams(ns::Id),
    AV_AUDIO_MIX_INPUT_PARAMETERS
);

impl ns::Copying for InputParams {}
impl ns::CopyingMut for InputParams {}

impl InputParams {
    pub fn copy(&self) -> Option<arc::R<InputParams>> {
        unsafe { std::mem::transmute(self.copy_with_zone(std::ptr::null_mut())) }
    }

    pub fn copy_mut(&self) -> Option<arc::R<InputParamsMut>> {
        unsafe { std::mem::transmute(self.copy_with_zone_mut(std::ptr::null_mut())) }
    }

    /// Indicates the track id of the audio track to which the parameters should be applied.
    #[objc::msg_send(trackID)]
    pub fn track_id(&self) -> cm::PersistentTrackId;

    #[objc::msg_send(audioTimePitchAlgorithm)]
    pub fn time_pitch_algorithm(&self) -> Option<arc::R<av::AudioTimePitchAlgorithm>>;

    #[cfg(all(feature = "mt", not(target_os = "watchos")))]
    #[objc::msg_send(audioTapProcessor)]
    pub fn tap_processor(&self) -> Option<&mt::AudioProcessingTap>;

    #[objc::msg_send(getVolumeRampForTime:startVolume:endVolume:timeRange:)]
    pub fn volume_ramp_for_time(
        &self,
        time: cm::Time,
        start_volume: *mut f32,
        end_volume: *mut f32,
        time_range: *mut cm::TimeRange,
    ) -> bool;
}

define_obj_type!(
    #[doc(alias = "AVMutableAudioMixInputParameters")]
    pub InputParamsMut(InputParams),
    AV_MUTABLE_AUDIO_MIX_INPUT_PARAMETERS
);

impl InputParamsMut {
    #[objc::msg_send(audioMixInputParametersWithTrack:)]
    pub fn with_track(track: Option<&av::AssetTrack>) -> arc::R<Self>;

    #[objc::msg_send(setTrackID:)]
    pub fn set_track_id(&mut self, val: cm::PersistentTrackId);

    #[objc::msg_send(setAudioTimePitchAlgorithm:)]
    pub fn set_time_pitch_algorithm(&mut self, val: Option<&av::AudioTimePitchAlgorithm>);

    #[cfg(all(feature = "mt", not(target_os = "watchos")))]
    #[objc::msg_send(setAudioTapProcessor:)]
    pub fn set_tap_processor(&mut self, val: Option<&mt::AudioProcessingTap>);

    #[objc::msg_send(setVolumeRampFromStartVolume:toEndVolume:timeRange:)]
    pub unsafe fn set_volume_ramp_throws(
        &mut self,
        start_volume: f32,
        end_volume: f32,
        time_range: cm::TimeRange,
    );

    pub fn set_volume_ramp<'ear>(
        &mut self,
        start_volume: f32,
        end_volume: f32,
        time_range: cm::TimeRange,
    ) -> ns::ExResult<'ear> {
        unsafe {
            ns::try_catch(|| self.set_volume_ramp_throws(start_volume, end_volume, time_range))
        }
    }

    #[objc::msg_send(setVolume:atTime:)]
    pub unsafe fn set_volume_at_time_throws(&mut self, volume: f32, time: cm::Time);

    pub fn set_volume_at_time<'ear>(&mut self, volume: f32, time: cm::Time) -> ns::ExResult<'ear> {
        unsafe { ns::try_catch(|| self.set_volume_at_time_throws(volume, time)) }
    }
}

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_AUDIO_MIX: &'static objc::Class<Mix>;
    static AV_MUTABLE_AUDIO_MIX: &'static objc::Class<MixMut>;
    static AV_AUDIO_MIX_INPUT_PARAMETERS: &'static objc::Class<InputParams>;
    static AV_MUTABLE_AUDIO_MIX_INPUT_PARAMETERS: &'static objc::Class<InputParamsMut>;
}

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        let mix = av::AudioMix::new();
        let _copy = mix.copy().unwrap();
        let _mut_copy = mix.copy_mut().unwrap();
    }
}
