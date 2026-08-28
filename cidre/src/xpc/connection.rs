use std::ffi::{CStr, c_char};

use crate::{arc, blocks, define_opts, define_xpc_type, dispatch, sys, xpc};

/// Delivered a message dictionary, or one of the `xpc::Error` constants.
///
/// Escaping and called many times, hence [`blocks::SyncBlock`].
#[doc(alias = "xpc_handler_t")]
pub type Handler = blocks::SyncBlock<fn(obj: &xpc::Object)>;

/// One-shot: the reply, or an `xpc::Error`.
pub type ReplyHandler = blocks::EscBlock<fn(reply: &xpc::Object)>;

define_opts!(
    #[doc(alias = "XPC_CONNECTION_MACH_SERVICE_*")]
    pub MachServiceFlags(u64)
);

impl MachServiceFlags {
    pub const NONE: Self = Self(0);

    /// Listen on a name advertised in this process' `launchd.plist(5)`.
    #[doc(alias = "XPC_CONNECTION_MACH_SERVICE_LISTENER")]
    pub const LISTENER: Self = Self(1 << 0);

    /// Look the name up in the privileged Mach bootstrap. A no-op alongside
    /// [`Self::LISTENER`].
    #[doc(alias = "XPC_CONNECTION_MACH_SERVICE_PRIVILEGED")]
    pub const PRIVILEGED: Self = Self(1 << 1);
}

define_xpc_type!(
    /// An activated, bidirectional connection to a peer.
    ///
    /// Only obtainable from [`Inactive::activate`] — see that type for why.
    /// Sending and cancelling are thread-safe, so they take `&self`.
    #[doc(alias = "XPC_TYPE_CONNECTION")]
    pub Connection, connection
);

/// A connection that has been created but not yet activated.
///
/// XPC aborts the process on two things a plain `arc::R<Connection>` cannot
/// prevent:
///
/// - *"Release of last reference on a suspended connection."* —
///   `xpc_connection_create` hands back a **suspended** connection, and
///   dropping it in that state is fatal. Cancelling does not help.
/// - *"Activation of a connection without an event handler."*
///
/// So the inactive state gets its own type: [`Self::activate`] takes the
/// handler and is the only way to reach a [`Connection`], and `Drop` winds a
/// connection you abandoned down through a legal path instead of aborting.
pub struct Inactive(Option<arc::R<Connection>>);

impl Inactive {
    /// A peer connection to an XPC service. `None` makes an anonymous
    /// listener, whose endpoint you can hand to another process.
    #[doc(alias = "xpc_connection_create")]
    #[inline]
    pub fn with_name(name: Option<&CStr>, target_q: Option<&dispatch::Queue>) -> Self {
        let name = name.map_or(std::ptr::null(), CStr::as_ptr);
        Self(Some(unsafe { xpc_connection_create(name, target_q) }))
    }

    #[doc(alias = "xpc_connection_create_mach_service")]
    #[inline]
    pub fn with_mach_service(
        name: &CStr,
        target_q: Option<&dispatch::Queue>,
        flags: MachServiceFlags,
    ) -> Self {
        Self(Some(unsafe {
            xpc_connection_create_mach_service(name.as_ptr(), target_q, flags)
        }))
    }

    #[doc(alias = "xpc_connection_create_from_endpoint")]
    #[inline]
    pub fn with_endpoint(endpoint: &xpc::Endpoint) -> Self {
        Self(Some(unsafe {
            xpc_connection_create_from_endpoint(endpoint)
        }))
    }

    #[inline]
    fn con_mut(&mut self) -> &mut Connection {
        // SAFETY: only `activate` and `drop` take the connection out.
        unsafe { self.0.as_mut().unwrap_unchecked() }
    }

    #[doc(alias = "xpc_connection_set_target_queue")]
    #[inline]
    pub fn set_target_queue(&mut self, queue: Option<&dispatch::Queue>) {
        unsafe { xpc_connection_set_target_queue(self.con_mut(), queue) }
    }

    /// Rejects peers that do not satisfy `req`. `false` if the requirement
    /// could not be compiled.
    #[doc(alias = "xpc_connection_set_peer_code_signing_requirement")]
    #[inline]
    pub fn set_peer_code_signing_req(&mut self, req: &CStr) -> bool {
        unsafe {
            xpc_connection_set_peer_code_signing_requirement(self.con_mut(), req.as_ptr()) == 0
        }
    }

    /// Installs the event handler and activates.
    #[doc(alias = "xpc_connection_activate")]
    #[inline]
    pub fn activate_block(mut self, handler: &mut Handler) -> arc::R<Connection> {
        // SAFETY: `self` is dropped without ever seeing `None`.
        let mut con = unsafe { self.0.take().unwrap_unchecked() };
        unsafe {
            xpc_connection_set_event_handler(&mut con, handler);
            xpc_connection_activate(&mut con);
        }
        con
    }

