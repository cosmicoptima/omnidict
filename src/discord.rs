use crate::prelude::*;

use twilight_http::Client as HttpClient;
use twilight_model::{
    channel::embed::{Embed, EmbedField},
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
};

pub fn embed(title: &str, description: &str) -> Embed {
    Embed {
        author: None,
        color: None,
        description: Some(description.to_string()),
        fields: vec![],
        footer: None,
        image: None,
        kind: "rich".to_string(),
        provider: None,
        thumbnail: None,
        timestamp: None,
        title: Some(title.to_string()),
        url: None,
        video: None,
    }
}

pub fn embed_fields(
    title: Option<&str>,
    description: Option<&str>,
    fields: Vec<(&str, &str)>,
) -> Embed {
    Embed {
        author: None,
        color: None,
        description: description.map(String::from),
        fields: fields
            .into_iter()
            .map(|(title, value)| EmbedField {
                inline: false,
                name: String::from(title),
                value: String::from(value),
            })
            .collect(),
        footer: None,
        image: None,
        kind: "rich".to_string(),
        provider: None,
        thumbnail: None,
        timestamp: None,
        title: title.map(String::from),
        url: None,
        video: None,
    }
}

pub async fn send_raw(http: &HttpClient, channel_id: Id<ChannelMarker>, content: &str) -> Res<()> {
    http.create_message(channel_id)
        .content(content)?
        .exec()
        .await?;
    Ok(())
}

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

pub async fn reply_embed(
    http: &HttpClient,
    channel_id: Id<ChannelMarker>,
    in_reply_to: Id<MessageMarker>,
    embed: &Embed,
) -> Res<()> {
    http.create_message(channel_id)
        .reply(in_reply_to)
        .embeds(&[embed.clone()])?
        .exec()
        .await?;
    Ok(())
}

// dict-specific
// =============

pub const GENERAL_ID: Id<ChannelMarker> = Id::new(878376227428245558);
pub const OWN_ID: Id<UserMarker> = Id::new(950988810697736192); // lmao
const PNPPC_ID: Id<GuildMarker> = Id::new(878376227428245555);

fn voice_filter(string: &str) -> String {
    format!("**__{}__**", string.trim().to_uppercase())
}

pub async fn reply(
    http: &HttpClient,
    channel_id: Id<ChannelMarker>,
    in_reply_to: Id<MessageMarker>,
    content: &str,
) -> Res<()> {
    reply_raw(http, channel_id, in_reply_to, &voice_filter(content)).await
}

pub async fn send(http: &HttpClient, channel_id: Id<ChannelMarker>, content: &str) -> Res<()> {
    send_raw(http, channel_id, &voice_filter(content)).await
}

pub async fn set_own_nickname(http: &HttpClient, name: &str) -> Res<()> {
    http.update_current_member(PNPPC_ID)
        .nick(Some(name))?
        .exec()
        .await?;
    Ok(())
}
