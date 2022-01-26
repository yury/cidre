
mod time;
pub use time::TimeEpoch;
pub use time::TimeFlags;
pub use time::TimeScale;
pub use time::TimeValue;
pub use time::Time;
pub use time::TimeRoundingMethod;


#[link(name = "CoreMedia", kind = "framework")]
extern "C" {
}