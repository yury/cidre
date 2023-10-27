use crate::{av::audio, define_obj_type, ns, objc};

// AVAudioIONode
define_obj_type!(IoNode(audio::Node));
impl IoNode {
    /// The presentation or hardware latency, applicable when the engine is rendering to/from an
    /// audio device.
    ///
    /// This corresponds to kAudioDevicePropertyLatency and kAudioStreamPropertyLatency.
    #[objc::msg_send(presentationLatency)]
    pub fn presentation_latency(&self) -> ns::TimeInterval;

    /// Indicates whether voice processing is enabled.
    #[objc::msg_send(isVoiceProcessingEnabled)]
    pub fn is_voice_processing_enabled(&self) -> bool;

    #[objc::msg_send(setVoiceProcessingEnabled:error:)]
    pub fn set_voice_processing_enabled_err(
        &mut self,
        value: bool,
        error: *mut Option<&ns::Error>,
    ) -> bool;

    pub fn set_voice_processing_enabled<'ar>(&mut self, value: bool) -> Result<(), &'ar ns::Error> {
        let mut error = None;
        if self.set_voice_processing_enabled_err(value, &mut error) {
            Ok(())
        } else {
            Err(unsafe { error.unwrap_unchecked() })
        }
    }
}

// AVAudioInputNode
define_obj_type!(InputNode(IoNode));

impl InputNode {
    /// Bypass all processing done by the voice processing unit.
    ///
    /// Querying this property when voice processing is disabled will return false.
    #[objc::msg_send(isVoiceProcessingBypassed)]
    pub fn is_voice_processing_bypassed(&self) -> bool;

    #[objc::msg_send(setVoiceProcessingBypassed:)]
    pub fn set_voice_processing_bypassed(&mut self, value: bool);

    #[objc::msg_send(isVoiceProcessingAGCEnabled)]
    pub fn is_voice_processing_agc_enabled(&self) -> bool;

    #[objc::msg_send(setVoiceProcessingAGCEnabled:)]
    pub fn set_voice_processing_agc_enabled(&mut self, value: bool);

    #[objc::msg_send(isVoiceProcessingInputMuted)]
    pub fn is_voice_processing_input_muted(&self) -> bool;

    #[objc::msg_send(setVoiceProcessingInputMuted:)]
    pub fn set_voice_porcessing_input_muted(&mut self, value: bool);
}

// AVAudioOutputNode
define_obj_type!(OutputNode(IoNode));
