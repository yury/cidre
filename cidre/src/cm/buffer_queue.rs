use std::marker::PhantomData;

use crate::{arc, cf, cm, define_cf_type, os};

pub mod err {
    use crate::os::Error;

    #[doc(alias = "kCMBufferQueueError_AllocationFailed")]
    pub const ALLOC_FAILED: Error = Error::new_unchecked(-12760);

    #[doc(alias = "kCMBufferQueueError_RequiredParameterMissing")]
    pub const REQUIRED_PARAM_MISSING: Error = Error::new_unchecked(-12761);

    #[doc(alias = "kCMBufferQueueError_InvalidCMBufferCallbacksStruct")]
    pub const INVALID_CM_BUF_CALLBACKS_STRUCT: Error = Error::new_unchecked(-12762);

    #[doc(alias = "kCMBufferQueueError_EnqueueAfterEndOfData")]
    pub const ENQUEUE_AFTER_END_OF_DATA: Error = Error::new_unchecked(-12763);

    #[doc(alias = "kCMBufferQueueError_QueueIsFull")]
    pub const QUEUE_IS_FULL: Error = Error::new_unchecked(-12764);

    #[doc(alias = "kCMBufferQueueError_BadTriggerDuration")]
    pub const BAD_TRIGGER_DURATION: Error = Error::new_unchecked(-12765);

    #[doc(alias = "kCMBufferQueueError_CannotModifyQueueFromTriggerCallback")]
    pub const CANNOT_MODIFY_QUEUE_FROM_TRIGGER_CALLBACK: Error = Error::new_unchecked(-12766);

    #[doc(alias = "kCMBufferQueueError_InvalidTriggerCondition")]
    pub const INVALID_TRIGGER_CONDITION: Error = Error::new_unchecked(-12767);

    #[doc(alias = "kCMBufferQueueError_InvalidTriggerToken")]
    pub const INVALID_TRIGGER_TOKEN: Error = Error::new_unchecked(-12768);

    #[doc(alias = "kCMBufferQueueError_InvalidBuffer")]
    pub const INVALID_BUF: Error = Error::new_unchecked(-12769);
}

#[cfg(feature = "blocks")]
use crate::blocks;

define_cf_type!(
    #[doc(alias = "CMBufferQueue")]
    #[doc(alias = "CMBufferQueueRef")]
    BufQueue(cf::Type)
);

define_cf_type!(
    #[doc(alias = "CMBufferRef")]
    Buf(cf::Type)
);

pub type BufTimeCb =
    extern "C-unwind" fn(buf: &mut Buf, ref_con: *mut std::ffi::c_void) -> cm::Time;

pub type BufBoolCb = extern "C-unwind" fn(buf: &mut Buf, ref_con: *mut std::ffi::c_void) -> bool;

#[cfg(feature = "blocks")]
pub type BufTimeHandler = blocks::SyncBlock<fn(buf: &mut Buf) -> cm::Time>;

#[cfg(feature = "blocks")]
pub type BufBoolHandler = blocks::SyncBlock<fn(buf: &mut Buf) -> bool>;

pub type BufCompareCb = extern "C-unwind" fn(
    buf1: &mut Buf,
    buf2: &mut Buf,
    ref_con: *mut std::ffi::c_void,
) -> cf::ComparisonResult;

#[cfg(feature = "blocks")]
pub type BufCompareHandler =
    blocks::SyncBlock<fn(buf1: &mut Buf, buf2: &mut Buf) -> cf::ComparisonResult>;

pub type BufSizeCb = extern "C-unwind" fn(buf: &mut Buf, ref_con: *mut std::ffi::c_void) -> usize;

#[cfg(feature = "blocks")]
pub type BufSizeHandler = blocks::SyncBlock<fn(buf: &mut Buf) -> usize>;

#[doc(alias = "CMBufferCallbacks")]
#[derive(Debug, Copy, Clone)]
#[repr(C, packed(4))]
pub struct BufCbs {
    version: u32,
    decode_ts: Option<BufTimeCb>,
    pts: Option<BufTimeCb>,
    duration: Option<BufTimeCb>,
    is_ready: Option<BufBoolCb>,
    compare: Option<BufCompareCb>,
    data_became_ready_notification: *const cf::String,
    size: Option<BufSizeCb>,
}

impl BufCbs {
    #[doc(alias = "CMBufferQueueGetCallbacksForUnsortedSampleBuffers")]
    pub fn sample_bufs_unsorted() -> &'static BufCbs {
        unsafe { CMBufferQueueGetCallbacksForUnsortedSampleBuffers() }
    }

    #[doc(alias = "CMBufferQueueGetCallbacksForSampleBuffersSortedByOutputPTS")]
    pub fn sample_bufs_sorted_by_output_pts() -> &'static BufCbs {
        unsafe { CMBufferQueueGetCallbacksForSampleBuffersSortedByOutputPTS() }
    }
}

