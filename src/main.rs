mod commands;
mod discord;
mod language;
mod prelude;

use commands::handle_command;
use discord::{reply, set_own_nickname, OWN_ID};
use language::qa_prompt;
use prelude::*;

use std::{fs, sync::Arc};

use futures_util::StreamExt;
use rand::{thread_rng, Rng};
use tokio::sync::Mutex;
use twilight_gateway::{Event, Intents, Shard};
use twilight_http::Client as HttpClient;

async fn handle_event_inner(event: Event, context: Context) -> Res<()> {
    match event {
        Event::MessageCreate(msg) if msg.author.id != OWN_ID => {
            let msg = (*msg).0;
            handle_command(&context, &msg).await?;

            if thread_rng().gen_bool(0.01) {
                let output = qa_prompt(msg.content.as_str()).await?;
                reply(&context.http, msg.channel_id, msg.id, output.as_str()).await?;
            }
        }
        _ => (),
    }
    Ok(())
}

async fn handle_event(event: Event, context: Context) {
    if let Err(e) = handle_event_inner(event, context).await {
        eprintln!("{}", e);
    }
}

async fn on_start_inner(http: Arc<HttpClient>) -> Res<()> {
    set_own_nickname(&http, "omnidict").await
}

async fn on_start(http: Arc<HttpClient>) {
    if let Err(e) = on_start_inner(http).await {
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

    let redis = redis::Client::open("redis://127.0.0.1")?;
    let conn = Arc::new(Mutex::new(redis.get_connection()?));

    tokio::spawn(on_start(http.clone()));

    let context = Context {
        http,
        conn,
        shard: Arc::new(shard),
    };

    while let Some(event) = events.next().await {
        tokio::spawn(handle_event(event, context.clone()));
    }

    Ok(())
}
