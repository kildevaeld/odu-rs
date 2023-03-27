use crate::Database;

pub struct CreateUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

pub struct User {}

impl User {
    pub fn create(db: &Database, create: CreateUser<'_>) {}
}
