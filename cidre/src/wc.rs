mod session;
pub use session::ActivationState as SessionActivationState;
pub use session::Session;

mod error;
pub use error::Code as ErrorCode;
pub use error::Domain as ErrorDomain;

#[link(name = "WatchConnectivity", kind = "framework")]
unsafe extern "C" {}

#[link(name = "wc", kind = "static")]
unsafe extern "C" {}
