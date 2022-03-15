mod commands;
mod discord;
mod language;
mod prelude;

use commands::handle_command;
use discord::OWN_ID;
use prelude::*;

use futures_util::StreamExt;
use std::{fs, sync::Arc};
use twilight_gateway::{Event, Intents, Shard};
use twilight_http::Client as HttpClient;

async fn handle_event_inner(event: Event, http: Arc<HttpClient>) -> Res<()> {
    match event {
        Event::MessageCreate(msg) if msg.author.id != OWN_ID => {
            handle_command(&http, (*msg).0).await?;
        }
        _ => (),
    }
    Ok(())
}

async fn handle_event(event: Event, http: Arc<HttpClient>) {
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
