use crate::{arc, cat, define_cls, define_obj_type, ns, objc};

use super::FramePos;

define_obj_type!(
    #[doc(alias = "AVAudioTime")]
    pub Time(ns::Id)
);

impl arc::A<Time> {
    #[objc::msg_send(initWithAudioTimeStamp:sampleRate:)]
    pub fn init_with_audio_ts_sample_rate(
        self,
        ts: &cat::AudioTimeStamp,
        sample_rate: f64,
    ) -> arc::R<Time>;

    #[objc::msg_send(initWithHostTime:)]
    pub fn init_with_host_time(self, host_time: u64) -> arc::R<Time>;

    #[objc::msg_send(initWithHostTime:sampleTime:atRate:)]
    pub fn init_with_host_time_sample_rate_at_rate(
        self,
        host_time: u64,
        sample_time: FramePos,
        at_rate: f64,
    ) -> arc::R<Time>;

    #[objc::msg_send(initWithSampleTime:atRate:)]
    pub fn init_with_sample_time_at_rate(self, time: FramePos, at_rate: f64) -> arc::R<Time>;
}

/// Represent a moment in time.
///
/// AVAudioTime is used in AVAudioEngine to represent time. Instances are immutable.
///
/// A single moment in time may be represented in two different ways:
/// 1. mach_absolute_time(), the system's basic clock. Commonly referred to as "host time."
/// 2. audio samples at a particular sample rate
///
/// A single AVAudioTime instance may contain either or both representations; it might
/// represent only a sample time, only a host time, or both.
///
/// Rationale for using host time:
/// a. internally we are using AudioTimeStamp, which uses host time, and it seems silly to divide
/// b. it is consistent with a standard system timing service
/// c. we do provide conveniences to convert between host ticks and seconds (host time divided by
/// frequency) so client code wanting to do what should be straightforward time computations can at
/// least not be cluttered by ugly multiplications and divisions by the host clock frequency.
impl Time {
    define_cls!(AV_AUDIO_TIME);

    pub fn with_ts(ts: &cat::AudioTimeStamp, sample_rate: f64) -> arc::R<Time> {
        Self::alloc().init_with_audio_ts_sample_rate(ts, sample_rate)
    }

    pub fn with_host_time(host_time: u64) -> arc::R<Time> {
        Self::alloc().init_with_host_time(host_time)
    }

    pub fn with_host_time_sample_rate_at_rate(
        host_time: u64,
        sample_time: FramePos,
        at_rate: f64,
    ) -> arc::R<Time> {
        Self::alloc().init_with_host_time_sample_rate_at_rate(host_time, sample_time, at_rate)
    }

    pub fn with_sample_time(time: FramePos, at_rate: f64) -> arc::R<Time> {
        Self::alloc().init_with_sample_time_at_rate(time, at_rate)
    }

    #[objc::msg_send(hostTime)]
    pub fn host_time(&self) -> u64;

    #[objc::msg_send(sampleRate)]
    pub fn sample_rate(&self) -> f64;

    #[objc::msg_send(sampleTime)]
    pub fn sample_time(&self) -> FramePos;

    #[objc::msg_send(isSampleTimeValid)]
    pub fn is_sample_time_valid(&self) -> bool;

    #[objc::msg_send(isHostTimeValid)]
    pub fn is_host_time_valid(&self) -> bool;

    #[objc::msg_send(audioTimeStamp)]
    pub fn audio_timestamp(&self) -> cat::AudioTimeStamp;

    /// If anchorTime is an AVAudioTime where both host time and sample time are valid,
    /// and self is another timestamp where only one of the two is valid, this method
    /// returns a new AVAudioTime copied from self and where any additional valid
    /// fields provided by the anchor are also valid.
    ///
    /// Note that the anchorTime must have both host and sample time valid, and self
    /// must have sample rate and at least one of host or sample time valid.
    /// Otherwise this method returns None.
    ///
    /// time0 has a valid audio sample representation,
    /// but no host time representation.AVAudioTime *time0 = [AVAudioTime timeWithSampleTime: 0.0 atRate: 44100.0];
    /// anchor has a valid host time representation and sample time representation.AVAudioTime *anchor = [player playerTimeForNodeTime: player.lastRenderTime];
    /// fill in valid host time representationAVAudioTime *fullTime0 = [time0 extrapolateTimeFromAnchor: anchor];
    #[objc::msg_send(extrapolateTimeFromAnchor:)]
    pub fn extrapolate_time_from_anchor_ar(&self, anchor: &Time) -> Option<arc::Rar<Time>>;

    #[objc::rar_retain]
    pub fn extrapolate_time_from_anchor(&self, anchor: &Time) -> Option<arc::R<Time>>;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_TIME: &'static objc::Class<Time>;
}

#[cfg(test)]
mod tests {
    use crate::av;
    #[test]
    fn basics() {
        let time = av::AudioTime::with_host_time(101);

        assert_eq!(time.host_time(), 101);
        assert_eq!(time.sample_rate(), 0f64);
        assert_eq!(time.is_sample_time_valid(), false);
        assert_eq!(time.is_host_time_valid(), true);
    }
}
