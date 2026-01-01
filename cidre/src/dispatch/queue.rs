use std::ffi::{CStr, c_char, c_long, c_void};

use crate::{arc, define_obj_type, dispatch};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "dispatch_queue")]
    #[doc(alias = "dispatch_queue_t")]
    #[doc(alias = "DispatchQueue")]
    pub Queue(dispatch::Object)
);

define_obj_type!(
    #[doc(alias = "dispatch_queue_global")]
    #[doc(alias = "dispatch_queue_global_t")]
    pub Global(Queue)
);

define_obj_type!(
    #[doc(alias = "dispatch_queue_serial")]
    #[doc(alias = "dispatch_queue_serial_t")]
    pub Serial(Queue)
);

define_obj_type!(
    #[doc(alias = "dispatch_queue_main")]
    #[doc(alias = "dispatch_queue_main_t")]
    pub Main(Serial)
);

define_obj_type!(
    #[doc(alias = "dispatch_queue_concurrent")]
    #[doc(alias = "dispatch_queue_concurrent_t")]
    pub Concurrent(Queue)
);

define_obj_type!(
    #[doc(alias = "dispatch_queue_attr")]
    #[doc(alias = "dispatch_queue_attr_t")]
    pub Attr(dispatch::Object)
);

#[doc(alias = "DispatchQoS")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct QosClass(pub u32);

impl QosClass {
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
    /// Items dispatched to the queue will run at high priority,
    /// i.e. the queue will be scheduled for execution before
    /// any default priority or low priority queue.
    pub const HIGH: Self = Self(2);

    /// Items dispatched to the queue will run at the default
    /// priority, i.e. the queue will be scheduled for execution
    /// after all high priority queues have been scheduled, but
    /// before any low priority queues have been scheduled.
    pub const DEFAULT: Self = Self(0);

    /// Items dispatched to the queue will run at low priority,
    /// i.e. the queue will be scheduled for execution after all
    /// default priority and high priority queues have been
    /// scheduled.   
    pub const LOW: Self = Self(-2);

    /// Items dispatched to the queue will run at background priority, i.e. the queue
    /// will be scheduled for execution after all higher priority queues have been
    /// scheduled and the system will run items on this queue on a thread with
    /// background status as per setpriority(2) (i.e. disk I/O is throttled and the
    /// thread's scheduling priority is set to lowest value).
    pub const BACKGROUND: Self = Self(-1 << 15);
}

#[doc(alias = "dispatch_autorelease_frequency_t")]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(usize)]
pub enum AutoreleaseFrequency {
    /// Dispatch queues with this autorelease frequency inherit the behavior from
    /// their target queue. This is the default behavior for manually created queues.
    #[doc(alias = "DISPATCH_AUTORELEASE_FREQUENCY_INHERIT")]
    #[doc(alias = "DispatchQueue.AutoreleaseFrequency.inherit")]
    Inherit = 0,

    /// Dispatch queues with this autorelease frequency push and pop an autorelease
    /// pool around the execution of every block that was submitted to it
    /// asynchronously.
    #[doc(alias = "DISPATCH_AUTORELEASE_FREQUENCY_WORK_ITEM")]
    #[doc(alias = "DispatchQueue.AutoreleaseFrequency.workItem")]
    WorkItem = 1,

    /// Dispatch queues with this autorelease frequency never set up an individual
    /// autorelease pool around the execution of a block that is submitted to it
    /// asynchronously. This is the behavior of the global concurrent queues.
    #[doc(alias = "DISPATCH_AUTORELEASE_FREQUENCY_NEVER")]
    #[doc(alias = "DispatchQueue.AutoreleaseFrequency.never")]
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
    /// Serial queue
    #[inline]
    pub fn new() -> arc::R<Self> {
        let attr = Attr::serial();
        Self::with_label_and_attrs(None::<&CStr>, attr)
    }

    /// Concurrent queue
    #[inline]
    pub fn concurrent() -> arc::R<Self> {
        let attr = Attr::concurrent();
        Self::with_label_and_attrs(None::<&CStr>, attr)
    }

    #[inline]
    pub fn serial_with_ar_pool() -> arc::R<Self> {
        let attr = Attr::serial_with_ar_pool();
        Self::with_label_and_attrs(None::<&CStr>, Some(&attr))
    }

    #[inline]
    pub fn concurrent_with_ar_pool() -> arc::R<Self> {
        let attr = Attr::concurrent_with_ar_pool();
        Self::with_label_and_attrs(None::<&CStr>, Some(&attr))
    }

