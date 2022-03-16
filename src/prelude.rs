use std::error::Error;
use std::sync::Arc;

use redis::Connection;
use tokio::sync::Mutex;
use twilight_http::Client as HttpClient;

pub type E = Box<dyn Error + Send + Sync>;
pub type Res<T> = Result<T, E>;

pub type Conn = Arc<Mutex<Connection>>;

pub struct Context {
    pub http: Arc<HttpClient>,
    pub conn: Conn,
}
