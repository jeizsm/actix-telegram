use crate::types::*;

/// Use this method to delete a message, including service messages, with the following limitations:- A message can only be deleted if it was sent less than 48 hours ago.- Bots can delete outgoing messages in private chats, groups, and supergroups.- Bots can delete incoming messages in private chats.- Bots granted can_post_messages permissions can delete outgoing messages in channels.- If the bot is an administrator of a group, it can delete any message there.- If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.Returns True on success.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct DeleteMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub(crate) chat_id: ChatIdOrUsername,
    /// Identifier of the message to delete
    pub(crate) message_id: Integer,
}