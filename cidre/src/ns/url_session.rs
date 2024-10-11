use crate::{
    arc, define_cls, define_obj_type, ns,
    objc::{self, Class},
};

define_obj_type!(
    #[doc(alias = "NSURLSessionConfiguration")]
    pub Cfg(ns::Id)
);

define_obj_type!(
    #[doc(alias = "NSURLSessionTask")]
    pub Task(ns::Id)
);

define_obj_type!(
    #[doc(alias = "NSURLSessionDataTask")]
    pub DataTask(Task)
);

define_obj_type!(
    #[doc(alias = "NSURLSessionUploadTask")]
    pub UploadTask(DataTask)
);

define_obj_type!(
    #[doc(alias = "NSURLSessionDownloadTask")]
    pub DownloadTask(Task)
);

define_obj_type!(
    #[doc(alias = "NSURLSessionStreamTask")]
    pub StreamTask(Task)
);

define_obj_type!(
    #[doc(alias = "NSURLSessionWebSocketTask")]
    pub WebSocketTask(Task)
);

define_obj_type!(
    #[doc(alias = "NSURLSession")]
    pub Session(ns::Id)
);

impl Session {
    define_cls!(NS_URL_SESSION);
    /// ```
    /// use cidre::ns;
    ///
    /// let session = ns::UrlSession::shared();
    /// println!("session: {:?}", session);
    /// let url = ns::Url::with_str("https://google.com").unwrap();
    /// let data_task = session.data_task_with_url(&url);
    /// assert!(data_task.error().is_none());
    /// assert_eq!(data_task.priority(), 0.5f32);
    /// assert_eq!(data_task.state(), ns::UrlSessionTaskState::Suspended);
    /// data_task.resume();
    /// assert_eq!(data_task.state(), ns::UrlSessionTaskState::Running);
    /// ```
    #[objc::msg_send(sharedSession)]
    pub fn shared() -> &'static Session;

    #[objc::msg_send(dataTaskWithURL:)]
    pub fn data_task_with_url(&self, url: &ns::Url) -> arc::R<DataTask>;

    #[objc::msg_send(dataTaskWithRequest:)]
    pub fn data_task_with_request(&self, request: &ns::UrlRequest) -> arc::R<DataTask>;
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
    #[objc::msg_send(resume)]
    pub fn resume(&self);

    #[objc::msg_send(cancel)]
    pub fn cancel(&self);

    #[objc::msg_send(suspend)]
    pub fn suspend(&self);

    #[objc::msg_send(state)]
    pub fn state(&self) -> TaskState;

    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<&ns::Error>;

    #[objc::msg_send(taskIdentifier)]
    pub fn task_id(&self) -> ns::UInteger;

    #[objc::msg_send(originalRequest)]
    pub fn original_request(&self) -> Option<&ns::UrlRequest>;

    #[objc::msg_send(currentRequest)]
    pub fn current_request(&self) -> Option<&ns::UrlRequest>;

    #[objc::msg_send(response)]
    pub fn response(&self) -> Option<&ns::UrlResponse>;

    #[objc::msg_send(priority)]
    pub fn priority(&self) -> f32;

