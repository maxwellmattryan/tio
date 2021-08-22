/// Convenient type alias of std's `Result` 
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for tio
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The message is unable to be broadcasted
    #[error("Unable to broadcast message")]
    CannotBroadcastMessage,

    /// The client for the IOTA node cannot be build
    #[error("Unable to build client for node")]
    CannotBuildNodeClient,

    /// The node information is unable to be retrieved
    #[error("Unable to retrieve the node information")]
    CannotGetNodeInfo,

    /// An (unknown) error occurred when executing some command
    #[error("Something went wrong")]
    Generic,

    /// Specified IOTA network is invalid
    #[error("\"{0}\" is not a valid network")]
    NetworkInvalid(String),

    /// The size of the content to send is too large
    #[error("{0} must be < 4kb, found {1} bytes")]
    MessageTooLarge(String, usize),
}
