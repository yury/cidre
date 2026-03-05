mod language;
pub use language::Lang;

mod language_recognizer;
pub use language_recognizer::LangRecognizer;

mod embedding;
pub use embedding::Distance;
pub use embedding::DistanceType;
pub use embedding::Embedding;

#[link(name = "NaturalLanguage", kind = "framework")]
unsafe extern "C" {}

#[link(name = "nl", kind = "static")]
unsafe extern "C" {}
