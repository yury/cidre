//! Link Presentation metadata.
mod link_metadata;
pub use link_metadata::LinkMetadata;

#[link(name = "LinkPresentation", kind = "framework")]
unsafe extern "C" {}
