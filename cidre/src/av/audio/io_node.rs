use crate::{av::audio, define_obj_type};

// AVAudioIONode
define_obj_type!(IONode(audio::Node));

// AVAudioInputNode
define_obj_type!(InputNode(IONode));

// AVAudioOutputNode
define_obj_type!(OutputNode(IONode));
