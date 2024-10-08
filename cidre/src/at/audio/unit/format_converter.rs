use crate::{
    at::{
        au,
        au::{Scope, Unit, UnitRef},
        audio,
        audio::component::{InitializedState, State, UninitializedState},
    },
    os,
};

/// It is better to use AudioConverterRef directly
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
    pub fn max_frames_per_slice(&self) -> os::Result<u32> {
        self.unit().max_frames_per_slice()
    }

    #[inline]
    pub fn last_render_err(&self) -> os::Result<os::Status> {
        self.unit().last_render_err()
    }

    #[inline]
    pub fn set_input_cb<const N: usize, T>(
        &mut self,
        cb: au::RenderCb<N, T>,
        ref_con: *const T,
    ) -> os::Result {
        self.0.set_input_cb(0, cb, ref_con)
    }

    #[inline]
    pub fn remove_input_cb(&mut self) -> os::Result {
        self.0.remove_input_cb(0)
    }

    #[inline]
    pub fn output_stream_format(&self) -> os::Result<audio::StreamBasicDesc> {
        self.unit().stream_format(Scope::OUTPUT, 0)
    }

    #[inline]
    pub fn input_stream_format(&self) -> os::Result<audio::StreamBasicDesc> {
        self.unit().stream_format(Scope::INPUT, 0)
    }
}

impl FormatConverter<UninitializedState> {
    pub fn new_apple() -> os::Result<Self> {
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

    pub fn allocate_resources(self) -> os::Result<FormatConverter<InitializedState>> {
        Ok(FormatConverter(self.0.initialize()?))
    }

    #[inline]
    pub fn set_max_frames_per_slice(&mut self, val: u32) -> os::Result {
        self.unit_mut().set_max_frames_per_slice(val)
    }

    #[inline]
    pub fn set_output_stream_format(&mut self, val: &audio::StreamBasicDesc) -> os::Result {
        self.unit_mut().set_stream_format(Scope::OUTPUT, 0, val)
    }

    #[inline]
    pub fn set_input_stream_format(&mut self, val: &audio::StreamBasicDesc) -> os::Result {
        self.unit_mut().set_stream_format(Scope::INPUT, 0, val)
    }
}

impl FormatConverter<InitializedState> {
    #[inline]
    pub fn deallocate_resources(self) -> os::Result<FormatConverter<UninitializedState>> {
        Ok(FormatConverter(self.0.unintialize()?))
    }

    #[inline]
    pub fn render<const N: usize>(
        &mut self,
        n_frames: u32,
        buf_list: &mut audio::BufList<N>,
    ) -> os::Result {
        let ts = audio::TimeStamp::invalid();
        self.0.render(&ts, 0, n_frames, buf_list)
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::c_void;

    use crate::{
        at,
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

        extern "C-unwind" fn render(
            in_ref_con: *mut c_void,
            _io_action_flags: &mut au::RenderActionFlags,
            _in_timestamp: &at::AudioTimeStamp,
            in_bus_num: u32,
            in_number_frames: u32,
            io_data: *mut at::AudioBufList<2>,
        ) -> os::Status {
            assert_eq!(in_bus_num, 0);
            assert_eq!(in_number_frames, 1024);
            assert!(!io_data.is_null());
            assert!(in_ref_con.is_null());
            os::Status::NO_ERR
        }

        conv.set_input_cb(render, std::ptr::null_mut()).unwrap();

        let mut conv = conv.allocate_resources().unwrap();
        let mut buf_list: audio::BufList<2> = Default::default();

        conv.render(1024, &mut buf_list).unwrap();
        conv.render(1024, &mut buf_list).unwrap();
        assert_eq!(os::Status::NO_ERR, conv.last_render_err().unwrap());

        conv.remove_input_cb().unwrap();
        let err = conv
            .render(1024, &mut buf_list)
            .expect_err("no connection err");

        assert_eq!(err, au::err::NO_CONNECTION);
        assert_eq!(err, conv.last_render_err().unwrap());
    }
}
