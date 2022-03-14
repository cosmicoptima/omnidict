use futures_util::StreamExt;
use std::{error::Error, fs, sync::Arc};
use twilight_gateway::{Event, Intents, Shard};
use twilight_http::Client as HttpClient;
use twilight_model::id::{marker::UserMarker, Id};

type E = Box<dyn Error + Send + Sync>;

const OWN_ID: Id<UserMarker> = Id::new(950988810697736192);

async fn handle_event(event: Event, http: Arc<HttpClient>) -> Result<(), E> {
    match event {
        Event::MessageCreate(msg) if msg.author.id != OWN_ID => {
            if msg.content == "gm" {
                http.create_message(msg.channel_id)
                    .reply(msg.id)
                    .content("__**GM, MOTHERFUCKER**__")?
                    .exec()
                    .await?;
            }
        }
        _ => (),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), E> {
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
