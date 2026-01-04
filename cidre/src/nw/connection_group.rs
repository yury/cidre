use crate::{arc, define_obj_type, dispatch, ns, nw};

define_obj_type!(
    #[doc(alias = "nw_connection_group")]
    #[doc(alias = "nw_connection_group_t")]
    pub ConnectionGroup(ns::Id)
);

#[doc(alias = "nw_connection_group_state_changed_handler_t")]
#[cfg(feature = "blocks")]
pub type ConnectionGroupStateChangeHandler =
    crate::blocks::SyncBlock<fn(state: State, error: Option<&nw::Error>)>;

#[doc(alias = "nw_connection_group_receive_handler_t")]
#[cfg(feature = "blocks")]
pub type ConnectionGroupRecvHandler = crate::blocks::SyncBlock<
    fn(content: Option<&dispatch::Data>, content_ctx: &nw::ContentCtx, is_complete: bool),
>;

#[doc(alias = "nw_connection_group_state_t")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum State {
    /// The state of the connection group is not valid.
    /// This state will never be delivered in the connection group's state update
    /// handler and can be treated as an unexpected value.
    #[doc(alias = "nw_connection_group_state_invalid")]
    Invalid = 0,

    /// The connection group is waiting for a usable network
    /// before being able to receive and process incoming messages.
    #[doc(alias = "nw_connection_group_state_waiting")]
    Waiting = 1,

    /// The connection group is ready and able to receive and
    /// process incoming messages.
    #[doc(alias = "nw_connection_group_state_ready")]
    Ready = 2,

    /// The connection group has irrecoverably failed.
    /// You should cancel the connection group and create a new connection group object
    /// if you wish to continue processing incoming messages.
    #[doc(alias = "nw_connection_group_state_failed")]
    Failed = 3,

    /// The connection group has been cancelled by
    /// the user. You should create a new connection group object if you wish to continue
    /// processing incoming messages.
    #[doc(alias = "nw_connection_group_state_cancelled")]
    Cancelled = 4,
}

impl ConnectionGroup {
    #[doc(alias = "nw_connection_group_create")]
    #[inline]
    pub fn with_desc(group_desc: &nw::GroupDesc, params: &nw::Params) -> Option<arc::R<Self>> {
        unsafe { nw_connection_group_create(group_desc, params) }
    }

    #[doc(alias = "nw_connection_group_copy_descriptor")]
    #[inline]
    pub fn desc(&self) -> arc::R<nw::GroupDesc> {
        unsafe { nw_connection_group_copy_descriptor(self) }
    }

    #[doc(alias = "nw_connection_group_copy_parameters")]
    #[inline]
    pub fn params(&self) -> arc::R<nw::Params> {
        unsafe { nw_connection_group_copy_parameters(self) }
    }

