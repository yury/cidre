use crate::{blocks, define_obj_type, ml, ns, objc};

define_obj_type!(
    #[doc(alias = "MLState")]
    pub State(ns::Id)
);

impl State {
    #[objc::msg_send(getMultiArrayForStateNamed:handler:)]
    pub fn multi_array_for_state_named_handler(
        &self,
        state_name: &ns::String,
        handler: &mut blocks::NoEscBlock<fn(&mut ml::MultiArray)>,
    );

    #[inline]
    #[objc::available(macos = 15.0, ios = 18.0, watchos = 11.0, tvos = 18.0)]
    pub fn with_multi_array_for_state_named(
        &self,
        state_name: &ns::String,
        mut handler: impl FnMut(&mut ml::MultiArray),
    ) {
        let mut block = unsafe { blocks::NoEscBlock::stack1(&mut handler) };
        self.multi_array_for_state_named_handler(state_name, &mut block);
    }
}
