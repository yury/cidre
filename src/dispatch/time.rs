

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Time(u64);

impl Time {
   pub fn now() -> Time {
     Time(0)
   } 
}