use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_content_context")]
    #[doc(alias = "nw_content_context_t")]
    pub ContentCtx(ns::Id)
);

impl ContentCtx {
    /// A send callback override that causes the write call to
    /// be treated as idempotent. Idempotent content is allowed to be sent
    /// before the connection is ready, and may be replayed across parallel connection
    /// attempts. This content can be sent as part of fast-open protocols, which allows
    /// the data to be sent out sooner than if it were required to wait for
    /// connection establishment.
    ///
    /// This override intentionally disallows the client from receiving callbacks
    /// for the write calls, since the content may be sent multiple times internally.
    /// For any large content, or content that need to be sensitive to sending backpressure,
    /// an explicit callback should be used.
    #[doc(alias = "NW_CONNECTION_SEND_IDEMPOTENT_CONTENT")]
    #[inline]
    pub fn send_idempotent_content() -> &'static Self {
        unsafe { _nw_connection_send_idempotent_content }
    }

    #[doc(alias = "NW_CONNECTION_DEFAULT_MESSAGE_CONTEXT")]
    #[inline]
    pub fn default_msg() -> &'static Self {
        unsafe { _nw_content_context_default_message }
    }

    #[doc(alias = "NW_CONNECTION_FINAL_MESSAGE_CONTEXT")]
    #[inline]
    pub fn final_msg_send() -> &'static Self {
        unsafe { _nw_content_context_final_send }
    }

    #[doc(alias = "NW_CONNECTION_DEFAULT_STREAM_CONTEXT")]
    #[inline]
    pub fn default_stream() -> &'static Self {
        unsafe { _nw_content_context_default_stream }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C-unwind" {
    static _nw_connection_send_idempotent_content: &'static ContentCtx;
    static _nw_content_context_default_message: &'static ContentCtx;
    static _nw_content_context_final_send: &'static ContentCtx;
    static _nw_content_context_default_stream: &'static ContentCtx;
}
