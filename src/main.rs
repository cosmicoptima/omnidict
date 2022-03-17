#![feature(never_type)]

mod commands;
mod data;
mod discord;
mod language;
mod prelude;

use commands::handle_command;
use discord::{reply, send, send_raw, set_own_nickname, GENERAL_ID, OWN_ID};
use language::{dictum_prompt, qa_prompt};
use prelude::*;

use std::{fs, sync::Arc, time::Duration};

use futures_util::StreamExt;
use rand::{thread_rng, Rng};
use tokio::{sync::Mutex, time::sleep};
use twilight_gateway::{Event, Intents, Shard};
use twilight_http::Client as HttpClient;

async fn handle_event_inner(event: Event, context: Context) -> Res<()> {
    match event {
        Event::MessageCreate(msg) if msg.author.id != OWN_ID => {
            let msg = (*msg).0;
            handle_command(&context, &msg).await?;

            if thread_rng().gen_bool(0.02) {
                let output = qa_prompt(msg.content.as_str()).await?;
                reply(&context.http, msg.channel_id, msg.id, output.as_str()).await?;
            }
        }
        _ => (),
    }
    Ok(())
}

async fn handle_event(event: Event, context: Context) {
    let http = context.http.clone();
    if let Err(e) = handle_event_inner(event, context).await {
        let message = format!("```\n{:?}\n```", e);
        send_raw(&http, GENERAL_ID, message.as_str()).await.unwrap();
    }
}

async fn on_start_inner(http: Arc<HttpClient>) -> Res<()> {
    send(&http, GENERAL_ID, "Rise and shine, bitches").await?;
    set_own_nickname(&http, "omnidict").await?;
    Ok(())
}

async fn on_start(http: Arc<HttpClient>) {
    if let Err(e) = on_start_inner(http.clone()).await {
        let message = format!("```\n{:?}\n```", e);
        send_raw(&http, GENERAL_ID, message.as_str()).await.unwrap();
    }
}

async fn random_event_loop_inner(http: Arc<HttpClient>) -> Res<!> {
    // random dictation
    loop {
        if thread_rng().gen_bool(0.00002) {
            let output = dictum_prompt().await?;
            send(&http, GENERAL_ID, output.as_str()).await?;
        }

        sleep(Duration::from_millis(100)).await;
    }
}

async fn random_event_loop(http: Arc<HttpClient>) {
    loop {
        if let Err(e) = random_event_loop_inner(http.clone()).await {
            let message = format!("```\n{:?}\n```", e);
            send_raw(&http, GENERAL_ID, message.as_str()).await.unwrap();
        }
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
    tokio::spawn(random_event_loop(http.clone()));

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
