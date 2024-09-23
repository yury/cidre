use crate::{
    at::{
        au,
        au::{Element, ParamId, PropId, Scope, Unit, UnitRef},
        audio,
        audio::component::{InitializedState, State, UninitializedState},
    },
    os,
};

#[repr(transparent)]
pub struct MultiChannelMixer<S>(UnitRef<S>)
where
    S: State<Unit>;

impl<S> MultiChannelMixer<S>
where
    S: State<Unit>,
{
    #[inline]
    pub fn unit(&self) -> &Unit {
        self.0.unit()
    }

    #[inline]
    pub fn unit_mut(&mut self) -> &mut Unit {
        self.0.unit_mut()
    }

    pub fn is_enabled(&self, scope: Scope, bus: u32) -> os::Result<bool> {
        let res = self
            .unit()
            .param(ParamId::MULTI_CHANNEL_MIXER_ENABLE, scope, Element(bus))?;
        Ok(res != 0.0)
    }

    #[inline]
    pub fn set_enabled(&mut self, scope: Scope, bus: u32, val: bool) -> os::Result {
        self.unit_mut().set_param(
            ParamId::MULTI_CHANNEL_MIXER_ENABLE,
            scope,
            Element(bus),
            val as u32 as f32,
            0,
        )
    }

    #[inline]
    pub fn set_output_enabled(&mut self, val: bool) -> os::Result {
        self.set_enabled(Scope::OUTPUT, 0, val)
    }

    #[inline]
    pub fn set_input_enabled(&mut self, bus: u32, val: bool) -> os::Result {
        self.set_enabled(Scope::INPUT, bus, val)
    }

    #[inline]
    pub fn is_output_enabled(&self) -> os::Result<bool> {
        self.is_enabled(Scope::OUTPUT, 0)
    }

    #[inline]
    pub fn is_input_enabled(&self, bus: u32) -> os::Result<bool> {
        self.is_enabled(Scope::INPUT, bus)
    }

    #[inline]
    pub fn volume(&self, scope: Scope, bus: u32) -> os::Result<f32> {
        self.unit()
            .param(ParamId::MULTI_CHANNEL_MIXER_VOLUME, scope, Element(bus))
    }

    #[inline]
    pub fn set_volume_at(
        &mut self,
        scope: Scope,
        bus: u32,
        val: f32,
        frames_offset: u32,
    ) -> os::Result {
        self.unit_mut().set_param(
            ParamId::MULTI_CHANNEL_MIXER_VOLUME,
            scope,
            Element(bus),
            val,
            frames_offset,
        )
    }

    #[inline]
    pub fn set_output_volume_at(&mut self, val: f32, frame_offset: u32) -> os::Result {
        self.set_volume_at(Scope::OUTPUT, 0, val, frame_offset)
    }

    #[inline]
    pub fn set_output_volume(&mut self, val: f32) -> os::Result {
        self.set_output_volume_at(val, 0)
    }

    #[inline]
    pub fn output_volume(&self) -> os::Result<f32> {
        self.volume(Scope::OUTPUT, 0)
    }

    #[inline]
    pub fn set_input_volume_at(&mut self, bus: u32, val: f32, frame_offset: u32) -> os::Result {
        self.set_volume_at(Scope::INPUT, bus, val, frame_offset)
    }

    #[inline]
    pub fn set_input_volume(&mut self, bus: u32, val: f32) -> os::Result {
        self.set_input_volume_at(bus, val, 0)
    }

    /// By default it is 0.0, so you should set it to 1.0 before use.
    /// Otherwise it seems like not working
    #[inline]
    pub fn input_volume(&self, bus: u32) -> os::Result<f32> {
        self.volume(Scope::INPUT, bus)
    }

    #[inline]
    pub fn is_metering_enabled(&self, scope: Scope, bus: u32) -> os::Result<bool> {
        let res: u32 = self
            .unit()
            .prop(PropId::METERING_MODE, scope, Element(bus))?;
        Ok(res != 0)
    }

    #[inline]
    pub fn set_metering_enabled(&mut self, scope: Scope, bus: u32, val: bool) -> os::Result {
        let val = val as u32;
        self.unit_mut()
            .set_prop(PropId::METERING_MODE, scope, Element(bus), &val)
    }

    #[inline]
    pub fn set_output_metering_enabled(&mut self, val: bool) -> os::Result {
        self.set_metering_enabled(Scope::OUTPUT, 0, val)
    }

    #[inline]
    pub fn set_input_metering_enabled(&mut self, bus: u32, val: bool) -> os::Result {
        self.set_metering_enabled(Scope::INPUT, bus, val)
    }

    #[inline]
    pub fn is_output_metering_enabled(&self) -> os::Result<bool> {
        self.is_metering_enabled(Scope::OUTPUT, 0)
    }

    #[inline]
    pub fn is_input_metering_enabled(&self, bus: u32) -> os::Result<bool> {
        self.is_metering_enabled(Scope::INPUT, bus)
    }

    /// level in dB
    #[inline]
    pub fn pre_avg_power(&self, scope: Scope, bus: u32, channel: u8) -> os::Result<f32> {
        let param_id = ParamId(ParamId::MULTI_CHANNEL_MIXER_PRE_AVERAGE_POWER.0 + channel as u32);
        self.unit().param(param_id, scope, Element(bus))
    }

    /// level in dB
    #[inline]
    pub fn pre_input_avg_power(&self, bus: u32, channel: u8) -> os::Result<f32> {
        self.pre_avg_power(Scope::INPUT, bus, channel)
    }

    /// level in dB
    #[inline]
    pub fn pre_output_avg_power(&self, channel: u8) -> os::Result<f32> {
        self.pre_avg_power(Scope::OUTPUT, 0, channel)
    }

    /// level in dB
    #[inline]
    pub fn pre_peak_hold_level(&self, scope: Scope, bus: u32, channel: u8) -> os::Result<f32> {
        let param_id = ParamId(ParamId::MULTI_CHANNEL_MIXER_PRE_PEAK_HOLD_LEVEL.0 + channel as u32);
        self.unit().param(param_id, scope, Element(bus))
    }

    /// level in dB
    #[inline]
    pub fn pre_input_peak_hold_level(&self, bus: u32, channel: u8) -> os::Result<f32> {
        self.pre_peak_hold_level(Scope::INPUT, bus, channel)
    }

    /// level in dB
    #[inline]
    pub fn pre_output_peak_hold_level(&self, channel: u8) -> os::Result<f32> {
        self.pre_peak_hold_level(Scope::OUTPUT, 0, channel)
    }

    /// level in dB
    #[inline]
    pub fn post_avg_power(&self, scope: Scope, bus: u32, channel: u8) -> os::Result<f32> {
        let param_id = ParamId(ParamId::MULTI_CHANNEL_MIXER_POST_AVERAGE_POWER.0 + channel as u32);
        self.unit().param(param_id, scope, Element(bus))
    }

    /// level in dB
    #[inline]
    pub fn post_input_avg_power(&self, bus: u32, channel: u8) -> os::Result<f32> {
        self.post_avg_power(Scope::INPUT, bus, channel)
    }

    /// level in dB
    #[inline]
    pub fn post_output_avg_power(&self, channel: u8) -> os::Result<f32> {
        self.post_avg_power(Scope::OUTPUT, 0, channel)
    }

    /// level in dB
    #[inline]
    pub fn post_peak_hold_level(&self, scope: Scope, bus: u32, channel: u8) -> os::Result<f32> {
        let param_id =
            ParamId(ParamId::MULTI_CHANNEL_MIXER_POST_PEAK_HOLD_LEVEL.0 + channel as u32);
        self.unit().param(param_id, scope, Element(bus))
    }

    /// level in dB
    #[inline]
    pub fn post_input_peak_hold_level(&self, bus: u32, channel: u8) -> os::Result<f32> {
        self.post_peak_hold_level(Scope::INPUT, bus, channel)
    }

    /// level in dB
    #[inline]
    pub fn post_output_peak_hold_level(&self, channel: u8) -> os::Result<f32> {
        self.post_peak_hold_level(Scope::OUTPUT, 0, channel)
    }

    #[inline]
    pub fn max_frames_per_slice(&self) -> os::Result<u32> {
        self.unit().max_frames_per_slice()
    }

    #[inline]
    pub fn last_render_sample_time(&self) -> os::Result<f64> {
        self.unit().last_render_sample_time()
    }

    #[inline]
    pub fn last_render_err(&self) -> os::Result<os::Status> {
        self.unit().last_render_err()
    }

    #[inline]
    pub fn set_input_cb<const N: usize, T>(
        &mut self,
        bus: u32,
        cb: au::RenderCb<N, T>,
        ref_con: *const T,
    ) -> os::Result {
        self.0.set_input_cb(bus, cb, ref_con)
    }

    #[inline]
    pub fn remove_input_cb(&mut self, bus: u32) -> os::Result {
        self.0.remove_input_cb(bus)
    }
}

