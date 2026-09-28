/// `Swift.Duration`: a signed count of attoseconds.
///
/// The standard library declares it frozen as two words, the low half of a
/// 128-bit count and the high half, so it crosses a call in two integer
/// registers and converts to and from [`std::time::Duration`] without calling
/// into Swift.
#[doc(alias = "Swift.Duration")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Duration {
    low: u64,
    high: i64,
}

impl Duration {
    const ATTOS_PER_SEC: i128 = 1_000_000_000_000_000_000;
    const ATTOS_PER_NANO: i128 = 1_000_000_000;

    #[inline]
    pub const fn from_attoseconds(attoseconds: i128) -> Self {
        Self {
            low: attoseconds as u64,
            high: (attoseconds >> 64) as i64,
        }
    }

    #[inline]
    pub const fn attoseconds(&self) -> i128 {
        ((self.high as i128) << 64) | self.low as i128
    }

    #[inline]
    pub const fn secs(secs: i64) -> Self {
        Self::from_attoseconds(secs as i128 * Self::ATTOS_PER_SEC)
    }

    #[inline]
    pub fn secs_f64(secs: f64) -> Self {
        Self::from_attoseconds((secs * Self::ATTOS_PER_SEC as f64) as i128)
    }
}

impl From<std::time::Duration> for Duration {
    #[inline]
    fn from(value: std::time::Duration) -> Self {
        Self::from_attoseconds(
            value.as_secs() as i128 * Self::ATTOS_PER_SEC
                + value.subsec_nanos() as i128 * Self::ATTOS_PER_NANO,
        )
    }
}

unsafe impl super::SwiftAbi for Duration {
    const CLASS: super::AbiClass = super::AbiClass::Words2;
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Duration {
        #[crate::swift::call(
            "static Swift.Duration(struct).seconds(_: Double) -> Swift.Duration(struct)"
        )]
        fn swift_seconds(seconds: f64) -> Duration;

        #[crate::swift::call(
            "static Swift.Duration(struct).milliseconds(_: Double) -> Swift.Duration(struct)"
        )]
        fn swift_milliseconds(milliseconds: f64) -> Duration;
    }

    /// There is no metadata to name the type by, so `self` for a static
    /// member is the metatype, which a frozen struct's is thin and unused.
    unsafe impl crate::swift::SwiftMetadata for Duration {
        fn metadata() -> *const crate::swift::abi::TypeMetadata {
            core::ptr::null()
        }
    }

    /// The native conversion has to agree with Swift's own.
    #[test]
    fn converts_as_swift_does() {
        assert_eq!(Duration::swift_seconds(1.5), Duration::secs_f64(1.5));
        assert_eq!(Duration::swift_seconds(-2.0), Duration::secs(-2));
        assert_eq!(
            Duration::swift_milliseconds(1_250.0),
            Duration::from(std::time::Duration::from_millis(1_250))
        );
        assert_eq!(
            1_250 * 1_000_000_000_000_000,
            Duration::swift_milliseconds(1_250.0).attoseconds()
        );
    }
}
