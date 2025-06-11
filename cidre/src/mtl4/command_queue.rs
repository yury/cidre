use crate::{api, arc, define_obj_type, dispatch, mtl, mtl4, ns, objc};

define_obj_type!(
    #[doc(alias = "MTL4CommitOptions")]
    pub CommitOpts(ns::Id),
    MTL4_COMMIT_OPTIONS,
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]

);

impl CommitOpts {
    #[objc::msg_send(addFeedbackHandler:)]
    pub fn add_feedback_handler_block(&mut self, block: &mut mtl4::CommitFeedbackHandler);

    pub fn add_feedback_handler(
        &mut self,
        block: impl FnMut(&mtl4::CommitFeedback) + 'static + Send,
    ) {
        let mut block = mtl4::CommitFeedbackHandler::new1(block);
        self.add_feedback_handler_block(&mut block);
    }
}

define_obj_type!(
    #[doc(alias = "MTL4CommandQueueDescriptor")]
    pub CmdQueueDesc(ns::Id),
    MTL4_COMMAND_QUEUE_DESCRIPTOR,
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
);

impl CmdQueueDesc {
    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(feedbackQueue)]
    pub fn feedback_queue(&self) -> Option<arc::R<dispatch::Queue>>;

    #[objc::msg_send(setFeedbackQueue:)]
    pub fn set_feedback_queue(&mut self, val: Option<&dispatch::Queue>);
}

#[doc(alias = "MTL4UpdateSparseTextureMappingOperation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct UpdateSparseTextureMappingOp {
    pub mode: mtl::SparseTextureMappingMode,
    pub texture_region: mtl::Region,
    pub texture_level: usize,
    pub texture_slice: usize,
    pub heap_offset: usize,
}

#[doc(alias = "MTL4CopySparseTextureMappingOperation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct CopySparseTextureMappingOp {
    pub src_region: mtl::Region,
    pub src_level: usize,
    pub src_slice: usize,
    pub dst_origin: mtl::Origin,
    pub dst_level: usize,
    pub dst_slice: usize,
}

#[doc(alias = "MTL4UpdateSparseBufferMappingOperation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct UpdateSparseBufMappingOp {
    pub mode: mtl::SparseTextureMappingMode,
    pub buf_range: ns::Range,
    pub heap_offset: usize,
}

#[doc(alias = "MTL4CopySparseBufferMappingOperation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct CopySparseBufMappingOp {
    pub src_range: ns::Range,
    pub dst_offset: usize,
}

define_obj_type!(
    #[doc(alias = "MTL4CommandQueue")]
    pub CmdQueue(ns::Id)
);

