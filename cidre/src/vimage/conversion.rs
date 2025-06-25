use crate::vimage;

impl vimage::Buf {
    pub fn to_f16_from_f32(&mut self, src: &Self, flags: vimage::Flags) -> vimage::Result {
        unsafe { vImageConvert_PlanarFtoPlanar16F(src, self, flags) }.result()
    }

    pub fn to_32_from_f16(&mut self, src: &Self, flags: vimage::Flags) -> vimage::Result {
        unsafe { vImageConvert_Planar16FtoPlanarF(src, self, flags) }.result()
    }

    pub fn inplace_to_f16_from_f32(&mut self) -> vimage::Result {
        unsafe {
            let ptr = self as *const Self;
            self.to_f16_from_f32(&*ptr, Default::default())
        }
    }
}

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    fn vImageConvert_PlanarFtoPlanar16F(
        src: *const vimage::Buf,
        dst: *mut vimage::Buf,
        flags: vimage::Flags,
    ) -> vimage::Status;
    fn vImageConvert_Planar16FtoPlanarF(
        src: *const vimage::Buf,
        dst: *mut vimage::Buf,
        flags: vimage::Flags,
    ) -> vimage::Status;
}
