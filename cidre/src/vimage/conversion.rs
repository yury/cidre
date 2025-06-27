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

    #[cfg(feature = "half")]
    #[inline]
    pub fn slice_f16_to_f32(src: &[half::f16], dst: &mut [f32]) -> vimage::Result {
        debug_assert_eq!(src.len(), dst.len());
        let src = vimage::Buf {
            data: src.as_ptr() as _,
            h: 1,
            w: src.len(),
            row_bytes: 2 * src.len(),
        };
        let mut dst = vimage::Buf {
            data: dst.as_mut_ptr() as _,
            h: 1,
            w: dst.len(),
            row_bytes: 4 * dst.len(),
        };
        dst.to_f32_from_f16(&src, vimage::Flags::DO_NOT_TILE)
    }

    #[cfg(feature = "half")]
    #[inline]
    pub fn slice_f32_to_f16(src: &[f32], dst: &mut [half::f16]) -> vimage::Result {
        debug_assert_eq!(src.len(), dst.len());
        let src = vimage::Buf {
            data: src.as_ptr() as _,
            h: 1,
            w: src.len(),
            row_bytes: 4 * src.len(),
        };
        let mut dst = vimage::Buf {
            data: dst.as_mut_ptr() as _,
            h: 1,
            w: dst.len(),
            row_bytes: 2 * dst.len(),
        };
        dst.to_f16_from_f32(&src, vimage::Flags::DO_NOT_TILE)
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

#[cfg(test)]
mod tests {
    use crate::vimage;

    #[cfg(feature = "half")]
    #[test]
    fn slice() {
        let src = [half::f16::from_f32(0.5); 10];
        let mut dst = [0.0; 10];
        vimage::Buf::slice_f16_to_f32(&src, &mut dst).unwrap();

        assert_eq!(dst, [0.5; 10]);

        let src = [0.5; 10];
        let mut dst = [half::f16::ONE; 10];
        vimage::Buf::slice_f32_to_f16(&src, &mut dst).unwrap();

        assert_eq!(dst, [half::f16::from_f32(0.5); 10]);
    }
}
