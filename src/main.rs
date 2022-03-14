use futures_util::StreamExt;
use std::{error::Error, fs, sync::Arc};
use twilight_gateway::{Event, Intents, Shard};
use twilight_http::Client as HttpClient;
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker, UserMarker},
    Id,
};

type E = Box<dyn Error + Send + Sync>;
type Res<T> = Result<T, E>;

const OWN_ID: Id<UserMarker> = Id::new(950988810697736192);

// discord wrappers

// async fn send_raw(http: &HttpClient, channel_id: Id<ChannelMarker>, content: &str) -> Res<()> {
//     http.create_message(channel_id)
//         .content(content)?
//         .exec()
//         .await?;
//     Ok(())
// }

async fn reply_raw(
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

// other

fn voice_filter(string: &str) -> String {
    format!("**__{}__**", string.to_uppercase())
}

// async fn send(http: &HttpClient, channel_id: Id<ChannelMarker>, content: &str) -> Res<()> {
//     send_raw(http, channel_id, &voice_filter(content)).await
// }

async fn reply(
    http: &HttpClient,
    channel_id: Id<ChannelMarker>,
    in_reply_to: Id<MessageMarker>,
    content: &str,
) -> Res<()> {
    reply_raw(http, channel_id, in_reply_to, &voice_filter(content)).await
}

// main

async fn handle_event(event: Event, http: Arc<HttpClient>) -> Res<()> {
    match event {
        Event::MessageCreate(msg) if msg.author.id != OWN_ID => {
            if msg.content == "gm" {
                reply(&http, msg.channel_id, msg.id, "gm motherfucker").await?;
            }
        }
        _ => (),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Res<()> {
    let token = fs::read_to_string("token.txt")
        .expect("token.txt not found")
        .trim_end()
        .to_string();

    let http = Arc::new(HttpClient::new(token.clone()));

    let intents = Intents::GUILD_MESSAGES;
    let (shard, mut events) = Shard::new(token, intents);
    shard.start().await?;

    while let Some(event) = events.next().await {
        tokio::spawn(handle_event(event, http.clone()));
    }

    Ok(())
}
