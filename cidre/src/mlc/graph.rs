use crate::{arc, define_obj_type, mlc, ns, objc};

define_obj_type!(pub Graph(ns::Id), MLC_GRAPH);
impl Graph {
    #[objc::msg_send(nodeWithLayer:sources:)]
    pub fn node_with_layer_sources_array_ar(
        &self,
        layer: &mlc::Layer,
        sources: &ns::Array<mlc::Tensor>,
    ) -> Option<arc::Rar<mlc::Tensor>>;

    #[objc::rar_retain]
    pub fn node_with_layer_sources_array(
        &self,
        layer: &mlc::Layer,
        sources: &ns::Array<mlc::Tensor>,
    ) -> Option<arc::R<mlc::Tensor>>;

    pub fn node_with_layer_sources(
        &self,
        layer: &mlc::Layer,
        sources: &[&mlc::Tensor],
    ) -> Option<arc::R<mlc::Tensor>> {
        let sources = ns::Array::from_slice(sources);
        self.node_with_layer_sources_array(layer, &sources)
    }
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_GRAPH: &'static objc::Class<Graph>;
}
