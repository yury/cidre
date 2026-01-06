use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_content_context")]
    #[doc(alias = "nw_content_context_t")]
    pub ContentCtx(ns::Id)
);

impl ContentCtx {
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
unsafe extern "C-unwind" {
    static _nw_content_context_default_message: &'static ContentCtx;
    static _nw_content_context_final_send: &'static ContentCtx;
    static _nw_content_context_default_stream: &'static ContentCtx;
}
