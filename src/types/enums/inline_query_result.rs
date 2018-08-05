use super::super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum InlineQueryResult {
    InlineQueryResultCachedAudio(InlineQueryResultCachedAudio),
    InlineQueryResultCachedDocument(InlineQueryResultCachedDocument),
    InlineQueryResultCachedGif(InlineQueryResultCachedGif),
    InlineQueryResultCachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    InlineQueryResultCachedPhoto(InlineQueryResultCachedPhoto),
    InlineQueryResultCachedSticker(InlineQueryResultCachedSticker),
    InlineQueryResultCachedVideo(InlineQueryResultCachedVideo),
    InlineQueryResultCachedVoice(InlineQueryResultCachedVoice),
    InlineQueryResultArticle(InlineQueryResultArticle),
    InlineQueryResultAudio(InlineQueryResultAudio),
    InlineQueryResultContact(InlineQueryResultContact),
    InlineQueryResultGame(InlineQueryResultGame),
    InlineQueryResultDocument(InlineQueryResultDocument),
    InlineQueryResultGif(InlineQueryResultGif),
    InlineQueryResultLocation(InlineQueryResultLocation),
    InlineQueryResultMpeg4Gif(InlineQueryResultMpeg4Gif),
    InlineQueryResultPhoto(InlineQueryResultPhoto),
    InlineQueryResultVenue(InlineQueryResultVenue),
    InlineQueryResultVideo(InlineQueryResultVideo),
    InlineQueryResultVoice(InlineQueryResultVoice),
}