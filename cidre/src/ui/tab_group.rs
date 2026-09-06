use crate::{arc, define_cls, define_obj_type, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "UITabGroupSidebarAppearance")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum SidebarAppearance {
    Automatic,
    Inline,
    RootSection,
}

define_obj_type!(
    #[doc(alias = "UITabGroup")]
    pub TabGroup(ui::Tab)
);

impl TabGroup {
    define_cls!(UI_TAB_GROUP);

    #[cfg(feature = "blocks")]
    #[objc::init(initWithTitle:image:identifier:children:viewControllerProvider:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn init_with_title_image_id_children_vc_provider(
        self,
        title: &ns::String,
        image: Option<&ui::Image>,
        id: &ns::String,
        children: &ns::Array<ui::Tab>,
        provider: Option<&mut blocks::EscBlock<fn(&ui::Tab) -> arc::Rar<ui::ViewController>>>,
    ) -> arc::R<TabGroup>;

    #[objc::msg_send(setChildren:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn set_children(&mut self, val: &ns::Array<ui::Tab>);

    #[objc::msg_send(setSelectedChild:)]
    #[objc::available(ios = 18.0, tvos = 18.0, visionos = 2.0)]
    pub fn set_selected_child(&mut self, val: Option<&ui::Tab>);

    #[objc::msg_send(setSidebarAppearance:)]
    #[objc::available(ios = 18.0, visionos = 2.0)]
    pub fn set_sidebar_appearance(&mut self, val: SidebarAppearance);

    #[objc::msg_send(setIsSidebarDestination:)]
    #[objc::available(ios = 26.0, visionos = 26.0)]
    pub fn set_is_sidebar_destination(&mut self, val: bool);
}

unsafe extern "C" {
    static UI_TAB_GROUP: &'static objc::Class<TabGroup>;
}