impl MultiChannelMixer<UninitializedState> {
    pub fn new_apple() -> os::Result<Self> {
        let desc = audio::ComponentDesc {
            type_: au::Type::MIXER.0,
            sub_type: au::SubType::MULTI_CHANNEL_MIXER.0,
            manufacturer: au::Manufacturer::APPLE.0,
            flags: 0,
            flags_mask: 0,
        };

        let comp = desc
            .into_iter()
            .next()
            .ok_or(au::component_err::UNSUPPORTED_TYPE)?;
        let unit = comp.open_unit()?;
        Ok(Self(unit))
    }

    pub fn allocate_resources(self) -> os::Result<MultiChannelMixer<InitializedState>> {
        Ok(MultiChannelMixer(self.0.initialize()?))
    }

    #[inline]
    pub fn set_max_frames_per_slice(&mut self, val: u32) -> os::Result {
        self.unit_mut().set_max_frames_per_slice(val)
    }

    #[inline]
    pub fn set_stream_format(
        &mut self,
        scope: Scope,
        bus: u32,
        val: &audio::StreamBasicDesc,
    ) -> os::Result {
        self.unit_mut().set_stream_format(scope, bus, val)
    }

    #[inline]
    pub fn set_output_stream_format(&mut self, val: &audio::StreamBasicDesc) -> os::Result {
        self.set_stream_format(Scope::OUTPUT, 0, val)
    }

