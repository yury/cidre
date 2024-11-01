use std::{
    ffi::c_void,
    ptr::{slice_from_raw_parts, slice_from_raw_parts_mut},
};

use crate::{arc, cf, define_cf_type, define_opts, os};

define_opts!(pub Flags(u32));

impl Flags {
    pub const NONE: Self = Self(0);
    /// When passed to routines that accept block allocators, causes the memory block
    /// to be allocated immediately.
    #[doc(alias = "kCMBlockBufferAssureMemoryNowFlag")]
    pub const ASSURE_MEM_NOW: Self = Self(1u32 << 0);
    /// Used with CMBlockBufferCreateContiguous() to cause it to always produce an allocated
    /// copy of the desired data.
    #[doc(alias = "kCMBlockBufferAlwaysCopyDataFlag")]
    pub const ALWAYS_COPY_DATA: Self = Self(1u32 << 1);

    /// Passed to CMBlockBufferAppendBufferReference()
    /// and CMBlockBufferCreateWithBufferReference()
    /// to suppress reference depth optimization
    #[doc(alias = "kCMBlockBufferDontOptimizeDepthFlag")]
    pub const DONT_OPTIMIZE_DEPTH: Self = Self(1u32 << 2);

    /// Passed to CMBlockBufferAppendBufferReference() and
    /// CMBlockBufferCreateWithBufferReference()
    /// to allow references into a CMBlockBuffer that may not yet be populated.
    #[doc(alias = "kCMBlockBufferPermitEmptyReferenceFlag")]
    pub const PERMIT_EMPTY_REFERENCE: Self = Self(1u32 << 3);
}

define_cf_type!(BlockBuf(cf::Type));
// TODO: termporary...
unsafe impl Send for BlockBuf {}
unsafe impl Sync for BlockBuf {}

