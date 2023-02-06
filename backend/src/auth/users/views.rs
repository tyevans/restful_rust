use actix_web::{delete, get, post, put, web};
use super::models::{UserData, User, UserGroupData, UserGroup, UserPermissionData};
use crate::auth::groups::models::{Group};
use super::persist;
use crate::common::models::{IdRequest, ListQuery, ObjectList};
use crate::common::pagination::list_query_to_page;

#[get("users")]
pub async fn api_list_users(query: web::Query<ListQuery>) -> web::Json<ObjectList<User>> {
    let page = list_query_to_page(query.into_inner());
    let users = persist::list_users(page).await;
    web::Json(users)
}

#[get("users/{id}")]
pub async fn api_read_user(query: web::Path<IdRequest>) -> web::Json<User> {
    let user = persist::read_user(query.into_inner()).await;
    web::Json(user)
}


#[post("users")]
pub async fn api_create_user(new_user: web::Json<UserData>) -> web::Json<User> {
    let user = persist::create_user(new_user.into_inner()).await;
    web::Json(user)
}

#[put("users/{id}")]
pub async fn api_update_user(query: web::Path<IdRequest>, updated_user: web::Json<UserData>) -> web::Json<User> {
    let user = persist::update_user(query.into_inner().id, updated_user.into_inner()).await;
    web::Json(user)
}



#[delete("users/{id}")]
pub async fn api_delete_user(query: web::Path<IdRequest>) -> &'static str {
    persist::delete_user(query.into_inner().id).await;
    "OK"
}


#[get("users/{id}/groups")]
pub async fn api_list_user_groups(
    params: web::Path<IdRequest>,
    query: web::Query<ListQuery>,
) -> web::Json<ObjectList<Group>> {
    let page = list_query_to_page(query.into_inner());
    let groups = persist::list_user_groups(
        params.into_inner().id,
        page,
    ).await;

    web::Json(groups)
}


#[post("users/{user_id}/groups/{group_id}")]
pub async fn api_add_user_group(new_user_group: web::Path<UserGroupData>) -> web::Json<UserGroup> {
    let user_group = persist::add_user_group(new_user_group.into_inner()).await;
    web::Json(user_group)
}


#[delete("users/{user_id}/groups/{group_id}")]
pub async fn api_delete_user_group(new_user_group: web::Path<UserGroupData>) -> &'static str {
    persist::delete_user_group(new_user_group.into_inner()).await;
    "OK"
}


#[get("users/{user_id}/perms/{permission_id}")]
pub async fn api_user_has_perm(user_perm: web::Path<UserPermissionData>) -> web::Json<bool> {
    web::Json(persist::user_has_perm(user_perm.into_inner()).await)
}
