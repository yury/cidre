use crate::cf::runtime::Autoreleased;
use crate::objc::Sel;
use crate::{define_mtl, define_obj_type};

use crate::ns::Id;

use super::CommandBuffer;

define_obj_type!(CommandQueue(Id));

impl CommandQueue {
    define_mtl!(device, label, set_label);

    #[inline]
    pub fn command_buffer<'pool>(&self) -> Option<Autoreleased<'pool, CommandBuffer>> {
        unsafe { self.rsel(sel_commandBuffer) }
    }
}

extern "C" {
    static sel_commandBuffer: &'static Sel;
}
