mod runtime;
pub use runtime::Type;

pub mod base;
pub use base::copy_type_id_desc;
pub use base::Allocator;
pub use base::AllocatorAllocateCb;
pub use base::AllocatorContext;
pub use base::AllocatorCopyDescCb;
pub use base::AllocatorDealloacteCb;
pub use base::ComparatorFunction;
pub use base::ComparisonResult;
pub use base::HashCode;
pub use base::Index;
pub use base::Null;
pub use base::OptionFlags;
pub use base::PropertyList;
pub use base::Range;
pub use base::TypeId;
pub use base::NOT_FOUND;

mod number;
pub use number::Boolean;
pub use number::Number;
pub use number::NumberType;

pub mod string;
pub use string::CompareFlags as StringCompareFlags;
pub use string::Encoding as StringEncoding;
pub use string::String;
pub use string::StringMut;

pub mod array;
pub use array::Array;
pub use array::ArrayMut;
pub use array::ArrayOf;
pub use array::ArrayOfMut;
pub use array::Cbs as ArrayCbs;
pub use array::CopyDescCb as ArrayCopyDescCb;
pub use array::EqualCb as ArrayEqualCb;
pub use array::ReleaseCb as ArrayReleaseCb;
pub use array::RetainCb as ArrayRetainCb;

pub mod dictionary;
pub use dictionary::ApplierFunction as DictionaryApplierFunction;
pub use dictionary::Dictionary;
pub use dictionary::DictionaryMut;
pub use dictionary::DictionaryOf;
pub use dictionary::DictionaryOfMut;
pub use dictionary::KeyCallBacks as DictionaryKeyCallBacks;
pub use dictionary::ValueCallBacks as DictionaryValueCallBacks;

pub mod date;
pub use date::absolute_time_current;
pub use date::AbsoluteTime;
pub use date::Date;
pub use date::TimeInterval;

mod url;
pub use url::PathStyle;
pub use url::Url;

pub mod locale;
pub use locale::Identifier as LocaleIdentifier;
pub use locale::Locale;

mod bundle;
pub use bundle::Bundle;

pub mod error;
pub use error::Domain as ErrorDomain;
pub use error::Error;

pub mod notification_center;
pub use notification_center::NotificationCenter;
pub use notification_center::NotificationName;

mod set;
pub use set::Set;
pub use set::SetMut;
pub use set::SetOf;

mod uuid;
pub use uuid::Uuid;

mod data;
pub use data::Data;
pub use data::DataMut;

pub mod run_loop;
pub use run_loop::Mode as RunLoopMode;
pub use run_loop::Observer as RunLoopObserver;
pub use run_loop::RunLoop;
pub use run_loop::RunResult as RunLoopRunResult;
pub use run_loop::Source as RunLoopSource;
pub use run_loop::Timer as RunLoopTimer;

pub mod socket;
pub use socket::CallBack as SocketCallBack;
pub use socket::CallBackType as SocketCallBackType;
pub use socket::Context as SocketContext;
pub use socket::Error as SocketError;
pub use socket::Flags as SocketFlags;
pub use socket::NativeHandle as SocketNativeHandle;
pub use socket::Signature as SocketSignature;
pub use socket::Socket;

mod mach_port;
pub use mach_port::MachPort;

mod attributed_string;
pub use attributed_string::AttrString;
pub use attributed_string::AttrStringMut;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {}
