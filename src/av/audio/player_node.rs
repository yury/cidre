use crate::{av::audio, define_obj_type};

/// Options controlling buffer scheduling.
#[repr(transparent)]
pub struct BufferOptions(pub usize);

impl BufferOptions {
    /// The buffer loops indefinitely.  
    pub const LOOPS: Self = Self(1usize << 0);
    /// The buffer interrupts any buffer already playing.
    pub const INTERRUPTS: Self = Self(1usize << 1);
    /// he buffer interrupts any buffer already playing, at its loop point.
    pub const INTERRUPTS_AT_LOOP: Self = Self(1usize << 2);
}

#[repr(isize)]
pub enum CompletionCallbackType {
    DataConsumed = 0,
    DataRendered = 1,
    DataPlayedBack = 2,
}

define_obj_type!(PlayerNode(audio::Node));
