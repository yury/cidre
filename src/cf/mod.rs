mod base;
pub use base::copy_type_id_description;
pub use base::ComparisonResult;
pub use base::AllocatorRef;
pub use base::ComparatorFunction;
pub use base::HashCode;
pub use base::Index;
pub use base::Null;
pub use base::OptionFlags;
pub use base::Range;
pub use base::TypeID;
pub use base::TypeRef;
pub use base::NOT_FOUND;

mod number;
pub use number::boolean_get_type_id;
pub use number::BooleanRef;
pub use number::number_get_type_id;
pub use number::Number;
pub use number::NumberRef;
pub use number::NumberType;

mod string;
pub use string::string_get_type_id;
pub use string::MutableString;
pub use string::MutableStringRef;
pub use string::String;
pub use string::StringCompareFlags;
pub use string::StringEncoding;
pub use string::StringRef;
pub use string::UniChar;

mod array;
pub use array::array_get_type_id;
pub use array::Array;
pub use array::ArrayCallbacks;
pub use array::ArrayRef;
pub use array::MutableArrayRef;

// pub type TimeInterval = f64;
// pub type AbsoluteTime = TimeInterval;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {}