impl BlockBuf {
    /// # Example
    /// ```
    /// use cidre::cm;
    ///
    /// let b = cm::BlockBuf::new(0, cm::BlockBufFlags::NONE).expect("hmm");
    ///
    /// assert_eq!(b.get_type_id(), cm::BlockBuf::type_id());
    /// ```
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CMBlockBufferGetTypeID() }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let b = cm::BlockBuf::new_in(0, cm::BlockBufFlags::NONE, None).expect("hmm");
    ///
    /// assert!(b.is_empty());
    /// ```
    #[doc(alias = "CMBlockBufferIsEmpty")]
    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { CMBlockBufferIsEmpty(self) }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let b = cm::BlockBuf::new(0, cm::BlockBufFlags::NONE)
    ///     .expect("empty block buffer");
    ///
    /// assert!(b.is_empty());
    /// assert!(b.data_len() == 0);
    ///
    /// ```
    #[doc(alias = "CMBlockBufferCreateEmpty")]
    #[inline]
    pub fn new_in(
        sub_block_capacity: u32,
        flags: Flags,
        structure_allocator: Option<&cf::Allocator>,
    ) -> os::Result<arc::R<BlockBuf>> {
        unsafe {
            os::result_unchecked(|res| {
                CMBlockBufferCreateEmpty(structure_allocator, sub_block_capacity, flags, res)
            })
        }
    }

    #[doc(alias = "CMBlockBufferCreateEmpty")]
    #[inline]
    pub fn new(sub_block_capacity: u32, flags: Flags) -> os::Result<arc::R<BlockBuf>> {
        Self::new_in(sub_block_capacity, flags, None)
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let b = cm::BlockBuf::with_mem_block(10, None)
    ///     .expect("empty block buffer");
    ///
    /// assert_eq!(false, b.is_empty());
    /// assert_eq!(10, b.data_len());
    ///
    /// ```
    #[inline]
    pub fn with_mem_block(
        len: usize,
        block_allocator: Option<&cf::Allocator>,
    ) -> os::Result<arc::R<BlockBuf>> {
        unsafe {
            Self::create_with_mem_block_in(
                std::ptr::null_mut(),
                len,
                block_allocator,
                0,
                len,
                Flags::ASSURE_MEM_NOW,
                None,
            )
        }
    }

    #[doc(alias = "CMBlockBufferCreateWithMemoryBlock")]
    #[inline]
    pub unsafe fn create_with_mem_block_in(
        memory_block: *mut c_void,
        block_length: usize,
        block_allocator: Option<&cf::Allocator>,
        // custom_block_source: *const c_void, // TODO: add block source
        offset_to_data: usize,
        data_length: usize,
        flags: Flags,
        structure_allocator: Option<&cf::Allocator>,
    ) -> os::Result<arc::R<BlockBuf>> {
        os::result_unchecked(|val| {
            CMBlockBufferCreateWithMemoryBlock(
                structure_allocator,
                memory_block,
                block_length,
                block_allocator,
                std::ptr::null(),
                offset_to_data,
                data_length,
                flags,
                val,
            )
        })
    }

    /// Obtains the total data length reachable via a cm::BlockBuf.
    ///
    /// Obtains the total data length reachable via a cm::BlockBuf. This total is the sum of the dataLengths
    /// of the cm::BlockBuf's memoryBlocks and buffer references. Note that the dataLengths are
    /// the _portions_ of those constituents that this cm::BlockBuf subscribes to. This cm::BlockBuf presents a
    /// contiguous range of offsets from zero to its totalDataLength as returned by this routine.
    #[doc(alias = "CMBlockBufferGetDataLength")]
    #[inline]
    pub fn data_len(&self) -> usize {
        unsafe { CMBlockBufferGetDataLength(self) }
    }

    #[doc(alias = "CMBlockBufferGetDataLength")]
    #[inline]
    pub fn len(&self) -> usize {
        self.data_len()
    }

    #[doc(alias = "CMBlockBufferIsRangeContiguous")]
    #[inline]
    pub fn is_range_contiguous(&self, offset: usize, length: usize) -> bool {
        unsafe { CMBlockBufferIsRangeContiguous(self, offset, length) }
    }

    /// Gains access to the data represented by a cm::BlockBuf.
    ///
    /// Gains access to the data represented by a cm::BlockBuf. A pointer into a memory block is returned
    /// which corresponds to the offset within the cm::BlockBuf. The number of bytes addressable at the
    /// pointer can also be returned. This length-at-offset may be smaller than the number of bytes actually
    /// available starting at the offset if the dataLength of the cm::BlockBuf is covered by multiple memory
    /// blocks (a noncontiguous cm::BlockBuf). The data pointer returned will remain valid as long as the
    /// original cm::BlockBuf is referenced - once the cm::BlockBuf is released for the last time, any pointers
    /// into it will be invalid.
    #[doc(alias = "CMBlockBufferGetDataPointer")]
    #[inline]
    pub unsafe fn data_ptr(
        &self,
        offset: usize,
        length_at_offset_out: *mut usize,
        total_length_out: *mut usize,
        data_pointer_out: *mut *mut u8,
    ) -> os::Result {
        CMBlockBufferGetDataPointer(
            self,
            offset,
            length_at_offset_out,
            total_length_out,
            data_pointer_out,
        )
        .result()
    }

    #[doc(alias = "CMBlockBufferGetDataPointer")]
    #[inline]
    pub fn data_ptr_at(&self, offset: usize) -> os::Result<(&[u8], usize)> {
        let mut length_at_offset_out = 0;
        let mut total_length_out = 0;
        let mut data_pointer_out = std::ptr::null_mut();
        unsafe {
            self.data_ptr(
                offset,
                &mut length_at_offset_out,
                &mut total_length_out,
                &mut data_pointer_out,
            )?;
            let s = slice_from_raw_parts(data_pointer_out, length_at_offset_out);
            Ok((&*s, total_length_out))
        }
    }

    #[inline]
    pub fn as_slice(&self) -> os::Result<&[u8]> {
        let mut length_at_offset_out = 0;
        let mut data_pointer_out = std::ptr::null_mut();
        unsafe {
            self.data_ptr(
                0,
                &mut length_at_offset_out,
                std::ptr::null_mut(),
                &mut data_pointer_out,
            )?;

            Ok(&*slice_from_raw_parts(
                data_pointer_out,
                length_at_offset_out,
            ))
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> os::Result<&mut [u8]> {
        let mut length_at_offset_out = 0;
        let mut data_pointer_out = std::ptr::null_mut();
        unsafe {
            self.data_ptr(
                0,
                &mut length_at_offset_out,
                std::ptr::null_mut(),
                &mut data_pointer_out,
            )?;
            Ok(&mut *slice_from_raw_parts_mut(
                data_pointer_out,
                length_at_offset_out,
            ))
        }
    }

    #[inline]
    pub fn with_buf_ref(
        buf_reference: &BlockBuf,
        offset_to_data: usize,
        data_length: usize,
        flags: Flags,
    ) -> os::Result<arc::R<BlockBuf>> {
        unsafe {
            os::result_unchecked(|val| {
                Self::create_with_buf_ref(
                    None,
                    buf_reference,
                    offset_to_data,
                    data_length,
                    flags,
                    val,
                )
            })
        }
    }

    #[doc(alias = "CMBlockBufferCreateWithBufferReference")]
    #[inline]
    pub unsafe fn create_with_buf_ref(
        structure_allocator: Option<&cf::Allocator>,
        buf_reference: &BlockBuf,
        offset_to_data: usize,
        data_length: usize,
        flags: Flags,
        block_buf_out: *mut Option<arc::R<BlockBuf>>,
    ) -> os::Result {
        CMBlockBufferCreateWithBufferReference(
            structure_allocator,
            buf_reference,
            offset_to_data,
            data_length,
            flags,
            block_buf_out,
        )
        .result()
    }

    /// Assures that the system allocates memory for all memory blocks in a
    /// block buffer.
    ///
    /// Traverses the possibly complex cm::BlockBuf, allocating the memory
    /// for any constituent memory blocks that are not yet allocated.
    #[doc(alias = "CMBlockBufferAssureBlockMemory")]
    #[inline]
    pub fn assure_block_mem(&mut self) -> os::Result {
        unsafe { CMBlockBufferAssureBlockMemory(self).result() }
    }
}

extern "C-unwind" {
    fn CMBlockBufferGetTypeID() -> cf::TypeId;
    fn CMBlockBufferIsEmpty(the_buffer: &BlockBuf) -> bool;

    fn CMBlockBufferCreateEmpty(
        structure_allocator: Option<&cf::Allocator>,
        sub_block_capacity: u32,
        flags: Flags,
        block_buffer_out: *mut Option<arc::R<BlockBuf>>,
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
        block_buffer_out: *mut Option<arc::R<BlockBuf>>,
    ) -> os::Status;

    fn CMBlockBufferGetDataLength(the_buffer: &BlockBuf) -> usize;

    fn CMBlockBufferIsRangeContiguous(the_buffer: &BlockBuf, offset: usize, length: usize) -> bool;

    fn CMBlockBufferGetDataPointer(
        the_buffer: &BlockBuf,
        offset: usize,
        length_at_offset_out: *mut usize,
        total_length_out: *mut usize,
        data_pointer_out: *mut *mut u8,
    ) -> os::Status;

    fn CMBlockBufferCreateWithBufferReference(
        structure_allocator: Option<&cf::Allocator>,
        buffer_reference: &BlockBuf,
        offset_to_data: usize,
        data_length: usize,
        flags: Flags,
        block_buffer_out: *mut Option<arc::R<BlockBuf>>,
    ) -> os::Status;

    fn CMBlockBufferAssureBlockMemory(buffer: &mut BlockBuf) -> os::Status;

}

pub mod err {
    use crate::os::Error;

    /// Returned when a cm::BlockBuffer-creating API gets a failure
    /// from the cf::Allocator provided for cm::BlockBuffer construction.
    pub const STRUCTURE_ALLOCATION_FAILED: Error = Error::new_unchecked(-12700);

    /// Returned when the allocator provided to allocate a memory block
    /// (as distinct from cm::BlockBuffer structures) fails.
    pub const BLOCK_ALLOCATION_FAILED: Error = Error::new_unchecked(-12701);

    /// The custom block source’s Allocate() routine was NULL when an allocation was attempted.
    pub const BAD_CUSTOM_BLOCK_SOURCE: Error = Error::new_unchecked(-12702);

    /// The offset provided to an API is out of the range of the relevent cm::BlockBuffer
    pub const BAD_OFFSET_PARAMETER: Error = Error::new_unchecked(-12703);

    /// The length provided to an API is out of the range of the relevent cm::BlockBuffer,
    /// or is not allowed to be zero.
    pub const BAD_LENGTH_PARAMETER: Error = Error::new_unchecked(-12704);

    /// A pointer parameter (e.g. cm::BlockBuffer reference, destination memory) is NULL
    /// or otherwise invalid.
    pub const BAD_POINTER_PARAMETER: Error = Error::new_unchecked(-12705);

    /// Expected a non-empty cm::BlockBuffer.
    pub const EMPTY_BBUF: Error = Error::new_unchecked(-12706);

    /// An unallocated memory block was encountered.
    pub const UNALLOCATED_BLOCK: Error = Error::new_unchecked(-12707);

    pub const INSUFFICIENT_SPACE: Error = Error::new_unchecked(-12708);
}
