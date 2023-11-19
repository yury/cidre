use crate::{arc, av, define_obj_type, ns, objc, os};

use super::{mixer_node::MixerNode, ConnectionPoint, Format, InputNode, Node, NodeBus, OutputNode};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ManualRenderingError(pub os::Status);

impl ManualRenderingError {
    /// The operation cannot be performed because the engine is either not in manual
    /// rendering mode or the right variant of it.
    pub const INVALID_MODE: Self = Self(os::Status(-80800));

    /// The operation cannot be performed because the engine is initialized (i.e. not stopped).
    pub const INITIALIZED: Self = Self(os::Status(-80801));

    /// The operation cannot be performed because the engine is not running (i.e. not started).
    pub const NOT_RUNNING: Self = Self(os::Status(-80801));
}

impl PartialEq<os::Status> for ManualRenderingError {
    fn eq(&self, other: &os::Status) -> bool {
        self.0 == *other
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(isize)]
pub enum ManualRenderingStatus {
    /// An error occurred when rendering and no data was returned. See the returned error code
    /// for the description of the error.
    Error = -1,
    /// All of the requested data was returned successfully.
    Success = 0,
    /// Applicable only to the input node, when it provides input data for rendering
    /// (see `AVAudioInputNode(setManualRenderingInputPCMFormat:inputBlock:)`).
    /// Indicates that not enough input data was returned by the input node to satisfy the
    /// render request at the current time. The output buffer may contain data rendered by other
    /// active sources in the engine's processing graph.
    InsufficientDataFromInputNode = 1,
    /// The operation could not be performed now, but the client could retry later if needed.
    /// This is usually to guard a realtime render operation (e.g. rendering through
    /// `manualRenderingBlock`) when a reconfiguration of the engine's internal state
    /// is in progress.
    CannotDoInCurrentContext = 2,
}

/// By default, the engine is connected to an audio device and automatically renders in realtime.
/// It can also be configured to operate in manual rendering mode, i.e. not connected to an
/// audio device and rendering in response to requests from the client.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(isize)]
pub enum ManualRenderingMode {
    /// The engine operates in an offline mode without any realtime constraints.
    Offline = 0,
    /// The engine operates under realtime constraints, i.e. it will not make any blocking call
    /// (e.g. calling libdispatch, blocking on a mutex, allocating memory etc.) while rendering.
    /// Note that only the block based render mechanism can be used in this mode
    /// (see `AVAudioEngine(manualRenderingBlock)`.
    Realtime = 1,
}

define_obj_type!(pub Engine(ns::Id), AV_AUDIO_ENGINE);

/// An AVAudioEngine contains a group of connected AVAudioNodes ("nodes"), each of which performs
/// an audio signal generation, processing, or input/output task.
///
/// Nodes are created separately and attached to the engine.
///
/// The engine supports dynamic connection, disconnection and removal of nodes while running,
/// with only minor limitations:
/// - all dynamic reconnections must occur upstream of a mixer
/// - while removals of effects will normally result in the automatic connection of the adjacent
///   nodes, removal of a node which has differing input vs. output channel counts, or which
///   is a mixer, is likely to result in a broken graph.
///
/// By default, the engine is connected to an audio device and automatically renders in realtime.
/// It can also be configured to operate in manual rendering mode, i.e. not connected to an
/// audio device and rendering in response to requests from the client, normally at or
/// faster than realtime rate.
impl Engine {
    #[objc::msg_send(attachNode:)]
    pub fn attach_node(&mut self, node: &Node);

    #[objc::msg_send(detachNode:)]
    pub fn detach_node(&mut self, node: &Node);

    #[objc::msg_send(connect:to:fromBus:toBus:format:)]
    pub fn connect_node_to_node_bus_to_bus(
        &mut self,
        node_from: &Node,
        node_to: &Node,
        from_bus: NodeBus,
        to_bus: NodeBus,
        format: Option<&Format>,
    );

    #[objc::msg_send(connect:to:format:)]
    pub fn connect_node_to_node(
        &mut self,
        node_from: &Node,
        node_to: &Node,
        format: Option<&Format>,
    );

    #[objc::msg_send(connect:toConnectionPoints:fromBus:format:)]
    pub fn connect_node_to_connection_points_from_bus(
        &mut self,
        node: &Node,
        connection_pods: &ns::Array<ConnectionPoint>,
        from_bus: NodeBus,
        format: Option<&Format>,
    );

