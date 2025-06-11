#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[doc(alias = "MTLSparseTextureMappingMode")]
#[repr(usize)]
pub enum SparseTextureMappingMode {
    Map = 0,
    Unmap = 1,
}
