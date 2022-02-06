use crate::{cf::Type, define_cf_type};

pub type Session = Type;
// define_cf_type!(Session(Type));

pub fn foo(session: &Session) {
    session.show()
}
