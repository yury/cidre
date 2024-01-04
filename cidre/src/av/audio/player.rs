use crate::{arc, av, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVAudioPlayer")]
    pub Player(ns::Id)
);

impl arc::A<Player> {
    #[objc::msg_send(initWithContentsOfURL:error:)]
    pub unsafe fn init_with_contents_of_url_err<'ear>(
        self,
        url: &ns::Url,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Player>>;

    #[objc::msg_send(initWithData:error:)]
    pub unsafe fn init_with_data_err<'ear>(
        self,
        data: &ns::Data,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Player>>;
}

impl Player {
    define_cls!(AV_AUDIO_PLAYER);

    pub fn with_contents_of_url<'ear>(url: &ns::Url) -> Result<arc::R<Self>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { Self::alloc().init_with_contents_of_url_err(url, err) })
    }

    pub fn with_data<'ear>(data: &ns::Data) -> Result<arc::R<Self>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { Self::alloc().init_with_data_err(data, err) })
    }

    #[objc::msg_send(prepareToPlay)]
    pub fn prepare_to_play(&mut self) -> bool;

    #[objc::msg_send(play)]
    pub fn play(&mut self) -> bool;

    #[objc::msg_send(pause)]
    pub fn pause(&mut self);

    #[objc::msg_send(stop)]
    pub fn stop(&mut self);

    #[objc::msg_send(isPlaying)]
    pub fn is_playing(&self) -> bool;

    #[objc::msg_send(numberOfChannels)]
    pub fn channels_num(&self) -> usize;

    #[objc::msg_send(duration)]
    pub fn duration(&self) -> ns::TimeInterval;

    #[objc::msg_send(url)]
    pub fn url(&self) -> Option<&ns::Url>;

    #[objc::msg_send(data)]
    pub fn data(&self) -> Option<&ns::Data>;

    #[objc::msg_send(pan)]
    pub fn pan(&self) -> f32;

    #[objc::msg_send(setPan:)]
    pub fn set_pan(&mut self, val: f32);

    #[objc::msg_send(volume)]
    pub fn volume(&self) -> f32;

    #[objc::msg_send(setVolume:)]
    pub fn set_volume(&mut self, val: f32);

    /// Fade to a new volume over a duration
    #[objc::msg_send(setVolume:fadeDuration:)]
    pub fn set_volume_with_fade_dur(&mut self, val: f32, fade_duration: ns::TimeInterval);

    #[objc::msg_send(enableRate)]
    pub fn enable_rate(&self) -> bool;

    #[objc::msg_send(setEnableRate:)]
    pub fn set_enable_rate(&self, val: bool);

    #[objc::msg_send(rate)]
    pub fn rate(&self) -> f32;

    #[objc::msg_send(setRate:)]
    pub fn set_rate(&self, val: f32);

    #[objc::msg_send(currentTime)]
    pub fn current_time(&self) -> ns::TimeInterval;

    #[objc::msg_send(setCurrentTime:)]
    pub fn set_current_time(&mut self, val: ns::TimeInterval);

    #[objc::msg_send(deviceCurrentTime)]
    pub fn device_current_time(&self) -> ns::TimeInterval;

    #[objc::msg_send(numberOfLoops)]
    pub fn number_of_loops(&self) -> isize;

    /// A value of zero means to play the sound just once.
    /// A value of one will result in playing the sound twice, and so on..
    /// Any negative number will loop indefinitely until stopped.
    #[objc::msg_send(setNumberOfLoops:)]
    pub fn set_number_of_loops(&mut self, val: isize);

    #[objc::msg_send(settings)]
    pub fn settings(&self) -> &ns::Dictionary<ns::String, ns::Id>;

    #[objc::msg_send(format)]
    pub fn format(&self) -> &av::AudioFormat;

    #[objc::msg_send(isMeteringEnabled)]
    pub fn is_metering_enabled(&self) -> bool;

    #[objc::msg_send(setMeteringEnabled:)]
    pub fn set_metering_enabled(&self, val: bool);

    #[objc::msg_send(updateMeters)]
    pub fn update_meters(&mut self);

    /// Returns the peak power, in decibels full-scale (dBFS), for an audio channel.
    #[objc::msg_send(peakPowerForChannel:)]
    pub fn peak_power_for_channel(&self, channel_number: usize) -> f32;

    /// Returns the average power, in decibels full-scale (dBFS), for an audio channel.
    #[objc::msg_send(averagePowerForChannel:)]
    pub fn average_power_for_channel(&self, channel_number: usize) -> f32;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_PLAYER: &'static objc::Class<Player>;
}

#[cfg(test)]
mod tests {
    use crate::{av, ns};

    #[test]
    fn basics() {
        let _ = av::AudioPlayer::with_data(&ns::Data::new()).expect_err("What?");
    }
}
