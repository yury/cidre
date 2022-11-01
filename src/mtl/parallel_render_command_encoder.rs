use crate::{define_obj_type, ns};

use super::{CommandEncoder, RenderCommandEncoder};

define_obj_type!(ParallelRenderCommandEncoder(CommandEncoder));

impl ParallelRenderCommandEncoder {
    #[inline]
    pub fn render_command_encoder<'ar>(&self) -> Option<&'ar RenderCommandEncoder> {
        unsafe { rsel_renderCommandEncoder(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_renderCommandEncoder<'ar>(id: &ns::Id) -> Option<&'ar RenderCommandEncoder>;
}