    #[doc(alias = "nw_connection_group_set_queue")]
    #[inline]
    pub fn set_queue(&mut self, queue: &dispatch::Queue) {
        unsafe {
            nw_connection_group_set_queue(self, queue);
        }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_connection_group_set_state_changed_handler")]
    pub fn set_state_change_handler_block(
        &mut self,
        block: &mut ConnectionGroupStateChangeHandler,
    ) {
        unsafe {
            nw_connection_group_set_state_changed_handler(self, block);
        }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_connection_group_set_state_changed_handler")]
    pub fn set_state_change_handler(
        &mut self,
        handler: impl FnMut(/* state: */ State, /* err: */ Option<&nw::Error>) + Send + Sync + 'static,
    ) {
        let mut block = ConnectionGroupStateChangeHandler::new2(handler);
        self.set_state_change_handler_block(&mut block);
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_connection_group_set_receive_handler")]
    pub fn set_recv_handler_block(&mut self, block: &mut ConnectionGroupRecvHandler) {
        unsafe {
            nw_connection_group_set_receive_handler(self, block);
        }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_connection_group_set_receive_handler")]
    pub fn set_recv_handler(
        &mut self,
        handler: impl FnMut(
            /*content: */ Option<&dispatch::Data>,
            /*content_ctx: */ &nw::ContentCtx,
            /* is_complete: */ bool,
        ) + Send
        + Sync
        + 'static,
    ) {
        let mut block = ConnectionGroupRecvHandler::new3(handler);
        self.set_recv_handler_block(&mut block);
    }

    #[doc(alias = "nw_connection_group_start")]
    #[inline]
    pub fn start(&mut self) {
        unsafe {
            nw_connection_group_start(self);
        }
    }

    #[doc(alias = "nw_connection_group_cancel")]
    #[inline]
    pub fn cancel(&mut self) {
        unsafe {
            nw_connection_group_cancel(self);
        }
    }

    #[doc(alias = "nw_connection_group_copy_remote_endpoint_for_message")]
    #[inline]
    pub fn remote_endpoint_for_msg(
        &self,
        content_ctx: &nw::ContentCtx,
    ) -> Option<arc::R<nw::Endpoint>> {
        unsafe { nw_connection_group_copy_remote_endpoint_for_message(self, content_ctx) }
    }

    #[doc(alias = "nw_connection_group_copy_local_endpoint_for_message")]
    #[inline]
    pub fn local_endpoint_for_msg(
        &self,
        content_ctx: &nw::ContentCtx,
    ) -> Option<arc::R<nw::Endpoint>> {
        unsafe { nw_connection_group_copy_local_endpoint_for_message(self, content_ctx) }
    }

    #[doc(alias = "nw_connection_group_copy_path_for_message")]
    #[inline]
    pub fn path_for_msg(&self, content_ctx: &nw::ContentCtx) -> Option<arc::R<nw::Path>> {
        unsafe { nw_connection_group_copy_path_for_message(self, content_ctx) }
    }

    #[doc(alias = "nw_connection_group_copy_protocol_metadata_for_message")]
    #[inline]
    pub fn metadata_for_msg(
        &self,
        content_ctx: &nw::ContentCtx,
        definition: &nw::ProtocolDefinition,
    ) -> Option<arc::R<nw::ProtocolMetadata>> {
        unsafe {
            nw_connection_group_copy_protocol_metadata_for_message(self, content_ctx, definition)
        }
    }

    #[doc(alias = "nw_connection_group_extract_connection_for_message")]
    #[inline]
    pub fn connection_for_msg(
        &self,
        content_ctx: &nw::ContentCtx,
    ) -> Option<arc::R<nw::Connection>> {
        unsafe { nw_connection_group_extract_connection_for_message(self, content_ctx) }
    }

    #[doc(alias = "nw_connection_group_reply")]
    #[inline]
    pub fn reply(
        &self,
        inbound_msg: &nw::ContentCtx,
        outbound_msg: &nw::ContentCtx,
        content: Option<dispatch::Data>,
    ) {
        unsafe {
            nw_connection_group_reply(self, inbound_msg, outbound_msg, content);
        }
    }

    #[doc(alias = "nw_connection_group_extract_connection")]
    #[inline]
    pub fn extract_connection(
        &mut self,
        endpoint: Option<&nw::Endpoint>,
        protocol_opts: Option<&nw::ProtocolOpts>,
    ) -> Option<arc::R<nw::Connection>> {
        unsafe { nw_connection_group_extract_connection(self, endpoint, protocol_opts) }
    }

    #[doc(alias = "nw_connection_group_reinsert_extracted_connection")]
    #[inline]
    pub fn reinsert_extracted_connection(&mut self, connection: &nw::Connection) -> bool {
        unsafe { nw_connection_group_reinsert_extracted_connection(self, connection) }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_connection_group_send_message")]
    #[inline]
    pub fn send_msg_block(
        &self,
        content: Option<&dispatch::Data>,
        endpoint: Option<&nw::Endpoint>,
        context: &nw::ContentCtx,
        completion: &mut ConnectionGroupSendCompletion,
    ) {
        unsafe {
            nw_connection_group_send_message(self, content, endpoint, context, completion);
        }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "nw_connection_group_set_new_connection_handler")]
    #[inline]
    pub fn set_new_connection_handler_block(
        &mut self,
        handler: Option<&mut ConnectionGroupNewConnectionHandler>,
    ) {
        unsafe {
            nw_connection_group_set_new_connection_handler(self, handler);
        }
    }

    #[doc(alias = "nw_connection_group_copy_protocol_metadata")]
    #[inline]
    pub fn protocol_metadata(
        &self,
        definition: &nw::ProtocolDefinition,
    ) -> Option<arc::R<nw::ProtocolMetadata>> {
        unsafe { nw_connection_group_copy_protocol_metadata(self, definition) }
    }
}

#[cfg(feature = "blocks")]
#[doc(alias = "nw_connection_group_send_completion_t")]
pub type ConnectionGroupSendCompletion = crate::blocks::SyncBlock<fn(error: Option<&nw::Error>)>;

#[cfg(feature = "blocks")]
#[doc(alias = "nw_connection_group_new_connection_handler_t")]
pub type ConnectionGroupNewConnectionHandler =
    crate::blocks::SyncBlock<fn(connection: &mut nw::Connection)>;

#[link(name = "Network", kind = "framework")]
unsafe extern "C" {
    fn nw_connection_group_create(
        group_desc: &nw::GroupDesc,
        params: &nw::Params,
    ) -> Option<arc::R<ConnectionGroup>>;
    fn nw_connection_group_copy_descriptor(group: &nw::ConnectionGroup) -> arc::R<nw::GroupDesc>;
    fn nw_connection_group_copy_parameters(group: &nw::ConnectionGroup) -> arc::R<nw::Params>;
    fn nw_connection_group_set_queue(group: &mut nw::ConnectionGroup, queue: &dispatch::Queue);

    #[cfg(feature = "blocks")]
    fn nw_connection_group_set_state_changed_handler(
        group: &mut nw::ConnectionGroup,
        handler: &mut ConnectionGroupStateChangeHandler,
    );
    #[cfg(feature = "blocks")]
    fn nw_connection_group_set_receive_handler(
        group: &mut nw::ConnectionGroup,
        handler: &mut ConnectionGroupRecvHandler,
    );
    fn nw_connection_group_start(group: &mut nw::ConnectionGroup);
    fn nw_connection_group_cancel(group: &mut nw::ConnectionGroup);
    fn nw_connection_group_copy_remote_endpoint_for_message(
        group: &nw::ConnectionGroup,
        content_ctx: &nw::ContentCtx,
    ) -> Option<arc::R<nw::Endpoint>>;
    fn nw_connection_group_copy_local_endpoint_for_message(
        group: &nw::ConnectionGroup,
        content_ctx: &nw::ContentCtx,
    ) -> Option<arc::R<nw::Endpoint>>;
    fn nw_connection_group_copy_path_for_message(
        group: &nw::ConnectionGroup,
        content_ctx: &nw::ContentCtx,
    ) -> Option<arc::R<nw::Path>>;
    fn nw_connection_group_copy_protocol_metadata_for_message(
        group: &nw::ConnectionGroup,
        content_ctx: &nw::ContentCtx,
        definition: &nw::ProtocolDefinition,
    ) -> Option<arc::R<nw::ProtocolMetadata>>;
    fn nw_connection_group_extract_connection_for_message(
        group: &nw::ConnectionGroup,
        content_ctx: &nw::ContentCtx,
    ) -> Option<arc::R<nw::Connection>>;
    fn nw_connection_group_reply(
        group: &nw::ConnectionGroup,
        inbound_msg: &nw::ContentCtx,
        outbound_msg: &nw::ContentCtx,
        content: Option<dispatch::Data>,
    );
    fn nw_connection_group_extract_connection(
        group: &mut nw::ConnectionGroup,
        endpoint: Option<&nw::Endpoint>,
        protocol_opts: Option<&nw::ProtocolOpts>,
    ) -> Option<arc::R<nw::Connection>>;
    fn nw_connection_group_reinsert_extracted_connection(
        group: &mut nw::ConnectionGroup,
        connection: &nw::Connection,
    ) -> bool;
    #[cfg(feature = "blocks")]
    fn nw_connection_group_send_message(
        group: &nw::ConnectionGroup,
        content: Option<&dispatch::Data>,
        endpoint: Option<&nw::Endpoint>,
        context: &nw::ContentCtx,
        completion: &mut ConnectionGroupSendCompletion,
    );
    #[cfg(feature = "blocks")]
    fn nw_connection_group_set_new_connection_handler(
        group: &mut nw::ConnectionGroup,
        handler: Option<&mut ConnectionGroupNewConnectionHandler>,
    );
    fn nw_connection_group_copy_protocol_metadata(
        group: &nw::ConnectionGroup,
        definition: &nw::ProtocolDefinition,
    ) -> Option<arc::R<nw::ProtocolMetadata>>;
}
