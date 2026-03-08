#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[doc(alias = "MLMultiArrayShapeConstraintType")]
#[repr(isize)]
pub enum MultiArrayShapeConstraintType {
    #[doc(alias = "MLMultiArrayShapeConstraintTypeUnspecified")]
    Unspecified = 1,

    #[doc(alias = "MLMultiArrayShapeConstraintTypeEnumerated")]
    Enumerated = 2,

    #[doc(alias = "MLMultiArrayShapeConstraintTypeRange")]
    Range = 3,
}
