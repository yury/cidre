use crate::{arc, at::audio, av, blocks, define_cls, define_obj_type, ns, objc, os};

pub type AudioUnitStatus = os::Status;

define_obj_type!(
    #[doc(alias = "AUAudioUnit")]
    pub AudioUnit(ns::Id)
);

impl arc::A<AudioUnit> {
    #[objc::msg_send(initWithComponentDescription:options:error:)]
    pub unsafe fn init_with_comp_desc_opts_err<'ear>(
        self,
        desc: audio::ComponentDesc,
        opts: audio::ComponentInstantiationOpts,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<AudioUnit>>;

    #[objc::msg_send(initWithComponentDescription:error:)]
    pub unsafe fn init_with_comp_desc_err<'ear>(
        self,
        desc: audio::ComponentDesc,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<AudioUnit>>;
}

impl AudioUnit {
    define_cls!(AU_AUDIO_UNIT);

    pub fn with_comp_desc_opts<'ear>(
        self,
        desc: audio::ComponentDesc,
        opts: audio::ComponentInstantiationOpts,
    ) -> ns::Result<'ear, arc::R<AudioUnit>> {
        ns::if_none(|err| unsafe { Self::alloc().init_with_comp_desc_opts_err(desc, opts, err) })
    }

    pub fn with_comp_desc<'ear>(
        self,
        desc: audio::ComponentDesc,
    ) -> ns::Result<'ear, arc::R<AudioUnit>> {
        ns::if_none(|err| unsafe { Self::alloc().init_with_comp_desc_err(desc, err) })
    }

    #[objc::msg_send(instantiateWithComponentDescription:options:completionHandler:)]
    pub fn instantiate_with_comp_desc_ch_block(
        desc: audio::ComponentDesc,
        opts: audio::ComponentInstantiationOpts,
        block: &mut blocks::ResultCh<Self>,
    );

    pub fn instantiate_with_comp_desc_ch(
        desc: audio::ComponentDesc,
        opts: audio::ComponentInstantiationOpts,
        handler: impl FnMut(Option<&Self>, Option<&ns::Error>) + 'static + std::marker::Sync,
    ) {
        let mut block = blocks::ResultCh::new2(handler);
        Self::instantiate_with_comp_desc_ch_block(desc, opts, &mut block);
    }

    #[cfg(feature = "async")]
    pub async fn instantiate_with_comp_desc(
        desc: audio::ComponentDesc,
        opts: audio::ComponentInstantiationOpts,
    ) -> Result<arc::R<Self>, arc::R<ns::Error>> {
        let (future, mut block) = blocks::result();
        Self::instantiate_with_comp_desc_ch_block(desc, opts, &mut block);
        future.await
    }

    /// The audio::ComponentDesc with which the audio unit was created.
    #[objc::msg_send(componentDescription)]
    pub fn component_desc(&self) -> audio::ComponentDesc;

    #[objc::msg_send(component)]
    pub fn component(&self) -> &audio::Component;

    #[objc::msg_send(component)]
    pub fn component_mut(&self) -> &mut audio::Component;

    #[objc::msg_send(componentName)]
    pub fn component_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(audioUnitName)]
    pub fn audio_unit_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(manufacturerName)]
    pub fn manufacturer_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(audioUnitShortName)]
    pub fn audio_unit_short_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(componentVersion)]
    pub fn component_version(&self) -> u32;

    #[objc::msg_send(allocateRenderResourcesAndReturnError:)]
    pub unsafe fn allocate_render_resources_err<'ear>(&mut self, err: *mut Option<&'ear ns::Error>);

    pub fn allocate_render_resources<'ear>(&mut self) -> ns::Result<'ear> {
        ns::if_err(|err| unsafe { self.allocate_render_resources_err(err) })
    }

    #[objc::msg_send(deallocateRenderResources)]
    pub fn deallocate_render_resources(&mut self);

    #[objc::msg_send(renderResourcesAllocated)]
    pub fn render_resources_allocated(&self) -> bool;

    #[objc::msg_send(reset)]
    pub fn reset(&mut self);

    #[objc::msg_send(inputBusses)]
    pub fn input_busses(&self) -> arc::R<AudioUnitBusArray>;

    #[objc::msg_send(outputBusses)]
    pub fn output_busses(&self) -> arc::R<AudioUnitBusArray>;

    //...

    #[objc::msg_send(fullState)]
    pub fn full_state(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;
}

define_obj_type!(
    #[doc(alias = "AUAudioUnitBusArray")]
    pub AudioUnitBusArray(ns::Id)
);

impl ns::FastEnum<AudioUnitBus> for AudioUnitBusArray {}

impl AudioUnitBusArray {
    #[objc::msg_send(lount)]
    pub fn count(&self) -> usize;

    #[inline]
    pub fn len(&self) -> usize {
        self.count()
    }

    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub unsafe fn get_throws(&self, index: usize) -> arc::R<AudioUnitBus>;

