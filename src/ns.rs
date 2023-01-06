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
pub use port::Port;

mod url_request;
pub use url_request::Attribution as URLRequestAttribution;
pub use url_request::CachePolicy as URLRequestCachePolicy;
pub use url_request::NetworkServiceType as URLRequestNetworkServiceType;
pub use url_request::URLRequest;
pub use url_request::URLRequestMut;

mod url_response;
pub use url_response::HTTPURLResponse;
pub use url_response::URLResponse;

mod url_session;
pub use url_session::Configuration as URLSessionConfiguration;
pub use url_session::DataTask as URLSessionDataTask;
pub use url_session::DownloadTask as URLSessionDownloadTask;
pub use url_session::MultipathServiceType as URLSessionMultipathServiceType;
pub use url_session::Session as URLSession;
pub use url_session::StreamTask as URLSessionStreamTask;
pub use url_session::Task as URLSessionTask;
pub use url_session::TaskPriority as URLSessionTaskPriority;
pub use url_session::TaskState as URLSessionTaskState;
pub use url_session::UploadTask as URLSessionUploadTask;
pub use url_session::WebSocketCloseCode as URLSessionWebSocketCloseCode;
pub use url_session::WebSocketMessage as URLSessionWebSocketMessage;
pub use url_session::WebSocketMessageType as URLSessionWebSocketMessageType;
pub use url_session::WebSocketTask as URLSessionWebSocketTask;

mod url_cache;
pub use url_cache::CachedURLResponse;
pub use url_cache::StoragePolicy as URLCacheStoragePolicy;
pub use url_cache::URLCache;

mod value;
pub use value::Number;
pub use value::Value;

mod array;
pub use array::Array;

mod data;
pub use data::Data;
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
pub use url::URL;
