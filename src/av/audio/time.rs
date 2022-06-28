use crate::{cat, cf, define_obj_type, ns};

use super::FramePosition;

define_obj_type!(Time(ns::Id));

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
    pub fn with_timestamp<'a>(
        ts: &cat::AudioTimeStamp,
        sample_rate: f64,
    ) -> cf::Retained<'a, Time> {
        unsafe { AVAudioTime_timeWithAudioTimeStamp_sampleRate(ts, sample_rate) }
    }

    pub fn with_host_time<'a>(host_time: u64) -> cf::Retained<'a, Time> {
        unsafe { AVAudioTime_timeWithHostTime(host_time) }
    }

    pub fn with_host_time_sample_rate_at_rate<'a>(
        host_time: u64,
        sample_time: FramePosition,
        at_rate: f64,
    ) -> cf::Retained<'a, Time> {
        unsafe { AVAudioTime_timeWithHostTime_sampleTime_atRate(host_time, sample_time, at_rate) }
    }

    pub fn with_sample_time<'a>(time: FramePosition, at_rate: f64) -> cf::Retained<'a, Time> {
        unsafe { AVAudioTime_timeWithSampleTime_atRate(time, at_rate) }
    }

    /// ```
    /// use cidre::av;
    /// let time = av::AudioTime::with_host_time(101);
    ///
    /// assert_eq!(time.host_time(), 101);
    /// assert_eq!(time.sample_rate(), 0f64);
    /// assert_eq!(time.is_sample_time_valid(), false);
    /// assert_eq!(time.is_host_time_valid(), true);
    /// ```
    pub fn host_time(&self) -> u64 {
        unsafe { rsel_hostTime(self) }
    }

    pub fn sample_rate(&self) -> f64 {
        unsafe { rsel_sampleRate(self) }
    }

    pub fn sample_time(&self) -> FramePosition {
        unsafe { rsel_sampleTime(self) }
    }

    pub fn is_sample_time_valid(&self) -> bool {
        unsafe { rsel_isSampleTimeValid(self) }
    }

    pub fn is_host_time_valid(&self) -> bool {
        unsafe { rsel_isHostTimeValid(self) }
    }

    pub fn audio_timestamp(&self) -> cat::AudioTimeStamp {
        unsafe { rsel_audioTimeStamp(self) }
    }

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
    pub fn extrapolate_time_from_anchor<'a>(
        &self,
        anchor: &Time,
    ) -> Option<cf::Retained<'a, Time>> {
        unsafe { rsel_extrapolateTimeFromAnchor(self, anchor) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVAudioTime_timeWithAudioTimeStamp_sampleRate<'a>(
        ts: &cat::AudioTimeStamp,
        sample_rate: f64,
    ) -> cf::Retained<'a, Time>;
    fn AVAudioTime_timeWithHostTime<'a>(host_time: u64) -> cf::Retained<'a, Time>;
    fn AVAudioTime_timeWithSampleTime_atRate<'a>(
        sample_time: FramePosition,
        sample_rate: f64,
    ) -> cf::Retained<'a, Time>;
    fn AVAudioTime_timeWithHostTime_sampleTime_atRate<'a>(
        host_time: u64,
        sample_time: FramePosition,
        at_rate: f64,
    ) -> cf::Retained<'a, Time>;

    fn rsel_hostTime(time: &Time) -> u64;
    fn rsel_sampleRate(time: &Time) -> f64;

    fn rsel_isSampleTimeValid(time: &Time) -> bool;
    fn rsel_isHostTimeValid(time: &Time) -> bool;
    fn rsel_sampleTime(time: &Time) -> FramePosition;
    fn rsel_audioTimeStamp(time: &Time) -> cat::AudioTimeStamp;
    fn rsel_extrapolateTimeFromAnchor<'a>(
        time: &Time,
        anchor: &Time,
    ) -> Option<cf::Retained<'a, Time>>;
}
