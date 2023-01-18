use crate::{define_obj_type, ns};

mod types;
pub use types::ActivationOptions;
pub use types::Category;
pub use types::CategoryOptions;
pub use types::IOType;
pub use types::InterruptionOptions;
pub use types::InterruptionReason;
pub use types::InterruptionType;
pub use types::Mode;
pub use types::Port;
pub use types::PortOverride;
pub use types::PromptStyle;
pub use types::RecordPermission;
pub use types::RouteChangeReason;
pub use types::RouteSharingPolicy;
pub use types::SetActiveOptions;
pub use types::SilenceSecondaryAudioHintType;
pub use types::StereoOrientation;

define_obj_type!(Session(ns::Id));

impl Session {}
