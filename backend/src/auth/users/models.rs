use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{auth_user_groups, auth_users};
use crate::auth::groups::models::Group;

#[derive(Identifiable, Serialize, Clone, Queryable)]
#[diesel(table_name = auth_users)]
pub struct User {
    pub id: uuid::Uuid,
    pub display_name: String,
    pub email: String,
    pub active: bool,
}

#[derive(Identifiable, Serialize, Clone, Queryable)]
#[diesel(table_name = auth_users)]
pub struct UserKey {
    pub id: uuid::Uuid,
    pub path_pub: String,
    pub path: String,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = auth_users)]
pub struct NewUser {
    pub display_name: String,
    pub email: String,
}

#[derive(Identifiable, Queryable, Associations, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = auth_user_groups)]
pub struct UserGroup {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = auth_user_groups)]
pub struct NewUserGroup {
    pub user_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
}

#[derive(Deserialize)]
pub struct UserPermissionData {
    pub user_id: uuid::Uuid,
    pub permission_id: uuid::Uuid,
}