use std::{
    ffi::c_void,
    ptr::{slice_from_raw_parts, slice_from_raw_parts_mut},
};

use crate::{arc, cf, define_cf_type, define_opts, os};

define_opts!(
    #[doc(alias = "CMBlockBufferFlags")]
    pub Flags(u32)
);

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

#[doc(alias = "CMBlockBufferCustomBlockSource")]
#[derive(Debug, Default)]
#[repr(C, packed(4))]
pub struct CustomSrc {
    pub version: u32,
    pub alloc: Option<extern "C" fn(refcon: *mut c_void, bytes_n: usize) -> *mut c_void>,
    pub free: Option<extern "C" fn(refcon: *mut c_void, doomed_mem: *mut c_void, bytes_n: usize)>,
    pub refcon: *mut c_void,
}

#[doc(alias = "kCMBlockBufferCustomBlockSourceVersion")]
pub const CUSTOM_SRC_VERSION: u32 = 0;

define_cf_type!(
    #[doc(alias = "CMBlockBufferRef")]
    BlockBuf(cf::Type)
);

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
    /// assert!(b.len() == 0);
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
    pub fn with_mem_block_in(
        len: usize,
        block_allocator: Option<&cf::Allocator>,
    ) -> os::Result<arc::R<BlockBuf>> {
        unsafe {
            Self::create_with_mem_block_in(
                std::ptr::null_mut(),
                len,
                block_allocator,
                std::ptr::null(),
                0,
                len,
                Flags::ASSURE_MEM_NOW,
                None,
            )
        }
    }

    #[inline]
    pub fn with_mem_block(len: usize) -> os::Result<arc::R<BlockBuf>> {
        unsafe {
            Self::create_with_mem_block_in(
                std::ptr::null_mut(),
                len,
                None,
                std::ptr::null(),
                0,
                len,
                Flags::ASSURE_MEM_NOW,
                None,
            )
        }
    }

    pub fn with_custom_src_flags(
        len: usize,
        src: &CustomSrc,
        flags: Flags,
    ) -> os::Result<arc::R<Self>> {
        unsafe {
            Self::create_with_mem_block_in(
                std::ptr::null_mut(),
                len,
                None,
                src,
                0,
                len,
                flags,
                None,
            )
        }
    }

    pub fn with_custom_src(len: usize, src: &CustomSrc) -> os::Result<arc::R<Self>> {
        unsafe {
            Self::create_with_mem_block_in(
                std::ptr::null_mut(),
                len,
                None,
                src,
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
        custom_block_source: *const CustomSrc,
        offset_to_data: usize,
        data_length: usize,
        flags: Flags,
        structure_allocator: Option<&cf::Allocator>,
    ) -> os::Result<arc::R<BlockBuf>> {
        unsafe {
            os::result_unchecked(|val| {
                CMBlockBufferCreateWithMemoryBlock(
                    structure_allocator,
                    memory_block,
                    block_length,
                    block_allocator,
                    custom_block_source,
                    offset_to_data,
                    data_length,
                    flags,
                    val,
                )
            })
        }
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
        unsafe {
            CMBlockBufferGetDataPointer(
                self,
                offset,
                length_at_offset_out,
                total_length_out,
                data_pointer_out,
            )
            .result()
        }
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

    pub fn try_contiguous_buf(&self) -> Option<ContiguousBlockBuf> {
        if self.is_range_contiguous(0, self.len()) {
            Some(ContiguousBlockBuf(self.retained()))
        } else {
            None
        }
    }

    #[doc(alias = "CMBlockBufferCreateContiguous")]
    #[inline]
    pub fn make_contiguous_in(
        &self,
        block_allocator: Option<&cf::Allocator>,
    ) -> os::Result<ContiguousBlockBuf> {
        let buf = unsafe {
            os::result_unchecked(|val| {
                CMBlockBufferCreateContiguous(
                    None,
                    self,
                    block_allocator,
                    std::ptr::null(),
                    0,
                    self.len(),
                    Flags::NONE,
                    val,
                )
            })
        }?;
        Ok(ContiguousBlockBuf(buf))
    }

    #[doc(alias = "CMBlockBufferCreateContiguous")]
    #[inline]
    pub fn make_contiguous(&self) -> os::Result<ContiguousBlockBuf> {
        let buf = unsafe {
            os::result_unchecked(|val| {
                CMBlockBufferCreateContiguous(
                    None,
                    self,
                    None,
                    std::ptr::null(),
                    0,
                    self.len(),
                    Flags::NONE,
                    val,
                )
            })
        }?;
        Ok(ContiguousBlockBuf(buf))
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
        unsafe {
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

    #[doc(alias = "CMBlockBufferCopyDataBytes")]
    #[inline]
    pub unsafe fn copy_bytes(
        &self,
        offset_to_data: usize,
        data_length: usize,
        dst: *mut u8,
    ) -> os::Result {
        unsafe { CMBlockBufferCopyDataBytes(self, offset_to_data, data_length, dst).result() }
    }

    #[doc(alias = "CMBlockBufferCopyDataBytes")]
    #[inline]
    pub fn copy_to(&self, offset: usize, slice: &mut [u8]) -> os::Result {
        unsafe { self.copy_bytes(offset, slice.len(), slice.as_mut_ptr()) }
    }
}

unsafe extern "C-unwind" {
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
        custom_block_source: *const CustomSrc,
        offset_to_data: usize,
        data_length: usize,
        flags: Flags,
        block_buffer_out: *mut Option<arc::R<BlockBuf>>,
    ) -> os::Status;

    fn CMBlockBufferCreateContiguous(
        structure_allocator: Option<&cf::Allocator>,
        src_buf: &BlockBuf,
        block_allocator: Option<&cf::Allocator>,
        custom_block_source: *const CustomSrc,
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

    fn CMBlockBufferCopyDataBytes(
        buf: &BlockBuf,
        offset_to_data: usize,
        data_length: usize,
        destination: *mut u8,
    ) -> os::Status;

}

pub mod err {
    use crate::os::Error;

    /// Returned when a cm::BlockBuffer-creating API gets a failure
    /// from the cf::Allocator provided for cm::BlockBuffer construction.
    #[doc(alias = "kCMBlockBufferStructureAllocationFailedErr")]
    pub const STRUCT_ALLOC_FAILED: Error = Error::new_unchecked(-12700);

    /// Returned when the allocator provided to allocate a memory block
    /// (as distinct from cm::BlockBuffer structures) fails.
    #[doc(alias = "kCMBlockBufferBlockAllocationFailedErr")]
    pub const BLOCK_ALLOC_FAILED: Error = Error::new_unchecked(-12701);

    /// The custom block source’s Allocate() routine was NULL when an allocation was attempted.
    #[doc(alias = "kCMBlockBufferBadCustomBlockSourceErr")]
    pub const BAD_CUSTOM_BLOCK_SRC: Error = Error::new_unchecked(-12702);

    /// The offset provided to an API is out of the range of the relevent cm::BlockBuffer
    #[doc(alias = "kCMBlockBufferBadOffsetParameterErr")]
    pub const BAD_OFFSET_PARAM: Error = Error::new_unchecked(-12703);

    /// The length provided to an API is out of the range of the relevent cm::BlockBuffer,
    /// or is not allowed to be zero.
    #[doc(alias = "kCMBlockBufferBadLengthParameterErr")]
    pub const BAD_LEN_PARAM: Error = Error::new_unchecked(-12704);

    /// A pointer parameter (e.g. cm::BlockBuffer reference, destination memory) is NULL
    /// or otherwise invalid.
    #[doc(alias = "kCMBlockBufferBadPointerParameterErr")]
    pub const BAD_POINTER_PARAM: Error = Error::new_unchecked(-12705);

    /// Expected a non-empty cm::BlockBuffer.
    #[doc(alias = "kCMBlockBufferEmptyBBufErr")]
    pub const EMPTY_BUF: Error = Error::new_unchecked(-12706);

    /// An unallocated memory block was encountered.
    #[doc(alias = "kCMBlockBufferUnallocatedBlockErr")]
    pub const UNALLOCATED_BLOCK: Error = Error::new_unchecked(-12707);

    #[doc(alias = "kCMBlockBufferInsufficientSpaceErr")]
    pub const INSUFFICIENT_SPACE: Error = Error::new_unchecked(-12708);
}

pub struct ContiguousBlockBuf(arc::R<BlockBuf>);

impl ContiguousBlockBuf {
    pub fn new_in(len: usize, block_allocator: Option<&cf::Allocator>) -> os::Result<Self> {
        let block = BlockBuf::with_mem_block_in(len, block_allocator)?;
        Ok(Self(block))
    }

    pub fn new(len: usize) -> os::Result<Self> {
        let block = BlockBuf::with_mem_block_in(len, None)?;
        Ok(Self(block))
    }

    pub fn retained(&self) -> Self {
        Self(self.0.retained())
    }
}

impl std::ops::Deref for ContiguousBlockBuf {
    type Target = BlockBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<BlockBuf> for ContiguousBlockBuf {
    #[inline]
    fn as_ref(&self) -> &BlockBuf {
        &self.0
    }
}

impl AsRef<[u8]> for ContiguousBlockBuf {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        unsafe { self.0.as_slice().unwrap_unchecked() }
    }
}

impl AsMut<[u8]> for ContiguousBlockBuf {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { self.0.as_mut_slice().unwrap_unchecked() }
    }
}

impl TryFrom<&BlockBuf> for Vec<u8> {
    type Error = crate::os::Error;

    fn try_from(value: &BlockBuf) -> Result<Self, Self::Error> {
        let len = value.len();
        if len == 0 {
            return Ok(vec![]);
        } else {
            let mut vec = Vec::with_capacity(len);
            unsafe { vec.set_len(len) };
            (unsafe { value.copy_bytes(0, len, vec.as_mut_ptr()) })?;
            Ok(vec)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        ffi::c_void,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use crate::cm;

    // bytes::Bytes::from_owner
    fn from_owner<T>(_owner: T)
    where
        T: AsRef<[u8]> + Send + 'static,
    {
    }

    #[test]
    fn basics() {
        let buf = cm::BlockBuf::with_mem_block(100).unwrap();
        let contiguous_buf = buf.try_contiguous_buf().unwrap();
        assert_eq!(contiguous_buf.len(), 100);

        from_owner(contiguous_buf);

        let vec: Vec<u8> = buf.as_ref().try_into().unwrap();
        assert_eq!(vec.len(), 100);

        let mut arr = [0u8; 100];
        buf.copy_to(0, &mut arr).unwrap();
        let err = buf.copy_to(10, &mut arr).unwrap_err();
        assert_eq!(err, cm::block_buf_err::BAD_LEN_PARAM);
    }

    #[test]
    fn custom_src() {
        static ALLOC_N: AtomicUsize = AtomicUsize::new(0);
        static FREE_N: AtomicUsize = AtomicUsize::new(0);
        extern "C" fn alloc(_refcon: *mut c_void, size: usize) -> *mut c_void {
            let vec = vec![0u8; size];
            assert_eq!(vec.len(), vec.capacity());
            ALLOC_N.fetch_add(1, Ordering::SeqCst);

            vec.into_raw_parts().0 as _
        }

        extern "C" fn free(_refcon: *mut c_void, buf: *mut c_void, size: usize) {
            FREE_N.fetch_add(1, Ordering::SeqCst);
            unsafe { Vec::from_raw_parts(buf as *mut u8, size, size) };
        }

        {
            let src = cm::BlockCustomSrc {
                alloc: Some(alloc),
                free: Some(free),
                ..Default::default()
            };

            let mut buf = cm::BlockBuf::with_custom_src(100, &src).unwrap();

            _ = buf.as_mut_slice();
            assert_eq!(buf.len(), 100);
        }

        assert_eq!(1, ALLOC_N.load(Ordering::SeqCst));
        assert_eq!(1, FREE_N.load(Ordering::SeqCst));
    }

    #[test]
    fn contiguous() {
        let mut buf = cm::ContiguousBlockBuf::new(100).unwrap();
        assert_eq!(buf.len(), 100);
        assert!(buf.is_range_contiguous(0, 100));
        let slice = buf.as_slice().unwrap();
        assert_eq!(slice[0], 0);
        let mut_slice: &mut [u8] = buf.as_mut();
        mut_slice[0] = 25;
        let slice = buf.as_slice().unwrap();
        assert_eq!(slice[0], 25);
    }
}
