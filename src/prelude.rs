pub use crate::data::*;
pub use crate::discord::{self, GENERAL_ID, OWN_ID, PNPPC_ID};
pub use crate::language::*;

pub use anyhow::{anyhow, Error, Result};
pub use redis::Commands;
pub use redis::Connection as DbConnection;
pub use std::sync::Arc;
pub use tokio::sync::Mutex;
pub use twilight_gateway::shard::Shard;
pub use twilight_http::Client as HttpClient;
pub use twilight_model::channel::message::Message;

#[derive(Clone)]
pub struct Context {
    pub http: Arc<HttpClient>,
    pub shard: Arc<Shard>,
    pub db: Arc<Mutex<DbConnection>>,
}
