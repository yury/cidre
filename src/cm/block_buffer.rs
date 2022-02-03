use crate::{
    cf::{Allocator, Retained, Type, TypeId},
    define_cf_type,
    os::{self, NO_ERR},
};

#[repr(transparent)]
pub struct BlockBufferFlags(pub u32);

impl BlockBufferFlags {
    pub const NONE: Self = Self(0);
    pub const ASSURE_MEMORY_NOW: Self = Self(1u32 << 0);
    pub const ALWAYS_COPY_DATA: Self = Self(1u32 << 1);
    pub const DONT_OPTIMIZE_DEPTH: Self = Self(1u32 << 2);
    pub const PERMIT_EMPTY_REFERENCE: Self = Self(1u32 << 3);
}

define_cf_type!(BlockBuffer(Type));

impl BlockBuffer {
    #[inline]
    pub fn get_type_id() -> TypeId {
        unsafe { CMBlockBufferGetTypeID() }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { CMBlockBufferIsEmpty(self) }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let b = cm::BlockBuffer::create_empty(None, 0, cm::BlockBufferFlags::NONE).expect("hmm");
    /// assert_eq!(true, b.is_empty());
    /// ```
    #[inline]
    pub fn create_empty<'a>(
        structure_allocator: Option<&Allocator>,
        sub_block_capacity: u32,
        flags: BlockBufferFlags,
    ) -> Result<Retained<'a, BlockBuffer>, os::Status> {
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
    fn CMBlockBufferGetTypeID() -> TypeId;
    fn CMBlockBufferIsEmpty(the_buffer: &BlockBuffer) -> bool;

    fn CMBlockBufferCreateEmpty<'a>(
        structure_allocator: Option<&Allocator>,
        sub_block_capacity: u32,
        flags: BlockBufferFlags,
        block_buffer_out: &mut Option<Retained<'a, BlockBuffer>>,
    ) -> os::Status;
}
