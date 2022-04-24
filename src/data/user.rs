use crate::prelude::*;

use bincode::{deserialize, serialize};
use twilight_model::id::{marker::UserMarker, Id};

pub struct UserData {
    id: Id<UserMarker>,
    pub health: i32,
    pub gender: Option<String>,
}

impl UserData {
    pub async fn load(id: Id<UserMarker>, db: Arc<Db>) -> Result<UserData> {
        let health: i32;
        if let Some(value) = db.get(format!("users.{id}.health"))? {
            health = deserialize(&*value)?;
        } else {
            health = 100;
        }

        let gender: Option<String>;
        if let Some(value) = db.get(format!("users.{id}.gender"))? {
            gender = Some(deserialize(&*value)?);
        } else {
            gender = None;
        }

        Ok(UserData { id, health, gender })
    }

    pub async fn save(&self, db: Arc<Db>) -> Result<()> {
        let id = self.id;
        db.insert(format!("users.{id}.health"), serialize(&self.health)?)?;
        db.insert(format!("users.{id}.gender"), serialize(&self.gender)?)?;
        Ok(())
    }
}
