//! all of dict's whims and thinking start in this file
//! and spread outward like a toxic spore

use crate::prelude::*;
use discord::embed_field;
use finalfusion::similarity::WordSimilarity;
use rand::{thread_rng, Rng};
use std::time::Duration;
use tokio::time::sleep;
use twilight_embed_builder::EmbedBuilder;
use twilight_gateway::Event;

pub async fn handle_start(http: Arc<HttpClient>) -> Result<()> {
    discord::send(&http.clone(), GENERAL_ID, "Rise and shine, bitches").await?;
    discord::set_own_nickname(&http, "omnidict").await?;
    Ok(())
}

/// do something occasionally
pub async fn catastrophe(http: Arc<HttpClient>) {
    loop {
        discord::post_error(
            try {
                loop {
                    if thread_rng().gen_bool(0.00002) {
                        let output = dictum_prompt().await?;
                        discord::send(&http.clone(), GENERAL_ID, &output).await?;
                    }

                    sleep(Duration::from_millis(100)).await;
                }
            },
            &http,
        )
        .await;
    }
}

pub async fn handle_gateway_event(event: Event, ctx: Context) -> Result<()> {
    match event {
        Event::MessageCreate(msg) if msg.author.id != OWN_ID => {
            let handled = handle_command(&ctx, &msg).await?;
            if !handled && thread_rng().gen_bool(0.02) {
                let output = qa_prompt(&msg.content).await?;
                discord::reply(&ctx.http, msg.channel_id, msg.id, &output).await?;
            }
        }
        _ => (),
    }
    Ok(())
}

pub async fn handle_command(ctx: &Context, msg: &Message) -> Result<bool> {
    let content = &msg.content;
    let http = &ctx.http;

    // bedtime!
    if content == "bedtime!" {
        discord::send(http, msg.channel_id, "i'm so tired...").await?;
        ctx.shard.shutdown();
        return Ok(true);
    }

    // gm
    if content == "gm" {
        let gm = if thread_rng().gen_bool(0.9) {
            "gm"
        } else {
            "fuck off"
        };

        discord::reply(http, msg.channel_id, msg.id, gm).await?;
        return Ok(true);
    }

    // questions to dict
    if let Some(question) = content.strip_prefix("dict, ") {
        let output = qa_prompt(question).await?;
        discord::reply(http, msg.channel_id, msg.id, &output).await?;
        return Ok(true);
    }

    // requests for dict's latest dictum
    if content == "what is your latest dictum" {
        let output = dictum_prompt().await?;
        discord::reply(http, msg.channel_id, msg.id, &output).await?;
        return Ok(true);
    }

    // requests for one's current gender
    if content == "what is my current gender" {
        let user_data = UserData::load(msg.author.id, ctx.db.clone()).await?;
        let description = match user_data.gender {
            None => "You have no gender.".to_string(),
            Some(gender) => format!("Your current gender is **{}**.", gender),
        };

        discord::reply_embed(
            http,
            msg.channel_id,
            msg.id,
            &EmbedBuilder::new()
                .title("Gender")
                .description(description)
                .build()?,
        )
        .await?;

        return Ok(true);
    }

    // requests for a new gender
    if content == "what is my latest gender" {
        let output = gender_prompt().await?;
        let output = output.as_str();

        let mut user = UserData::load(msg.author.id, ctx.db.clone()).await?;
        user.gender = Some(output.to_string());
        user.save(ctx.db.clone()).await?;

        discord::reply_embed(
            http,
            msg.channel_id,
            msg.id,
            &EmbedBuilder::new()
                .title("Gender")
                .description(format!("Your new gender is **{}**.", output))
                .build()?,
        )
        .await?;

        return Ok(true);
    }

    if content == "profile" {
        let user_data = UserData::load(msg.author.id, ctx.db.clone()).await?;
        let mut embed = EmbedBuilder::new().title("Your profile").build()?;

        let health = format!("{}", user_data.health);
        embed.fields.push(embed_field("Health", &health));

        if let Some(gender) = user_data.gender {
            embed.fields.push(embed_field("Gender", &gender));
        }

        discord::reply_embed(http, msg.channel_id, msg.id, &embed).await?;
        return Ok(true);
    }

    

    // testing word embeddings
    if let Some(word) = content.strip_prefix("DEBUG similarity ") {
        if let Some(results) = ctx.embeddings.word_similarity(word, 10) {
            discord::reply_embed(
                http,
                msg.channel_id,
                msg.id,
                &EmbedBuilder::new()
                    .title("Similar words")
                    .description(format!(
                        "{}",
                        results
                            .iter()
                            .map(|result| result.word())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                    .build()?,
            )
            .await?;
        } else {
            discord::reply(http, msg.channel_id, msg.id, "Nope").await?;
        }
    }

    Ok(false)
}
