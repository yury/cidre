use crate::{define_obj_type, ns};

pub mod cache;
pub use cache::Cache as AssetCache;

pub mod reader;
pub use reader::Reader;
pub use reader::Status as ReaderStatus;

pub mod reader_output;
pub use reader_output::ReaderAudioMixOutput;
pub use reader_output::ReaderOutput;
pub use reader_output::ReaderOutputCaptionAdaptor;
pub use reader_output::ReaderOutputMetadataAdaptor;
pub use reader_output::ReaderSampleReferenceOutput;
pub use reader_output::ReaderTrackOutput;
pub use reader_output::ReaderVideoCompositionOutput;

pub mod writer;
pub use writer::Status as WriterStatus;
pub use writer::Writer as AssetWriter;

pub mod writer_input;
pub use writer_input::WriterInput;

pub mod track;
pub use track::FragmentedTrack;
pub use track::Track;

define_obj_type!(Asset(ns::Id));
define_obj_type!(URLAsset(Asset));
define_obj_type!(FragmentedAsset(URLAsset));
define_obj_type!(FragmentedAssetMinder(ns::Id));