    #[objc::msg_send(disconnectNodeInput:bus:)]
    pub fn disconnect_node_input_bus(&mut self, node: &Node, bus: NodeBus);

    #[objc::msg_send(disconnectNodeInput:)]
    pub fn disconnect_node_input(&mut self, node: &Node);

    #[objc::msg_send(disconnectNodeOutput:bus:)]
    pub fn disconnect_node_output_bus(&mut self, node: &Node, bus: NodeBus);

    #[objc::msg_send(disconnectNodeOutput:)]
    pub fn disconnect_node_output(&mut self, node: &Node);

    #[objc::msg_send(prepare)]
    pub fn prepare(&mut self);

    #[objc::msg_send(startAndReturnError:)]
    pub fn start_and_return_err<'ar>(&self, error: &mut Option<&'ar ns::Error>) -> bool;

    #[inline]
    pub fn start<'ar>(&mut self) -> Result<(), &'ar ns::Error> {
        unsafe {
            let mut error = None;
            if self.start_and_return_err(&mut error) {
                Ok(())
            } else {
                Err(error.unwrap_unchecked())
            }
        }
    }

    /// ```
    /// use cidre::av;
    ///
    /// let engine = av::audio::Engine::new();
    /// let input_node = engine.input_node();
    /// let en = input_node.engine().expect("engine");
    /// ```
    #[objc::msg_send(inputNode)]
    pub fn input_node(&self) -> &InputNode;

    #[objc::msg_send(inputNode)]
    pub fn input_node_mut(&mut self) -> &mut InputNode;

    /// ```
    /// use cidre::av;
    ///
    /// let engine = av::audio::Engine::new();
    /// let output_node = engine.output_node();
    ///
    /// ```
    #[objc::msg_send(outputNode)]
    pub fn output_node(&self) -> &OutputNode;

    #[objc::msg_send(outputNode)]
    pub fn output_node_mut(&mut self) -> &mut OutputNode;

    #[objc::msg_send(mainMixerNode)]
    pub fn main_mixer_node(&self) -> &MixerNode;

    #[objc::msg_send(mainMixerNode)]
    pub fn main_mixer_node_mut(&mut self) -> &mut MixerNode;

    #[objc::msg_send(reset)]
    pub fn reset(&mut self);

    #[objc::msg_send(pause)]
    pub fn pause(&mut self);

    #[objc::msg_send(stop)]
    pub fn stop(&mut self);

    #[objc::msg_send(isRunning)]
    pub fn is_running(&self) -> bool;

