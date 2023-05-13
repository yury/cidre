use crate::{arc, define_obj_type, mlc, objc};

define_obj_type!(TrainingGraph(mlc::Graph), MLC_TRAINING_GRAPH);
impl TrainingGraph {}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_TRAINING_GRAPH: &'static objc::Class<TrainingGraph>;
}
