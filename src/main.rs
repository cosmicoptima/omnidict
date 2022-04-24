use finalfusion::{compat::fasttext::ReadFastText, embeddings::Embeddings};
use futures_util::StreamExt;
use omnidict::*;
use std::io::BufReader;
use std::sync::Arc;
use std::{fs, fs::File};
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

    let mut reader = BufReader::new(File::open("data/crawl-300d-2M.vec")?);
    let embeddings = Embeddings::read_fasttext(&mut reader)?;

    util::discord::post_error(pfc::handle_start(http.clone()).await, &http.clone()).await;
    tokio::spawn(pfc::catastrophe(http.clone()));

    let ctx = prelude::Context {
        http,
        db,
        shard: Arc::new(shard),
        embeddings: Arc::new(embeddings),
    };
    while let Some(event) = events.next().await {
        tokio::spawn(pfc::handle_gateway_event(event, ctx.clone()));
    }

    Ok(())
}
