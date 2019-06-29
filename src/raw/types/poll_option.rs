use crate::types::*;

/// This object contains information about one answer option in a poll.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub(crate) text: String,
    /// Number of users that voted for this option
    pub(crate) voter_count: Integer,
}