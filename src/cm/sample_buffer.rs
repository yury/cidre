
use super::super::cf;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct SampleBufferRef(cf::TypeRef);