    /// Set the engine to operate in a manual rendering mode with the specified render format and
    /// maximum frame count.
    ///
    /// Use this method to configure the engine to render in response to requests from the client.
    /// The engine must be in a stopped state before calling this method.
    /// The render format must be a PCM format and match the format of the buffer to which
    /// the engine is asked to render (see `renderOffline:toBuffer:error:`).
    ///
    /// It is advised to enable manual rendering mode soon after the engine is created, and
    /// before accessing any of mainMixerNode, inputNode or outputNode of the engine.
    /// Otherwise, accessing or interacting with the engine before enabling manual rendering
    /// mode could have the unintended side-effect of configuring the hardware for device-rendering
    /// mode.
    ///
    /// The input data in manual rendering mode can be supplied through the source nodes, e.g.
    /// `av::audio::PlayerNode`, `av::audio::InputNode` etc.
    ///
    /// When switching to manual rendering mode, the engine:
    /// 1. Switches the input and output nodes to manual rendering mode. Their input and output
    ///     formats may change.
    /// 2. Removes any taps previously installed on the input and output nodes.
    /// 3. Maintains all the engine connections as is.
    #[objc::msg_send(enableManualRenderingMode:format:maximumFrameCount:error:)]
    pub fn enable_manual_rendering_mode_err<'ar>(
        &mut self,
        mode: ManualRenderingMode,
        format: &av::AudioFormat,
        max_frame_count: av::AudioFrameCount,
        error: &mut Option<&'ar ns::Error>,
    );

    #[inline]
    pub fn enable_manual_rendering_mode<'ar>(
        &mut self,
        mode: ManualRenderingMode,
        format: &av::AudioFormat,
        max_frame_count: av::AudioFrameCount,
    ) -> Result<(), &'ar ns::Error> {
        let mut error = None;
        self.enable_manual_rendering_mode_err(mode, format, max_frame_count, &mut error);
        if let Some(err) = error {
            return Err(err);
        }
        Ok(())
    }

    #[objc::msg_send(disableManualRenderingMode)]
    pub fn disable_manual_rendering_mode(&mut self);

    #[objc::msg_send(renderOffline:toBuffer:error:)]
    pub fn render_offline_err<'ar>(
        &mut self,
        number_of_frames: av::AudioFrameCount,
        to_buf: &mut av::audio::PcmBuf,
        error: *mut Option<&'ar ns::Error>,
    ) -> av::audio::EngineManualRenderingStatus;

    #[inline]
    pub fn render_offline<'ar>(
        &mut self,
        number_of_frames: av::AudioFrameCount,
        to_buf: &mut av::audio::PcmBuf,
    ) -> Result<av::audio::EngineManualRenderingStatus, &'ar ns::Error> {
        let mut error = None;
        let status = self.render_offline_err(number_of_frames, to_buf, &mut error);
        if status == ManualRenderingStatus::Error && error.is_some() {
            return Err(unsafe { error.unwrap_unchecked() });
        }
        Ok(status)
    }

    #[objc::msg_send(isInManualRenderingMode)]
    pub fn is_in_manual_rendering_mode(&self) -> bool;

    #[objc::msg_send(manualRenderingMode)]
    pub fn manual_rendering_mode(&self) -> ManualRenderingMode;

    /// The render format of the engine in manual rendering mode.
    ///
    /// Querying this property when the engine is not in manual rendering mode will return an
    /// invalid format, with zero sample rate and channel count.
    #[objc::msg_send(manualRenderingFormat)]
    pub fn manual_rendering_format(&self) -> &av::audio::Format;

    /// The maximum number of PCM sample frames the engine can produce in any single render call in
    /// the manual rendering mode.
    ///
    /// Querying this property when the engine is not in manual rendering mode will return zero.
    #[objc::msg_send(manualRenderingMaximumFrameCount)]
    pub fn manual_rendering_max_frame_count(&self) -> av::audio::FrameCount;

    /// Indicates where the engine is on its render timeline in manual rendering mode.
    ///
    /// The timeline in manual rendering mode starts at a sample time of zero, and is in terms
    /// of the render format's sample rate. Resetting the engine (see `reset`) will reset the
    /// timeline back to zero.
    #[objc::msg_send(manualRenderingSampleTime)]
    pub fn manual_rendering_sample_time(&self) -> av::audio::FramePosition;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_ENGINE: &'static objc::Class<Engine>;
}

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        let engine = av::audio::Engine::new();
        assert!(!engine.is_running());
        assert!(!engine.is_in_manual_rendering_mode());
        assert_eq!(engine.manual_rendering_format().channel_count(), 0);
        let _output_node = engine.output_node();
        let input_node = engine.input_node();
        let _en = input_node.engine().expect("engine");
    }

    #[test]
    fn manual_redering_modes() {
        let format =
            av::audio::Format::standard_with_sample_rate_and_channels(44_100.0, 2).unwrap();
        let mut engine = av::audio::Engine::new();
        engine
            .enable_manual_rendering_mode(
                av::audio::engine::ManualRenderingMode::Offline,
                &format,
                1024,
            )
            .expect("Failed to enter manual rendering mode");
        assert!(engine.is_in_manual_rendering_mode());

        assert_eq!(
            engine.manual_rendering_format().channel_count(),
            format.channel_count()
        );

        let mut pcm_buf = av::audio::PcmBuf::with_format_and_frame_capacity(&format, 1024).unwrap();

        pcm_buf.set_frame_length(1024);

        engine.start().expect("Failed to start engine");
        assert!(engine.is_running());

        // this is segfault with swift too https://gist.github.com/yury/2a74943c65b69a3a593a2c096ac36b54
        // we didn't connect any inputs :)
        // let status = engine.render_offline_err(1024, &mut pcm_buf, std::ptr::null_mut());

        engine.stop();

        let player_node = av::AudioPlayerNode::new();
        engine.attach_node(&player_node);
        let mixer_node = engine.main_mixer_node().retained();
        engine.connect_node_to_node(&player_node, &mixer_node, None);
        engine.start().expect("Failed to start engine");
        assert_eq!(engine.manual_rendering_sample_time(), 0);
        player_node.play();

        let status = engine.render_offline(1024, &mut pcm_buf).unwrap();

        assert_eq!(status, av::audio::EngineManualRenderingStatus::Success);
        assert_eq!(pcm_buf.frame_length(), 1024);
        assert_eq!(engine.manual_rendering_sample_time(), 1024);
    }
}
