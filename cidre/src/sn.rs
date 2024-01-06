use crate::define_obj_type;
use crate::ns;

mod types;
pub use types::Id;

mod error;
pub use error::Code as ErrorCode;
pub use error::Domain as ErrorDomain;

define_obj_type!(pub Request(ns::Id));

mod result;
pub use result::Result;
pub use result::ResultsObserving;
pub use result::ResultsObservingImpl;

mod analyzer;
pub use analyzer::AudioFileAnalyzer;
pub use analyzer::AudioStreamAnalyzer;

mod time_duration_constraint;
pub use time_duration_constraint::Constraint as TimeDurationConstraint;
pub use time_duration_constraint::Type as TimeDurationConstraintType;

mod classify_sound_request;
pub use classify_sound_request::ClassifySoundRequest;

mod classification_result;
pub use classification_result::Classification;
pub use classification_result::ClassificationResult;
