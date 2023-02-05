use diesel::prelude::*;

use crate::common::models::{IdRequest, ObjectList};
use crate::blog::models::{NewPost, Post};
use crate::database::establish_connection;
use crate::schema::posts;
use crate::schema::posts::dsl::posts as posts_query;

pub async fn list_posts() -> ObjectList<Post> {
    let connection = &mut establish_connection();

    let results = posts_query
        .filter(posts::published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    ObjectList {
        objects: results
    }
}

pub async fn read_post(query: IdRequest) -> Post {
    let connection = &mut establish_connection();

    let result = posts_query
        .filter(posts::published.eq(true))
        .filter(posts::id.eq(query.id))
        .load::<Post>(connection)
        .unwrap();

    result[0].clone()
}


pub async fn create_post(new_post: NewPost) -> Post {
    let connection = &mut establish_connection();

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(connection)
        .expect("Error saving new post")
}
