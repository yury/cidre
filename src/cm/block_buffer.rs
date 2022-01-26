
use std::ops::{Deref, DerefMut};

use crate::{os::{self, NO_ERR}, cf::AllocatorRef};

use super::super::cf;

#[inline]
pub fn block_buffer_get_type_id() -> cf::TypeID {
   unsafe {
    CMBlockBufferGetTypeID()
   }
}

#[repr(transparent)]
pub struct BlockBufferFlags(pub u32);

impl BlockBufferFlags {
  pub const NONE: Self = Self(0);
  pub const ASSURE_MEMORY_NOW:Self = Self(1u32 << 0);
  pub const ALWAYS_COPY_DATA: Self = Self(1u32 << 1);
  pub const DONT_OPTIMIZE_DEPTH: Self = Self(1u32 << 2);
  pub const PERMIT_EMPTY_REFERENCE: Self = Self(1u32 << 3);
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BlockBufferRef(cf::TypeRef);

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
  pub fn create_empty(structure_allocator: Option<AllocatorRef>, sub_block_capacity: u32,  flags: BlockBufferFlags) -> Result<BlockBuffer, os::Status> {
    unsafe {
      let mut block_buffer_out = None;
      let res = CMBlockBufferCreateEmpty(structure_allocator, sub_block_capacity, flags, &mut block_buffer_out);
      if res == NO_ERR {
        Ok(block_buffer_out.unwrap_unchecked())
      } else {
        Err(res)
      }
    }
  }
}

impl Deref for BlockBufferRef {
  type Target = cf::TypeRef;

  #[inline]
  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

impl DerefMut for BlockBufferRef {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
  }
}

#[repr(transparent)]
pub struct BlockBuffer(BlockBufferRef);

impl Deref for BlockBuffer {
  type Target = BlockBufferRef;

  #[inline]
  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

impl DerefMut for BlockBuffer {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
  }
}

impl Drop for BlockBuffer {
  fn drop(&mut self) {
      unsafe { self.release() }
  }
}

extern "C" {
  fn CMBlockBufferGetTypeID() -> cf::TypeID;
  fn CMBlockBufferIsEmpty(the_buffer: BlockBufferRef) -> bool;

  fn CMBlockBufferCreateEmpty(
    structure_allocator: Option<cf::AllocatorRef>,
    sub_block_capacity: u32,
    flags: BlockBufferFlags,
    block_buffer_out: &mut Option<BlockBuffer>,
  ) -> os::Status;
  // OSStatus	CMBlockBufferCreateEmpty(
	// 	CFAllocatorRef CM_NULLABLE structureAllocator,
	// 	uint32_t subBlockCapacity, 
	// 	CMBlockBufferFlags flags, 
	// 	CM_RETURNS_RETAINED_PARAMETER CMBlockBufferRef CM_NULLABLE * CM_NONNULL blockBufferOut)
	// 						API_AVAILABLE(macos(10.7), ios(4.0), tvos(9.0), watchos(6.0));
}