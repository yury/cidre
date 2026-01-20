use crate::arc;
use crate::objc;

pub type Result<'ear, R = (), E = &'ear Error> = std::result::Result<R, E>;
pub type ExResult<'ear, R = ()> = std::result::Result<R, &'ear Exception>;

pub use objc::{Class, Id, Sel, ns};

mod bundle;
pub use bundle::Bundle;

mod range;
pub use range::Range;

mod process_info;
pub use process_info::OsVersion;
pub use process_info::ProcessInfo;
pub use process_info::ThermalState as ProcessInfoThermalState;

mod predicate;
pub use predicate::Predicate;

mod formatter;
pub use formatter::Formatter;

mod date_formatter;
pub use date_formatter::DateFormatter;
pub use date_formatter::DateFormatterStyle;

mod iso_8601_date_formatter;
pub use iso_8601_date_formatter::Iso8601DateFormatOpts;
pub use iso_8601_date_formatter::Iso8601DateFormatter;

mod progress;
pub use progress::Progress;

pub mod objc_runtime;
pub use objc_runtime::ExceptionName;
pub use objc_runtime::NOT_FOUND;
pub use objc_runtime::class_from_ns_string;
pub use objc_runtime::protocol_from_ns_string;
pub use objc_runtime::selector_from_ns_string;
pub use objc_runtime::string_from_class;
pub use objc_runtime::string_from_protocol;
pub use objc_runtime::string_from_selector;

pub mod exception;
pub use exception::Exception;
pub use exception::UncaughtExceptionHandler;
pub use exception::set_uncaught_exception_handler;
pub use exception::try_catch;
pub use exception::try_catch_err;
pub use exception::uncaught_exception_handler;

mod port;
pub use port::MachPort;
pub use port::MachPortDelegate;
pub use port::MachPortDelegateImpl;
pub use port::Port;

mod url_request;
pub use url_request::Attribution as UrlRequestAttribution;
pub use url_request::CachePolicy as UrlRequestCachePolicy;
pub use url_request::NetworkServiceType as UrlRequestNetworkServiceType;
pub use url_request::UrlRequest;
pub use url_request::UrlRequestMut;

mod url_response;
pub use url_response::HttpUrlResponse;
pub use url_response::UrlResponse;

mod url_session;
pub use url_session::Cfg as UrlSessionCfg;
pub use url_session::DataTask as UrlSessionDataTask;
pub use url_session::DownloadTask as UrlSessionDownloadTask;
pub use url_session::MultipathServiceType as UrlSessionMultipathServiceType;
pub use url_session::Session as UrlSession;
pub use url_session::StreamTask as UrlSessionStreamTask;
pub use url_session::Task as UrlSessionTask;
pub use url_session::TaskPriority as UrlSessionTaskPriority;
pub use url_session::TaskState as UrlSessionTaskState;
pub use url_session::UploadTask as UrlSessionUploadTask;
pub use url_session::WebSocketCloseCode as UrlSessionWebSocketCloseCode;
pub use url_session::WebSocketMessage as UrlSessionWebSocketMessage;
pub use url_session::WebSocketMessageType as UrlSessionWebSocketMessageType;
pub use url_session::WebSocketTask as UrlSessionWebSocketTask;

mod url_cache;
pub use url_cache::CachedUrlResponse;
pub use url_cache::StoragePolicy as UrlCacheStoragePolicy;
pub use url_cache::UrlCache;

mod user_activity;
pub use user_activity::UserActivity;
pub use user_activity::UserActivityDelegate;
pub use user_activity::UserActivityDelegateImpl;
pub use user_activity::UserActivityPersistentId;

mod user_defaults;
pub use user_defaults::UserDefaults;

mod value;
pub use value::Number;
pub use value::Value;

mod null;
pub use null::Null;

mod index_path;
pub use index_path::IndexPath;

mod index_set;
pub use index_set::IndexSet;
pub use index_set::IndexSetMut;

mod array;
pub use array::Array;
pub use array::ArrayMut;
pub use array::arr;

mod dictionary;
pub use dictionary::Dictionary;
pub use dictionary::DictionaryMut;

mod enumerator;
pub use enumerator::FastEnum;
pub use enumerator::FeIterator;
pub use enumerator::FeState;

mod set;
pub use set::Set;
pub use set::SetMut;

mod data;
pub use data::Data;
pub use data::DataMut;
pub use data::ReadOpts as DataReadOpts;
pub use data::SearchOpts as DataSearchOpts;
pub use data::WriteOpts as DataWriteOpts;

mod regular_expression;
pub use regular_expression::MatchFlags;
pub use regular_expression::MatchOpts;
pub use regular_expression::Opts as RegexOpts;
pub use regular_expression::Regex;

mod text_checking_result;
pub use text_checking_result::TextCheckingResult;
pub use text_checking_result::Type as TextCheckingType;

mod string;
pub use string::Encoding as StringEncoding;
pub use string::String;
pub use string::StringMut;
pub use string::str;

mod attributed_string;
pub use attributed_string::AttrString;
pub use attributed_string::AttrStringFormatCtxKey;
pub use attributed_string::AttrStringKey;
pub use attributed_string::AttrStringMut;
pub use attributed_string::InlinePresentationIntent;

mod url;
pub use url::ResKey as UrlResKey;
pub use url::Url;

mod uuid;
pub use uuid::Uuid;

mod run_loop;
pub use run_loop::RunLoop;
pub use run_loop::RunLoopMode;

mod date;
pub use date::Date;
pub use date::TIME_INTERVAL_SINCE_1970;
pub use date::TimeInterval;

mod error;
pub use error::Domain as ErrorDomain;
pub use error::Error;
pub use error::if_err;
pub use error::if_false;
pub use error::if_none;
pub use error::user_info_keys as error_user_info_keys;

