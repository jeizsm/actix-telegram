use crate::types::*;

/// This object represents a message.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Message {
    /// Unique message identifier inside this chat
    pub(crate) message_id: Integer,
    /// Sender, empty for messages sent to channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) from: Option<User>,
    /// Date the message was sent in Unix time
    pub(crate) date: Integer,
    /// Conversation the message belongs to
    pub(crate) chat: Chat,
    /// For forwarded messages, sender of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) forward_from: Option<User>,
    /// For messages forwarded from channels, information about the original channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) forward_from_chat: Option<Chat>,
    /// For messages forwarded from channels, identifier of the original message in the channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) forward_from_message_id: Option<Integer>,
    /// For messages forwarded from channels, signature of the post author if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) forward_signature: Option<String>,
    /// Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) forward_sender_name: Option<String>,
    /// For forwarded messages, date the original message was sent in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) forward_date: Option<Integer>,
    /// For replies, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_to_message: Option<Box<Message>>,
    /// Date the message was last edited in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) edit_date: Option<Integer>,
    /// The unique identifier of a media message group this message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) media_group_id: Option<String>,
    /// Signature of the post author for messages in channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) entities: Option<Vec<MessageEntity>>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption_entities: Option<Vec<MessageEntity>>,
    /// Message is an audio file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) audio: Option<Audio>,
    /// Message is a general file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) document: Option<Document>,
    /// Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) animation: Option<Animation>,
    /// Message is a game, information about the game. More about games »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) game: Option<Game>,
    /// Message is a photo, available sizes of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sticker: Option<Sticker>,
    /// Message is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) video: Option<Video>,
    /// Message is a voice message, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) voice: Option<Voice>,
    /// Message is a video note, information about the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) video_note: Option<VideoNote>,
    /// Caption for the animation, audio, document, photo, video or voice, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) caption: Option<String>,
    /// Message is a shared contact, information about the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) contact: Option<Contact>,
    /// Message is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) location: Option<Location>,
    /// Message is a venue, information about the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) venue: Option<Venue>,
    /// Message is a native poll, information about the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) poll: Option<Poll>,
    /// New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) new_chat_members: Option<Vec<User>>,
    /// A member was removed from the group, information about them (this member may be the bot itself)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) left_chat_member: Option<User>,
    /// A chat title was changed to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) new_chat_title: Option<String>,
    /// A chat photo was change to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) new_chat_photo: Option<Vec<PhotoSize>>,
    /// Service message: the chat photo was deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) delete_chat_photo: Option<True>,
    /// Service message: the group has been created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) group_chat_created: Option<True>,
    /// Service message: the supergroup has been created. This field can‘t be received in a message coming through updates, because bot can’t be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) supergroup_chat_created: Option<True>,
    /// Service message: the channel has been created. This field can‘t be received in a message coming through updates, because bot can’t be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) channel_chat_created: Option<True>,
    /// The group has been migrated to a supergroup with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) migrate_to_chat_id: Option<Integer>,
    /// The supergroup has been migrated from a group with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) migrate_from_chat_id: Option<Integer>,
    /// Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it is itself a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pinned_message: Option<Box<Message>>,
    /// Message is an invoice for a payment, information about the invoice. More about payments »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) invoice: Option<Invoice>,
    /// Message is a service message about a successful payment, information about the payment. More about payments »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) successful_payment: Option<SuccessfulPayment>,
    /// The domain name of the website on which the user has logged in. More about Telegram Login »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) connected_website: Option<String>,
    /// Telegram Passport data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) passport_data: Option<PassportData>,
    /// Inline keyboard attached to the message. login_url buttons are represented as ordinary url buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
}