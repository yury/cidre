use std::{ffi::c_void, num::NonZeroU16};

use crate::{arc, cf, define_cf_type, os};

define_cf_type!(
    #[doc(alias = "CMSimpleQueue")]
    SimpleQueue(cf::Type)
);

pub mod err {
    use crate::os::Error;
    /// An allocation failed.
    pub const ALLOC_FAILED: Error = Error::new_unchecked(-12770);

    /// NULL or 0 was passed for a required parameter.
    pub const REQUIRED_PARAM_MISSING: Error = Error::new_unchecked(-12771);

    /// An out-of-range value was passed for a parameter with a restricted valid range.
    pub const PARAM_OUT_OF_RANGE: Error = Error::new_unchecked(-12772);

    /// Operation failed because queue was full.
    pub const QUEUE_IS_FULL: Error = Error::new_unchecked(-12773);
}

impl SimpleQueue {
    #[doc(alias = "CMSimpleQueueGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CMSimpleQueueGetTypeID() }
    }

    #[doc(alias = "CMSimpleQueueCreate")]
    #[inline]
    pub unsafe fn create_in(
        capacity: i32,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<arc::R<Self>> {
        unsafe { os::result_unchecked(|res| CMSimpleQueueCreate(allocator, capacity, res)) }
    }

    #[doc(alias = "CMSimpleQueueCreate")]
    #[inline]
    pub fn with_capacity_in(
        capacity: NonZeroU16,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<arc::R<Self>> {
        unsafe { Self::create_in(capacity.get() as _, allocator) }
    }

    #[doc(alias = "CMSimpleQueueCreate")]
    #[inline]
    pub fn with_capacity(capacity: NonZeroU16) -> os::Result<arc::R<Self>> {
        Self::with_capacity_in(capacity, None)
    }

    #[doc(alias = "CMSimpleQueueGetCount")]
    #[inline]
    pub fn len(&self) -> usize {
        self.count() as _
    }

    #[doc(alias = "CMSimpleQueueGetCount")]
    #[inline]
    pub fn count(&self) -> i32 {
        unsafe { CMSimpleQueueGetCount(self) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[doc(alias = "CMSimpleQueueEnqueue")]
    #[inline]
    pub fn enqueue(&mut self, element: *const c_void) -> os::Result {
        unsafe { std::mem::transmute(CMSimpleQueueEnqueue(self, element)) }
    }

    #[doc(alias = "CMSimpleQueueDequeue")]
    #[inline]
    pub fn dequeue(&mut self) -> *const c_void {
        unsafe { CMSimpleQueueDequeue(self) }
    }

    #[doc(alias = "CMSimpleQueueGetHead")]
    #[inline]
    pub fn head(&self) -> *const c_void {
        unsafe { CMSimpleQueueGetHead(self) }
    }

    #[doc(alias = "CMSimpleQueueReset")]
    #[inline]
    pub fn reset(&mut self) -> os::Result {
        unsafe { CMSimpleQueueReset(self).result() }
    }

    #[doc(alias = "CMSimpleQueueGetCapacity")]
    #[inline]
    pub fn capacity(&self) -> usize {
        unsafe { CMSimpleQueueGetCapacity(self) as _ }
    }

    #[doc(alias = "CMSimpleQueueGetFullness")]
    #[inline]
    pub fn fullness(&self) -> f32 {
        unsafe {
            let cap = CMSimpleQueueGetCapacity(self) as f32;
            if cap > 0.0 {
                CMSimpleQueueGetCount(self) as f32 / cap
            } else {
                0.0
            }
        }
    }
}

unsafe extern "C-unwind" {
    fn CMSimpleQueueGetTypeID() -> cf::TypeId;
    fn CMSimpleQueueCreate(
        allocator: Option<&cf::Allocator>,
        capacity: i32,
        queue_out: *mut Option<arc::R<SimpleQueue>>,
    ) -> os::Status;

    fn CMSimpleQueueGetCount(queue: &SimpleQueue) -> i32;
    fn CMSimpleQueueEnqueue(queue: &mut SimpleQueue, element: *const c_void) -> os::Status;
    fn CMSimpleQueueDequeue(queue: &mut SimpleQueue) -> *const c_void;
    fn CMSimpleQueueGetHead(queue: &SimpleQueue) -> *const c_void;
    fn CMSimpleQueueReset(queue: &mut SimpleQueue) -> os::Status;
    fn CMSimpleQueueGetCapacity(queue: &SimpleQueue) -> i32;
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;

    use crate::cm;

    #[test]
    fn basics() {
        let mut queue = cm::SimpleQueue::with_capacity(NonZeroU16::new(1).unwrap()).unwrap();
        assert!(queue.is_empty());
        assert_eq!(queue.capacity(), 1);

        queue.enqueue(5usize as _).unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(1.0f32, queue.fullness());

        let err = queue.enqueue(5usize as _).expect_err("should be err");
        assert_eq!(err, cm::simple_queue_err::QUEUE_IS_FULL);

        let r = queue.head() as usize;
        assert_eq!(r, 5usize);

        let r = queue.dequeue() as usize;
        assert_eq!(r, 5usize);
        assert!(queue.is_empty());
        assert_eq!(0.0f32, queue.fullness());

        queue.reset().unwrap();
    }
}
