use crate::discord::reply;
use crate::language::{complete_prompt, qa_prompt};
use crate::prelude::*;

use twilight_http::Client as HttpClient;
use twilight_model::channel::message::Message;

pub async fn handle_command(http: &HttpClient, msg: Message) -> Res<bool> {
    let content = msg.content;

    // gm
    if content == "gm" {
        reply(http, msg.channel_id, msg.id, "gm motherfucker").await?;
        return Ok(true);
    }

    // questions to dict
    if let Some(question) = content.strip_prefix("dict, ") {
        let output = complete_prompt(qa_prompt(), vec![("question", question)]).await?;
        reply(&http, msg.channel_id, msg.id, output.as_str()).await?;
        return Ok(true);
    }

    // requests for dict's latest dictum
    if content == "what is your latest dictum" {
        // TODO
        return Ok(true);
    }

    Ok(false)
}
