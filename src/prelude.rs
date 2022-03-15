use std::error::Error;

pub type E = Box<dyn Error + Send + Sync>;
pub type Res<T> = Result<T, E>;
