use crate::{define_obj_type, mtl, mtl4, objc};

define_obj_type!(
    #[doc(alias = "MTL4FXSpatialScaler")]
    pub SpatialScaler(mtl::fx::SpatialScalerBase)
);

impl SpatialScaler {
    #[objc::msg_send(encodeToCommandBuffer:)]
    #[objc::available(macos = 26.0, ios = 26.0)]
    pub fn encode_to_cmd_buf(&self, command_buffer: &mtl4::CmdBuf);
}
