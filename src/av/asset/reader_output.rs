use crate::{av::MediaType, define_obj_type, ns};

define_obj_type!(ReaderOutput(ns::Id));

define_obj_type!(ReaderTrackOutput(ns::Id));

impl ReaderOutput {
    pub fn media_type(&self) -> &MediaType {
        unsafe { rsel_mediaType(self) }
    }

    pub fn always_copies_sample_data(&self) -> bool {
        unsafe { rsel_alwaysCopiesSampleData(self) }
    }

    pub fn set_always_copies_sample_data(&self, value: bool) {
        unsafe { wsel_setAlwaysCopiesSampleData(self, value) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_mediaType(id: &ns::Id) -> &MediaType;

    fn rsel_alwaysCopiesSampleData(id: &ns::Id) -> bool;
    fn wsel_setAlwaysCopiesSampleData(id: &ns::Id, value: bool);
}
