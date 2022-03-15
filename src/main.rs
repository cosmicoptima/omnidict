mod discord;
mod language;
mod prelude;

use discord::{reply, OWN_ID};
use language::{complete_prompt, qa_prompt};
use prelude::*;

use futures_util::StreamExt;
use std::{fs, sync::Arc};
use twilight_gateway::{Event, Intents, Shard};
use twilight_http::Client as HttpClient;

async fn handle_event_inner(event: Event, http: Arc<HttpClient>) -> Res<()> {
    match event {
        Event::MessageCreate(msg) if msg.author.id != OWN_ID => {
            if msg.content == "gm" {
                reply(&http, msg.channel_id, msg.id, "gm motherfucker").await?;
            /*
            } else if let Some(prompt) = msg.content.strip_prefix("j1test ") {
                reply(
                    &http,
                    msg.channel_id,
                    msg.id,
                    get_j1(prompt, vec!["\n"]).await?.as_str(),
                )
                .await?;
            */
            } else if let Some(question) = msg.content.strip_prefix("dict, ") {
                let output = complete_prompt(qa_prompt(), vec![("question", question)]).await?;
                reply(&http, msg.channel_id, msg.id, output.as_str()).await?;
            }
        }
        _ => (),
    }
    Ok(())
}

async fn handle_event(event: Event, http: Arc<HttpClient>) -> () {
    if let Err(e) = handle_event_inner(event, http).await {
        eprintln!("{}", e);
    }
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
