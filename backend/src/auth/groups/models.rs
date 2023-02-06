use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{auth_group_permissions, auth_groups};
use crate::auth::perms::models::Permission;

#[derive(Identifiable, Serialize, Clone, Queryable)]
#[diesel(table_name = auth_groups)]
pub struct Group {
    pub id: uuid::Uuid,
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
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Identifiable, Queryable, Associations, Serialize)]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Permission))]
#[diesel(table_name = auth_group_permissions)]
pub struct GroupPermission {
    pub id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub permission_id: uuid::Uuid,
}


#[derive(Deserialize, Insertable, Clone)]
#[diesel(table_name = auth_group_permissions)]
pub struct NewGroupPermission {
    pub group_id: uuid::Uuid,
    pub permission_id: uuid::Uuid,
}
