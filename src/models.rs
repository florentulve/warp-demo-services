use diesel::Queryable;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub uuid: Uuid,
}