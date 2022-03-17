use crate::prelude::*;

use redis::Commands;
use twilight_model::id::{marker::UserMarker, Id};

pub struct UserData {
    pub health: i32,
    pub gender: Option<String>,
}

pub async fn get_user(user_id: Id<UserMarker>, conn: Conn) -> Res<UserData> {
    let mut conn = conn.lock().await;

    let health: i32 = conn.get(format!("users:{user_id}:health")).unwrap_or(100);
    let gender: Option<String> = conn.get(format!("users:{user_id}:gender"))?;

    Ok(UserData { health, gender })
}

/*
pub async fn set_user(user_id: Id<UserMarker>, user_data: UserData, conn: Conn) -> Res<()> {
    let mut conn = conn.lock().await;

    let _: () = conn.set(format!("users:{user_id}:health"), user_data.health)?;
    let _: () = conn.set(format!("users:{user_id}:gender"), user_data.gender)?;

    Ok(())
}
*/
