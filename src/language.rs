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
const TEXTSYNTH_TOKEN: &str = include_str!("../sensitive/textsynth.token");

const DICTUM_PROMPT_TEXT: &str = include_str!("../assets/dictum.prompt");
pub async fn dictum_prompt() -> Res<String> {
    let prompt = Prompt {
        text: DICTUM_PROMPT_TEXT.trim().to_string(),
        stop_seqs: vec!["\n".to_string()],
    };
    Ok(complete_prompt(prompt, vec![]).await?.trim().to_string())
}

const GENDER_PROMPT_TEXT: &str = include_str!("../assets/gender.prompt");
pub async fn gender_prompt() -> Res<String> {
    let prompt = Prompt {
        text: GENDER_PROMPT_TEXT.trim().to_string(),
        stop_seqs: vec!["\n".to_string()],
    };
    Ok(complete_prompt(prompt, vec![]).await?.trim().to_string())
}

const QA_PROMPT_TEXT: &str = include_str!("../assets/qa.prompt");
pub async fn qa_prompt(question: &str) -> Res<String> {
    let prompt = Prompt {
        text: QA_PROMPT_TEXT.trim().to_string(),
        stop_seqs: vec!["\n".to_string()],
    };
    let annotation = [
        "Omnidict's reply is incredibly florid",
        "Here, Omnidict's response weaves in one of his peculiar obsessions",
        // "Here, Omnidict opines on a country whose leader he recently spoke to",
        "In this case, Omnidict's reply is relatively unexpected",
    ]
    .choose(&mut rand::thread_rng())
    .unwrap();

    Ok(complete_prompt(
        prompt,
        vec![("question", question), ("annotation", annotation)],
    )
    .await?
    .trim()
    .to_string())
}

async fn complete_prompt(prompt: Prompt, parameters: Vec<(&str, &str)>) -> Res<String> {
    let mut text = String::from(prompt.text);

    for (key, value) in parameters {
        let key = format!("[[{}]]", key);
        text = text.replace(&key, value);
    }

    if cfg!(feature = "fairseq") {
        get_fairseq(text.as_str(), prompt.stop_seqs).await
    } else {
        get_j1(text.as_str(), prompt.stop_seqs).await
    }
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

async fn get_fairseq(prompt: &str, stop_seqs: Vec<String>) -> Res<String> {
    let max_tokens: u64 = 64;
    let temperature: f64 = 1.;
    let top_p: f64 = 0.9;

    let body = &json!({
        "prompt": prompt,
        "max_tokens": max_tokens,
        "stop": stop_seqs,
        "temperature": temperature,
        "top_p": top_p,
    });

    let token = format!("Bearer {}", TEXTSYNTH_TOKEN.trim());
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(token.as_str())?,
    );

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.textsynth.com/v1/engines/fairseq_gpt_13B/completions")
        .headers(headers)
        .json(body)
        .send()
        .await?;

    let res_json = res.json::<serde_json::Value>().await?;
    Ok(res_json["text"]
        .as_str()
        .ok_or(format!("Bad TextSynth response: {}", res_json))?
        .to_string())
}
