use crate::prelude::*;

use rand::seq::SliceRandom;
use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;

pub struct Prompt {
    text: String,
    stop_seqs: Vec<String>,
}

const AI21_TOKEN: &str = include_str!("../sensitive/ai21.token");

const DICTUM_PROMPT_TEXT: &str = include_str!("../assets/dictum.prompt");
pub async fn dictum_prompt() -> Res<String> {
    let prompt = Prompt {
        text: DICTUM_PROMPT_TEXT.trim().to_string(),
        stop_seqs: vec!["\n".to_string()],
    };
    complete_prompt(prompt, vec![]).await
}

const QA_PROMPT_TEXT: &str = include_str!("../assets/qa.prompt");
pub async fn qa_prompt(question: &str) -> Res<String> {
    let prompt = Prompt {
        text: QA_PROMPT_TEXT.trim().to_string(),
        stop_seqs: vec!["\n".to_string()],
    };
    let annotation = [
        "Notice that Omnidict's response is incredibly florid",
        "Here, Omnidict's response weaves in one of his peculiar obsessions",
        "Notice that Omnidict opines on foreign affairs here",
        "In this case, Omnidict's reply is relatively unexpected",
    ]
    .choose(&mut rand::thread_rng()).unwrap();
    complete_prompt(prompt, vec![("question", question), ("annotation", annotation)]).await
}

async fn complete_prompt(prompt: Prompt, parameters: Vec<(&str, &str)>) -> Res<String> {
    let mut text = String::from(prompt.text);

    for (key, value) in parameters {
        let key = format!("[[{}]]", key);
        text = text.replace(&key, value);
    }
    get_j1(text.as_str(), prompt.stop_seqs).await
}

async fn get_j1(prompt: &str, stop_seqs: Vec<String>) -> Res<String> {
    let max_tokens: u64 = 256;
    let temperature: f64 = 1.;
    let top_p: f64 = 0.9;

    let body = &json!({
        "prompt": prompt,
        "maxTokens": max_tokens,
        "stopSequences": stop_seqs,
        "presencePenalty": {"scale": 0.4},
        "temperature": temperature,
        "topP": top_p,
    });

    let token = format!("Bearer {}", AI21_TOKEN.trim());
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(token.as_str())?,
    );

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.ai21.com/studio/v1/j1-jumbo/complete")
        .headers(headers)
        .json(body)
        .send()
        .await?;

    let res_json = res.json::<serde_json::Value>().await?;
    Ok(res_json["completions"][0]["data"]["text"]
        .as_str()
        .ok_or(format!("Bad AI21 response: {}", res_json))?
        .to_string())
}
