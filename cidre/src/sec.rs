mod base;
pub use base::*;

pub mod certificate;
pub use certificate::oids as cert_oids;
#[cfg(target_os = "macos")]
pub use certificate::prop_keys;
#[cfg(target_os = "macos")]
pub use certificate::prop_types;

pub mod item;
pub use item::class;
pub use item::class_key;
pub use item::match_keys;
pub use item::match_limit;
pub use item::matching as item_matching;
pub use item::return_data;

pub mod identity;
pub use identity::ProtocolIdentity;
pub mod key;
pub use key::attrs as key_attrs;
pub use key::key_classes;
pub use key::key_types;
pub mod protocol_metadata;
pub use protocol_metadata::ProtocolMetadata;
pub mod protocol_types;
pub use protocol_types::ProtocolTrust;
#[cfg(feature = "blocks")]
pub use protocol_types::{ProtocolVerify, ProtocolVerifyComplete};

pub mod policy;
// pub use policy::Policy;

#[link(name = "Security", kind = "framework")]
unsafe extern "C" {}
