use crate::cf;

/// cf::TimeInterval derived by calling mach_absolute_time()
/// and converting the result to seconds.
/// ```
/// use cidre::ca;
///
/// println!("{:?}", ca::current_media_time());
/// ```
#[inline]
pub fn current_media_time() -> cf::TimeInterval {
    unsafe { CACurrentMediaTime() }
}

#[link(name = "QuartzCore", kind = "framework")]
extern "C" {
    fn CACurrentMediaTime() -> cf::TimeInterval;
}
