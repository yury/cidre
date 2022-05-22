use crate::cf;

/// ```
/// use cidre::ca;
/// 
/// println!("{:?}", ca::current_media_time());
/// ```
pub fn current_media_time() -> cf::TimeInterval {
    unsafe { CACurrentMediaTime() }
}

// #[link()]
// QuartzCore
#[link(name = "QuartzCore", kind = "framework")]
extern "C" {
    fn CACurrentMediaTime() -> cf::TimeInterval;
}
