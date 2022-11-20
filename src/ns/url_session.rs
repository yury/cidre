use crate::{define_obj_type, ns};

define_obj_type!(Task(ns::Id));
define_obj_type!(DataTask(Task));
define_obj_type!(UploadTask(DataTask));
define_obj_type!(DownloadTask(Task));
define_obj_type!(StreamTask(Task));
define_obj_type!(WebSocketTask(Task));

define_obj_type!(URLSession(ns::Id));

impl URLSession {

    /// ```
    /// use cidre::ns;
    /// 
    /// let session = ns::URLSession::shared();
    /// println!("session: {:?}", session);
    /// ```
    pub fn shared() -> &'static URLSession {
        unsafe { NSURLSession_sharedSession() }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSURLSession_sharedSession() -> &'static URLSession;
}
