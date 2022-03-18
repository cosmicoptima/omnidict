use crate::prelude::*;
use twilight_model::id::{marker::UserMarker, Id};

pub struct UserData {
    id: Id<UserMarker>,
    pub health: i32,
    pub gender: Option<String>,
}

impl UserData {
    pub async fn load(id: Id<UserMarker>, db: Arc<Mutex<DbConnection>>) -> Result<UserData> {
        let mut db = db.lock().await;
        let health: i32 = db.get(format!("users:{id}:health")).unwrap_or(100);
        let gender: Option<String> = db.get(format!("users:{id}:gender"))?;
        Ok(UserData { id, health, gender })
    }

    pub async fn save(&self, db: Arc<Mutex<DbConnection>>) -> Result<()> {
        let mut db = db.lock().await;
        let id = self.id;
        db.set(format!("users:{id}:health"), &self.health)?;
        db.set(format!("users:{id}:gender"), &self.gender)?;
        Ok(())
    }
}
