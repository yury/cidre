#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum FeatureType {
    #[doc(alias = "MLFeatureTypeInvalid")]
    Invalid = 0,

    #[doc(alias = "MLFeatureTypeInt64")]
    I64 = 1,

    #[doc(alias = "MLFeatureTypeDouble")]
    F64 = 2,

    #[doc(alias = "MLFeatureTypeString")]
    String = 3,

    /// &cv::PixelBuf
    #[doc(alias = "MLFeatureTypeImage")]
    Image = 4,

    #[doc(alias = "MLFeatureTypeMultiArray")]
    MultiArray = 5,

    /// Numerically weighted hashable objects (e.g. word counts)
    #[doc(alias = "MLFeatureTypeDictionary")]
    Dictionary = 6,

    #[doc(alias = "MLFeatureTypeSequence")]
    Sequence = 7,

    /// ml::State
    #[doc(alias = "MLFeatureTypeState")]
    State = 8,
}
