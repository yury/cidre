use crate::{define_obj_type, ns, objc};

#[doc(alias = "NSStreamStatus")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum Status {
    NotOpen = 0,
    Opening = 1,
    Open = 2,
    Reading = 3,
    Writing = 4,
    AtEnd = 5,
    Closed = 6,
    Error = 7,
}

define_obj_type!(
    pub Stream(ns::Id)
);

impl Stream {
    #[objc::msg_send(open)]
    pub fn open(&mut self);

    #[objc::msg_send(close)]
    pub fn close(&mut self);
}

define_obj_type!(
    pub InputStream(Stream)
);

define_obj_type!(
    pub OutputStream(Stream)
);
