use actix_web::{get, post, delete, web};
use crate::auth::perms::models::Permission;
use crate::auth::users::models::NewUserGroup;

use super::models::{Group, NewGroup, GroupPermission, NewGroupPermission};
use super::persist;

use crate::common::models::{IdRequest, ListPage, ListQuery, ObjectList};
use crate::common::pagination::list_query_to_page;


#[get("groups")]
pub async fn api_list_groups(query: web::Query<ListQuery>) -> web::Json<ObjectList<Group>> {
    let page = list_query_to_page(query.into_inner());
    let groups = persist::list_groups(page).await;
    web::Json(groups)
}


#[get("groups/{id}")]
pub async fn api_read_group(
    query: web::Path<IdRequest>
) -> web::Json<Group> {
    let group = persist::read_group(query.into_inner()).await;
    web::Json(group)
}


#[post("groups")]
pub async fn api_create_group(new_group: web::Json<NewGroup>) -> web::Json<Group> {
    let group = persist::create_group(new_group.into_inner()).await;
    web::Json(group)
}

#[delete("groups/{id}")]
pub async fn api_delete_group(
    query: web::Path<IdRequest>
) -> &'static str {
    persist::delete_group(query.into_inner().id).await;
    "OK"
}


#[get("groups/{id}/users")]
pub async fn api_list_group_users(
    params: web::Path<IdRequest>,
    query: web::Query<ListQuery>,
) -> web::Json<ObjectList<Permission>> {
    let page = list_query_to_page(query.into_inner());
    let permissions = persist::list_group_perms(
        params.into_inner().id,
        page,
    ).await;

    web::Json(permissions)
}


#[get("groups/{id}/perms")]
pub async fn api_list_group_permissions(
    params: web::Path<IdRequest>,
    query: web::Query<ListQuery>,
) -> web::Json<ObjectList<Permission>> {
    let page = list_query_to_page(query.into_inner());
    let permissions = persist::list_group_perms(
        params.into_inner().id,
        page,
    ).await;

    web::Json(permissions)
}


#[post("groups/{group_id}/perms/{perm_id}")]
pub async fn api_add_group_permission(
    new_group_perm: web::Query<NewGroupPermission>
) -> web::Json<ObjectList<Permission>> {
    let group_perm_request = new_group_perm.into_inner().clone();
    persist::add_group_permission(
        group_perm_request.clone()
    ).await;
    let permissions = persist::list_group_perms(
        group_perm_request.group_id,
        ListPage::default(),
    ).await;

    web::Json(permissions)
}


#[delete("groups/{group_id}/perms/{perm_id}")]
pub async fn api_remove_group_permission(
    new_group_perm: web::Json<NewGroupPermission>
) -> web::Json<GroupPermission> {
    let group_perm = persist::add_group_permission(
        new_group_perm.into_inner()
    ).await;
    web::Json(group_perm)
}
