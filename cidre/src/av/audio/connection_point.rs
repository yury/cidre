use crate::{arc, av::audio, define_cls, define_obj_type, ns, objc};

define_obj_type!(pub ConnectionPoint(ns::Id));

impl arc::A<ConnectionPoint> {
    #[objc::msg_send(initWithNode:bus:)]
    pub fn init_with_node_bus(
        self,
        node: &audio::Node,
        bus: audio::NodeBus,
    ) -> arc::R<ConnectionPoint>;
}

impl ConnectionPoint {
    define_cls!(AV_AUDIO_CONNECTION_POINT);

    pub fn with_node_bus(node: &audio::Node, bus: audio::NodeBus) -> arc::R<Self> {
        Self::alloc().init_with_node_bus(node, bus)
    }

    #[objc::msg_send(node)]
    pub fn node(&self) -> &audio::Node;

    #[objc::msg_send(bus)]
    pub fn bus(&self) -> audio::NodeBus;
}

extern "C" {
    static AV_AUDIO_CONNECTION_POINT: &'static objc::Class<ConnectionPoint>;
}
