use crate::objc;

pub use objc::{
    ns::{Integer, UInteger},
    Class, Id, Sel,
};

mod range;
pub use range::Range;

mod process_info;
pub use process_info::ProcessInfo;
pub use process_info::ThermalState as ProcessInfoThermalState;

pub mod objc_runtime;
pub use objc_runtime::ExceptionName;

pub mod exception;
pub use exception::get_uncaought_exception_handler;
pub use exception::set_uncaught_exception_handler;
pub use exception::try_catch;
pub use exception::Exception;
pub use exception::UncaughtExceptionHandler;

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
pub use url_response::HTTPURLResponse;
pub use url_response::UrlResponse;

mod url_session;
pub use url_session::Configuration as UrlSessionConfiguration;
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

mod value;
pub use value::Number;
pub use value::Value;

mod null;
pub use null::Null;

mod array;
pub use array::Array;
pub use array::ArrayMut;

mod dictionary;
pub use dictionary::Dictionary;
pub use dictionary::DictionaryMut;

mod enumerator;
pub use enumerator::FEIterator;
pub use enumerator::FastEnumeration;
pub use enumerator::FastEnumerationState;

mod set;
pub use set::Set;
pub use set::SetMut;

mod data;
pub use data::Data;
pub use data::DataMut;
pub use data::ReadingOptions as DataReadingOptions;
pub use data::SearchOptions as DataSearchOptions;
pub use data::WritingOptions as DataWritingOptions;

mod regular_expression;
pub use regular_expression::MatchingFlags;
pub use regular_expression::MatchingOptions;
pub use regular_expression::Options as RegularExpressionOptions;
pub use regular_expression::RegularExpression;

mod text_checking_result;
pub use text_checking_result::TextCheckingResult;
pub use text_checking_result::Type as TextCheckingType;

mod string;
pub use string::Encoding as StringEncoding;
pub use string::String;
pub use string::StringMut;

mod url;
pub use url::ResourceKey as UrlResourceKey;
pub use url::Url;

mod uuid;
pub use uuid::UUID;

mod run_loop;
pub use run_loop::RunLoop;
pub use run_loop::RunLoopMode;

mod date;
pub use date::Date;
pub use date::TimeInterval;
pub use date::TIME_INTERVAL_SINCE_1970;

mod error;
pub use error::Domain as ErrorDomain;
pub use error::Error;

mod timer;
pub use timer::Timer;

mod file_manager;
pub use file_manager::DirectoryEnumerationOptions;
pub use file_manager::FileAttrKey;
pub use file_manager::FileAttrType;
pub use file_manager::FileManager;
pub use file_manager::FileProtectionType;
pub use file_manager::ItemReplacementOptions as FileManagerItemReplacementOptions;
pub use file_manager::UrlRelationship;
pub use file_manager::VolumeEnumerationOptions;

mod path_utilities;
pub use path_utilities::full_user_name;
pub use path_utilities::home_directory;
pub use path_utilities::home_directory_for_user;
pub use path_utilities::home_directory_for_user_ar;
pub use path_utilities::search_path_for_dirs_in_domains;
pub use path_utilities::search_path_for_dirs_in_domains_ar;
pub use path_utilities::temporary_directory;
pub use path_utilities::user_name;
pub use path_utilities::SearchPathDirectory;
pub use path_utilities::SearchPathDomainMask;

mod notification;
pub use notification::Notification;
pub use notification::NotificationCenter;
pub use notification::NotificationName;

mod operation;
pub use operation::BlockOperation;
pub use operation::Operation;
pub use operation::OperationQueue;

mod coder;
pub use coder::Coder;
pub use coder::DecodingFailurePolicy;

mod key_value_observing;
pub use key_value_observing::CidreObserver;
pub use key_value_observing::KVChange;
pub use key_value_observing::KVChangeKey;
pub use key_value_observing::KVOOpts;
pub use key_value_observing::KVObserverRegistration;
pub use key_value_observing::KVObserverRegistrationImpl;
pub use key_value_observing::KVObserving;
pub use key_value_observing::KVSetMutationKind;
pub use key_value_observing::Observer;

pub fn log(str: &crate::ns::String) {
    unsafe {
        cidre_log(str);
    }
}

#[macro_export]
macro_rules! ns_log {
    ($($arg:tt)*) => {{
        let rstr = format!("{}",format_args!($($arg)*));
        let ns_str = $crate::ns::String::with_str_no_copy(&rstr);
        $crate::ns::log(ns_str.as_ref());
    }};
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn cidre_log(str: &crate::ns::String);
}
