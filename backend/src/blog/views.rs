use actix_web::{get, post, web};

use crate::common::models::{IdRequest, ObjectList};
use crate::blog::models::{NewPost, Post};
use crate::blog::persistence;

#[get("posts")]
pub async fn api_list_posts() -> web::Json<ObjectList<Post>> {
    let posts = persistence::list_posts().await;
    web::Json(posts)
}

#[get("posts/{id}")]
pub async fn api_read_post(query: web::Path<IdRequest>) -> web::Json<Post> {
    let post = persistence::read_post(query.into_inner()).await;
    web::Json(post)
}


#[post("posts")]
pub async fn api_create_post(new_post: web::Json<NewPost>) -> web::Json<Post> {
    let post = persistence::create_post(new_post.into_inner()).await;
    web::Json(post)
}


pub fn blog_route_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(api_list_posts)
        .service(api_read_post)
        .service(api_create_post);
}
