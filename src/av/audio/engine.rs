use crate::{define_obj_type, ns, os};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ManualRenderingError(pub os::Status);

impl ManualRenderingError {
    /// The operation cannot be performed because the engine is either not in manual
    ///	rendering mode or the right variant of it.
    pub const INVALID_MODE: Self = Self(os::Status(-80800));

    /// The operation cannot be performed because the engine is initialized (i.e. not stopped).
    pub const INITIALIZED: Self = Self(os::Status(-80801));

    /// The operation cannot be performed because the engine is not running (i.e. not started).
    pub const NOT_RUNNING: Self = Self(os::Status(-80801));
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(isize)]
pub enum ManualRenderingStatus {
    /// An error occurred when rendering and no data was returned. See the returned error code
    /// for the description of the error.
    Error = -1,
    /// All of the requested data was returned successfully.
    Succes = 0,
    /// Applicable only to the input node, when it provides input data for rendering
    /// (see `AVAudioInputNode(setManualRenderingInputPCMFormat:inputBlock:)`).
    /// Indicates that not enough input data was returned by the input node to satisfy the
    /// render request at the current time. The output buffer may contain data rendered by other
    /// active sources in the engine's processing graph.
    InsufficientDataFromInputNode = 1,
    /// The operation could not be performed now, but the client could retry later if needed.
    /// This is usually to guard a realtime render operation (e.g. rendering through
    /// `manualRenderingBlock`) when a reconfiguration of the engine's internal state
    /// is in progress.
    CannotDoInCurrentContext = 2,
}

/// By default, the engine is connected to an audio device and automatically renders in realtime.
/// It can also be configured to operate in manual rendering mode, i.e. not connected to an
/// audio device and rendering in response to requests from the client.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(isize)]
pub enum ManualRenderingMode {
    /// The engine operates in an offline mode without any realtime constraints.
    ManualRenderingModeOffline = 0,
    /// The engine operates under realtime constraints, i.e. it will not make any blocking call
    ///	(e.g. calling libdispatch, blocking on a mutex, allocating memory etc.) while rendering.
    /// Note that only the block based render mechanism can be used in this mode
    /// (see `AVAudioEngine(manualRenderingBlock)`.
    ManualRenderingModeRealtime = 1,
}

define_obj_type!(Engine(ns::Id));
