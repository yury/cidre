use crate::{define_obj_type, ns, cf, cat};

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
    pub fn with_timestamp<'a>(ts: &cat::AudioTimeStamp, sample_rate: f64) -> cf::Retained<'a, Time> {
        unsafe {
            AVAudioTime_initWithAudioTimeStamp_sampleRate(ts, sample_rate)
        }
    }

    pub fn with_host_time<'a>(host_time: u64) -> cf::Retained<'a, Time> {
        unsafe {
            AVAudioTime_initWithHostTime(host_time)
        }
    }
    
    pub fn with_sample_time<'a>(time: FramePosition, at_rate: f64) -> cf::Retained<'a, Time> {
        unsafe {
            AVAudioTime_initWithSampleTime_atRate(time, at_rate)
        }
    }

    /// ```
    /// use cidre::av;
    /// let time = av::AudioTime::with_host_time(101);
    /// 
    /// assert_eq!(time.host_time(), 101);
    /// assert_eq!(time.sample_rate(), 0f64);
    /// assert_eq!(time.is_sample_time_valid(), false);
    /// ```
    pub fn host_time(&self) -> u64 {
        unsafe {
            rsel_hostTime(self)
        }
    }

    pub fn sample_rate(&self) -> f64 {
        unsafe {
            rsel_sampleRate(self)
        }
    }

    pub fn sample_time(&self) -> FramePosition {
        unsafe {
            rsel_sampleTime(self)
        }
    }

    pub fn is_sample_time_valid(&self) -> bool {
        unsafe {
            rsel_isSampleTimeValid(self)
        }
    }

    pub fn audio_timestamp(&self) -> cat::AudioTimeStamp {
        unsafe {
            rsel_audioTimeStamp(self)
        }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVAudioTime_initWithAudioTimeStamp_sampleRate<'a>(ts: &cat::AudioTimeStamp, sample_rate: f64) -> cf::Retained<'a, Time>;
    fn AVAudioTime_initWithHostTime<'a>(host_time: u64) -> cf::Retained<'a, Time>;
    fn AVAudioTime_initWithSampleTime_atRate<'a>(sample_time: FramePosition, sample_rate: f64) -> cf::Retained<'a, Time>;

    fn rsel_hostTime(time: &Time) -> u64;
    fn rsel_sampleRate(time: &Time) -> f64;

    fn rsel_isSampleTimeValid(time: &Time) -> bool;
    fn rsel_sampleTime(time: &Time) -> FramePosition;
    fn rsel_audioTimeStamp(time: &Time) -> cat::AudioTimeStamp;
}
