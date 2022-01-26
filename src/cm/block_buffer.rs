use crate::{
    cf::{AllocatorRef, TypeID, TypeRef},
    os::{self, NO_ERR}, define_ref,
};

/// ```
/// use cidre::cm;
///
/// assert_eq!(256, cm::block_buffer_get_type_id());
/// ```
#[inline]
pub fn block_buffer_get_type_id() -> TypeID {
    unsafe { CMBlockBufferGetTypeID() }
}

#[repr(transparent)]
pub struct BlockBufferFlags(pub u32);

impl BlockBufferFlags {
    pub const NONE: Self = Self(0);
    pub const ASSURE_MEMORY_NOW: Self = Self(1u32 << 0);
    pub const ALWAYS_COPY_DATA: Self = Self(1u32 << 1);
    pub const DONT_OPTIMIZE_DEPTH: Self = Self(1u32 << 2);
    pub const PERMIT_EMPTY_REFERENCE: Self = Self(1u32 << 3);
}

define_ref!(TypeRef, BlockBufferRef, BlockBuffer);

impl BlockBufferRef {
    pub fn is_empty(&self) -> bool {
        unsafe { CMBlockBufferIsEmpty(*self) }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let b = cm::BlockBufferRef::create_empty(None, 0, cm::BlockBufferFlags::NONE).expect("hmm");
    /// assert_eq!(true, b.is_empty());
    /// ```
    pub fn create_empty(
        structure_allocator: Option<AllocatorRef>,
        sub_block_capacity: u32,
        flags: BlockBufferFlags,
    ) -> Result<BlockBuffer, os::Status> {
        unsafe {
            let mut block_buffer_out = None;
            let res = CMBlockBufferCreateEmpty(
                structure_allocator,
                sub_block_capacity,
                flags,
                &mut block_buffer_out,
            );
            if res == NO_ERR {
                Ok(block_buffer_out.unwrap_unchecked())
            } else {
                Err(res)
            }
        }
    }
}

extern "C" {
    fn CMBlockBufferGetTypeID() -> TypeID;
    fn CMBlockBufferIsEmpty(the_buffer: BlockBufferRef) -> bool;

    fn CMBlockBufferCreateEmpty(
        structure_allocator: Option<AllocatorRef>,
        sub_block_capacity: u32,
        flags: BlockBufferFlags,
        block_buffer_out: &mut Option<BlockBuffer>,
    ) -> os::Status;
}
