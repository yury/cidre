use std::ffi::c_uint;

#[repr(C)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Class(pub c_uint);

impl Class {
    /// A QOS class which indicates work performed by this thread
    /// is interactive with the user.
    ///
    /// Such work is requested to run at high priority relative to other
    /// work on the system. Specifying this QOS class is a request to run with
    /// nearly all available system CPU and I/O bandwidth even under contention.
    /// This is not an energy-efficient QOS class to use for large tasks. The use of
    /// this QOS class should be limited to critical interaction with the user such
    /// as handling events on the main event loop, view drawing, animation, etc.
    #[doc(alias = "QOS_CLASS_USER_INTERACTIVE")]
    pub const USER_INTERACTIVE: Self = Self(0x21);

    /// A QOS class which indicates work performed by this thread
    /// was initiated by the user and that the user is likely waiting for the
    /// results.
    ///
    /// Such work is requested to run at a priority below critical user-
    /// interactive work, but relatively higher than other work on the system. This
    /// is not an energy-efficient QOS class to use for large tasks. Its use
    /// should be limited to operations of short enough duration that the user is
    /// unlikely to switch tasks while waiting for the results. Typical
    /// user-initiated work will have progress indicated by the display of
    /// placeholder content or modal user interface.
    #[doc(alias = "QOS_CLASS_USER_INITIATED")]
    pub const USER_INITIATED: Self = Self(0x19);

    /// A default QOS class used by the system in cases where more specific
    /// QOS class information is not available.
    ///
    /// Such work is requested to run at a priority below critical user-
    /// interactive and user-initiated work, but relatively higher than utility and
    /// background tasks. Threads created by pthread_create() without an attribute
    /// specifying a QOS class will default to QOS_CLASS_DEFAULT. This QOS class
    /// value is not intended to be used as a work classification, it should only be
    /// set when propagating or restoring QOS class values provided by the system.
    #[doc(alias = "QOS_CLASS_DEFAULT")]
    pub const DEFAULT: Self = Self(0x15);

    /// A QOS class which indicates work performed by this thread
    /// may or may not be initiated by the user and that the user is unlikely to be
    /// immediately waiting for the results.
    ///
    /// Such work is requested to run at a priority below critical user-
    /// interactive and user-initiated work, but relatively higher than low-level
    /// system maintenance tasks. The use of this QOS class indicates the work
    /// should be run in an energy and thermally-efficient manner. The progress of
    /// utility work may or may not be indicated to the user, but the effect of such
    /// work is user-visible.
    #[doc(alias = "QOS_CLASS_UTILITY")]
    pub const UTILITY: Self = Self(0x11);

    /// A QOS class which indicates work performed by this thread was not
    /// initiated by the user and that the user may be unaware of the results.
    ///
    /// Such work is requested to run at a priority below other work.
    /// The use of this QOS class indicates the work should be run in the most energy
    /// and thermally-efficient manner.
    #[doc(alias = "QOS_CLASS_BACKGROUND")]
    pub const BACKGROUND: Self = Self(0x09);

    /// A QOS class value which indicates the absence or removal of QOS
    /// class information.
    ///
    /// As an API return value, may indicate that threads or pthread
    /// attributes were configured with legacy API incompatible or in conflict with
    /// the QOS class system.
    #[doc(alias = "QOS_CLASS_UNSPECIFIED")]
    pub const UNSPECIFIED: Self = Self(0x00);
}

/// The minimum relative priority that may be specified within a
/// QOS class. These priorities are relative only within a given QOS class
/// and meaningful only for the current process.
pub const MIN_RELATIVE_PRIORITY: i32 = -15;

impl Class {
    #[doc(alias = "qos_class_self")]
    pub fn current() -> Self {
        unsafe { qos_class_self() }
    }

    #[doc(alias = "qos_class_main")]
    pub fn main() -> Self {
        unsafe { qos_class_main() }
    }
}

unsafe extern "C" {
    fn qos_class_self() -> Class;
    fn qos_class_main() -> Class;
}

#[cfg(test)]
mod tests {
    use crate::sys::qos;

    #[test]
    fn basics() {
        let class = qos::Class::current();
        assert_eq!(class, qos::Class::DEFAULT);

        let class = qos::Class::main();
        assert_eq!(class, qos::Class::USER_INTERACTIVE);
    }
}
