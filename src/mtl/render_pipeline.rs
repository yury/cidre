use crate::{define_obj_type, objc::Id};

use super::PixelFormat;

#[repr(usize)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 2,
    OneMinusSourceColor = 3,
    SourceAlpha = 4,
    OneMinusSourceAlpha = 5,
    DestinationColor = 6,
    OneMinusDestinationColor = 7,
    DestinationAlpha = 8,
    OneMinusDestinationAlpha = 9,
    SourceAlphaSaturated = 10,
    BlendColor = 11,
    OneMinusBlendColor = 12,
    BlendAlpha = 13,
    OneMinusBlendAlpha = 14,
    Source1Color = 15,
    OneMinusSource1Color = 16,
    Source1Alpha = 17,
    OneMinusSource1Alpha = 18,
}

#[repr(usize)]
pub enum BlendOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

#[repr(usize)]
pub enum ColorWriteMask {
    None = 0,
    Red = 0x1 << 3,
    Green = 0x1 << 2,
    Blue = 0x1 << 1,
    Alpha = 0x1 << 0,
    All = 0xf,
}

#[repr(usize)]
pub enum PrimitiveTopologyClass {
    Unspecified = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
}

#[repr(usize)]
pub enum TessellationPartitionMode {
    MTLTessellationPartitionModePow2 = 0,
    MTLTessellationPartitionModeInteger = 1,
    MTLTessellationPartitionModeFractionalOdd = 2,
    MTLTessellationPartitionModeFractionalEven = 3,
}

#[repr(usize)]
pub enum TessellationFactorStepFunction {
  Constant = 0,
  PerPatch = 1,
  PerInstance = 2,
  PerPatchAndPerInstance = 3,
}

#[repr(usize)]
pub enum TessellationFactorFormat {
  MTLTessellationFactorFormatHalf = 0,
}


#[repr(usize)]
pub enum TessellationControlPointIndexType {
  None = 0,
  UInt16 = 1,
  UInt32 = 2,
}

define_obj_type!(RenderPipelineColorAttachmentDescriptor(Id));

impl RenderPipelineColorAttachmentDescriptor {

    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { rsel_pixelFormat(self) }
    }

    #[inline]
    pub fn set_pixel_format(&mut self, value: PixelFormat) {
        unsafe { wsel_setPixelFormat(self, value) }
    }

    pub fn blening_enabled(&self) -> bool {
      unsafe { rsel_isBlendingEnabled(self) }
    }

    pub fn set_blening_enabled(&mut self, value: bool) {
      unsafe {
        wsel_setBlendingEnabled(self, value)
      }
    }

    //rwsel(, id, isBlendingEnabled, setBlendingEnabled, BOOL)


}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_pixelFormat(id: &Id) -> PixelFormat;
    fn wsel_setPixelFormat(id: &mut Id, value: PixelFormat);

    fn rsel_isBlendingEnabled(id: &Id) -> bool;
    fn wsel_setBlendingEnabled(id: &mut Id, val: bool);
}