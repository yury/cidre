use crate::{arc, swift};

crate::define_swift!(
    #[swift::class("WidgetKit.ControlCenter")]
    pub ControlCenter
);

impl ControlCenter {
    /// WidgetKit `ControlCenter.shared`.
    #[doc(alias = "ControlCenter.shared")]
    #[swift::call(
        "static WidgetKit.ControlCenter(class).shared: WidgetKit.ControlCenter(class) { get }"
    )]
    pub fn shared() -> arc::R<Self>;

    /// Asks the system to reload the controls of `kind`, as their state changed.
    #[doc(alias = "ControlCenter.reloadControls(ofKind:)")]
    #[swift::call("WidgetKit.ControlCenter(class).reloadControls(ofKind: String) thunk")]
    pub fn reload_controls_of_kind(&self, kind: &swift::String);

    #[doc(alias = "ControlCenter.reloadControls(ofKind:)")]
    pub fn reload_controls(&self, kind: &str) {
        self.reload_controls_of_kind(&swift::String::from(kind));
    }

    /// Asks the system to reload every control of the app.
    #[doc(alias = "ControlCenter.reloadAllControls()")]
    #[swift::call("WidgetKit.ControlCenter(class).reloadAllControls() thunk")]
    pub fn reload_all_controls(&self);
}

#[cfg(test)]
mod tests {
    use super::ControlCenter;

    #[test]
    fn basics() {
        let center = ControlCenter::shared();
        center.reload_controls("com.example.none");
        center.reload_all_controls();
    }
}
