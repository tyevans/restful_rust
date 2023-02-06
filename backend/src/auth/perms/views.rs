use actix_web::{delete, get, post, web};
use crate::auth::perms::models::NewPermission;

use super::models::Permission;
use super::persist;

use crate::common::models::{IdRequest, ListQuery, ObjectList};
use crate::common::pagination::list_query_to_page;

#[get("perms")]
pub async fn api_list_perms(query: web::Query<ListQuery>) -> web::Json<ObjectList<Permission>> {
    let page = list_query_to_page(query.into_inner());
    let perms = persist::list_perms(page).await;
    web::Json(perms)
}

#[get("perms/{id}")]
pub async fn api_read_perm(query: web::Path<IdRequest>) -> web::Json<Permission> {
    let perm = persist::read_perm(query.into_inner()).await;
    web::Json(perm)
}

#[post("perms")]
pub async fn api_create_perm(new_perm: web::Json<NewPermission>) -> web::Json<Permission> {
    let perm = persist::create_perm(new_perm.into_inner()).await;
    web::Json(perm)
}

#[delete("perms/{id}")]
pub async fn api_delete_perm(query: web::Path<IdRequest>) -> &'static str {
    persist::delete_perm(query.into_inner().id).await;
    "OK"
}
