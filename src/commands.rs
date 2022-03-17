use crate::discord::{embed, reply, reply_embed, send};
use crate::language::{dictum_prompt, gender_prompt, qa_prompt};
use crate::prelude::*;

use redis::Commands;
use twilight_model::channel::message::Message;

pub async fn handle_command(context: &Context, msg: &Message) -> Res<bool> {
    let content = &msg.content;
    let http = &context.http;

    // bedtime!
    if content == "bedtime!" {
        send(http, msg.channel_id, "i'm so tired...").await?;
        context.shard.shutdown();
    }

    // gm
    if content == "gm" {
        reply(http, msg.channel_id, msg.id, "gm motherfucker").await?;
        return Ok(true);
    }

    // questions to dict
    if let Some(question) = content.strip_prefix("dict, ") {
        let output = qa_prompt(question).await?;
        reply(&http, msg.channel_id, msg.id, output.as_str()).await?;
        return Ok(true);
    }

    // requests for dict's latest dictum
    if content == "what is your latest dictum" {
        let output = dictum_prompt().await?;
        reply(&http, msg.channel_id, msg.id, output.as_str()).await?;
        return Ok(true);
    }

    // requests for one's current gender
    if content == "what is my current gender" {
        let mut conn = context.conn.lock().await;
        let gender: String = conn.get(format!("users:{}:gender", msg.author.id))?;

        reply_embed(
            &http,
            msg.channel_id,
            msg.id,
            &embed(
                "Gender",
                format!("Your current gender is **{}**.", gender).as_str(),
            ),
        )
        .await?;

        return Ok(true);
    }

    // requests for a new gender
    if content == "what is my latest gender" {
        let output = gender_prompt().await?;
        let output = output.as_str();

        let mut conn = context.conn.lock().await;
        let _: () = conn.set(format!("users:{}:gender", msg.author.id), output)?;

        reply_embed(
            &http,
            msg.channel_id,
            msg.id,
            &embed(
                "Gender",
                format!("Your new gender is **{}**.", output).as_str(),
            ),
        )
        .await?;

        return Ok(true);
    }

    Ok(false)
}
