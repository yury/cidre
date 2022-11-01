use std::{
    ffi::c_void,
    ptr::{slice_from_raw_parts, slice_from_raw_parts_mut},
};

use crate::{
    cf::{self, Retained},
    define_cf_type, define_options, os,
};

define_options!(Flags(u32));

impl Flags {
    pub const NONE: Self = Self(0);
    pub const ASSURE_MEMORY_NOW: Self = Self(1u32 << 0);
    pub const ALWAYS_COPY_DATA: Self = Self(1u32 << 1);
    pub const DONT_OPTIMIZE_DEPTH: Self = Self(1u32 << 2);
    pub const PERMIT_EMPTY_REFERENCE: Self = Self(1u32 << 3);
}

define_cf_type!(BlockBuffer(cf::Type));

impl BlockBuffer {
    /// # Example
    /// ```
    /// use cidre::cm;
    ///
    /// let b = cm::BlockBuffer::create_empty(None, 0, cm::BlockBufferFlags::NONE).expect("hmm");
    ///
    /// assert_eq!(b.get_type_id(), cm::BlockBuffer::type_id());
    /// ```
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CMBlockBufferGetTypeID() }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let b = cm::BlockBuffer::create_empty(None, 0, cm::BlockBufferFlags::NONE).expect("hmm");
    ///
    /// assert!(b.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { CMBlockBufferIsEmpty(self) }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let b = cm::BlockBuffer::create_empty(None, 0, cm::BlockBufferFlags::NONE)
    ///     .expect("empty block buffer");
    ///
    /// assert!(b.is_empty());
    /// assert!(b.data_len() == 0);
    ///
    /// ```
    #[inline]
    pub fn create_empty(
        structure_allocator: Option<&cf::Allocator>,
        sub_block_capacity: u32,
        flags: Flags,
    ) -> Result<Retained<BlockBuffer>, os::Status> {
        unsafe {
            let mut block_buffer_out = None;
            CMBlockBufferCreateEmpty(
                structure_allocator,
                sub_block_capacity,
                flags,
                &mut block_buffer_out,
            )
            .to_result_unchecked(block_buffer_out)
        }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let b = cm::BlockBuffer::with_memory_block(10, None)
    ///     .expect("empty block buffer");
    ///
    /// assert_eq!(false, b.is_empty());
    /// assert_eq!(10, b.data_len());
    ///
    /// ```
    #[inline]
    pub fn with_memory_block(
        len: usize,
        block_allocator: Option<&cf::Allocator>,
    ) -> Result<Retained<BlockBuffer>, os::Status> {
        unsafe {
            Self::create_with_memory_block(
                None,
                std::ptr::null_mut(),
                len,
                block_allocator,
                0,
                len,
                Flags::ASSURE_MEMORY_NOW,
            )
        }
    }

    #[inline]
    pub unsafe fn create_with_memory_block(
        structure_allocator: Option<&cf::Allocator>,
        memory_block: *mut c_void,
        block_length: usize,
        block_allocator: Option<&cf::Allocator>,
        // custom_block_source: *const c_void, // TODO: add block source
        offset_to_data: usize,
        data_length: usize,
        flags: Flags,
    ) -> Result<Retained<BlockBuffer>, os::Status> {
        let mut block_buffer_out = None;
        CMBlockBufferCreateWithMemoryBlock(
            structure_allocator,
            memory_block,
            block_length,
            block_allocator,
            std::ptr::null(),
            offset_to_data,
            data_length,
            flags,
            &mut block_buffer_out,
        )
        .to_result_unchecked(block_buffer_out)
    }

    /// Obtains the total data length reachable via a cm::BlockBuffer.
    ///
    /// Obtains the total data length reachable via a cm::BlockBuffer. This total is the sum of the dataLengths
    /// of the cm::BlockBuffer's memoryBlocks and buffer references. Note that the dataLengths are
    /// the _portions_ of those constituents that this cm::BlockBuffer subscribes to. This cm::BlockBuffer presents a
    /// contiguous range of offsets from zero to its totalDataLength as returned by this routine.
    #[inline]
    pub fn data_len(&self) -> usize {
        unsafe { CMBlockBufferGetDataLength(self) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data_len()
    }

    #[inline]
    pub fn is_range_contiguous(&self, offset: usize, length: usize) -> bool {
        unsafe { CMBlockBufferIsRangeContiguous(self, offset, length) }
    }

    /// Gains access to the data represented by a cm::BlockBuffer.
    ///
    /// Gains access to the data represented by a cm::BlockBuffer. A pointer into a memory block is returned
    /// which corresponds to the offset within the cm::BlockBuffer. The number of bytes addressable at the
    /// pointer can also be returned. This length-at-offset may be smaller than the number of bytes actually
    /// available starting at the offset if the dataLength of the CMBlockBuffer is covered by multiple memory
    /// blocks (a noncontiguous cm::BlockBuffer). The data pointer returned will remain valid as long as the
    /// original cm::BlockBuffer is referenced - once the cm::BlockBuffer is released for the last time, any pointers
    /// into it will be invalid.
    #[inline]
    pub unsafe fn get_data_pointer(
        &self,
        offset: usize,
        length_at_offset_out: *mut usize,
        total_length_out: *mut usize,
        data_pointer_out: *mut *mut u8,
    ) -> os::Status {
        CMBlockBufferGetDataPointer(
            self,
            offset,
            length_at_offset_out,
            total_length_out,
            data_pointer_out,
        )
    }

    #[inline]
    pub fn data_pointer_at(&self, offset: usize) -> Result<(&[u8], usize), os::Status> {
        let mut length_at_offset_out = 0;
        let mut total_length_out = 0;
        let mut data_pointer_out = std::ptr::null_mut();
        unsafe {
            let res = self.get_data_pointer(
                offset,
                &mut length_at_offset_out,
                &mut total_length_out,
                &mut data_pointer_out,
            );
            if res.is_err() {
                return Err(res);
            }
            let s = slice_from_raw_parts_mut(data_pointer_out, length_at_offset_out);
            Ok((&*s, total_length_out))
        }
    }

    #[inline]
    pub fn data_pointer(&self) -> Result<&[u8], os::Status> {
        let mut length_at_offset_out = 0;
        let mut data_pointer_out = std::ptr::null_mut();
        unsafe {
            let res = self.get_data_pointer(
                0,
                &mut length_at_offset_out,
                std::ptr::null_mut(),
                &mut data_pointer_out,
            );
            if res.is_err() {
                return Err(res);
            }
            Ok(&*slice_from_raw_parts(
                data_pointer_out,
                length_at_offset_out,
            ))
        }
    }

    #[inline]
    pub fn data_pointer_mut(&mut self) -> Result<&mut [u8], os::Status> {
        let mut length_at_offset_out = 0;
        let mut data_pointer_out = std::ptr::null_mut();
        unsafe {
            let res = self.get_data_pointer(
                0,
                &mut length_at_offset_out,
                std::ptr::null_mut(),
                &mut data_pointer_out,
            );
            if res.is_err() {
                return Err(res);
            }
            Ok(&mut *slice_from_raw_parts_mut(
                data_pointer_out,
                length_at_offset_out,
            ))
        }
    }

    #[inline]
    pub fn with_buffer_reference(
        buffer_reference: &BlockBuffer,
        offset_to_data: usize,
        data_length: usize,
        flags: Flags,
    ) -> Result<Retained<BlockBuffer>, os::Status> {
        unsafe {
            let mut block_buffer_out = None;
            Self::create_with_buffer_reference(
                None,
                buffer_reference,
                offset_to_data,
                data_length,
                flags,
                &mut block_buffer_out,
            )
            .to_result_unchecked(block_buffer_out)
        }
    }

    #[inline]
    pub unsafe fn create_with_buffer_reference(
        structure_allocator: Option<&cf::Allocator>,
        buffer_reference: &BlockBuffer,
        offset_to_data: usize,
        data_length: usize,
        flags: Flags,
        block_buffer_out: &mut Option<Retained<BlockBuffer>>,
    ) -> os::Status {
        CMBlockBufferCreateWithBufferReference(
            structure_allocator,
            buffer_reference,
            offset_to_data,
            data_length,
            flags,
            block_buffer_out,
        )
    }
}

extern "C" {
    fn CMBlockBufferGetTypeID() -> cf::TypeId;
    fn CMBlockBufferIsEmpty(the_buffer: &BlockBuffer) -> bool;

    fn CMBlockBufferCreateEmpty(
        structure_allocator: Option<&cf::Allocator>,
        sub_block_capacity: u32,
        flags: Flags,
        block_buffer_out: &mut Option<Retained<BlockBuffer>>,
    ) -> os::Status;

    fn CMBlockBufferCreateWithMemoryBlock(
        structure_allocator: Option<&cf::Allocator>,
        memory_block: *mut c_void,
        block_length: usize,
        block_allocator: Option<&cf::Allocator>,
        custom_block_source: *const c_void, // TODO: add block source
        offset_to_data: usize,
        data_length: usize,
        flags: Flags,
        block_buffer_out: &mut Option<Retained<BlockBuffer>>,
    ) -> os::Status;

    fn CMBlockBufferGetDataLength(the_buffer: &BlockBuffer) -> usize;

    fn CMBlockBufferIsRangeContiguous(
        the_buffer: &BlockBuffer,
        offset: usize,
        length: usize,
    ) -> bool;

    fn CMBlockBufferGetDataPointer(
        the_buffer: &BlockBuffer,
        offset: usize,
        length_at_offset_out: *mut usize,
        total_length_out: *mut usize,
        data_pointer_out: *mut *mut u8,
    ) -> os::Status;

    fn CMBlockBufferCreateWithBufferReference(
        structure_allocator: Option<&cf::Allocator>,
        buffer_reference: &BlockBuffer,
        offset_to_data: usize,
        data_length: usize,
        flags: Flags,
        block_buffer_out: &mut Option<Retained<BlockBuffer>>,
    ) -> os::Status;

}

pub mod errors {
    use crate::os::Status;

    /// Returned when a cm::BlockBuffer-creating API gets a failure
    /// from the cf::Allocator provided for cm::BlockBuffer construction.
    pub const STRUCTURE_ALLOCATION_FAILED: Status = Status(-12700);

    /// Returned when the allocator provided to allocate a memory block
    /// (as distinct from cm::BlockBuffer structures) fails.
    pub const BLOCK_ALLOCATION_FAILED: Status = Status(-12701);

    /// The custom block source’s Allocate() routine was NULL when an allocation was attempted.
    pub const BAD_CUSTOM_BLOCK_SOURCE: Status = Status(-12702);

    /// The offset provided to an API is out of the range of the relevent cm::BlockBuffer
    pub const BAD_OFFSET_PARAMETER: Status = Status(-12703);

    /// The length provided to an API is out of the range of the relevent cm::BlockBuffer,
    /// or is not allowed to be zero.
    pub const BAD_LENGTH_PARAMETER: Status = Status(-12704);

    /// A pointer parameter (e.g. cm::BlockBuffer reference, destination memory) is NULL
    /// or otherwise invalid.
    pub const BAD_POINTER_PARAMETER: Status = Status(-12705);

    /// Expected a non-empty cm::BlockBuffer.
    pub const EMPTY_BBUF: Status = Status(-12706);

    /// An unallocated memory block was encountered.
    pub const UNALLOCATED_BLOCK: Status = Status(-12707);

    pub const INSUFFICIENT_SPACE: Status = Status(-12708);
}