impl CmdQueue {
    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(commit:count:)]
    pub unsafe fn commit_count(&mut self, bufs: *const &mtl4::CmdBuf, count: usize);

    #[inline]
    pub fn commit(&mut self, bufs: &[&mtl4::CmdBuf]) {
        unsafe {
            self.commit_count(bufs.as_ptr(), bufs.len());
        }
    }

    #[objc::msg_send(commit:count:options:)]
    pub unsafe fn commit_count_opts(
        &mut self,
        bufs: *const &mtl4::CmdBuf,
        count: usize,
        options: &mtl4::CommitOpts,
    );

    #[inline]
    pub fn commit_with_opts(&mut self, bufs: &[&mtl4::CmdBuf], options: &mtl4::CommitOpts) {
        unsafe {
            self.commit_count_opts(bufs.as_ptr(), bufs.len(), options);
        }
    }

    #[objc::msg_send(signalEvent:value:)]
    pub fn signal_event(&mut self, event: &mtl::Event, val: u64);

    #[objc::msg_send(waitForEvent:value:)]
    pub fn wait_for_event(&self, event: &mtl::Event, val: u64);

    #[objc::msg_send(signalDrawable:)]
    pub fn signal_drawable<T: objc::Obj, D: mtl::Drawable<T>>(&self, drawable: &D);

    #[objc::msg_send(waitDrawable:)]
    pub fn wait_drawable<T: objc::Obj, D: mtl::Drawable<T>>(&self, drawable: &D);

    #[objc::msg_send(addResidencySet:)]
    pub fn add_residency_set(&mut self, val: &mtl::ResidencySet);

    #[objc::msg_send(addResidencySets:count:)]
    pub unsafe fn add_residency_sets_count(
        &mut self,
        sets: *const &mtl::ResidencySet,
        count: usize,
    );

    #[inline]
    pub fn add_residency_sets(&mut self, sets: &[&mtl::ResidencySet]) {
        unsafe {
            self.add_residency_sets_count(sets.as_ptr(), sets.len());
        }
    }

    #[objc::msg_send(removeResidencySet:)]
    pub fn remove_residency_set(&mut self, val: &mtl::ResidencySet);

    #[objc::msg_send(removeResidencySets:count:)]
    pub unsafe fn remove_residency_sets_count(
        &mut self,
        sets: *const &mtl::ResidencySet,
        count: usize,
    );

    #[inline]
    pub fn remove_residency_sets(&mut self, sets: &[&mtl::ResidencySet]) {
        unsafe {
            self.remove_residency_sets_count(sets.as_ptr(), sets.len());
        }
    }

    #[objc::msg_send(updateTextureMappings:heap:operations:count:)]
    pub unsafe fn update_texture_mappings_count(
        &self,
        texture: &mut mtl::Texture,
        heap: Option<&mtl::Heap>,
        ops: *const UpdateSparseTextureMappingOp,
        count: usize,
    );

    #[inline]
    pub fn update_texture_mappings(
        &self,
        texture: &mut mtl::Texture,
        heap: Option<&mtl::Heap>,
        ops: &[UpdateSparseTextureMappingOp],
    ) {
        unsafe {
            self.update_texture_mappings_count(texture, heap, ops.as_ptr(), ops.len());
        }
    }

    #[objc::msg_send(copyTextureMappingsFromTexture:toTexture:operations:count:)]
    pub unsafe fn copy_texture_mappings_count(
        &self,
        src_texture: &mtl::Texture,
        dst_texture: &mut mtl::Texture,
        ops: *const CopySparseTextureMappingOp,
        count: usize,
    );

    #[inline]
    pub fn copy_texture_mappings(
        &self,
        src_texture: &mtl::Texture,
        dst_texture: &mut mtl::Texture,
        ops: &[CopySparseTextureMappingOp],
    ) {
        unsafe {
            self.copy_texture_mappings_count(src_texture, dst_texture, ops.as_ptr(), ops.len());
        }
    }

    #[objc::msg_send(updateBufferMappings:heap:operations:count:)]
    pub unsafe fn update_buf_mappings_count(
        &self,
        buf: &mut mtl::Buf,
        heap: Option<&mtl::Heap>,
        ops: *const UpdateSparseBufMappingOp,
        count: usize,
    );

    #[inline]
    pub fn update_buf_mappings(
        &self,
        buf: &mut mtl::Buf,
        heap: Option<&mtl::Heap>,
        ops: &[UpdateSparseBufMappingOp],
    ) {
        unsafe {
            self.update_buf_mappings_count(buf, heap, ops.as_ptr(), ops.len());
        }
    }

    #[objc::msg_send(copyBufferMappingsFromBuffer:toBuffer:operations:count:)]
    pub unsafe fn copy_buf_mappings_count(
        &self,
        src_buf: &mtl::Buf,
        dst_buf: &mut mtl::Buf,
        ops: *const CopySparseBufMappingOp,
        count: usize,
    );

    #[inline]
    pub fn copy_buf_mappings(
        &self,
        src_buf: &mtl::Buf,
        dst_buf: &mut mtl::Buf,
        ops: &[CopySparseBufMappingOp],
    ) {
        unsafe {
            self.copy_buf_mappings_count(src_buf, dst_buf, ops.as_ptr(), ops.len());
        }
    }
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL4_COMMIT_OPTIONS: &'static objc::Class<CommitOpts>;
    static MTL4_COMMAND_QUEUE_DESCRIPTOR: &'static objc::Class<CmdQueueDesc>;
}