    #[objc::msg_send(setPriority:)]
    pub fn set_priority(&mut self, value: f32);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum WebSocketCloseCode {
    #[doc(alias = "NSURLSessionWebSocketCloseCodeInvalid")]
    Invalid = 0,

    #[doc(alias = "NSURLSessionWebSocketCloseCodeNormalClosure")]
    NormalClosure = 1000,

    #[doc(alias = "NSURLSessionWebSocketCloseCodeGoingAway")]
    GoingAway = 1001,

    #[doc(alias = "NSURLSessionWebSocketCloseCodeProtocolError")]
    ProtocolError = 1002,

    #[doc(alias = "NSURLSessionWebSocketCloseCodeUnsupportedData")]
    UnsupportedData = 1003,

    #[doc(alias = "NSURLSessionWebSocketCloseCodeNoStatusReceived")]
    NoStatusReceived = 1005,

    #[doc(alias = "NSURLSessionWebSocketCloseCodeAbnormalClosure")]
    AbnormalClosure = 1006,

    #[doc(alias = "NSURLSessionWebSocketCloseCodeInvalidFramePayloadData")]
    InvalidFramePayloadData = 1007,

    #[doc(alias = "NSURLSessionWebSocketCloseCodePolicyViolation")]
    PolicyViolation = 1008,

    #[doc(alias = "NSURLSessionWebSocketCloseCodeMessageTooBig")]
    MessageTooBig = 1009,

    #[doc(alias = "NSURLSessionWebSocketCloseCodeMandatoryExtensionMissing")]
    MandatoryExtensionMissing = 1010,

    #[doc(alias = "NSURLSessionWebSocketCloseCodeInternalServerError")]
    InternalServerError = 1011,

    #[doc(alias = "NSURLSessionWebSocketCloseCodeTLSHandshakeFailure")]
    TlsHandshakeFailure = 1015,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TaskPriority(pub f32);

impl TaskPriority {
    #[doc(alias = "NSURLSessionTaskPriorityDefault")]
    #[inline]
    pub fn default_priority() -> Self {
        unsafe { NSURLSessionTaskPriorityDefault }
    }

    #[doc(alias = "NSURLSessionTaskPriorityLow")]
    #[inline]
    pub fn low() -> Self {
        unsafe { NSURLSessionTaskPriorityLow }
    }

    #[doc(alias = "NSURLSessionTaskPriorityHigh")]
    #[inline]
    pub fn high() -> Self {
        unsafe { NSURLSessionTaskPriorityHigh }
    }
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::default_priority()
    }
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    static NSURLSessionTaskPriorityDefault: TaskPriority;
    static NSURLSessionTaskPriorityLow: TaskPriority;
    static NSURLSessionTaskPriorityHigh: TaskPriority;
}

#[doc(alias = "NSURLSessionWebSocketMessageType")]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum WebSocketMessageType {
    Data = 0,
    String = 1,
}

define_obj_type!(
    #[doc(alias = "NSURLSessionWebSocketMessage")]
    pub WebSocketMessage(ns::Id)
);

impl arc::A<WebSocketMessage> {
    #[objc::msg_send(initWithData:)]
    pub fn init_with_data(self, data: &ns::Data) -> arc::R<WebSocketMessage>;

    #[objc::msg_send(initWithString:)]
    pub fn init_with_string(self, string: &ns::String) -> arc::R<WebSocketMessage>;
}

impl WebSocketMessage {
    define_cls!(NS_URL_SESSION_WEB_SOCKET_MESSAGE);

    #[inline]
    pub fn with_data(data: &ns::Data) -> arc::R<Self> {
        Self::alloc().init_with_data(data)
    }

    #[inline]
    pub fn with_string(string: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_string(string)
    }

    #[objc::msg_send(data)]
    pub fn data(&self) -> Option<&ns::Data>;

    #[objc::msg_send(string)]
    pub fn string(&self) -> Option<&ns::String>;

    #[objc::msg_send(type)]
    pub fn type_(&self) -> WebSocketMessageType;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_URL_SESSION: &'static Class<Session>;
    static NS_URL_SESSION_WEB_SOCKET_MESSAGE: &'static Class<WebSocketMessage>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let session = ns::UrlSession::shared();
        println!("session: {:?}", session);
        let url = ns::Url::with_str("https://google.com").unwrap();
        let data_task = session.data_task_with_url(&url);
        assert!(data_task.error().is_none());
        assert_eq!(data_task.priority(), 0.5f32);
        assert_eq!(data_task.state(), ns::UrlSessionTaskState::Suspended);
        data_task.resume();
        assert_eq!(data_task.state(), ns::UrlSessionTaskState::Running);
    }
}
