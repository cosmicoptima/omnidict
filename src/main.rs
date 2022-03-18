#![feature(never_type)]
#![feature(try_blocks)]

pub mod brain;
pub mod data;
pub mod discord;
pub mod language;
pub mod prelude;

use futures_util::StreamExt;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;
use twilight_gateway::{Intents, Shard};
use twilight_http::Client as HttpClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = fs::read_to_string("token.txt")
        .expect("token.txt not found")
        .trim_end()
        .to_string();

    let http = Arc::new(HttpClient::new(token.clone()));

    let intents = Intents::GUILD_MESSAGES;
    let (shard, mut events) = Shard::new(token, intents);
    shard.start().await?;

    let redis = redis::Client::open("redis://127.0.0.1")?;
    let db = Arc::new(Mutex::new(redis.get_connection()?));

    discord::post_error(brain::handle_start(http.clone()).await, &http.clone()).await;
    tokio::spawn(brain::catastrophe(http.clone()));

    let ctx = prelude::Context {
        http,
        db,
        shard: Arc::new(shard),
    };
    while let Some(event) = events.next().await {
        tokio::spawn(brain::handle_gateway_event(event, ctx.clone()));
    }

    Ok(())
}