    pub fn get<'ear>(&self, index: usize) -> ns::ExResult<'ear, arc::R<AudioUnitBus>> {
        ns::try_catch(|| unsafe { self.get_throws(index) })
    }

    #[objc::msg_send(isCountChangeable)]
    pub fn is_count_changeable(&self) -> bool;

    #[objc::msg_send(setBusCount:error:)]
    pub unsafe fn set_bus_count_err<'ear>(&mut self, val: usize, err: *mut Option<&'ear ns::Error>);

    pub fn set_bus_count<'ear>(&mut self, val: usize) -> ns::Result<'ear> {
        ns::if_err(|err| unsafe { self.set_bus_count_err(val, err) })
    }

    #[objc::msg_send(ownerAudioUnit)]
    pub fn owner_audio_unit(&self) -> arc::R<AudioUnit>;

    #[objc::msg_send(busType)]
    pub fn bus_type(&self) -> AudioUnitBusType;
}

define_obj_type!(
    #[doc(alias = "AUAudioUnitBus")]
    pub AudioUnitBus(ns::Id)
);

impl AudioUnitBus {
    /// The audio format and channel layout of audio being transferred on the bus.
    ///
    /// Bridged to the v2 property kAudioUnitProperty_StreamFormat.
    #[objc::msg_send(format)]
    pub fn format(&self) -> arc::R<av::AudioFormat>;

    #[objc::msg_send(setFormat:error:)]
    pub unsafe fn set_format_err<'ear>(
        &mut self,
        val: &av::AudioFormat,
        err: *mut Option<&'ear ns::Error>,
    );

    /// Sets the bus's audio format.
    ///
    /// Audio units can generally be expected to support AVAudioFormat's standard format
    /// (deinterleaved 32-bit float), at any sample rate. Channel counts can be more complex;
    /// see AUAudioUnit.channelCapabilities.
    pub fn set_format<'ear>(&mut self, val: &av::AudioFormat) -> ns::Result<'ear> {
        ns::if_err(|err| unsafe { self.set_format_err(val, err) })
    }

    #[objc::msg_send(shouldAllocateBuffer)]
    pub fn should_allocate_buffer(&self) -> bool;

    /// Controls the audio unit's allocation strategy for a bus.
    ///
    /// Hosts can set this flag to communicate whether an audio unit should allocate its own buffer.
    /// By default this flag is set to true.
    ///
    /// On the output side, shouldAllocateBuffer=false means the AU can assume that it will be
    /// called with non-null output buffers. If shouldAllocateBuffer=true (the default), the AU must
    /// be prepared to be called with null pointers and replace them with pointers to its internally
    /// allocated buffer.
    ///
    /// On the input side, shouldAllocateBuffer=false means the AU can pull for input using a buffer
    /// list with null buffer pointers, and assume that the pull input block will provide pointers.
    /// If shouldAllocateBuffer=true (the default), the AU must pull with non-null pointers while
    /// still being prepared for the source to replace them with pointers of its own.
    ///
    /// Bridged to the v2 property kAudioUnitProperty_ShouldAllocateBuffer.
    #[objc::msg_send(setShouldAllocateBuffer:)]
    pub fn set_should_allocate_buffer(&mut self, val: bool);

    /// Whether the bus is active.
    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    /// Hosts must enable input busses before using them. The reason for this is to allow a unit
    /// such as a mixer to be prepared to render a large number of inputs, but avoid the work
    /// of preparing to pull inputs which are not in use.
    ///
    /// Bridged to the v2 properties kAudioUnitProperty_MakeConnection and
    /// kAudioUnitProperty_SetRenderCallback.
    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    /// A name for the bus. Can be set by host.
    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<arc::R<ns::String>>;

    /// The index of this bus in the containing array.
    #[objc::msg_send(index)]
    pub fn index(&self) -> usize;

    #[objc::msg_send(busType)]
    pub fn bus_type(&self) -> AudioUnitBusType;

    #[objc::msg_send(ownerAudioUnit)]
    pub fn owner_audio_unit(&self) -> arc::R<AudioUnit>;

    #[objc::msg_send(supportedChannelLayoutTags)]
    pub fn supported_channel_layout_tags(&self) -> Option<arc::R<ns::Array<ns::Number>>>;

    /// Information about latency in the audio unit's processing context.
    ///
    /// Bridged to the v2 property kAudioUnitProperty_PresentationLatency.
    #[objc::msg_send(contextPresentationLatency)]
    pub fn context_presentation_latency(&self) -> ns::TimeInterval;

    #[objc::msg_send(setContextPresentationLatency:)]
    pub fn set_context_presentation_latency(&mut self, val: ns::TimeInterval);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum AudioUnitBusType {
    Input = 1,
    Ouptut = 2,
}

#[link(name = "at", kind = "static")]
unsafe extern "C" {
    static AU_AUDIO_UNIT: &'static objc::Class<AudioUnit>;
}
