use std::alloc::Layout;

use crate::{cg, vimage};

#[derive(Debug)]
pub struct AllocatedBuf(vimage::Buf, std::alloc::Layout);

impl AllocatedBuf {
    pub fn with_size(w: usize, h: usize, pixel_bits: u32) -> vimage::Result<AllocatedBuf> {
        let mut buf = vimage::Buf::new();
        let align = buf.alignment(w, h, pixel_bits);
        if align <= 0 {
            panic!("failed to get buf aligment");
        }

        let size = buf.h * buf.row_bytes;

        let layout = Layout::from_size_align(size, align as usize)
            .expect("Failed to get layout from size and align");

        unsafe {
            vImageBuffer_Init(&mut buf, h, w, pixel_bits, Default::default()).result()?;
        }
        Ok(Self(buf, layout))
    }
}

impl Drop for AllocatedBuf {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.0.data as *mut u8, self.1) };
    }
}

impl std::ops::Deref for AllocatedBuf {
    type Target = vimage::Buf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for AllocatedBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl vimage::Buf {
    #[inline]
    pub fn size(&self) -> cg::Size {
        unsafe { vImageBuffer_GetSize(self) }
    }

    pub fn new() -> Self {
        Self {
            data: std::ptr::null_mut(),
            h: 0,
            w: 0,
            row_bytes: 0,
        }
    }

    pub fn alignment(&mut self, w: usize, h: usize, pixel_bits: u32) -> isize {
        unsafe { vImageBuffer_Init(self, h, w, pixel_bits, vimage::Flags::NO_ALLOCATE).0 }
    }
}

impl Default for vimage::Buf {
    fn default() -> Self {
        Self::new()
    }
}

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    fn vImageBuffer_GetSize(buf: *const vimage::Buf) -> cg::Size;
    fn vImageBuffer_Init(
        buf: *mut vimage::Buf,
        height: vimage::PixelCount,
        width: vimage::PixelCount,
        pixel_bits: u32,
        flags: vimage::Flags,
    ) -> vimage::Status;
}

#[cfg(test)]
mod tests {

    use crate::{cg, vimage};

    #[test]
    fn basics() {
        let mut b = vimage::Buf::new();
        let alignment = b.alignment(10, 1, 8);
        println!("align {alignment:?} {b:?}");
        let mut buf = vimage::AllocatedBuf::with_size(1, 1, 32).unwrap();
        let size = buf.size();
        assert_eq!(
            size,
            cg::Size {
                width: 1f64,
                height: 1f64
            }
        );

        println!("buf {:?}", buf);
        buf.inplace_to_f16_from_f32().unwrap();
        // println!("{size:?}");
        // let size = buf.size();
        // assert_eq!(
        //     size,
        //     cg::Size {
        //         width: 1024f64,
        //         height: 1f64
        //     }
        // );
    }
}
