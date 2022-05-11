use crate::{
    cf::{Allocator, Retained, Type, TypeId},
    define_cf_type, os,
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
    /// ```
    /// use cidre::cm;
    ///
    /// let b = cm::BlockBuffer::create_empty(None, 0, cm::BlockBufferFlags::NONE).expect("hmm");
    ///
    /// assert_eq!(b.get_type_id(), cm::BlockBuffer::type_id());
    /// ```
    #[inline]
    pub fn type_id() -> TypeId {
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
    /// let b = cm::BlockBuffer::create_empty(None, 0, cm::BlockBufferFlags::NONE).expect("hmm");
    ///
    /// assert!(b.is_empty());
    /// ```
    #[inline]
    pub fn create_empty<'a>(
        structure_allocator: Option<&Allocator>,
        sub_block_capacity: u32,
        flags: BlockBufferFlags,
    ) -> Result<Retained<'a, BlockBuffer>, os::Status> {
        unsafe {
            let mut block_buffer_out = None;
            CMBlockBufferCreateEmpty(
                structure_allocator,
                sub_block_capacity,
                flags,
                &mut block_buffer_out,
            )
            .to_result(block_buffer_out)
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
