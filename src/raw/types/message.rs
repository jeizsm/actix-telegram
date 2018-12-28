use crate::types::*;

/// This object represents a message.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct Message {
    /// Unique message identifier inside this chat
    message_id: Integer,
    /// Sender, empty for messages sent to channels
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<User>,
    /// Date the message was sent in Unix time
    date: Integer,
    /// Conversation the message belongs to
    chat: Chat,
    /// For forwarded messages, sender of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_from: Option<User>,
    /// For messages forwarded from channels, information about the original channel
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_from_chat: Option<Chat>,
    /// For messages forwarded from channels, identifier of the original message in the channel
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_from_message_id: Option<Integer>,
    /// For messages forwarded from channels, signature of the post author if present
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_signature: Option<String>,
    /// For forwarded messages, date the original message was sent in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_date: Option<Integer>,
    /// For replies, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message: Option<Box<Message>>,
    /// Date the message was last edited in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    edit_date: Option<Integer>,
    /// The unique identifier of a media message group this message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    media_group_id: Option<String>,
    /// Signature of the post author for messages in channels
    #[serde(skip_serializing_if = "Option::is_none")]
    author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<MessageEntity>>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<MessageEntity>>,
    /// Message is an audio file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<Audio>,
    /// Message is a general file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    document: Option<Document>,
    /// Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    animation: Option<Animation>,
    /// Message is a game, information about the game. More about games »
    #[serde(skip_serializing_if = "Option::is_none")]
    game: Option<Game>,
    /// Message is a photo, available sizes of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    sticker: Option<Sticker>,
    /// Message is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<Video>,
    /// Message is a voice message, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    voice: Option<Voice>,
    /// Message is a video note, information about the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    video_note: Option<VideoNote>,
    /// Caption for the audio, document, photo, video or voice, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    /// Message is a shared contact, information about the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<Contact>,
    /// Message is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<Location>,
    /// Message is a venue, information about the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    venue: Option<Venue>,
    /// New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[serde(skip_serializing_if = "Option::is_none")]
    new_chat_members: Option<Vec<User>>,
    /// A member was removed from the group, information about them (this member may be the bot itself)
    #[serde(skip_serializing_if = "Option::is_none")]
    left_chat_member: Option<User>,
    /// A chat title was changed to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    new_chat_title: Option<String>,
    /// A chat photo was change to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    new_chat_photo: Option<Vec<PhotoSize>>,
    /// Service message: the chat photo was deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_chat_photo: Option<True>,
    /// Service message: the group has been created
    #[serde(skip_serializing_if = "Option::is_none")]
    group_chat_created: Option<True>,
    /// Service message: the supergroup has been created. This field can‘t be received in a message coming through updates, because bot can’t be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    supergroup_chat_created: Option<True>,
    /// Service message: the channel has been created. This field can‘t be received in a message coming through updates, because bot can’t be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_chat_created: Option<True>,
    /// The group has been migrated to a supergroup with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    migrate_to_chat_id: Option<Integer>,
    /// The supergroup has been migrated from a group with the specified identifier. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    migrate_from_chat_id: Option<Integer>,
    /// Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it is itself a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pinned_message: Option<Box<Message>>,
    /// Message is an invoice for a payment, information about the invoice. More about payments »
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice: Option<Invoice>,
    /// Message is a service message about a successful payment, information about the payment. More about payments »
    #[serde(skip_serializing_if = "Option::is_none")]
    successful_payment: Option<SuccessfulPayment>,
    /// The domain name of the website on which the user has logged in. More about Telegram Login »
    #[serde(skip_serializing_if = "Option::is_none")]
    connected_website: Option<String>,
    /// Telegram Passport data
    #[serde(skip_serializing_if = "Option::is_none")]
    passport_data: Option<PassportData>,
}
