use crate::{arc, define_xpc_type, xpc};

define_xpc_type!(
    /// A connection in serialized form.
    ///
    /// Inert — unlike a connection it carries no runtime state, so it is safe
    /// to put in a message. The recipient turns it into as many connections as
    /// it likes with [`xpc::connection::Inactive::with_endpoint`].
    #[doc(alias = "XPC_TYPE_ENDPOINT")]
    pub Endpoint, endpoint
);

impl Endpoint {
    /// `connection` must be a listener connection.
    #[doc(alias = "xpc_endpoint_create")]
    #[inline]
    pub fn with_connection(connection: &xpc::Connection) -> arc::R<Self> {
        unsafe { xpc_endpoint_create(connection) }
    }
}

#[link(name = "System", kind = "dylib")]
unsafe extern "C-unwind" {
    fn xpc_endpoint_create(connection: &xpc::Connection) -> arc::R<Endpoint>;
}
