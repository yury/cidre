extern "C" {
    #[link_name = "objc_msgSend$registryID"]
    pub fn registry_id();

    #[link_name = "objc_msgSend$sampleBuffer"]
    pub fn sample_buffer();

    #[link_name = "objc_msgSend$setSampleBuffer:"]
    pub fn set_sample_buffer();

    #[link_name = "objc_msgSend$dispatchType"]
    pub fn dispatch_type();

    #[link_name = "objc_msgSend$setDispatchType:"]
    pub fn set_dispatch_type();

    #[link_name = "objc_msgSend$sampleBufferAttachments"]
    pub fn sample_buffer_attachments();

    #[link_name = "objc_msgSend$maxThreadsPerThreadgroup"]
    pub fn max_threads_per_threadgroup();

    #[link_name = "objc_msgSend$hasUnifiedMemory"]
    pub fn has_unified_memory();

    #[link_name = "objc_msgSend$readWriteTextureSupport"]
    pub fn read_write_texture_support();

    #[link_name = "objc_msgSend$argumentBuffersSupport"]
    pub fn argument_buffers_support();

    #[link_name = "objc_msgSend$setRenderPipelineState:"]
    pub fn set_render_pipeline_state();

}