#[cfg(feature = "blocks")]
#[doc(alias = "CMBufferHandlers")]
#[repr(C)]
pub struct BufHandlers {
    version: usize,
    decode_ts: *mut BufTimeHandler,
    pts: *mut BufTimeHandler,
    duration: *mut BufTimeHandler,
    is_ready: *mut BufBoolCb,
    compare: *mut BufCompareCb,
    data_became_ready_notification: *const cf::String,
    size: *mut BufSizeHandler,
}

impl BufQueue {
    #[doc(alias = "CMBufferQueueGetTypeID")]
    pub fn type_id() -> cf::TypeId {
        unsafe { CMBufferQueueGetTypeID() }
    }

    #[doc(alias = "CMBufferQueueCreate")]
    pub fn with_callbacks_in(
        capacity: cm::ItemCount,
        callbacks: &BufCbs,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<Option<arc::R<Self>>> {
        unsafe {
            os::result_init(|queue| CMBufferQueueCreate(allocator, capacity, callbacks, queue))
        }
    }

    #[doc(alias = "CMBufferQueueCreate")]
    pub fn with_callbacks(capacity: cm::ItemCount, callbacks: &BufCbs) -> os::Result<arc::R<Self>> {
        unsafe { Ok(Self::with_callbacks_in(capacity, callbacks, None)?.unwrap_unchecked()) }
    }

    #[doc(alias = "CMBufferQueueCreateWithHandlers")]
    #[cfg(feature = "blocks")]
    pub fn with_handlers_in(
        capacity: cm::ItemCount,
        handlers: &BufHandlers,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<Option<arc::R<Self>>> {
        unsafe {
            os::result_init(|queue| {
                CMBufferQueueCreateWithHandlers(allocator, capacity, handlers, queue)
            })
        }
    }

    #[doc(alias = "CMBufferQueueCreateWithHandlers")]
    #[cfg(feature = "blocks")]
    pub fn with_handlers(
        capacity: cm::ItemCount,
        handlers: &BufHandlers,
    ) -> os::Result<arc::R<Self>> {
        unsafe { Ok(Self::with_handlers_in(capacity, handlers, None)?.unwrap_unchecked()) }
    }

    #[doc(alias = "CMBufferQueueEnqueue")]
    #[inline]
    pub fn enqueue(&mut self, buf: &Buf) -> os::Result {
        unsafe { CMBufferQueueEnqueue(self, buf).result() }
    }

    #[doc(alias = "CMBufferQueueDequeueAndRetain")]
    #[inline]
    pub fn dequeue(&mut self) -> Option<arc::R<Buf>> {
        unsafe { CMBufferQueueDequeueAndRetain(self) }
    }

    #[doc(alias = "CMBufferQueueDequeueIfDataReadyAndRetain")]
    #[inline]
    pub fn dequeue_if_data_ready(&mut self) -> Option<arc::R<Buf>> {
        unsafe { CMBufferQueueDequeueIfDataReadyAndRetain(self) }
    }

    #[cfg(any(
        all(target_os = "macos", feature = "macos_14_0"),
        all(target_os = "ios", feature = "ios_17_0"),
        all(target_os = "tvos", feature = "tvos_17_0"),
        all(target_os = "watchos", feature = "watchos_10_0"),
        all(target_os = "visionos", feature = "visionos_1_0"),
    ))]
    #[inline]
    pub fn head(&self) -> Option<arc::R<Buf>> {
        unsafe { CMBufferQueueCopyHead(self) }
    }

    #[doc(alias = "CMBufferQueueIsEmpty")]
    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { CMBufferQueueIsEmpty(self) }
    }

    #[doc(alias = "CMBufferQueueMarkEndOfData")]
    #[inline]
    pub fn mark_end_of_data(&mut self) {
        unsafe { CMBufferQueueMarkEndOfData(self) }
    }

    #[doc(alias = "CMBufferQueueContainsEndOfData")]
    #[inline]
    pub fn contains_end_of_data(&self) -> bool {
        unsafe { CMBufferQueueContainsEndOfData(self) }
    }

    #[doc(alias = "CMBufferQueueIsAtEndOfData")]
    #[inline]
    pub fn is_at_end_of_data(&self) -> bool {
        unsafe { CMBufferQueueIsAtEndOfData(self) }
    }

    #[doc(alias = "CMBufferQueueReset")]
    #[inline]
    pub fn reset(&mut self) {
        unsafe { CMBufferQueueReset(self) }
    }
}

#[derive(Debug)]
pub struct BufQueueOf<T: AsRef<Buf>> {
    inner: arc::R<BufQueue>,
    phantom: std::marker::PhantomData<T>,
}

pub type SampleBufQueue = BufQueueOf<cm::SampleBuf>;

