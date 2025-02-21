use crate::{arc, define_obj_type, mlc, objc};

define_obj_type!(pub TrainingGraph(mlc::Graph), MLC_TRAINING_GRAPH);
impl TrainingGraph {
    /// The optimizer to be used with the training graph
    #[objc::msg_send(optimizer)]
    pub fn optimizer(&self) -> Option<&mlc::Optimizer>;

    /// The device memory size used by the training graph
    ///
    /// Returns the total size in bytes of device memory used for all intermediate tensors
    /// for forward, gradient passes and optimizer update for all layers in the training graph.
    /// We recommend executing an iteration before checking the device memory size as
    /// the buffers needed get allocated when the corresponding pass such as gradient,
    /// optimizer update is executed.
    #[objc::msg_send(deviceMemorySize)]
    pub fn device_mem_size(&self) -> usize;
}

#[link(name = "mlc", kind = "static")]
unsafe extern "C" {
    static MLC_TRAINING_GRAPH: &'static objc::Class<TrainingGraph>;
}

#[cfg(test)]
mod tests {
    use crate::mlc;

    #[test]
    fn basics() {
        let graph = mlc::TrainingGraph::new();
        assert!(graph.optimizer().is_none());
        assert_eq!(graph.device_mem_size(), 0);
    }
}