mod thread;
pub use thread::Thread;

mod timer;
pub use timer::Timer;

mod file_manager;
pub use file_manager::DirEnumOpts;
pub use file_manager::FileAttrKey;
pub use file_manager::FileAttrType;
pub use file_manager::FileManager;
pub use file_manager::FileProtectionType;
pub use file_manager::ItemReplacementOpts as FileManagerItemReplacementOpts;
pub use file_manager::UnmountOpts as FileManagerUnmountOpts;
pub use file_manager::UrlRelationship;
pub use file_manager::VolumeEnumOpts;

mod path_utilities;
pub use path_utilities::SearchPathDirectory;
pub use path_utilities::SearchPathDomainMask;
pub use path_utilities::full_user_name;
pub use path_utilities::home_dir;
pub use path_utilities::home_dir_for_user;
pub use path_utilities::home_dir_for_user_ar;
pub use path_utilities::search_path_for_dirs_in_domains;
pub use path_utilities::search_path_for_dirs_in_domains_ar;
pub use path_utilities::tmp_dir;
pub use path_utilities::user_name;

mod notification;
pub use notification::Notification;
pub use notification::NotificationCenter;
pub use notification::NotificationGuard;
pub use notification::NotificationName;

mod operation;
pub use operation::BlockOp;
pub use operation::Op;
pub use operation::OpQueue;

mod coder;
pub use coder::Coder;
pub use coder::DecodingFailurePolicy;

mod locale;
pub use locale::Locale;

mod key_value_observing;
pub use key_value_observing::CidreObserver;
pub use key_value_observing::KvChange;
pub use key_value_observing::KvChangeKey;
pub use key_value_observing::KvObserverRegistration;
pub use key_value_observing::KvObserverRegistrationImpl;
pub use key_value_observing::KvObserving;
pub use key_value_observing::KvSetMutationKind;
pub use key_value_observing::KvoOpts;
pub use key_value_observing::Observer;

mod key_value_coding;

mod stream;
pub use stream::InputStream;
pub use stream::OutputStream;
pub use stream::Status as StreamStatus;

mod keyed_archiver;
pub use keyed_archiver::KeyedArchiver;

mod keyed_unarchiver;
pub use keyed_unarchiver::KeyedUnarchiver;

#[cfg(any(
    all(feature = "app", target_os = "macos"),
    all(
        feature = "ui",
        any(target_os = "ios", target_os = "tvos", target_os = "visionos")
    )
))]
mod diffable_data_source_snapshot;
#[cfg(any(
    all(feature = "app", target_os = "macos"),
    all(
        feature = "ui",
        any(target_os = "ios", target_os = "tvos", target_os = "visionos")
    )
))]
pub use diffable_data_source_snapshot::DiffableDataSrcSnapshot;

#[cfg(all(
    feature = "ui",
    any(
        all(target_os = "ios", feature = "ios_14_0"),
        all(target_os = "tvos", feature = "tvos_14_0"),
        all(target_os = "visionos", feature = "visionos_1_0")
    )
))]
mod diffable_data_source_section_snapshot;
#[cfg(all(
    feature = "ui",
    any(
        all(target_os = "ios", feature = "ios_14_0"),
        all(target_os = "tvos", feature = "tvos_14_0"),
        all(target_os = "visionos", feature = "visionos_1_0")
    )
))]
pub use diffable_data_source_section_snapshot::DiffableDataSrcSectionSnapshot;

mod ordered_collection_change;

pub use ordered_collection_change::CollectionChangeType;
pub use ordered_collection_change::OrderedCollectionChange;

pub fn log_string(str: &crate::ns::String) {
    unsafe {
        cidre_log(str);
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let rstr = format!("{}",format_args!($($arg)*));
        let ns_str = $crate::ns::String::with_str_no_copy(&rstr);
        $crate::ns::log_string(ns_str.as_ref());
    }};
}

pub use log;

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    fn cidre_log(str: &crate::ns::String);
}

/// App Kit
#[cfg(all(feature = "app", target_os = "macos"))]
mod app;

#[cfg(all(feature = "app", target_os = "macos"))]
pub use app::*;

#[cfg(target_os = "macos")]
mod geometry;

#[cfg(target_os = "macos")]
pub use geometry::*;

#[cfg(feature = "xpc")]
pub mod xpc;

/// Special Exception or Error type
/// some API can still throw exception even if they
/// return Error
///
/// See av::AudioSession API
#[derive(Debug)]
pub enum ExErr<'ar> {
    Ex(&'ar Exception),
    Err(&'ar Error),
}

#[cfg(any(
    all(
        any(target_os = "ios", target_os = "tvos", target_os = "watchos"),
        feature = "ui"
    ),
    all(target_os = "macos", feature = "app")
))]
mod text_attachment;
#[cfg(any(
    all(
        any(target_os = "ios", target_os = "tvos", target_os = "watchos"),
        feature = "ui"
    ),
    all(target_os = "macos", feature = "app")
))]
pub use text_attachment::TextAttachment;

pub use objc::ns::Integer;
pub use objc::ns::UInteger;

mod zone;
pub use zone::Zone;

#[objc::protocol(NSCopying)]
pub trait Copying {
    #[objc::msg_send(copyWithZone:)]
    unsafe fn copy_with_zone(&self, zone: *mut Zone) -> Option<arc::Retained<Id>>;
}

#[objc::protocol(NSMutableCopying)]
pub trait CopyingMut {
    #[objc::msg_send(mutableCopyWithZone:)]
    unsafe fn copy_with_zone_mut(&self, zone: *mut Zone) -> Option<arc::Retained<Id>>;
}
