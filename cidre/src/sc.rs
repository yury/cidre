pub mod stream;

pub use stream::Configuration as StreamConfiguration;
pub use stream::ContentFilter;
pub use stream::Delegate as StreamDelegate;
pub use stream::DelegateImpl as StreamDelegateImpl;
pub use stream::FrameInfo;
pub use stream::FrameStatus;
pub use stream::Output as StreamOutputImpl;
pub use stream::Output as StreamOutput;
pub use stream::OutputType;
pub use stream::Stream;

pub mod shareable_content;
pub use shareable_content::Display;
pub use shareable_content::RunningApplication;
pub use shareable_content::ShareableContent;
pub use shareable_content::Window;

pub mod error;
