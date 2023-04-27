/// Connection states sent by nw_connection_set_state_changed_handler.
/// States generally progress forward and do not move backwards, with the
/// exception of preparing and waiting, which may alternate before the connection
/// becomes ready or failed.
#[doc(alias = "nw_connection_state_t")]
pub struct State(pub u32);

impl State {
    ///	The state of the connection is not valid. This state
    ///	will never be delivered in the connection's state update handler, and can be treated as
    ///	an unexpected value.
    #[doc(alias = "nw_connection_state_invalid")]
    pub const INVALID: Self = Self(0);

    ///  The connection is waiting for a usable network before re-attempting
    #[doc(alias = "nw_connection_state_waiting")]
    pub const WAITING: Self = Self(1);

    /// The connection is in the process of establishing
    #[doc(alias = "nw_connection_state_preparing")]
    pub const PREPARING: Self = Self(2);

    /// The connection is established and ready to send and receive data upon
    #[doc(alias = "nw_connection_state_ready")]
    pub const READY: Self = Self(3);

    /// The connection has irrecoverably closed or failed
    #[doc(alias = "nw_connection_state_failed")]
    pub const FAILED: Self = Self(4);

    /// The connection has been cancelled by the caller
    #[doc(alias = "nw_connection_state_cancelled")]
    pub const CANCELLED: Self = Self(5);
}
