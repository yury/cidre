use crate::vimage;

impl vimage::Buf {
    #[doc(alias = "vImageConvert_PlanarFtoPlanar16F")]
    pub fn to_f16_from_f32(&mut self, src: &Self, flags: vimage::Flags) -> vimage::Result {
        unsafe { vImageConvert_PlanarFtoPlanar16F(src, self, flags) }.result()
    }

    #[doc(alias = "vImageConvert_Planar16FtoPlanarF")]
    pub fn to_f32_from_f16(&mut self, src: &Self, flags: vimage::Flags) -> vimage::Result {
        unsafe { vImageConvert_Planar16FtoPlanarF(src, self, flags) }.result()
    }

    #[doc(alias = "vImageConvert_Planar8toPlanar16F")]
    pub fn to_f16_from_i8(&mut self, src: &Self, flags: vimage::Flags) -> vimage::Result {
        unsafe { vImageConvert_Planar8toPlanar16F(src, self, flags) }.result()
    }

    #[doc(alias = "vImageConvert_Planar16FtoPlanar8")]
    pub fn to_i8_from_f16(&mut self, src: &Self, flags: vimage::Flags) -> vimage::Result {
        unsafe { vImageConvert_Planar16FtoPlanar8(src, self, flags) }.result()
    }

    #[doc(alias = "vImageConvert_PlanarFtoPlanar16F")]
    pub fn inplace_to_f16_from_f32(&mut self, flags: vimage::Flags) -> vimage::Result {
        unsafe {
            let ptr = self as *const Self;
            self.to_f16_from_f32(&*ptr, flags)
        }
    }

    #[doc(alias = "vImageConvert_Planar16FtoPlanar8")]
    pub fn inplace_to_i8_from_f16(&mut self, flags: vimage::Flags) -> vimage::Result {
        unsafe {
            let ptr = self as *const Self;
            self.to_i8_from_f16(&*ptr, flags)
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

    fn vImageConvert_Planar8toPlanar16F(
        src: *const vimage::Buf,
        dst: *mut vimage::Buf,
        flags: vimage::Flags,
    ) -> vimage::Status;

    fn vImageConvert_Planar16FtoPlanar8(
        src: *const vimage::Buf,
        dst: *mut vimage::Buf,
        flags: vimage::Flags,
    ) -> vimage::Status;
}
