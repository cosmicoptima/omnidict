pub use crate::data::*;
pub use crate::util::discord::{self, GENERAL_ID, OWN_ID, PNPPC_ID};
pub use crate::util::language::*;

pub use anyhow::{anyhow, Error, Result};
pub use sled::Db;
pub use std::sync::Arc;
pub use tokio::sync::Mutex;
pub use twilight_gateway::shard::Shard;
pub use twilight_http::Client as HttpClient;
pub use twilight_model::channel::message::Message;

use finalfusion::embeddings::Embeddings as EmbeddingsT;
use finalfusion::{storage::NdArray, vocab::FastTextSubwordVocab};
pub type Embeddings = EmbeddingsT<FastTextSubwordVocab, NdArray>;

#[derive(Clone)]
pub struct Context {
    pub http: Arc<HttpClient>,
    pub shard: Arc<Shard>,
    pub db: Arc<Db>,
    pub embeddings: Arc<Embeddings>,
}
