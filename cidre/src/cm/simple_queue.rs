use std::{ffi::c_void, num::NonZeroU16};

use crate::{arc, cf, define_cf_type, os};

define_cf_type!(SimpleQueue(cf::Type));

#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Error(pub os::Status);

impl Error {
    /// An allocation failed.
    pub const ALLOCATION_FAILED: Self = Self(os::Status(-12770));

    /// NULL or 0 was passed for a required parameter.
    pub const REQUIRED_PARAMETER_MISSING: Self = Self(os::Status(-12771));

    /// An out-of-range value was passed for a parameter with a restricted valid range.
    pub const PARAMETER_OUT_OF_RANGE: Self = Self(os::Status(-12772));

    /// Operation failed because queue was full.
    pub const QUEUE_IS_FULL: Self = Self(os::Status(-12773));
}

impl SimpleQueue {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CMSimpleQueueGetTypeID() }
    }

    #[inline]
    pub unsafe fn create_in(
        capacity: i32,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<Self>, Error> {
        let mut out = None;
        let res = CMSimpleQueueCreate(allocator, capacity, &mut out);
        std::mem::transmute(res.to_result_unchecked(out))
    }

    #[inline]
    pub fn with_capacity_in(
        capacity: NonZeroU16,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<Self>, Error> {
        unsafe { Self::create_in(capacity.get() as _, allocator) }
    }

    #[inline]
    pub fn with_capacity(capacity: NonZeroU16) -> Result<arc::R<Self>, Error> {
        Self::with_capacity_in(capacity, None)
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { CMSimpleQueueGetCount(self) as _ }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn enqueue(&mut self, element: *const c_void) -> Result<(), Error> {
        unsafe { std::mem::transmute(CMSimpleQueueEnqueue(self, element).result()) }
    }

    #[inline]
    pub fn dequeue(&mut self) -> *const c_void {
        unsafe { CMSimpleQueueDequeue(self) }
    }

    #[inline]
    pub fn head(&self) -> *const c_void {
        unsafe { CMSimpleQueueGetHead(self) }
    }

    #[inline]
    pub fn reset(&mut self) -> Result<(), Error> {
        unsafe { std::mem::transmute(CMSimpleQueueReset(self).result()) }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        unsafe { CMSimpleQueueGetCapacity(self) as _ }
    }

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

extern "C" {
    fn CMSimpleQueueGetTypeID() -> cf::TypeId;
    fn CMSimpleQueueCreate(
        allocator: Option<&cf::Allocator>,
        capacity: i32,
        queue_out: &mut Option<arc::R<SimpleQueue>>,
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
        assert_eq!(err, cm::SimpleQueueError::QUEUE_IS_FULL);

        let r = queue.head() as usize;
        assert_eq!(r, 5usize);

        let r = queue.dequeue() as usize;
        assert_eq!(r, 5usize);
        assert!(queue.is_empty());
        assert_eq!(0.0f32, queue.fullness());

        queue.reset().unwrap();
    }
}
