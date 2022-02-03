pub mod runtime;

pub use runtime::Retained;
pub use runtime::Type;

pub mod base;
pub use base::copy_type_id_description;
pub use base::Allocator;
pub use base::ComparatorFunction;
pub use base::ComparisonResult;
pub use base::HashCode;
pub use base::Index;
pub use base::Null;
pub use base::OptionFlags;
pub use base::Range;
pub use base::TypeID;
pub use base::NOT_FOUND;

pub mod number;
pub use number::boolean_get_type_id;
pub use number::number_get_type_id;
pub use number::Boolean;
pub use number::Number;
pub use number::NumberType;

pub mod string;
pub use string::string_get_type_id;
pub use string::MutableString;
pub use string::String;
pub use string::StringCompareFlags;
pub use string::StringEncoding;
pub use string::UniChar;

pub mod array;
pub use array::array_get_type_id;
pub use array::Array;
pub use array::ArrayCallbacks;
pub use array::ArrayCopyDescriptionCallBack;
pub use array::ArrayEqualCallBack;
pub use array::ArrayReleaseCallBack;
pub use array::ArrayRetainCallBack;
pub use array::MutableArray;

pub mod dictionary;
pub use dictionary::dictionary_get_type_id;
pub use dictionary::Dictionary;
pub use dictionary::DictionaryApplierFunction;
pub use dictionary::DictionaryKeyCallBacks;
pub use dictionary::DictionaryValueCallBacks;

// pub type TimeInterval = f64;
// pub type AbsoluteTime = TimeInterval;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {}