    /// Installs `handler` and activates.
    ///
    /// The handler runs on the target queue and is called for every incoming
    /// message and for the error that ends the connection —
    /// [`xpc::Event`](xpc::Event) sorts the two apart.
    #[inline]
    pub fn activate(
        self,
        handler: impl FnMut(&xpc::Object) + 'static + Sync,
    ) -> arc::R<Connection> {
        let mut handler = Handler::new1(handler);
        self.activate_block(&mut handler)
    }
}

impl Drop for Inactive {
    fn drop(&mut self) {
        let Some(mut con) = self.0.take() else { return };
        // Releasing a suspended connection is fatal, and so is activating one
        // with no event handler. Walk it through both, then cancel.
        let mut noop = Handler::new1(|_: &xpc::Object| {});
        unsafe {
            xpc_connection_set_event_handler(&mut con, &mut noop);
            xpc_connection_activate(&mut con);
            xpc_connection_cancel(&con);
        }
    }
}

impl Connection {
    /// Create, install `handler`, activate.
    #[inline]
    pub fn with_name(
        name: Option<&CStr>,
        target_q: Option<&dispatch::Queue>,
        handler: impl FnMut(&xpc::Object) + 'static + Sync,
    ) -> arc::R<Self> {
        Inactive::with_name(name, target_q).activate(handler)
    }

    /// Create, install `handler`, activate.
    #[inline]
    pub fn with_mach_service(
        name: &CStr,
        target_q: Option<&dispatch::Queue>,
        flags: MachServiceFlags,
        handler: impl FnMut(&xpc::Object) + 'static + Sync,
    ) -> arc::R<Self> {
        Inactive::with_mach_service(name, target_q, flags).activate(handler)
    }

    /// Fire and forget. Delivery is not acknowledged; failure surfaces on the
    /// event handler as [`xpc::Error::connection_invalid`].
    #[doc(alias = "xpc_connection_send_message")]
    #[inline]
    pub fn send_msg(&self, msg: &xpc::Dictionary) {
        unsafe { xpc_connection_send_message(self, msg) }
    }

    #[doc(alias = "xpc_connection_send_message_with_reply")]
    #[inline]
    pub fn send_msg_with_reply_block(
        &self,
        msg: &xpc::Dictionary,
        reply_q: Option<&dispatch::Queue>,
        handler: &mut ReplyHandler,
    ) {
        unsafe { xpc_connection_send_message_with_reply(self, msg, reply_q, handler) }
    }

    #[inline]
    pub fn send_msg_with_reply(
        &self,
        msg: &xpc::Dictionary,
        reply_q: Option<&dispatch::Queue>,
        handler: impl FnMut(&xpc::Object) + 'static,
    ) {
        let mut handler = ReplyHandler::new1(handler);
        self.send_msg_with_reply_block(msg, reply_q, &mut handler);
    }

    /// Blocks the calling thread. Never call this from a queue that the
    /// connection itself targets.
    #[doc(alias = "xpc_connection_send_message_with_reply_sync")]
    #[inline]
    pub fn send_msg_with_reply_sync(&self, msg: &xpc::Dictionary) -> arc::R<xpc::Object> {
        unsafe { xpc_connection_send_message_with_reply_sync(self, msg) }
    }

    /// Idempotent. The event handler gets a final
    /// [`xpc::Error::connection_invalid`].
    #[doc(alias = "xpc_connection_cancel")]
    #[inline]
    pub fn cancel(&self) {
        unsafe { xpc_connection_cancel(self) }
    }

    #[doc(alias = "xpc_connection_get_name")]
    #[inline]
    pub fn name(&self) -> Option<&CStr> {
        let ptr = unsafe { xpc_connection_get_name(self) };
        (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) })
    }

    #[doc(alias = "xpc_connection_get_pid")]
    #[inline]
    pub fn pid(&self) -> sys::Pid {
        unsafe { xpc_connection_get_pid(self) }
    }

    #[doc(alias = "xpc_connection_get_euid")]
    #[inline]
    pub fn euid(&self) -> sys::Uid {
        unsafe { xpc_connection_get_euid(self) }
    }

    #[doc(alias = "xpc_connection_get_egid")]
    #[inline]
    pub fn egid(&self) -> sys::Gid {
        unsafe { xpc_connection_get_egid(self) }
    }

    /// Audit session id.
    #[doc(alias = "xpc_connection_get_asid")]
    #[inline]
    pub fn asid(&self) -> sys::Pid {
        unsafe { xpc_connection_get_asid(self) }
    }
}

