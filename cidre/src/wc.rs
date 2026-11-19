mod session;
pub use session::ActivationState as SessionActivationState;
pub use session::{AnyDelegate as AnySessionDelegate, Delegate as SessionDelegate, DelegateImpl as SessionDelegateImpl, Session};

mod error;
pub use error::Code as ErrorCode;
pub use error::Domain as ErrorDomain;

#[link(name = "WatchConnectivity", kind = "framework")]
unsafe extern "C" {}
