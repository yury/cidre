use crate::{av::MediaType, define_obj_type, ns};

define_obj_type!(Track(ns::Id));
define_obj_type!(FragmentedTrack(Track));

impl Track {
    pub fn media_type(&self) -> MediaType {
        unsafe { rsel_mediaType(self) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_mediaType(id: &ns::Id) -> MediaType;

    // fn rsel_alwaysCopiesSampleData(id: &ns::Id) -> bool;
    // fn wsel_setAlwaysCopiesSampleData(id: &ns::Id, value: bool);
}
