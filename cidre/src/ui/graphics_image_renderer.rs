use crate::{arc, blocks, cg, define_cls, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIGraphicsImageRendererContext")]
    pub GraphicsImageRendererCtx(ns::Id)
);

define_obj_type!(
    #[doc(alias = "UIGraphicsImageRenderer")]
    pub GraphicsImageRenderer(ns::Id)
);

impl GraphicsImageRenderer {
    define_cls!(sym UIGraphicsImageRenderer);

    #[objc::init(initWithSize:)]
    fn init_with_size(self, size: cg::Size) -> arc::R<GraphicsImageRenderer>;

    #[objc::available(ios = 10.0, tvos = 10.0)]
    pub fn with_size(size: cg::Size) -> arc::R<Self> {
        Self::alloc().init_with_size(size)
    }

    #[objc::msg_send(imageWithActions:)]
    #[objc::available(ios = 10.0, tvos = 10.0)]
    pub fn image_with_actions_block(
        &self,
        actions: &mut blocks::NoEscBlock<fn(&GraphicsImageRendererCtx)>,
    ) -> arc::R<ui::Image>;

    #[objc::available(ios = 10.0, tvos = 10.0)]
    pub fn image(&self, mut draw: impl FnMut(&GraphicsImageRendererCtx)) -> arc::R<ui::Image> {
        let mut block = unsafe { blocks::NoEscBlock::stack1(&mut draw) };
        self.image_with_actions_block(&mut block)
    }
}
