use crate::{define_obj_type, ns, cf::{Retained, self}};

define_obj_type!(Task(ns::Id));
define_obj_type!(DataTask(Task));
define_obj_type!(UploadTask(DataTask));
define_obj_type!(DownloadTask(Task));
define_obj_type!(StreamTask(Task));
define_obj_type!(WebSocketTask(Task));

define_obj_type!(Session(ns::Id));

impl Session {

    /// ```
    /// use cidre::{ns, cf};
    /// 
    /// let session = ns::URLSession::shared();
    /// println!("session: {:?}", session);
    /// let url = cf::URL::from_str("https://google.com").unwrap();
    /// let data_task = session.data_task_with_url(&url);
    /// ```
    pub fn shared() -> &'static Session {
      
        unsafe { NSURLSession_sharedSession() }
    }

    pub fn data_task_with_url(&self, url: &cf::URL) -> Retained<DataTask> {
      unsafe {
        rsel_dataTaskWithURL(self, url)
      }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSURLSession_sharedSession() -> &'static Session;
    fn rsel_dataTaskWithURL(session: &Session, url: &cf::URL) -> Retained<DataTask>;
}
