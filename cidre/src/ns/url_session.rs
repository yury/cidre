use crate::{
    arc, define_obj_type, ns,
    objc::{self, Class},
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
    /// ```no_run
    /// use cidre::ns;
    ///
    /// let session = ns::URLSession::shared();
    /// println!("session: {:?}", session);
    /// let url = ns::URL::with_str("https://google.com").unwrap();
    /// let data_task = session.data_task_with_url(&url);
    /// assert!(data_task.error().is_none());
    /// assert_eq!(data_task.priority(), 0.5f32);
    /// assert_eq!(data_task.state(), ns::URLSessionTaskState::Suspended);
    /// data_task.resume();
    /// assert_eq!(data_task.state(), ns::URLSessionTaskState::Running);
    /// ```
    #[objc::cls_msg_send(sharedSession)]
    pub fn shared() -> &'static Session;

    #[inline]
    pub fn cls() -> &'static Class<Self> {
        unsafe { NS_URL_SESSION }
    }

    #[objc::msg_send2(dataTaskWithURL:)]
    pub fn data_task_with_url_ar(&self, url: &ns::URL) -> arc::Rar<DataTask>;

    #[objc::rar_retain()]
    pub fn data_task_with_url(&self, url: &ns::URL) -> arc::R<DataTask>;

    #[objc::msg_send2(dataTaskWithRequest:)]
    pub fn data_task_with_request_ar(&self, request: &ns::URLRequest) -> arc::Rar<DataTask>;

    #[objc::rar_retain()]
    pub fn data_task_with_request(&self, request: &ns::URLRequest) -> arc::R<DataTask>;
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
    #[objc::msg_send2(resume)]
    pub fn resume(&self);

    #[objc::msg_send2(cancel)]
    pub fn cancel(&self);

    #[objc::msg_send2(suspend)]
    pub fn suspend(&self);

    #[objc::msg_send2(state)]
    pub fn state(&self) -> TaskState;

    #[objc::msg_send2(error)]
    pub fn error(&self) -> Option<&ns::Error>;

    #[objc::msg_send2(taskIdentifier)]
    pub fn task_identifier(&self) -> ns::UInteger;

    #[objc::msg_send2(originalRequest)]
    pub fn original_request(&self) -> Option<&ns::URLRequest>;

    #[objc::msg_send2(currentRequest)]
    pub fn current_request(&self) -> Option<&ns::URLRequest>;

    #[objc::msg_send2(response)]
    pub fn response(&self) -> Option<&ns::URLResponse>;

    #[objc::msg_send2(priority)]
    pub fn priority(&self) -> f32;

    #[objc::msg_send2(setPriority:)]
    pub fn set_priority(&mut self, value: f32);
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

pub struct TaskPriority;

impl TaskPriority {
    #[inline]
    pub fn default() -> f32 {
        unsafe { NSURLSessionTaskPriorityDefault }
    }

    #[inline]
    pub fn low() -> f32 {
        unsafe { NSURLSessionTaskPriorityLow }
    }

    #[inline]
    pub fn high() -> f32 {
        unsafe { NSURLSessionTaskPriorityHigh }
    }
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    static NSURLSessionTaskPriorityDefault: f32;
    static NSURLSessionTaskPriorityLow: f32;
    static NSURLSessionTaskPriorityHigh: f32;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum WebSocketMessageType {
    Data = 0,
    String = 1,
}

define_obj_type!(WebSocketMessage(ns::Id));

impl WebSocketMessage {
    #[inline]
    pub fn with_data(data: &ns::Data) -> arc::R<Self> {
        unsafe { NSURLSessionWebSocketMessage_initWithData(data) }
    }

    #[inline]
    pub fn with_string(string: &ns::String) -> arc::R<Self> {
        unsafe { NSURLSessionWebSocketMessage_initWithString(string) }
    }

    #[inline]
    pub fn data(&self) -> Option<&ns::Data> {
        unsafe { NSURLSessionWebSocketMessage_data(self) }
    }

    #[inline]
    pub fn string(&self) -> Option<&ns::String> {
        unsafe { NSURLSessionWebSocketMessage_string(self) }
    }

    #[inline]
    pub fn type_(&self) -> WebSocketMessageType {
        unsafe { NSURLSessionWebSocketMessage_type(self) }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_URL_SESSION: &'static Class<Session>;

    fn NSURLSessionWebSocketMessage_initWithData(data: &ns::Data) -> arc::R<WebSocketMessage>;
    fn NSURLSessionWebSocketMessage_initWithString(data: &ns::String) -> arc::R<WebSocketMessage>;
    fn NSURLSessionWebSocketMessage_data(msg: &WebSocketMessage) -> Option<&ns::Data>;
    fn NSURLSessionWebSocketMessage_string(msg: &WebSocketMessage) -> Option<&ns::String>;
    fn NSURLSessionWebSocketMessage_type(msg: &WebSocketMessage) -> WebSocketMessageType;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let session = ns::URLSession::shared();
        println!("session: {:?}", session);
        let url = ns::URL::with_str("https://google.com").unwrap();
        let data_task = session.data_task_with_url(&url);
        assert!(data_task.error().is_none());
        assert_eq!(data_task.priority(), 0.5f32);
        assert_eq!(data_task.state(), ns::URLSessionTaskState::Suspended);
        data_task.resume();
        assert_eq!(data_task.state(), ns::URLSessionTaskState::Running);
    }
}
