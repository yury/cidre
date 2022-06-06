use std::ffi::{c_void, CStr};
use std::intrinsics::transmute;
use std::os::raw::{c_char, c_long};
use std::ptr::NonNull;

use crate::cf::Retained;
use crate::define_obj_type;
use crate::dispatch::{Function, Object};

define_obj_type!(Queue(Object));
define_obj_type!(Global(Queue));
define_obj_type!(Serial(Queue));
define_obj_type!(Main(Serial));
define_obj_type!(Concurent(Queue));

define_obj_type!(Attr(Object));

#[repr(transparent)]
pub struct QOSClass(pub u32);

impl QOSClass {
    pub const USER_INTERACTIVE: Self = Self(0x21);
    pub const USER_INITIATED: Self = Self(0x19);
    pub const DEFAULT: Self = Self(0x15);
    pub const UTILITY: Self = Self(0x11);
    pub const BACKGROUND: Self = Self(0x09);
    pub const UNSPECIFIED: Self = Self(0x00);

    pub const QOS_MIN_RELATIVE_PRIORITY: i32 = -15;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(pub c_long);

impl Priority {
    pub const HIGH: Self = Self(2);
    pub const DEFAULT: Self = Self(0);
    pub const LOW: Self = Self(-2);
    pub const BACKGROUND: Self = Self(-1 << 15);
}

#[repr(usize)]
pub enum AutoreleaseFrequency {
    Inherit = 0,
    WorkItem = 1,
    Never = 2,
}

/// ```
/// use cidre::dispatch;
///
/// let q = dispatch::Queue::main();
///
/// q.as_type_ref().show();
/// ```
impl Queue {
    #[inline]
    pub fn new<'a>() -> Retained<'a, Queue> {
        Self::with_label_and_attrs(None, None)
    }

    #[inline]
    pub fn serial_with_autoreleasepool<'a>() -> Retained<'a, Queue> {
        let attr = Attr::serial_with_autoreleasepool();
        Self::with_label_and_attrs(None, Some(&attr))
    }

    #[inline]
    pub fn with_label_and_attrs<'a>(
        label: Option<&CStr>,
        attr: Option<&Attr>,
    ) -> Retained<'a, Queue> {
        unsafe {
            let label = label.map(|f| NonNull::new_unchecked(f.as_ptr() as *mut _));
            dispatch_queue_create(label, attr)
        }
    }

    #[inline]
    pub fn main<'a>() -> &'a Queue {
        Main::default()
    }

    /// ```
    /// use cidre::dispatch;
    ///
    /// let q = dispatch::Queue::global_with_qos(dispatch::QOSClass::BACKGROUND).unwrap();
    ///
    /// ```
    #[inline]
    pub fn global_with_qos<'a>(qos: QOSClass) -> Option<&'a Global> {
        unsafe { Self::global_with_flags(qos.0 as _, 0) }
    }

    /// ```
    /// use cidre::dispatch;
    ///
    /// let q = dispatch::Queue::global_with_priority(dispatch::QueuePriority::BACKGROUND).unwrap();
    ///
    /// ```
    #[inline]
    pub fn global_with_priority<'a>(priority: Priority) -> Option<&'a Global> {
        unsafe { Self::global_with_flags(priority.0 as _, 0) }
    }

    #[inline]
    pub fn global<'a>(identifier: isize) -> Option<&'a Global> {
        unsafe { Self::global_with_flags(identifier, 0) }
    }

    #[inline]
    pub unsafe fn global_with_flags<'a>(identifier: isize, flags: usize) -> Option<&'a Global> {
        dispatch_get_global_queue(identifier, flags)
    }

    #[inline]
    pub fn async_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_async_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn sync_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_sync_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn async_and_wait_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_async_and_wait_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn after_f<T>(&self, when: super::Time, context: *mut T, work: Function<T>) {
        unsafe { dispatch_after_f(when, self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn barrier_async_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_barrier_async_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn barrier_sync_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_barrier_sync_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn barrier_async_and_wait_f<T>(&self, context: *mut T, work: Function<T>) {
        unsafe { dispatch_barrier_async_and_wait_f(self, context as _, transmute(work)) }
    }

    #[inline]
    pub fn group_async_f<T>(&self, group: &super::Group, context: *mut T, work: Function<T>) {
        unsafe { dispatch_group_async_f(group, self, context as _, transmute(work)) }
    }
}

impl Main {
    #[inline]
    fn default<'a>() -> &'a Main {
        unsafe { &_dispatch_main_q }
    }
}

impl Attr {
    #[inline]
    pub fn serial<'a>() -> Option<&'a Attr> {
        None
    }

    #[inline]
    pub fn concurrent<'a>() -> Option<&'a Attr> {
        unsafe { Some(&_dispatch_queue_attr_concurrent) }
    }

    #[inline]
    pub fn serial_inactive<'a>() -> Retained<'a, Attr> {
        Self::make_initially_inactive(Self::serial())
    }

    #[inline]
    pub fn concurrent_inactive<'a>() -> Retained<'a, Attr> {
        Self::make_initially_inactive(Self::concurrent())
    }

    #[inline]
    pub fn serial_with_autoreleasepool<'a>() -> Retained<'a, Attr> {
        Self::make_with_autorelease_frequencey(Self::serial(), AutoreleaseFrequency::WorkItem)
    }

    #[inline]
    pub fn concurrent_with_autoreleasepool<'a>() -> Retained<'a, Attr> {
        Self::make_with_autorelease_frequencey(Self::concurrent(), AutoreleaseFrequency::WorkItem)
    }

    #[inline]
    pub fn make_with_autorelease_frequencey<'a>(
        attr: Option<&Attr>,
        frequency: AutoreleaseFrequency,
    ) -> Retained<'a, Self> {
        unsafe { dispatch_queue_attr_make_with_autorelease_frequency(attr, frequency) }
    }

    #[inline]
    pub fn make_initially_inactive<'a>(attr: Option<&Attr>) -> Retained<'a, Attr> {
        unsafe { dispatch_queue_attr_make_initially_inactive(attr) }
    }

    #[inline]
    pub fn make_with_qos_class<'a>(
        attr: Option<&Attr>,
        qos_class: QOSClass,
        relative_priority: i32,
    ) -> Retained<'a, Attr> {
        unsafe { dispatch_queue_attr_make_with_qos_class(attr, qos_class, relative_priority) }
    }

    #[inline]
    pub fn initially_inactive<'a>(&self) -> Retained<'a, Attr> {
        unsafe { dispatch_queue_attr_make_initially_inactive(Some(self)) }
    }

    #[inline]
    pub fn with_autorelease_frequencey<'a>(
        &self,
        frequency: AutoreleaseFrequency,
    ) -> Retained<'a, Attr> {
        unsafe { dispatch_queue_attr_make_with_autorelease_frequency(Some(self), frequency) }
    }
}

