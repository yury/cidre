use std::ffi::c_void;

use crate::{blocks, cm, dispatch, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum Status {
    Unknown = 0,
    Rendering = 1,
    Failed = 2,
}

#[objc::obj_trait]
pub trait QueuedSampleBufferRendering: objc::Obj {
    #[objc::msg_send(timebase)]
    fn timebase(&self) -> &cm::Timebase;

    #[objc::msg_send(enqueueSampleBuffer:)]
    fn enqueue_sample_buf(&mut self, buf: &cm::SampleBuf);

    #[objc::msg_send(flush)]
    fn flush(&mut self);

    #[objc::msg_send(readyForMoreMediaData)]
    fn ready_for_more_media_data(&mut self);

    #[objc::msg_send(requestMediaDataWhenReadyOnQueue:usingBlock:)]
    fn _request_media_data_when_ready_on_queue(
        &mut self,
        queue: &dispatch::Queue,
        block: *mut c_void,
    );

    fn request_media_data_when_ready_on_queue<F>(
        &mut self,
        queue: &dispatch::Queue,
        block: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce() + Send + 'static,
    {
        self._request_media_data_when_ready_on_queue(queue, block.as_ptr());
    }

    #[objc::msg_send(stopRequestingMediaData)]
    fn stop_requesting_media_data(&mut self);

    #[objc::msg_send(hasSufficientMediaDataForReliablePlaybackStart)]
    fn has_sufficient_media_data_for_reliable_playback_start(&self) -> bool;
}
