mod update;
pub use self::update::Update;
mod webhook_info;
pub use self::webhook_info::WebhookInfo;
mod user;
pub use self::user::User;
mod chat;
pub use self::chat::Chat;
mod message;
pub use self::message::Message;
mod message_entity;
pub use self::message_entity::MessageEntity;
mod photo_size;
pub use self::photo_size::PhotoSize;
mod audio;
pub use self::audio::Audio;
mod document;
pub use self::document::Document;
mod video;
pub use self::video::Video;
mod animation;
pub use self::animation::Animation;
mod voice;
pub use self::voice::Voice;
mod video_note;
pub use self::video_note::VideoNote;
mod contact;
pub use self::contact::Contact;
mod location;
pub use self::location::Location;
mod venue;
pub use self::venue::Venue;
mod user_profile_photos;
pub use self::user_profile_photos::UserProfilePhotos;
mod file;
pub use self::file::File;
mod reply_keyboard_markup;
pub use self::reply_keyboard_markup::ReplyKeyboardMarkup;
mod keyboard_button;
pub use self::keyboard_button::KeyboardButton;
mod reply_keyboard_remove;
pub use self::reply_keyboard_remove::ReplyKeyboardRemove;
mod inline_keyboard_markup;
pub use self::inline_keyboard_markup::InlineKeyboardMarkup;
mod inline_keyboard_button;
pub use self::inline_keyboard_button::InlineKeyboardButton;
mod callback_query;
pub use self::callback_query::CallbackQuery;
mod force_reply;
pub use self::force_reply::ForceReply;
mod chat_photo;
pub use self::chat_photo::ChatPhoto;
mod chat_member;
pub use self::chat_member::ChatMember;
mod response_parameters;
pub use self::response_parameters::ResponseParameters;
mod input_media_photo;
pub use self::input_media_photo::InputMediaPhoto;
mod input_media_video;
pub use self::input_media_video::InputMediaVideo;
mod input_media_animation;
pub use self::input_media_animation::InputMediaAnimation;
mod input_media_audio;
pub use self::input_media_audio::InputMediaAudio;
mod input_media_document;
pub use self::input_media_document::InputMediaDocument;
mod sticker;
pub use self::sticker::Sticker;
mod sticker_set;
pub use self::sticker_set::StickerSet;
mod mask_position;
pub use self::mask_position::MaskPosition;
mod inline_query;
pub use self::inline_query::InlineQuery;
mod inline_query_result_article;
pub use self::inline_query_result_article::InlineQueryResultArticle;
mod inline_query_result_photo;
pub use self::inline_query_result_photo::InlineQueryResultPhoto;
mod inline_query_result_gif;
pub use self::inline_query_result_gif::InlineQueryResultGif;
mod inline_query_result_mpeg4_gif;
pub use self::inline_query_result_mpeg4_gif::InlineQueryResultMpeg4Gif;
mod inline_query_result_video;
pub use self::inline_query_result_video::InlineQueryResultVideo;
mod inline_query_result_audio;
pub use self::inline_query_result_audio::InlineQueryResultAudio;
mod inline_query_result_voice;
pub use self::inline_query_result_voice::InlineQueryResultVoice;
mod inline_query_result_document;
pub use self::inline_query_result_document::InlineQueryResultDocument;
mod inline_query_result_location;
pub use self::inline_query_result_location::InlineQueryResultLocation;
mod inline_query_result_venue;
pub use self::inline_query_result_venue::InlineQueryResultVenue;
mod inline_query_result_contact;
pub use self::inline_query_result_contact::InlineQueryResultContact;
mod inline_query_result_game;
pub use self::inline_query_result_game::InlineQueryResultGame;
mod inline_query_result_cached_photo;
pub use self::inline_query_result_cached_photo::InlineQueryResultCachedPhoto;
mod inline_query_result_cached_gif;
pub use self::inline_query_result_cached_gif::InlineQueryResultCachedGif;
mod inline_query_result_cached_mpeg4_gif;
pub use self::inline_query_result_cached_mpeg4_gif::InlineQueryResultCachedMpeg4Gif;
mod inline_query_result_cached_sticker;
pub use self::inline_query_result_cached_sticker::InlineQueryResultCachedSticker;
mod inline_query_result_cached_document;
pub use self::inline_query_result_cached_document::InlineQueryResultCachedDocument;
mod inline_query_result_cached_video;
pub use self::inline_query_result_cached_video::InlineQueryResultCachedVideo;
mod inline_query_result_cached_voice;
pub use self::inline_query_result_cached_voice::InlineQueryResultCachedVoice;
mod inline_query_result_cached_audio;
pub use self::inline_query_result_cached_audio::InlineQueryResultCachedAudio;
mod input_text_message_content;
pub use self::input_text_message_content::InputTextMessageContent;
mod input_location_message_content;
pub use self::input_location_message_content::InputLocationMessageContent;
mod input_venue_message_content;
pub use self::input_venue_message_content::InputVenueMessageContent;
mod input_contact_message_content;
pub use self::input_contact_message_content::InputContactMessageContent;
mod chosen_inline_result;
pub use self::chosen_inline_result::ChosenInlineResult;
mod labeled_price;
pub use self::labeled_price::LabeledPrice;
mod invoice;
pub use self::invoice::Invoice;
mod shipping_address;
pub use self::shipping_address::ShippingAddress;
mod order_info;
pub use self::order_info::OrderInfo;
mod shipping_option;
pub use self::shipping_option::ShippingOption;
mod successful_payment;
pub use self::successful_payment::SuccessfulPayment;
mod shipping_query;
pub use self::shipping_query::ShippingQuery;
mod pre_checkout_query;
pub use self::pre_checkout_query::PreCheckoutQuery;
mod passport_data;
pub use self::passport_data::PassportData;
mod passport_file;
pub use self::passport_file::PassportFile;
mod encrypted_passport_element;
pub use self::encrypted_passport_element::EncryptedPassportElement;
mod encrypted_credentials;
pub use self::encrypted_credentials::EncryptedCredentials;
mod passport_element_error_data_field;
pub use self::passport_element_error_data_field::PassportElementErrorDataField;
mod passport_element_error_front_side;
pub use self::passport_element_error_front_side::PassportElementErrorFrontSide;
mod passport_element_error_reverse_side;
pub use self::passport_element_error_reverse_side::PassportElementErrorReverseSide;
mod passport_element_error_selfie;
pub use self::passport_element_error_selfie::PassportElementErrorSelfie;
mod passport_element_error_file;
pub use self::passport_element_error_file::PassportElementErrorFile;
mod passport_element_error_files;
pub use self::passport_element_error_files::PassportElementErrorFiles;
mod game;
pub use self::game::Game;
mod game_high_score;
pub use self::game_high_score::GameHighScore;
pub mod enums;
pub use self::enums::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Integer(i64);

#[derive(Serialize, Deserialize, Debug)]
pub struct True(bool);

#[derive(Serialize, Deserialize, Debug)]
pub struct Float(f64);

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackGame;

#[derive(Serialize, Deserialize, Debug)]
pub struct InputFile(String);