    #[inline]
    pub fn set_input_stream_format(
        &mut self,
        bus: u32,
        val: &audio::StreamBasicDesc,
    ) -> os::Result {
        self.set_stream_format(Scope::INPUT, bus, val)
    }

    pub fn offline_render(&self) -> os::Result<bool> {
        self.unit().offline_render()
    }

    pub fn set_offline_render(&mut self, val: bool) -> os::Result {
        self.unit_mut().set_offline_render(val)
    }
}

impl MultiChannelMixer<InitializedState> {
    pub fn deallocate_resources(self) -> os::Result<MultiChannelMixer<UninitializedState>> {
        Ok(MultiChannelMixer(self.0.unintialize()?))
    }

    pub fn render<const N: usize>(
        &mut self,
        ts: &audio::TimeStamp,
        n_frames: u32,
        buf_list: &mut audio::BufList<N>,
    ) -> os::Result {
        self.0.render(ts, 0, n_frames, buf_list)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        at::{au, audio},
        os,
    };

    #[test]
    fn basics() {
        let mut mixer =
            au::MultiChannelMixer::new_apple().expect("failed to open multichannel mixer");

        let max_frames_per_slice = mixer.max_frames_per_slice().unwrap();
        assert_eq!(1156, max_frames_per_slice);

        mixer.set_max_frames_per_slice(1024).unwrap();
        let max_frames_per_slice = mixer.max_frames_per_slice().unwrap();
        assert_eq!(1024, max_frames_per_slice);

        assert!(mixer.is_input_enabled(0).unwrap());
        mixer.set_input_enabled(0, false).unwrap();
        assert!(!mixer.is_input_enabled(0).unwrap());

        // default volumes are zero
        assert_eq!(0.0f32, mixer.input_volume(0).unwrap());
        assert_eq!(0.0f32, mixer.output_volume().unwrap());

        // metering is not enabled by default, so we can't access params
        assert_eq!(false, mixer.is_input_metering_enabled(0).unwrap());

        let err = mixer
            .pre_input_avg_power(0, 0)
            .expect_err("metering is not enabled");

        assert_eq!(err, au::err::PROPERTY_NOT_IN_USE);

        mixer.set_input_metering_enabled(0, true).unwrap();
        mixer.set_output_metering_enabled(true).unwrap();

        let left_avg_power = mixer
            .pre_input_avg_power(0, 0)
            .expect("failed to get avg power");
        assert_eq!(-160.0, left_avg_power);
        let right_avg_power = mixer
            .pre_input_avg_power(0, 1)
            .expect("failed to get avg power");
        assert_eq!(-160.0, right_avg_power);

        let mut mixer = mixer.allocate_resources().unwrap();

        let mut buf_list: audio::BufList<2> = Default::default();
        let mut ts = audio::TimeStamp::with_sample_time(0.0);

        mixer.render(&ts, 1024, &mut buf_list).unwrap();
        assert_eq!(0.0, mixer.last_render_sample_time().unwrap());
        ts.sample_time += 1024.0;
        mixer.render(&ts, 1024, &mut buf_list).unwrap();
        assert_eq!(1024.0, mixer.last_render_sample_time().unwrap());

        assert_eq!(os::Status::NO_ERR, mixer.last_render_err().unwrap());

        let left_avg_power = mixer
            .pre_input_avg_power(0, 0)
            .expect("failed to get avg power");
        assert_eq!(-120.0, left_avg_power);
        let right_avg_power = mixer
            .pre_input_avg_power(0, 1)
            .expect("failed to get avg power");
        assert_eq!(-120.0, right_avg_power);
    }
}
