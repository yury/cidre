use crate::{define_obj_type, objc::Id};

use super::{CommandEncoder, RenderCommandEncoder};

define_obj_type!(ParallelRenderCommandEncoder(CommandEncoder));

impl ParallelRenderCommandEncoder {

  #[inline]
  pub fn render_command_encoder<'autoreleased>(&self) -> Option<&'autoreleased RenderCommandEncoder> {
    unsafe {
      rsel_renderCommandEncoder(self)
    }
  }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
  fn rsel_renderCommandEncoder<'autoreleased>(id: &Id) -> Option<&'autoreleased RenderCommandEncoder>;
}