#[link(name = "System", kind = "dylib")]
unsafe extern "C-unwind" {
    fn xpc_connection_create(
        name: *const c_char,
        target_q: Option<&dispatch::Queue>,
    ) -> arc::R<Connection>;

    fn xpc_connection_create_mach_service(
        name: *const c_char,
        target_q: Option<&dispatch::Queue>,
        flags: MachServiceFlags,
    ) -> arc::R<Connection>;

    fn xpc_connection_create_from_endpoint(endpoint: &xpc::Endpoint) -> arc::R<Connection>;

    fn xpc_connection_set_event_handler(con: &mut Connection, handler: &mut Handler);
    fn xpc_connection_set_target_queue(con: &mut Connection, queue: Option<&dispatch::Queue>);
    fn xpc_connection_set_peer_code_signing_requirement(
        con: &mut Connection,
        req: *const c_char,
    ) -> i32;

    fn xpc_connection_activate(con: &mut Connection);
    fn xpc_connection_cancel(con: &Connection);

    fn xpc_connection_send_message(con: &Connection, msg: &xpc::Dictionary);
    fn xpc_connection_send_message_with_reply(
        con: &Connection,
        msg: &xpc::Dictionary,
        reply_q: Option<&dispatch::Queue>,
        handler: &mut ReplyHandler,
    );
    fn xpc_connection_send_message_with_reply_sync(
        con: &Connection,
        msg: &xpc::Dictionary,
    ) -> arc::R<xpc::Object>;

    fn xpc_connection_get_name(con: &Connection) -> *const c_char;
    fn xpc_connection_get_pid(con: &Connection) -> sys::Pid;
    fn xpc_connection_get_euid(con: &Connection) -> sys::Uid;
    fn xpc_connection_get_egid(con: &Connection) -> sys::Gid;
    fn xpc_connection_get_asid(con: &Connection) -> sys::Pid;
}

/// The `XPC_ERROR_*` constants.
///
/// They are process-lifetime singletons, so an event handler identifies them
/// by pointer, exactly as C does.
impl xpc::Error {
    /// The peer closed or crashed. The connection stays usable: messages sent
    /// after this are delivered once the peer comes back.
    #[doc(alias = "XPC_ERROR_CONNECTION_INTERRUPTED")]
    #[inline]
    pub fn connection_interrupted() -> &'static Self {
        unsafe { &_xpc_error_connection_interrupted }
    }

    /// The connection is dead for good — the name went away, or it was
    /// cancelled. No further messages will be delivered.
    #[doc(alias = "XPC_ERROR_CONNECTION_INVALID")]
    #[inline]
    pub fn connection_invalid() -> &'static Self {
        unsafe { &_xpc_error_connection_invalid }
    }

    /// The service is being shut down.
    #[doc(alias = "XPC_ERROR_TERMINATION_IMMINENT")]
    #[inline]
    pub fn termination_imminent() -> &'static Self {
        unsafe { &_xpc_error_termination_imminent }
    }

    /// The peer failed the code-signing requirement set on the connection.
    #[doc(alias = "XPC_ERROR_PEER_CODE_SIGNING_REQUIREMENT")]
    #[inline]
    pub fn peer_code_signing_requirement() -> &'static Self {
        unsafe { &_xpc_error_peer_code_signing_requirement }
    }

    /// A human-readable description, e.g. `"Connection invalid"`.
    #[doc(alias = "XPC_ERROR_KEY_DESCRIPTION")]
    #[inline]
    pub fn desc(&self) -> Option<&CStr> {
        let ptr = unsafe { xpc_dictionary_get_string(self, _xpc_error_key_description) };
        (!ptr.is_null()).then(|| unsafe { CStr::from_ptr(ptr) })
    }
}

impl std::fmt::Display for xpc::Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.desc() {
            Some(desc) => write!(f, "{}", desc.to_string_lossy()),
            None => write!(f, "xpc error"),
        }
    }
}

/// What a connection's event handler receives.
#[derive(Debug)]
pub enum Event<'a> {
    Msg(&'a xpc::Dictionary),
    Err(&'a xpc::Error),
    /// A type the binding does not model — a peer connection arriving on a
    /// listener, say.
    Other(&'a xpc::Object),
}

impl<'a> From<&'a xpc::Object> for Event<'a> {
    #[inline]
    fn from(obj: &'a xpc::Object) -> Self {
        if let Some(err) = obj.try_cast::<xpc::Error>() {
            Self::Err(err)
        } else if let Some(msg) = obj.try_cast::<xpc::Dictionary>() {
            Self::Msg(msg)
        } else {
            Self::Other(obj)
        }
    }
}

#[link(name = "System", kind = "dylib")]
unsafe extern "C-unwind" {
    static _xpc_error_connection_interrupted: xpc::Error;
    static _xpc_error_connection_invalid: xpc::Error;
    static _xpc_error_termination_imminent: xpc::Error;
    static _xpc_error_peer_code_signing_requirement: xpc::Error;

    /// Unlike the objects above, this symbol really is a pointer variable.
    static _xpc_error_key_description: *const c_char;

    fn xpc_dictionary_get_string(xdict: &xpc::Error, key: *const c_char) -> *const c_char;
}
