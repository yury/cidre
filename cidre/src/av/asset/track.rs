use crate::{av::MediaType, define_obj_type, ns, objc};

define_obj_type!(Track(ns::Id));
define_obj_type!(FragmentedTrack(Track));

impl Track {
    #[objc::msg_send(mediaType)]
    pub fn media_type(&self) -> &MediaType;
}
