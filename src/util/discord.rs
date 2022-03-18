use anyhow::Result;
use twilight_http::Client as HttpClient;
use twilight_model::channel::embed::{Embed, EmbedField};
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
    Id,
};

pub fn embed_field(name: &str, value: &str) -> EmbedField {
    EmbedField {
        inline: false,
        name: name.to_string(),
        value: value.to_string(),
    }
}

pub async fn send_raw(
    http: &HttpClient,
    channel_id: Id<ChannelMarker>,
    content: &str,
) -> Result<()> {
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
) -> Result<()> {
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
) -> Result<()> {
    http.create_message(channel_id)
        .reply(in_reply_to)
        .embeds(&[embed.clone()])?
        .exec()
        .await?;
    Ok(())
}

// dict-specific
// =============

pub const PNPPC_ID: Id<GuildMarker> = Id::new(878376227428245555);
pub const GENERAL_ID: Id<ChannelMarker> = Id::new(878376227428245558);
pub const OWN_ID: Id<UserMarker> = Id::new(950988810697736192); // lmao

fn voice_filter(string: &str) -> String {
    format!("**__{}__**", string.trim().to_uppercase())
}

pub async fn reply(
    http: &HttpClient,
    channel_id: Id<ChannelMarker>,
    in_reply_to: Id<MessageMarker>,
    content: &str,
) -> Result<()> {
    reply_raw(http, channel_id, in_reply_to, &voice_filter(content)).await
}

pub async fn send(http: &HttpClient, channel_id: Id<ChannelMarker>, content: &str) -> Result<()> {
    send_raw(http, channel_id, &voice_filter(content)).await
}

pub async fn set_own_nickname(http: &HttpClient, name: &str) -> Result<()> {
    http.update_current_member(PNPPC_ID)
        .nick(Some(name))?
        .exec()
        .await?;
    Ok(())
}

pub async fn post_error(res: anyhow::Result<()>, http: &HttpClient) {
    if let Err(e) = res {
        let message = format!("```\n{:?}\n```", e);
        send_raw(http, GENERAL_ID, &message).await.unwrap();
    }
}
