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
pub use string::CompareFlags as StringCompareFlags;
pub use string::Encoding as StringEncoding;
pub use string::MutableString;
pub use string::String;

pub mod array;
pub use array::Array;
pub use array::ArrayOf;
pub use array::Callbacks as ArrayCallbacks;
pub use array::CopyDescriptionCallBack as ArrayCopyDescriptionCallBack;
pub use array::EqualCallBack as ArrayEqualCallBack;
pub use array::MutableArray;
pub use array::ReleaseCallBack as ArrayReleaseCallBack;
pub use array::RetainCallBack as ArrayRetainCallBack;

pub mod dictionary;
pub use dictionary::ApplierFunction as DictionaryApplierFunction;
pub use dictionary::Dictionary;
pub use dictionary::KeyCallBacks as DictionaryKeyCallBacks;
pub use dictionary::MutableDictionary;
pub use dictionary::ValueCallBacks as DictionaryValueCallBacks;

pub mod date;
pub use date::absolute_time_get_current;
pub use date::AbsoluteTime;
pub use date::Date;
pub use date::TimeInterval;

pub mod url;
pub use url::PathStyle;
pub use url::URL;

pub mod locale;
pub use locale::Identifier as LocaleIdentifier;
pub use locale::Locale;

pub mod bundle;
pub use bundle::Bundle;

pub mod error;
pub use error::Domain as ErrorDomain;
pub use error::Error;

// pub type TimeInterval = f64;
// pub type AbsoluteTime = TimeInterval;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {}
