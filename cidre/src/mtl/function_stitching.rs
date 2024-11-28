use crate::{arc, define_cls, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTLFunctionStitchingAttribute")]
    pub FnStitchingAttr(ns::Id)
);

define_obj_type!(
    #[doc(alias = "MTLFunctionStitchingAttributeAlwaysInline")]
    pub FnStitchingAttrAlwaysInline(FnStitchingAttr),
    MTL_FUNCTION_STITCHING_ATTRIBUTE_ALWAYS_INLINE
);

define_obj_type!(
    #[doc(alias = "MTLFunctionStitchingNode")]
    pub FnStitchingNode(ns::Id)
);

define_obj_type!(
    #[doc(alias = "MTLFunctionStitchingInputNode")]
    pub FnStitchingInputNode(FnStitchingNode),
    MTL_FUNCTION_STITCHING_INPUT_NODE
);

impl arc::A<FnStitchingInputNode> {
    #[objc::msg_send(initWithArgumentIndex:)]
    pub fn init_with_arg_index(self, index: usize) -> arc::R<FnStitchingInputNode>;
}

impl FnStitchingInputNode {
    pub fn with_arg_index(index: usize) -> arc::R<Self> {
        Self::alloc().init_with_arg_index(index)
    }

    #[objc::msg_send(argumentIndex)]
    pub fn arg_index(&self) -> usize;

    #[objc::msg_send(setArgumentIndex:)]
    pub fn set_arg_index(&mut self, val: usize);
}

define_obj_type!(
    #[doc(alias = "MTLFunctionStitchingFunctionNode")]
    pub FnStitchingFnNode(FnStitchingNode)
);

impl arc::A<FnStitchingFnNode> {
    #[objc::msg_send(initWithName:arguments:controlDependencies:)]
    pub fn init_with_name(
        self,
        name: &ns::String,
        args: &ns::Array<FnStitchingNode>,
        control_deps: &ns::Array<FnStitchingFnNode>,
    ) -> arc::R<FnStitchingFnNode>;
}

impl FnStitchingFnNode {
    define_cls!(MTL_FUNCTION_STITCHING_FUNCTION_NODE);

    pub fn with_name(
        name: &ns::String,
        args: &ns::Array<FnStitchingNode>,
        control_deps: &ns::Array<FnStitchingFnNode>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_name(name, args, control_deps)
    }

    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, val: &ns::String);

    #[objc::msg_send(arguments)]
    pub fn args(&self) -> arc::R<ns::Array<FnStitchingNode>>;

    #[objc::msg_send(setArguments:)]
    pub fn set_args(&mut self, val: &ns::Array<FnStitchingNode>);

    #[objc::msg_send(controlDependencies)]
    pub fn control_deps(&self) -> arc::R<ns::Array<FnStitchingFnNode>>;

    #[objc::msg_send(setControlDependencies:)]
    pub fn set_control_deps(&mut self, val: &ns::Array<FnStitchingFnNode>);
}

define_obj_type!(
    #[doc(alias = "MTLFunctionStitchingGraph")]
    pub FnStitchingGraph(ns::Id)
);

impl arc::A<FnStitchingGraph> {
    #[objc::msg_send(initWithFunctionName:nodes:outputNode:attributes:)]
    pub fn init_with_fn_name(
        self,
        fn_name: &ns::String,
        nodes: &ns::Array<FnStitchingFnNode>,
        output_node: Option<&FnStitchingFnNode>,
        attribtues: &ns::Array<FnStitchingAttr>,
    ) -> arc::R<FnStitchingGraph>;
}

impl FnStitchingGraph {
    define_cls!(MTL_FUNCTION_STITCHING_GRAPH);

    pub fn with_fn_name(
        fn_name: &ns::String,
        nodes: &ns::Array<FnStitchingFnNode>,
        output_node: Option<&FnStitchingFnNode>,
        attributes: &ns::Array<FnStitchingAttr>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_fn_name(fn_name, nodes, output_node, attributes)
    }

    #[objc::msg_send(functionName)]
    pub fn fn_name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setFunctionName:)]
    pub fn set_fn_name(&mut self, val: &ns::String);

    #[objc::msg_send(nodes)]
    pub fn nodes(&self) -> arc::R<ns::Array<FnStitchingFnNode>>;

    #[objc::msg_send(setNodes:)]
    pub fn set_nodes(&self, val: &ns::Array<FnStitchingFnNode>);

    #[objc::msg_send(outputNode)]
    pub fn output_node(&self) -> Option<arc::R<FnStitchingFnNode>>;

    #[objc::msg_send(setOutputNode:)]
    pub fn set_output_node(&mut self, val: Option<&FnStitchingFnNode>);

    #[objc::msg_send(attributes)]
    pub fn attrs(&self) -> arc::R<ns::Array<FnStitchingAttr>>;

    #[objc::msg_send(setAttributes:)]
    pub fn set_attrs(&mut self, val: &ns::Array<FnStitchingAttr>);
}

define_obj_type!(
    #[doc(alias = "MTLStitchedLibraryDescriptor")]
    pub FnStitchedLibDesc(ns::Id),
    MTL_FUNCTION_STITCHED_LIBRARY_DESCRIPTOR
);

impl FnStitchedLibDesc {
    #[objc::msg_send(functionGraphs)]
    pub fn fn_graphs(&self) -> arc::R<ns::Array<FnStitchingGraph>>;

    #[objc::msg_send(setFunctionGraphs:)]
    pub fn set_fn_graphs(&mut self, val: &ns::Array<FnStitchingGraph>);

    #[objc::msg_send(functions)]
    pub fn fns(&self) -> arc::R<ns::Array<mtl::Fn>>;

    #[objc::msg_send(setFunctions:)]
    pub fn set_fns(&self, val: &ns::Array<mtl::Fn>);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_FUNCTION_STITCHING_GRAPH: &'static objc::Class<FnStitchingGraph>;
    static MTL_FUNCTION_STITCHING_INPUT_NODE: &'static objc::Class<FnStitchingInputNode>;
    static MTL_FUNCTION_STITCHING_FUNCTION_NODE: &'static objc::Class<FnStitchingFnNode>;
    static MTL_FUNCTION_STITCHING_ATTRIBUTE_ALWAYS_INLINE:
        &'static objc::Class<FnStitchingAttrAlwaysInline>;
    static MTL_FUNCTION_STITCHED_LIBRARY_DESCRIPTOR: &'static objc::Class<FnStitchedLibDesc>;
}

#[cfg(test)]
mod tests {
    use crate::{mtl, ns};

    #[test]
    fn basics() {
        let name = ns::str!(c"a");
        let args = ns::Array::new();
        let deps = ns::Array::new();
        let fn_node = mtl::FnStitchingFnNode::with_name(name, &args, &deps);
        let name = fn_node.name();
        eprintln!("name {:?}", name);
        let args = fn_node.args();
        eprintln!("name {:?}", args);

        let desc = mtl::FnStitchedLibDesc::new();
        assert!(desc.fns().is_empty());
        assert!(desc.fn_graphs().is_empty());
    }
}
