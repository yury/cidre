use crate::{define_obj_type, ns};

mod types;
pub use types::Port;
pub use types::RecordPermission;
pub use types::StereoOrientation;
pub use types::PromptStyle;
pub use types::RouteSharingPolicy;
pub use types::IOType;
pub use types::SilenceSecondaryAudioHintType;
pub use types::SetActiveOptions;
pub use types::InterruptionReason;
pub use types::InterruptionOptions;
pub use types::InterruptionType;
pub use types::CategoryOptions;
pub use types::RouteChangeReason;
pub use types::PortOverride;
pub use types::ActivationOptions;
pub use types::Category;
pub use types::Mode;


define_obj_type!(Session(ns::Id));
