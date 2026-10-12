mod photo_library;
pub use photo_library::AccessLevel;
pub use photo_library::AuthorizationStatus;
pub use photo_library::PhotoLib;

mod asset_creation_request;
pub use asset_creation_request::AssetCreationRequest;
pub use asset_creation_request::AssetResourceType;

#[link(name = "Photos", kind = "framework")]
unsafe extern "C" {}
