use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{auth_permissions};


#[derive(Identifiable, Serialize, Clone, Queryable)]
#[diesel(table_name = auth_permissions)]
pub struct Permission {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
}


#[derive(Deserialize, Insertable)]
#[diesel(table_name = auth_permissions)]
pub struct NewPermission {
    pub name: String,
    pub description: String,
}
