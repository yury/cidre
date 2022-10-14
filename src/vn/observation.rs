use crate::{define_obj_type, objc};

define_obj_type!(Observation(objc::Id));

define_obj_type!(DetectedObjectObservation(Observation));

define_obj_type!(RectangleObservation(DetectedObjectObservation));

define_obj_type!(TextObservation(RectangleObservation));

define_obj_type!(RecognizedTextObservation(RectangleObservation));

define_obj_type!(HorizonObservation(Observation));
