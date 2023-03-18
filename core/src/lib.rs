use odu_types::Type;
use odu_value::Value;
use sea_orm::EntityTrait;

mod entities;

pub struct User {}

pub trait Projects {
    fn find(&self, user: &User);
    fn find_one(&self, user: &User);
}

pub struct ResourceId;

pub struct ResourceType {
    name: String,
    bluepint: Type,
}

pub struct Resource {
    resource_type: ResourceId,
    value: Value,
}

pub async fn open() {
    let db = sea_orm::Database::connect("sqlite:./test.sqlite")
        .await
        .expect("msk");
}
