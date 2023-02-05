use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{auth_users, auth_groups, auth_permissions, auth_user_groups, auth_group_permissions};

#[derive(Identifiable, Serialize, Clone, Queryable)]
#[diesel(table_name = auth_users)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub active: bool,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = auth_users)]
pub struct NewUser {
    pub display_name: String,
    pub email: String,
}

#[derive(Identifiable, Serialize, Clone, Queryable)]
#[diesel(table_name = auth_groups)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = auth_groups)]
pub struct NewGroup {
    pub name: String,
    pub description: String,
}


#[derive(Serialize, Clone)]
pub struct GroupDetails {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub users: Vec<User>,
}

#[derive(Identifiable, Serialize, Clone, Queryable)]
#[diesel(table_name = auth_permissions)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Permission))]
#[diesel(table_name = auth_group_permissions)]
pub struct GroupPermission {
    pub id: i32,
    pub group_id: i32,
    pub permission_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = auth_user_groups)]
pub struct UserGroup {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
}
