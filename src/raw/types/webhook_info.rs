use crate::types::*;

/// Contains information about the current status of a webhook.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub(crate) url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub(crate) has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub(crate) pending_update_count: Integer,
    /// Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) last_error_date: Option<Integer>,
    /// Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) last_error_message: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_connections: Option<Integer>,
    /// A list of update types the bot is subscribed to. Defaults to all update types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_updates: Option<Vec<String>>,
}
