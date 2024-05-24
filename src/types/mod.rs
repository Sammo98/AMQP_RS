pub mod message;
pub use message::*;

pub mod queue_definition;
pub use queue_definition::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
