/// Convenient type alias of std's `Result`.
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for `tio`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The message is unable to be broadcasted.
    #[error("Unable to broadcast message")]
    CannotBroadcastMessage,

    /// The client for the IOTA node cannot be build.
    #[error("Unable to build client for node")]
    CannotBuildNodeClient,

    /// The node information is unable to be retrieved.
    #[error("Unable to retrieve the node information")]
    CannotGetNodeInfo,

    /// An (unknown) error occurred when executing some command.
    #[error("Something went wrong")]
    Generic,

    /// Specified IOTA network is invalid.
    #[error("\"{0}\" is not a valid network")]
    NetworkInvalid(String),

    /// The data within a message is invalid.
    #[error("The data in the message is not valid")]
    MessageDataInvalid,

    /// There is no data within a message.
    #[error("The message contains no data")]
    MessageEmpty,

    /// The hash of the message is invalid.
    #[error("\"{0}\" is not a valid message ID hash")]
    MessageHashInvalid(String),

    /// The message cannot be found given a hash ID.
    #[error("The message cannot be found")]
    MessageNotFound,

    /// The data contents of the message is too large.
    #[error("The message data must be < 4kb, found {0} bytes")]
    MessageDataTooLarge(usize),

    /// The data index of the message is too large.
    #[error("The message data index must be between 1-64 bytes long, found {0} bytes")]
    MessageDataIndexTooLarge(usize),

    /// The payload type of the message is wrong.
    #[error("The message's payload type is wrong (should be Indexation)")]
    MessageWrongPayload,
}
