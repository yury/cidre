use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(#[doc(alias = "UIActivityViewController")] pub ActivityViewController(ui::ViewController), sym UIActivityViewController);

impl ActivityViewController {
    #[objc::init(initWithActivityItemsConfiguration:)]
    pub fn init_with_items_cfg(self, configuration: &ui::ActivityItemsCfg) -> arc::R<ActivityViewController>;

    #[objc::init(initWithActivityItems:applicationActivities:)]
    pub fn init_with_items(
        self,
        items: &ns::Array<ns::Id>,
        activities: Option<&ns::Array<ui::Activity>>,
    ) -> arc::R<ActivityViewController>;
}