extern "C" {
    static _dispatch_main_q: Main;
    static _dispatch_queue_attr_concurrent: Attr;

    fn dispatch_async_f(queue: &Queue, context: *mut c_void, work: Function<c_void>);
    fn dispatch_sync_f(queue: &Queue, context: *mut c_void, work: Function<c_void>);
    fn dispatch_queue_create<'a>(
        label: Option<NonNull<c_char>>,
        attr: Option<&Attr>,
    ) -> Retained<'a, Queue>;

    fn dispatch_async_and_wait_f(queue: &Queue, context: *mut c_void, work: Function<c_void>);

    fn dispatch_queue_attr_make_initially_inactive<'a>(attr: Option<&Attr>) -> Retained<'a, Attr>;
    fn dispatch_queue_attr_make_with_qos_class<'a>(
        attr: Option<&Attr>,
        qos_class: QOSClass,
        relative_priority: i32,
    ) -> Retained<'a, Attr>;
    fn dispatch_queue_attr_make_with_autorelease_frequency<'a>(
        attr: Option<&Attr>,
        frequency: AutoreleaseFrequency,
    ) -> Retained<'a, Attr>;

    fn dispatch_after_f(
        when: crate::dispatch::Time,
        queue: &Queue,
        context: *mut c_void,
        work: Function<c_void>,
    );
    fn dispatch_barrier_async_f(queue: &Queue, context: *mut c_void, work: Function<c_void>);
    fn dispatch_barrier_sync_f(queue: &Queue, context: *mut c_void, work: Function<c_void>);
    fn dispatch_barrier_async_and_wait_f(
        queue: &Queue,
        context: *mut c_void,
        work: Function<c_void>,
    );

    fn dispatch_group_async_f(
        group: &super::Group,
        queue: &Queue,
        context: *mut c_void,
        work: Function<c_void>,
    );
    fn dispatch_get_global_queue<'a>(identifier: isize, flags: usize) -> Option<&'a Global>;
}

#[cfg(test)]
mod tests {

    use std::ffi::c_void;

    use crate::dispatch;

    extern "C" fn foo(_ctx: *mut c_void) {
        println!("nice");
    }

    #[test]
    fn test_attrs() {
        let _attr = dispatch::Attr::make_with_autorelease_frequencey(
            None,
            dispatch::AutoreleaseFrequency::Never,
        );
    }

    #[test]
    fn test_queue() {
        let q = dispatch::Queue::new();

        q.as_type_ref().show();

        q.sync_f(std::ptr::null_mut(), foo);

        q.async_and_wait_f(std::ptr::null_mut(), foo);
    }

    #[test]
    fn test_global_queue() {
        let q = dispatch::Queue::global_with_qos(dispatch::QOSClass::BACKGROUND).unwrap();

        q.as_type_ref().show();
        q.sync_f(std::ptr::null_mut(), foo);
        q.async_and_wait_f(std::ptr::null_mut(), foo);

        let q = dispatch::Queue::global_with_priority(dispatch::QueuePriority::HIGH).unwrap();

        q.as_type_ref().show();
        q.sync_f(std::ptr::null_mut(), foo);
        q.async_and_wait_f(std::ptr::null_mut(), foo);
    }
}
