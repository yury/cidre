use crate::{arc, define_obj_type, mtl, objc};

define_obj_type!(ParallelRenderCmdEncoder(mtl::RenderCmdEncoder));

impl ParallelRenderCmdEncoder {
    #[objc::msg_send(renderCommandEncoder)]
    pub fn render_cmd_encoder_ar(&self) -> Option<arc::Rar<Self>>;

    #[objc::rar_retain]
    pub fn render_cmd_encoder(&self) -> Option<arc::R<Self>>;

    /// If the the store action for a given color attachment was set to mtl::StoreAction::Unknown
    /// when the render command encoder was created,
    /// set_color_store_action_at must be used to finalize the store action before endEncoding
    /// is called.
    #[objc::msg_send(setColorStoreAction:atIndex:)]
    pub fn set_color_store_action_at(&mut self, val: mtl::StoreAction, index: usize);

    /// If the the store action for the depth attachment was set to mtl::StoreAction::Unknown
    /// when the render command encoder was created,
    /// set_depth_store_action must be used to finalize the store action before end_encoding
    /// is called.
    #[objc::msg_send(setDepthStoreAction:)]
    pub fn set_depth_store_action(&mut self, val: mtl::StoreAction);

    /// If the the store action for the stencil attachment was set to mtl::StoreAction::Unknown
    /// when the render command encoder was created,
    /// set_stencil_store_action must be used to finalize the store action before end_encoding
    /// is called.
    #[objc::msg_send(setStencilStoreAction:)]
    pub fn set_stencil_store_action(&mut self, val: mtl::StoreAction);

    /// If the the store action for a given color attachment was set to mtl::StoreAction::Unknown
    /// when the render command encoder was created,
    /// set_color_store_action_options may be used to finalize the store action options before
    /// end_encoding is called.
    ///
    /// # Arguments
    /// * `store_action_options` - The desired store action options for the given color attachment.
    /// * `color_attachment_index` - The index of the color attachment
    #[objc::msg_send(setColorStoreActionOptions:atIndex:)]
    pub fn set_color_store_action_options(&mut self, val: mtl::StoreActionOpts, index: usize);

    /// If the the store action for the depth attachment was set to mtl::StoreAction::Unknown
    /// when the render command encoder was created,
    /// set_depth_store_action_options may be used to finalize the store action options before
    /// end_encoding is called.
    #[objc::msg_send(setDepthStoreActionOptions:)]
    pub fn set_depth_store_action_options(&mut self, val: mtl::StoreActionOpts);

    /// If the the store action for the stencil attachment was set to mtl::StoreAction::Unknown
    /// when the render command encoder was created,
    /// set_stencil_store_action_options may be used to finalize the store action options before
    /// end_encoding is called.
    #[objc::msg_send(setStencilStoreActionOptions:)]
    pub fn set_stencil_store_action_options(&mut self, val: mtl::StoreActionOpts);
}
