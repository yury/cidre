use crate::{arc, define_obj_type, ns, objc, ui};

#[doc(alias = "UIDropOperation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum DropOperation {
    Cancel,
    Forbidden,
    Copy,
    Move,
}

define_obj_type!(
    /// What a drop would do, as a drop interaction's delegate answers while the drag moves.
    #[doc(alias = "UIDropProposal")]
    pub DropProposal(ns::Id),
    UI_DROP_PROPOSAL
);

impl arc::A<DropProposal> {
    #[objc::msg_send(initWithDropOperation:)]
    pub fn init_with_drop_op(self, op: DropOperation) -> arc::R<DropProposal>;
}

impl DropProposal {
    pub fn with_drop_op(op: DropOperation) -> arc::R<Self> {
        Self::alloc().init_with_drop_op(op)
    }

    #[objc::msg_send(operation)]
    pub fn op(&self) -> DropOperation;
}

define_obj_type!(
    /// Lets a view take drops: its delegate says what it accepts and takes it.
    #[doc(alias = "UIDropInteraction")]
    pub DropInteraction(ns::Id),
    UI_DROP_INTERACTION
);

impl ui::Interaction for DropInteraction {}

impl arc::A<DropInteraction> {
    #[objc::msg_send(initWithDelegate:)]
    pub fn init_with_delegate<D: DropInteractionDelegate>(
        self,
        delegate: &D,
    ) -> arc::R<DropInteraction>;
}

impl DropInteraction {
    /// The interaction, `delegate` held weakly.
    pub fn with_delegate<D: DropInteractionDelegate>(delegate: &D) -> arc::R<Self> {
        Self::alloc().init_with_delegate(delegate)
    }

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyDropInteractionDelegate>>;

    #[objc::msg_send(allowsSimultaneousDropSessions)]
    pub fn allows_simultaneous_drop_sessions(&self) -> bool;

    #[objc::msg_send(setAllowsSimultaneousDropSessions:)]
    pub fn set_allows_simultaneous_drop_sessions(&mut self, val: bool);
}

#[objc::protocol(UIDropInteractionDelegate)]
pub trait DropInteractionDelegate: objc::Obj {
    /// Whether the view takes what `session` carries at all.
    #[objc::optional]
    #[objc::msg_send(dropInteraction:canHandleSession:)]
    fn drop_interaction_can_handle_session(
        &mut self,
        interaction: &mut DropInteraction,
        session: &mut ui::AnyDropSession,
    ) -> bool;

    #[objc::optional]
    #[objc::msg_send(dropInteraction:sessionDidEnter:)]
    fn drop_interaction_session_did_enter(
        &mut self,
        interaction: &mut DropInteraction,
        session: &mut ui::AnyDropSession,
    );

    /// What a drop here and now would do; required to take the drop.
    #[objc::optional]
    #[objc::msg_send(dropInteraction:sessionDidUpdate:)]
    fn drop_interaction_session_did_update(
        &mut self,
        interaction: &mut DropInteraction,
        session: &mut ui::AnyDropSession,
    ) -> arc::R<DropProposal>;

    #[objc::optional]
    #[objc::msg_send(dropInteraction:sessionDidExit:)]
    fn drop_interaction_session_did_exit(
        &mut self,
        interaction: &mut DropInteraction,
        session: &mut ui::AnyDropSession,
    );

    /// The user dropped: load the items from the session's item providers.
    #[objc::optional]
    #[objc::msg_send(dropInteraction:performDrop:)]
    fn drop_interaction_perform_drop(
        &mut self,
        interaction: &mut DropInteraction,
        session: &mut ui::AnyDropSession,
    );

    #[objc::optional]
    #[objc::msg_send(dropInteraction:sessionDidEnd:)]
    fn drop_interaction_session_did_end(
        &mut self,
        interaction: &mut DropInteraction,
        session: &mut ui::AnyDropSession,
    );
}

define_obj_type!(
    pub AnyDropInteractionDelegate(ns::Id)
);

impl DropInteractionDelegate for AnyDropInteractionDelegate {}

unsafe extern "C" {
    static UI_DROP_PROPOSAL: &'static objc::Class<DropProposal>;
    static UI_DROP_INTERACTION: &'static objc::Class<DropInteraction>;
}

#[cfg(test)]
mod tests {
    use crate::ui;

    #[test]
    fn proposal() {
        let proposal = ui::DropProposal::with_drop_op(ui::DropOperation::Copy);
        assert_eq!(proposal.op(), ui::DropOperation::Copy);
    }
}
