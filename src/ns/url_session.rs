use crate::{
    cf::{self, Retained},
    define_obj_type, ns,
};

define_obj_type!(Configuration(ns::Id));

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
    /// assert!(data_task.error().is_none());
    /// assert_eq!(data_task.state(), ns::URLSessionTaskState::Suspended);
    /// data_task.resume();
    /// assert_eq!(data_task.state(), ns::URLSessionTaskState::Running);
    /// ```
    pub fn shared() -> &'static Session {
        unsafe { NSURLSession_sharedSession() }
    }

    pub fn data_task_with_url(&self, url: &cf::URL) -> Retained<DataTask> {
        unsafe { rsel_dataTaskWithURL(self, url) }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(isize)]
pub enum TaskState {
    /// The task is currently being serviced by the session.
    Running = 0,

    /// The task was suspended by the app.
    Suspended = 1,

    /// The task has received a cancel message.
    Canceling = 2,

    /// The task has completed (without being canceled), and the task's
    /// delegate receives no further callbacks.
    Completed = 3,
}

/// defines constants that
/// can be used to specify the multipath service type to associate an NSURLSession.  The
/// multipath service type determines whether multipath TCP should be attempted and the conditions
/// for creating and switching between subflows.  Using these service types requires the appropriate entitlement.  Any connection attempt will fail if the process does not have the required entitlement.
/// A primary interface is a generally less expensive interface in terms of both cost and power (such as WiFi or ethernet).  A secondary interface is more expensive (such as 3G or LTE).
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(isize)]
pub enum MultipathServiceType {
    None = 0,

    /// Specifies that a secondary subflow should only be used
    /// when the primary subflow is not performing adequately.
    /// Requires the com.apple.developer.networking.multipath entitlement.
    Handover = 1,

    /// Specifies that a secondary subflow should be used if the
    /// primary subflow is not performing adequately (packet loss, high round trip times, bandwidth issues).  The secondary
    /// subflow will be created more aggressively than with NSURLSessionMultipathServiceTypeHandover.
    /// Requires the com.apple.developer.networking.multipath entitlement.
    Interactive = 2, /* Interactive - secondary flows created more aggressively. */

    /// Specifies that multiple subflows across multiple interfaces should be
    /// used for better bandwidth.  This mode is only available for experimentation on devices configured for development use.
    /// It can be enabled in the Developer section of the Settings app.
    Aggregate = 3, /* Aggregate - multiple subflows used for greater bandwidth. */
}

impl Task {
    pub fn resume(&self) {
        unsafe {
            NSURLSessionTask_wsel_resume(self);
        }
    }

    pub fn cancel(&self) {
        unsafe {
            NSURLSessionTask_wsel_cancel(self);
        }
    }

    pub fn suspend(&self) {
        unsafe {
            NSURLSessionTask_wsel_suspend(self);
        }
    }

    pub fn state(&self) -> TaskState {
        unsafe { NSURLSessionTask_rsel_state(self) }
    }

    pub fn error(&self) -> Option<&cf::Error> {
        unsafe { NSURLSessionTask_rsel_error(self) }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum WebSocketCloseCode {
    Invalid = 0,
    NormalClosure = 1000,
    GoingAway = 1001,
    ProtocolError = 1002,
    UnsupportedData = 1003,
    NoStatusReceived = 1005,
    AbnormalClosure = 1006,
    InvalidFramePayloadData = 1007,
    PolicyViolation = 1008,
    MessageTooBig = 1009,
    MandatoryExtensionMissing = 1010,
    InternalServerError = 1011,
    TLSHandshakeFailure = 1015,
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSURLSession_sharedSession() -> &'static Session;
    fn rsel_dataTaskWithURL(session: &Session, url: &cf::URL) -> Retained<DataTask>;

    fn NSURLSessionTask_wsel_resume(task: &Task);
    fn NSURLSessionTask_wsel_cancel(task: &Task);
    fn NSURLSessionTask_wsel_suspend(task: &Task);
    fn NSURLSessionTask_rsel_state(task: &Task) -> TaskState;
    fn NSURLSessionTask_rsel_error(task: &Task) -> Option<&cf::Error>;
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    static NSURLSessionTaskPriorityDefault: f32;
    static NSURLSessionTaskPriorityLow: f32;
    static NSURLSessionTaskPriorityHigh: f32;
}
