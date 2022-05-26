use crate::{define_obj_type, av::audio};

// AVAudioIONode
define_obj_type!(IONode(audio::Node));

// AVAudioInputNode
define_obj_type!(InputNode(IONode));

// AVAudioOutputNode
define_obj_type!(OutputNode(IONode));