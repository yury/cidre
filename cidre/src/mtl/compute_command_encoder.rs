use crate::{define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTLComputeCommandEncoder")]
    pub ComputeCmdEncoder(mtl::CmdEncoder)
);

impl ComputeCmdEncoder {
    define_mtl!(
        update_fence,
        wait_for_fence,
        use_resource,
        use_resources,
        use_heap
    );

    /// Set the compute pipeline state that will be used.
    ///
    /// You must set a compute pipeline state before encoding any commands to dispatch
    /// compute kernels.
    #[objc::msg_send(setComputePipelineState:)]
    pub fn set_compute_ps(&mut self, state: &mtl::ComputePipelineState);

    #[objc::msg_send(setBytes:length:atIndex:)]
    pub fn _set_bytes_at(&mut self, bytes: *const std::ffi::c_void, length: usize, index: usize);

    /// Set the data (by copy) for a given buffer binding point.
    /// This will remove any existing mtl::Buffer from the binding point
    pub fn set_bytes_at(&mut self, buf: &[u8], index: usize) {
        self._set_bytes_at(buf.as_ptr() as _, buf.len(), index)
    }

    /// Set a global buffer for all compute kernels at the given bind point index.
    ///
    /// For buffers in the device address space, align the offset to the data type
    /// consumed by the compute function (which is always less than or equal to 16 bytes).
    ///
    /// For buffers in the constant address space, align the offset to 256 bytes in macOS.
    /// In iOS, align the offset to the maximum of either the data type consumed by
    /// the compute function, or 4 bytes. A 16-byte alignment is safe in iOS if you don't
    /// need to consider the data type.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The mtl::Buf object to set in the argument table.`
    /// * `offset` - Where the data begins, in bytes, from the start of the buffer.
    /// * `index` - An index in the buffer argument table.
    #[objc::msg_send(setBuffer:offset:atIndex:)]
    pub fn set_buf_at(&mut self, buffer: Option<&mtl::Buf>, offset: usize, index: usize);

    /// Set the offset within the current global buffer for all compute
    /// kernels at the given bind point index.
    ///
    #[objc::msg_send(setBufferOffset:atIndex:)]
    pub fn set_buf_offset_at(&mut self, offset: usize, index: usize);

    /// Set an array of global buffers for all compute kernels with the given bind point range
    #[objc::msg_send(setBuffers:offsets:withRange:)]
    pub fn set_bufs_offsets_with_range(
        &mut self,
        buffers: *const &mtl::Buf,
        offsets: *const usize,
        range: ns::Range,
    );

    #[inline]
    pub fn set_bufs_offsets_at<const N: usize>(
        &mut self,
        bufs: &[&mtl::Buf; N],
        offsets: &[usize; N],
        start_index: usize,
    ) {
        self.set_bufs_offsets_with_range(
            bufs.as_ptr(),
            offsets.as_ptr(),
            ns::Range::new(start_index, N),
        )
    }

    /// Set a visible function table at the given buffer index
    ///
    /// # Arguments
    ///
    /// * `visible_fn_table` - The visible function table to set in the buffer argument table.
    /// * `at_buffer_index` - An index in the buffer argument table.
    #[objc::msg_send(setVisibleFunctionTable:atBufferIndex:)]
    pub fn set_visible_fn_table_at(
        &mut self,
        val: Option<&mtl::VisibleFnTable>,
        at_buffer_index: usize,
    );

    #[objc::msg_send(setTexture:atIndex:)]
    pub fn set_texture_at(&mut self, val: Option<&mtl::Texture>, index: usize);

    #[objc::msg_send(setTextures:withRange:)]
    pub fn set_textures_with_range(&mut self, textures: *const &mtl::Texture, range: ns::Range);

    #[inline]
    pub fn set_textures_at(&mut self, textures: &[&mtl::Texture], start_index: usize) {
        self.set_textures_with_range(
            textures.as_ptr(),
            ns::Range::new(start_index, textures.len()),
        )
    }

    #[objc::msg_send(dispatchThreads:threadsPerThreadgroup:)]
    pub fn dispatch_threads(
        &mut self,
        threads_per_grid: mtl::Size,
        threads_per_threadgroup: mtl::Size,
    );

    #[objc::msg_send(dispatchThreadgroups:threadsPerThreadgroup:)]
    pub fn dispatch_threadgroups(
        &mut self,
        threadgroups_per_grid: mtl::Size,
        threads_per_threadgroup: mtl::Size,
    );

    /// Sets the size, in pixels, of the imageblock.
    ///
    /// You use tile memory for both imageblocks and threadgroups.
    /// The sum of these allocations can't exceed the maximum total tile memory limit.
    #[objc::msg_send(setImageblockWidth:height:)]
    pub fn set_image_block_size(&mut self, width: usize, height: usize);

    /// Execute commands in the buffer within the range specified.
    ///
    /// The same indirect command buffer may be executed any number of times within the same encoder.
    #[objc::msg_send(executeCommandsInBuffer:withRange:)]
    pub fn execute_commands_in_buf(&mut self, icb: &mtl::IndirectCmdBuf, range: ns::Range);
}