    #[inline]
    pub fn concurrent_without_ar_pool() -> arc::R<Self> {
        let attr = Attr::concurrent_without_ar_pool();
        Self::with_label_and_attrs(None::<&CStr>, Some(&attr))
    }

    #[inline]
    pub fn with_label_and_attrs(
        label: Option<impl AsRef<CStr>>,
        attr: Option<&Attr>,
    ) -> arc::R<Self> {
        unsafe {
            let label = label.map_or(std::ptr::null(), |f| f.as_ref().as_ptr());
            dispatch_queue_create(label, attr)
        }
    }

    #[inline]
    pub fn main() -> &'static Main {
        Main::default()
    }

    /// ```
    /// use cidre::dispatch;
    ///
    /// let q = dispatch::Queue::global_with_qos(dispatch::QosClass::BACKGROUND).unwrap();
    ///
    /// ```
    #[inline]
    pub fn global_with_qos(qos: QosClass) -> Option<&'static Global> {
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

    #[doc(alias = "dispatch_get_global_queue")]
    #[inline]
    pub unsafe fn global_with_flags<'a>(identifier: isize, flags: usize) -> Option<&'a Global> {
        unsafe { dispatch_get_global_queue(identifier, flags) }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "dispatch_sync")]
    #[inline]
    pub fn sync_b(&self, block: &mut dispatch::Block<blocks::NoEsc>) {
        unsafe {
            dispatch_sync(self, block);
        }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "dispatch_async")]
    #[inline]
    pub fn async_b(&self, block: &mut dispatch::Block) {
        unsafe { dispatch_async(self, block) };
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn sync_mut(&self, mut f: impl FnMut()) {
        let mut block = unsafe { dispatch::Block::<blocks::NoEsc>::stack0(&mut f) };
        self.sync_b(&mut block);
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn sync_once<R>(&self, f: impl FnOnce() -> R) -> R {
        let mut closure = Some(f);
        let mut result = None;

        self.sync_mut(|| {
            let closure = unsafe { closure.take().unwrap_unchecked() };
            result.replace(closure());
        });

        unsafe { result.unwrap_unchecked() }
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn sync<R>(&self, mut f: impl FnMut() -> R) -> R {
        let mut result = None;
        let closure = || {
            result.replace(f());
        };
        self.sync_mut(closure);
        unsafe { result.unwrap_unchecked() }
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn sync_fn(&self, block: extern "C" fn(*const c_void)) {
        let mut block = blocks::StaticBlock::new0(block);
        self.sync_b(block.as_noesc_mut());
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn async_mut(&self, block: impl FnMut() + Send + 'static) {
        let mut block = dispatch::Block::<blocks::Send>::new0(block);
        self.async_b(&mut block);
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn async_once(&self, block: impl FnOnce() + Send + 'static) {
        let mut block = Some(block);
        let mut block = dispatch::Block::<blocks::Send>::new0(move || {
            if let Some(block) = block.take() {
                block();
            }
        });
        self.async_b(&mut block);
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn async_fn(&self, block: extern "C" fn(*const c_void)) {
        let mut block = blocks::StaticBlock::new0(block);
        self.async_b(block.as_send_mut());
    }

    #[doc(alias = "dispatch_async_f")]
    #[inline]
    pub fn async_f<T>(&self, context: *mut T, work: dispatch::Fn<T>) {
        unsafe { dispatch_async_f(self, context as _, std::mem::transmute(work)) }
    }

    #[doc(alias = "dispatch_sync_f")]
    #[inline]
    pub fn sync_f<T>(&self, context: *mut T, work: dispatch::Fn<T>) {
        unsafe { dispatch_sync_f(self, context as _, std::mem::transmute(work)) }
    }

    #[doc(alias = "dispatch_async_and_wait_f")]
    #[inline]
    pub fn async_and_wait_f<T>(&self, context: *mut T, work: dispatch::Fn<T>) {
        unsafe { dispatch_async_and_wait_f(self, context as _, std::mem::transmute(work)) }
    }

    #[doc(alias = "dispatch_after_f")]
    #[inline]
    pub fn after_f<T>(&self, when: super::Time, context: *mut T, work: dispatch::Fn<T>) {
        unsafe { dispatch_after_f(when, self, context as _, std::mem::transmute(work)) }
    }

    #[doc(alias = "dispatch_barrier_async_f")]
    #[inline]
    pub fn barrier_async_f<T>(&self, context: *mut T, work: dispatch::Fn<T>) {
        unsafe { dispatch_barrier_async_f(self, context as _, std::mem::transmute(work)) }
    }

    #[doc(alias = "dispatch_barrier_sync_f")]
    #[inline]
    pub fn barrier_sync_f<T>(&self, context: *mut T, work: dispatch::Fn<T>) {
        unsafe { dispatch_barrier_sync_f(self, context as _, std::mem::transmute(work)) }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "dispatch_barrier_sync")]
    #[inline]
    pub fn barrier_sync_b(&self, block: &mut dispatch::Block<blocks::NoEsc>) {
        unsafe {
            dispatch_barrier_sync(self, block);
        }
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn barrier_sync_mut(&self, mut f: impl FnMut()) {
        let mut block = unsafe { dispatch::Block::<blocks::NoEsc>::stack0(&mut f) };
        self.barrier_sync_b(&mut block);
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn barrier_sync<R>(&self, mut f: impl FnMut() -> R) -> R {
        let mut result = None;
        let closure = || {
            result.replace(f());
        };
        self.barrier_sync_mut(closure);
        unsafe { result.unwrap_unchecked() }
    }

    #[doc(alias = "dispatch_barrier_async_and_wait_f")]
    #[inline]
    pub fn barrier_async_and_wait_f<T>(&self, context: *mut T, work: dispatch::Fn<T>) {
        unsafe { dispatch_barrier_async_and_wait_f(self, context as _, std::mem::transmute(work)) }
    }

    #[doc(alias = "dispatch_group_async_f")]
    #[inline]
    pub fn group_async_f<T>(&self, group: &super::Group, context: *mut T, work: dispatch::Fn<T>) {
        unsafe { dispatch_group_async_f(group, self, context as _, std::mem::transmute(work)) }
    }

    pub const TARGET_QUEUE_DEFAULT: Option<&'static Self> = None;
}

impl Main {
    #[inline]
    fn default() -> &'static Main {
        unsafe { &_dispatch_main_q }
    }
}

impl Attr {
    #[inline]
    pub fn serial() -> Option<&'static Attr> {
        None
    }

    #[inline]
    pub fn concurrent() -> Option<&'static Attr> {
        unsafe { Some(&_dispatch_queue_attr_concurrent) }
    }

    #[inline]
    pub fn serial_inactive() -> arc::R<Attr> {
        Self::make_initially_inactive(Self::serial())
    }

    #[inline]
    pub fn concurrent_inactive() -> arc::R<Attr> {
        Self::make_initially_inactive(Self::concurrent())
    }

    #[inline]
    pub fn serial_with_ar_pool() -> arc::R<Attr> {
        Self::make_with_ar_frequency(Self::serial(), AutoreleaseFrequency::WorkItem)
    }

    #[inline]
    pub fn concurrent_with_ar_pool() -> arc::R<Attr> {
        Self::make_with_ar_frequency(Self::concurrent(), AutoreleaseFrequency::WorkItem)
    }

    #[inline]
    pub fn concurrent_without_ar_pool() -> arc::R<Attr> {
        Self::make_with_ar_frequency(Self::concurrent(), AutoreleaseFrequency::Never)
    }

    #[inline]
    pub fn make_with_ar_frequency(
        attr: Option<&Attr>,
        frequency: AutoreleaseFrequency,
    ) -> arc::R<Self> {
        unsafe { dispatch_queue_attr_make_with_autorelease_frequency(attr, frequency) }
    }

    #[inline]
    pub fn make_initially_inactive(attr: Option<&Attr>) -> arc::R<Attr> {
        unsafe { dispatch_queue_attr_make_initially_inactive(attr) }
    }

    #[inline]
    pub fn make_with_qos_class(
        attr: Option<&Attr>,
        qos_class: QosClass,
        relative_priority: i32,
    ) -> arc::R<Attr> {
        unsafe { dispatch_queue_attr_make_with_qos_class(attr, qos_class, relative_priority) }
    }

    pub fn serial_with_qos_class(qos_class: QosClass, relative_priority: i32) -> arc::R<Attr> {
        Self::make_with_qos_class(Self::serial(), qos_class, relative_priority)
    }

    #[inline]
    pub fn initially_inactive(&self) -> arc::R<Attr> {
        unsafe { dispatch_queue_attr_make_initially_inactive(Some(self)) }
    }

    #[inline]
    pub fn with_autorelease_frequencey(&self, frequency: AutoreleaseFrequency) -> arc::R<Attr> {
        unsafe { dispatch_queue_attr_make_with_autorelease_frequency(Some(self), frequency) }
    }
}

#[link(name = "System", kind = "dylib")]
unsafe extern "C-unwind" {
    static _dispatch_main_q: Main;
    static _dispatch_queue_attr_concurrent: Attr;

    #[cfg(feature = "blocks")]
    fn dispatch_sync(queue: &Queue, block: &mut dispatch::Block<blocks::NoEsc>);
    #[cfg(feature = "blocks")]
    fn dispatch_async(queue: &Queue, block: &mut dispatch::Block);

    fn dispatch_async_f(queue: &Queue, context: *mut c_void, work: dispatch::Fn<c_void>);
    fn dispatch_sync_f(queue: &Queue, context: *mut c_void, work: dispatch::Fn<c_void>);
    fn dispatch_queue_create(label: *const c_char, attr: Option<&Attr>) -> arc::R<Queue>;

    fn dispatch_async_and_wait_f(queue: &Queue, context: *mut c_void, work: dispatch::Fn<c_void>);

    fn dispatch_queue_attr_make_initially_inactive(attr: Option<&Attr>) -> arc::R<Attr>;
    fn dispatch_queue_attr_make_with_qos_class(
        attr: Option<&Attr>,
        qos_class: QosClass,
        relative_priority: i32,
    ) -> arc::R<Attr>;
    fn dispatch_queue_attr_make_with_autorelease_frequency(
        attr: Option<&Attr>,
        frequency: AutoreleaseFrequency,
    ) -> arc::R<Attr>;

    fn dispatch_after_f(
        when: crate::dispatch::Time,
        queue: &Queue,
        context: *mut c_void,
        work: dispatch::Fn<c_void>,
    );
    #[cfg(feature = "blocks")]
    fn dispatch_barrier_sync(queue: &Queue, block: &mut dispatch::Block<blocks::NoEsc>);
    fn dispatch_barrier_async_f(queue: &Queue, context: *mut c_void, work: dispatch::Fn<c_void>);
    fn dispatch_barrier_sync_f(queue: &Queue, context: *mut c_void, work: dispatch::Fn<c_void>);
    fn dispatch_barrier_async_and_wait_f(
        queue: &Queue,
        context: *mut c_void,
        work: dispatch::Fn<c_void>,
    );

    fn dispatch_group_async_f(
        group: &super::Group,
        queue: &Queue,
        context: *mut c_void,
        work: dispatch::Fn<c_void>,
    );
    fn dispatch_get_global_queue<'a>(identifier: isize, flags: usize) -> Option<&'a Global>;
}

#[cfg(test)]
mod tests {

    use std::ffi::c_void;

    use crate::dispatch;

    extern "C-unwind" fn foo(_ctx: *mut c_void) {
        println!("nice");
    }

    #[test]
    fn attrs() {
        let _attr =
            dispatch::Attr::make_with_ar_frequency(None, dispatch::AutoreleaseFrequency::Never);
    }

    #[test]
    fn queue() {
        let q = dispatch::Queue::new();

        q.as_type_ref().show();

        q.sync_f(std::ptr::null_mut(), foo);

        q.async_and_wait_f(std::ptr::null_mut(), foo);
    }

    #[derive(Debug)]
    struct Foo {}

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("drop!")
        }
    }

    #[test]
    fn sync_block() {
        let q = dispatch::Queue::new();

        let foo = Foo {};
        q.as_type_ref().show();
        let b = move || {
            println!("nice! {:?}", foo);
        };
        q.async_mut(b);

        let foo2 = Foo {};
        q.sync_mut(move || {
            println!("nice {:?}", foo2);
        });
    }

    #[test]
    fn global_queue() {
        let q = dispatch::Queue::global_with_qos(dispatch::QosClass::BACKGROUND).unwrap();

        q.as_type_ref().show();
        q.sync_f(std::ptr::null_mut(), foo);
        q.async_and_wait_f(std::ptr::null_mut(), foo);

        let q = dispatch::Queue::global_with_priority(dispatch::QueuePriority::HIGH).unwrap();

        q.as_type_ref().show();
        q.sync_f(std::ptr::null_mut(), foo);
        q.async_and_wait_f(std::ptr::null_mut(), foo);
    }

    #[test]
    fn sync() {
        let q = dispatch::Queue::new();
        let res = q.sync(|| 10);

        assert_eq!(res, 10);
    }
}
