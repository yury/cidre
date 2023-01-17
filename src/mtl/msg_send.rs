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

    #[link_name = "objc_msgSend$useHeap:"]
    pub fn use_heap();

    #[link_name = "objc_msgSend$pushDebugGroup:"]
    pub fn push_debug_group();

    #[link_name = "objc_msgSend$popDebugGroup"]
    pub fn pop_debug_group();

    #[link_name = "objc_msgSend$storageMode"]
    pub fn storage_mode();

    #[link_name = "objc_msgSend$setStorageMode:"]
    pub fn set_storage_mode();

    #[link_name = "objc_msgSend$newHeapWithDescriptor:"]
    pub fn new_heap_with_descriptor();

    #[link_name = "objc_msgSend$newBufferWithBytes:length:options:"]
    pub fn new_buffer_with_bytes_length_options();

    #[link_name = "objc_msgSend$cpuCacheMode"]
    pub fn cpu_cache_mode();

    #[link_name = "objc_msgSend$setCpuCacheMode:"]
    pub fn set_cpu_cache_mode();

    #[link_name = "objc_msgSend$resourceOptions"]
    pub fn resource_options();

    #[link_name = "objc_msgSend$setResourceOptions:"]
    pub fn set_resource_options();

    #[link_name = "objc_msgSend$hazardTrackingMode"]
    pub fn hazard_tracking_mode();

    #[link_name = "objc_msgSend$setHazardTrackingMode:"]
    pub fn set_hazard_tracking_mode();

    #[link_name = "objc_msgSend$gpuResourceID"]
    pub fn gpu_resource_id();

    #[link_name = "objc_msgSend$newBufferWithLength:options:"]
    pub fn new_buffer_with_length_options();

    #[link_name = "objc_msgSend$newDefaultLibrary"]
    pub fn new_default_library();

    #[link_name = "objc_msgSend$newTextureWithDescriptor:"]
    pub fn new_texture_with_descriptor();

}
