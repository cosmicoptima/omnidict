use crate::prelude::*;

use twilight_http::Client as HttpClient;
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker, UserMarker},
    Id,
};

/*
async fn send_raw(http: &HttpClient, channel_id: Id<ChannelMarker>, content: &str) -> Res<()> {
    http.create_message(channel_id)
        .content(content)?
        .exec()
        .await?;
    Ok(())
}
*/

pub async fn reply_raw(
    http: &HttpClient,
    channel_id: Id<ChannelMarker>,
    in_reply_to: Id<MessageMarker>,
    content: &str,
) -> Res<()> {
    http.create_message(channel_id)
        .reply(in_reply_to)
        .content(content)?
        .exec()
        .await?;
    Ok(())
}

// dict-specific
// =============

pub const OWN_ID: Id<UserMarker> = Id::new(950988810697736192); // lmao

fn voice_filter(string: &str) -> String {
    format!("**__{}__**", string.trim().to_uppercase())
}

/*
pub async fn send(http: &HttpClient, channel_id: Id<ChannelMarker>, content: &str) -> Res<()> {
    send_raw(http, channel_id, &voice_filter(content)).await
}
*/

pub async fn reply(
    http: &HttpClient,
    channel_id: Id<ChannelMarker>,
    in_reply_to: Id<MessageMarker>,
    content: &str,
) -> Res<()> {
    reply_raw(http, channel_id, in_reply_to, &voice_filter(content)).await
}
