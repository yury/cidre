#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Time(u64);

impl Time {
    pub const NOW: Time = Time(0);
    pub const FOREVER: Time = Time(!0);

    pub fn when(when: Time, delta: i64) -> Time {
      unsafe {
        dispatch_time(when, delta)
      }
    }
}

extern "C" {
  fn dispatch_time(when: Time, delta: i64) -> Time;
}