impl<T: AsRef<Buf> + arc::Retain> BufQueueOf<T> {
    pub fn enqueue(&mut self, buf: &T) -> os::Result {
        self.inner.enqueue(buf.as_ref())
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn mark_end_of_data(&mut self) {
        self.inner.mark_end_of_data()
    }

    pub fn contains_end_of_data(&self) -> bool {
        self.inner.contains_end_of_data()
    }

    pub fn is_at_end_of_data(&self) -> bool {
        self.inner.is_at_end_of_data()
    }

    pub fn reset(&mut self) {
        self.inner.reset();
    }

    pub fn dequeue(&mut self) -> Option<arc::R<T>> {
        unsafe { std::mem::transmute(self.inner.dequeue()) }
    }

    pub fn dequeue_if_data_ready(&mut self) -> Option<arc::R<T>> {
        unsafe { std::mem::transmute(self.inner.dequeue_if_data_ready()) }
    }

    #[cfg(any(
        all(target_os = "macos", feature = "macos_14_0"),
        all(target_os = "ios", feature = "ios_17_0"),
        all(target_os = "tvos", feature = "tvos_17_0"),
        all(target_os = "watchos", feature = "watchos_10_0"),
        all(target_os = "visionos", feature = "visionos_1_0"),
    ))]
    pub fn head(&mut self) -> Option<arc::R<cm::SampleBuf>> {
        unsafe { std::mem::transmute(self.inner.head()) }
    }
}

impl BufQueueOf<cm::SampleBuf> {
    pub fn sample_bufs_unsorted(capacity: usize) -> os::Result<Self> {
        let inner = BufQueue::with_callbacks(capacity as _, BufCbs::sample_bufs_unsorted())?;
        Ok(Self {
            inner,
            phantom: PhantomData,
        })
    }

    pub fn sample_bufs_sorted_by_output_pts(capacity: usize) -> os::Result<Self> {
        let inner =
            BufQueue::with_callbacks(capacity as _, BufCbs::sample_bufs_sorted_by_output_pts())?;
        Ok(Self {
            inner,
            phantom: PhantomData,
        })
    }
}

unsafe extern "C-unwind" {
    fn CMBufferQueueGetCallbacksForUnsortedSampleBuffers() -> &'static BufCbs;
    fn CMBufferQueueGetCallbacksForSampleBuffersSortedByOutputPTS() -> &'static BufCbs;

    fn CMBufferQueueCreate(
        allocator: Option<&cf::Allocator>,
        capacity: cm::ItemCount,
        callbacks: &BufCbs,
        queue: *mut Option<arc::R<BufQueue>>,
    ) -> os::Status;

    #[cfg(feature = "blocks")]
    fn CMBufferQueueCreateWithHandlers(
        allocator: Option<&cf::Allocator>,
        capacity: cm::ItemCount,
        handlers: &BufHandlers,
        queue: *mut Option<arc::R<BufQueue>>,
    ) -> os::Status;

    fn CMBufferQueueGetTypeID() -> cf::TypeId;

    fn CMBufferQueueEnqueue(queue: &mut BufQueue, buf: &Buf) -> os::Status;

    fn CMBufferQueueDequeueAndRetain(queue: &mut BufQueue) -> Option<arc::R<Buf>>;
    fn CMBufferQueueDequeueIfDataReadyAndRetain(queue: &mut BufQueue) -> Option<arc::R<Buf>>;
    #[cfg(any(
        all(target_os = "macos", feature = "macos_14_0"),
        all(target_os = "ios", feature = "ios_17_0"),
        all(target_os = "tvos", feature = "tvos_17_0"),
        all(target_os = "watchos", feature = "watchos_10_0"),
        all(target_os = "visionos", feature = "visionos_1_0"),
    ))]
    fn CMBufferQueueCopyHead(queue: &BufQueue) -> Option<arc::R<Buf>>;
    fn CMBufferQueueIsEmpty(queue: &BufQueue) -> bool;
    fn CMBufferQueueMarkEndOfData(queue: &mut BufQueue);
    fn CMBufferQueueContainsEndOfData(queue: &BufQueue) -> bool;
    fn CMBufferQueueIsAtEndOfData(queue: &BufQueue) -> bool;
    fn CMBufferQueueReset(queue: &mut BufQueue);
}

#[cfg(test)]
mod tests {
    use crate::cm;

    #[test]
    fn basics() {
        let mut queue = cm::BufQueueOf::sample_bufs_unsorted(0).unwrap();
        assert!(queue.is_empty());
        assert!(queue.dequeue().is_none());
        assert!(queue.dequeue_if_data_ready().is_none());
        let mut queue = cm::BufQueueOf::sample_bufs_sorted_by_output_pts(0).unwrap();
        assert!(queue.is_empty());
        assert!(queue.dequeue().is_none());
        assert!(queue.dequeue_if_data_ready().is_none());
    }
}
