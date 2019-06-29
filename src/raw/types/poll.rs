use crate::types::*;

/// This object contains information about a poll.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Poll {
    /// Unique poll identifier
    pub(crate) id: String,
    /// Poll question, 1-255 characters
    pub(crate) question: String,
    /// List of poll options
    pub(crate) options: Vec<PollOption>,
    /// True, if the poll is closed
    pub(crate) is_closed: bool,
}