use crate::data::{get_user, modify_user, UserData};
use crate::discord::{embed, embed_fields, reply, reply_embed, send};
use crate::language::{dictum_prompt, gender_prompt, qa_prompt};
use crate::prelude::*;

use rand::{thread_rng, Rng};
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
        let gm = if thread_rng().gen_bool(0.9) {
            "gm"
        } else {
            "fuck off"
        };

        reply(http, msg.channel_id, msg.id, gm).await?;
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
        let user_data = get_user(msg.author.id, context.conn.clone()).await?;
        let description = match user_data.gender {
            None => "You have no gender.".to_string(),
            Some(gender) => format!("Your current gender is **{}**.", gender),
        };

        reply_embed(
            &http,
            msg.channel_id,
            msg.id,
            &embed("Gender", description.as_str()),
        )
        .await?;

        return Ok(true);
    }

    // requests for a new gender
    if content == "what is my latest gender" {
        let output = gender_prompt().await?;
        let output = output.as_str();

        modify_user(
            msg.author.id,
            |u| UserData {
                gender: Some(output.to_string()),
                ..u
            },
            context.conn.clone(),
        )
        .await?;

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

    if content == "profile" {
        let user_data = get_user(msg.author.id, context.conn.clone()).await?;

        let mut fields = vec![];

        let health = format!("{}", user_data.health);
        fields.push(("Health", health.as_str()));

        let gender: String;
        if let Some(gender_) = user_data.gender {
            gender = gender_;
            fields.push(("Gender", gender.as_str()));
        }

        reply_embed(
            &http,
            msg.channel_id,
            msg.id,
            &embed_fields(Some("Your profile"), None, fields),
        )
        .await?;

        return Ok(true);
    }

    Ok(false)
}
