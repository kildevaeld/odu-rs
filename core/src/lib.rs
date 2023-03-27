use odu_types::Type;
use odu_value::Value;
use sea_orm::{ConnectOptions, EntityTrait};

mod entities;
mod user;

pub struct Database(sea_orm::Database);

impl Database {
    pub async fn open<C>(connect: C) -> Result<Database, sea_orm::error::DbErr>
    where
        C: Into<ConnectOptions>,
    {
        let db = sea_orm::Database::connect(connect).await?;
        Ok(Database(db))
    }
}
