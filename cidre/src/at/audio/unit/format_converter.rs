use crate::{
    at::{
        au,
        au::{Scope, Unit, UnitRef},
        audio,
        audio::component::{InitializedState, State, UninitializedState},
    },
    os,
};

#[repr(transparent)]
pub struct FormatConverter<S>(UnitRef<S>)
where
    S: State<Unit>;

impl<S> FormatConverter<S>
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

    #[inline]
    pub fn max_frames_per_slice(&self) -> Result<u32, os::Status> {
        self.unit().max_frames_per_slice()
    }

    #[inline]
    pub fn last_render_sample_time(&self) -> Result<f64, os::Status> {
        self.unit().last_render_sample_time()
    }

    #[inline]
    pub fn last_render_err(&self) -> Result<os::Status, os::Status> {
        self.unit().last_render_err()
    }

    #[inline]
    pub fn set_input_cb<const N: usize, T>(
        &mut self,
        bus: u32,
        cb: au::RenderCb<N, T>,
        ref_con: *const T,
    ) -> Result<(), os::Status> {
        self.0.set_input_cb(bus, cb, ref_con)
    }

    #[inline]
    pub fn remove_input_cb(&mut self, bus: u32) -> Result<(), os::Status> {
        self.0.remove_input_cb(bus)
    }
}

impl FormatConverter<UninitializedState> {
    pub fn new_apple() -> Result<Self, os::Status> {
        let desc = audio::ComponentDesc {
            type_: au::Type::FORMAT_CONVERTER.0,
            sub_type: au::SubType::CONVERTER.0,
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

    pub fn allocate_resources(self) -> Result<FormatConverter<InitializedState>, os::Status> {
        Ok(FormatConverter(self.0.initialize()?))
    }

    #[inline]
    pub fn set_max_frames_per_slice(&mut self, val: u32) -> Result<(), os::Status> {
        self.unit_mut().set_max_frames_per_slice(val)
    }

    #[inline]
    pub fn set_stream_format(
        &mut self,
        scope: Scope,
        val: &audio::StreamBasicDesc,
    ) -> Result<(), os::Status> {
        self.unit_mut().set_stream_format(scope, 0, val)
    }

    #[inline]
    pub fn set_output_stream_format(
        &mut self,
        val: &audio::StreamBasicDesc,
    ) -> Result<(), os::Status> {
        self.set_stream_format(Scope::OUTPUT, val)
    }

    #[inline]
    pub fn set_input_stream_format(
        &mut self,
        val: &audio::StreamBasicDesc,
    ) -> Result<(), os::Status> {
        self.set_stream_format(Scope::INPUT, val)
    }
}

impl FormatConverter<InitializedState> {
    pub fn deallocate_resources(self) -> Result<FormatConverter<UninitializedState>, os::Status> {
        Ok(FormatConverter(self.0.unintialize()?))
    }

    pub fn render<const N: usize>(
        &mut self,
        ts: &audio::TimeStamp,
        nframes: u32,
        buf_list: &mut audio::BufList<N>,
    ) -> Result<(), os::Status> {
        self.0.render(ts, 0, nframes, buf_list)
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
        let mut conv = au::FormatConverter::new_apple().expect("failed to open format converter");

        let max_frames_per_slice = conv.max_frames_per_slice().unwrap();
        assert_eq!(1156, max_frames_per_slice);

        conv.set_max_frames_per_slice(1024).unwrap();
        let max_frames_per_slice = conv.max_frames_per_slice().unwrap();
        assert_eq!(1024, max_frames_per_slice);

        let mut conv = conv.allocate_resources().unwrap();

        let mut buf_list: audio::BufList<2> = Default::default();
        let mut ts = audio::TimeStamp::with_sample_time(0.0);

        // conv.render(&ts, 1024, &mut buf_list).unwrap();
        // assert_eq!(0.0, conv.last_render_sample_time().unwrap());
        // ts.sample_time += 1024.0;
        // conv.render(&ts, 1024, &mut buf_list).unwrap();
        // assert_eq!(1024.0, conv.last_render_sample_time().unwrap());

        assert_eq!(os::Status::NO_ERR, conv.last_render_err().unwrap());
    }
}
