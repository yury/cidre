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
pub use base::TypeId;
pub use base::NOT_FOUND;

pub mod number;
pub use number::Boolean;
pub use number::Number;
pub use number::NumberType;

pub mod string;
pub use string::MutableString;
pub use string::String;
pub use string::StringCompareFlags;
pub use string::StringEncoding;

pub mod array;
pub use array::Array;
pub use array::ArrayCallbacks;
pub use array::ArrayCopyDescriptionCallBack;
pub use array::ArrayEqualCallBack;
pub use array::ArrayReleaseCallBack;
pub use array::ArrayRetainCallBack;
pub use array::MutableArray;

pub mod dictionary;
pub use dictionary::Dictionary;
pub use dictionary::DictionaryApplierFunction;
pub use dictionary::DictionaryKeyCallBacks;
pub use dictionary::DictionaryValueCallBacks;
pub use dictionary::MutableDictionary;

pub mod date;
pub use date::absolute_time_get_current;
pub use date::AbsoluteTime;
pub use date::Date;
pub use date::TimeInterval;

pub mod url;
pub use url::PathStyle;
pub use url::URL;

pub mod locale;

// pub type TimeInterval = f64;
// pub type AbsoluteTime = TimeInterval;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
  
}
