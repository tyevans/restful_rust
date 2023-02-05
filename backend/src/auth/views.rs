use actix_web::{get, post, web};

use crate::common::models::{IdRequest, ObjectList};
use crate::auth::models::{Group, GroupDetails, NewGroup, NewUser, User};
use crate::auth::persistence;

#[get("users")]
pub async fn api_list_users() -> web::Json<ObjectList<User>> {
    let users = persistence::list_users().await;
    web::Json(users)
}

#[get("users/{id}")]
pub async fn api_read_user(query: web::Path<IdRequest>) -> web::Json<User> {
    let user = persistence::read_user(query.into_inner()).await;
    web::Json(user)
}


#[post("users")]
pub async fn api_create_user(new_user: web::Json<NewUser>) -> web::Json<User> {
    let user = persistence::create_user(new_user.into_inner()).await;
    web::Json(user)
}

#[get("groups")]
pub async fn api_list_groups() -> web::Json<ObjectList<Group>> {
    let groups = persistence::list_groups().await;
    web::Json(groups)
}

#[get("groups/{id}")]
pub async fn api_read_group(query: web::Path<IdRequest>) -> web::Json<GroupDetails> {
    let group = persistence::read_group(query.into_inner()).await;
    web::Json(group)
}


#[post("groups")]
pub async fn api_create_group(new_group: web::Json<NewGroup>) -> web::Json<Group> {
    let group = persistence::create_group(new_group.into_inner()).await;
    web::Json(group)
}

pub fn auth_route_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(api_list_users)
        .service(api_read_user)
        .service(api_create_user)
        .service(api_list_groups)
        .service(api_read_group)
        .service(api_create_group);
}
