use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::posts;


#[derive(Serialize, Clone, Queryable)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}
