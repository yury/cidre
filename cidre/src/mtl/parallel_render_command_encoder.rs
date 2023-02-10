use crate::{arc, define_obj_type, objc};

use super::RenderCmdEncoder;

define_obj_type!(ParallelRenderCmdEncoder(RenderCmdEncoder));

impl ParallelRenderCmdEncoder {
    #[objc::msg_send(renderCommandEncoder)]
    pub fn render_cmd_encoder_ar(&self) -> Option<&'ar Self>;

    #[objc::rar_retain]
    pub fn render_cmd_encoder(&self) -> Option<arc::R<Self>>;
}
