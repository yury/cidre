use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(RunLoopMode(ns::String));

define_obj_type!(RunLoop(ns::Id), NS_RUN_LOOP);

impl RunLoop {
    #[objc::cls_msg_send(currentRunLoop)]
    pub fn current() -> &'static Self;

    #[objc::cls_msg_send(mainRunLoop)]
    pub fn main() -> &'static Self;

    #[objc::msg_send(run)]
    pub fn run(&self);

    // pub fn run_until(&self, date: ns::Date)
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_RUN_LOOP: &'static objc::Class<ns::Id>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let current = ns::RunLoop::current();
        let main = ns::RunLoop::main();

        assert!(!current.is_equal(main));
    }
}
