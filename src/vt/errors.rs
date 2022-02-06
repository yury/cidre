#[repr(transparent)]
pub struct DecodeFrameFlags(pub u32);

impl DecodeFrameFlags {
    pub const ENABLE_ASYNCHRONOUS_DECOMPRESSION: Self = Self(1 << 0);
    pub const DO_NOT_OUTPUT_FRAME: Self = Self(1 << 1);
    pub const _1X_REAL_TIME_PLAYBACK: Self = Self(1 << 2);
    pub const ENABLE_TEMPORAL_PROCESSING: Self = Self(1 << 3);
}

#[repr(u32)]
pub enum DecodeInfoFlags {
    Asynchronous = 1u32 << 0,
    FrameDropped = 1u32 << 1,
    ImageBufferModifiable = 1u32 << 2,
}

#[repr(u32)]
pub enum EncodeInfoFlags {
    Asynchronous = 1u32 << 0,
    FrameDropped = 1u32 << 1,
}
