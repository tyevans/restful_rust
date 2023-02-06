use diesel::prelude::*;
use uuid::Uuid;
use crate::auth::groups::models::{GroupPermission, NewGroupPermission};
use crate::auth::perms::models::Permission;

use super::models::{Group, NewGroup};
use crate::auth::users::models::{User, UserGroup};
use crate::common::models::{IdRequest, ListPage, ObjectList, Page};
use crate::database::establish_connection;
use crate::schema::{auth_group_permissions, auth_groups, auth_permissions, auth_user_groups, auth_users};
use crate::schema::auth_groups::table as groups_query;
use crate::schema::auth_users::table as users_query;
use crate::schema::auth_permissions::table as permissions_query;


pub async fn list_groups(query: ListPage) -> ObjectList<Group> {
    let connection = &mut establish_connection();

    let page = query.page;
    let per_page = query.per_page;
    let offset = (page - 1) * per_page;
    let limit = per_page;

    let results = groups_query
        .offset(offset)
        .limit(limit)
        .load::<Group>(connection)
        .expect("Error loading groups");

    let count = results.len() as i64;

    ObjectList {
        objects: results,
        page: Page {
            page,
            per_page,
            count,
        },
    }
}


pub async fn list_group_users(group_id: Uuid, query: ListPage) -> ObjectList<User> {
    let connection = &mut establish_connection();

    let page = query.page;
    let per_page = query.per_page;
    let offset = (page - 1) * per_page;
    let limit = per_page;

    let group = Group { id: group_id, name: "".parse().unwrap(), description: "".parse().unwrap() };
    let user_ids = UserGroup::belonging_to(&group).select(auth_user_groups::user_id);

    let results = users_query
        .filter(auth_users::id.eq_any(user_ids))
        .offset(offset)
        .limit(limit)
        .load::<User>(connection)
        .expect("could not load users");

    ObjectList {
        objects: results.clone(),
        page: Page {
            page,
            per_page,
            count: results.len() as i64,
        },
    }
}


pub async fn list_group_perms(group_id: Uuid, query: ListPage) -> ObjectList<Permission> {
    let connection = &mut establish_connection();

    let page = query.page;
    let per_page = query.per_page;
    let offset = (page - 1) * per_page;
    let limit = per_page;

    let group = Group { id: group_id, name: "".parse().unwrap(), description: "".parse().unwrap() };
    let permission_ids = GroupPermission::belonging_to(&group).select(auth_group_permissions::permission_id);

    let results = permissions_query
        .filter(auth_permissions::id.eq_any(permission_ids))
        .offset(offset)
        .limit(limit)
        .load::<Permission>(connection)
        .expect("Error loading group permissions");

    ObjectList {
        objects: results.clone(),
        page: Page {
            page,
            per_page,
            count: results.len() as i64,
        },
    }
}


pub async fn read_group(query: IdRequest) -> Group {
    let connection = &mut establish_connection();

    let result = groups_query
        .filter(auth_groups::id.eq(query.id))
        .load::<Group>(connection)
        .expect("Error loading group");

    let group = result[0].clone();

    Group {
        id: group.id,
        name: group.name,
        description: group.description,
    }
}


pub async fn create_group(new_group: NewGroup) -> Group {
    let connection = &mut establish_connection();

    diesel::insert_into(auth_groups::table)
        .values(&new_group)
        .get_result::<Group>(connection)
        .expect("Error saving new group")
}


pub async fn delete_group(group_id: uuid::Uuid) {
    let connection = &mut establish_connection();

    diesel::delete(auth_groups::table)
        .filter(auth_groups::id.eq(group_id))
        .execute(connection)
        .expect("Failed to delete group");
}


pub async fn add_group_permission(group_perm: NewGroupPermission) -> GroupPermission {
    let connection = &mut establish_connection();

    diesel::insert_into(auth_group_permissions::table)
        .values(&group_perm)
        .get_result::<GroupPermission>(connection)
        .expect("Error saving new group")
}


pub async fn delete_group_permission(group_id: uuid::Uuid, permission_id: uuid::Uuid) {
    let connection = &mut establish_connection();

    diesel::delete(auth_group_permissions::table)
        .filter(auth_group_permissions::group_id.eq(group_id))
        .filter(auth_group_permissions::permission_id.eq(permission_id))
        .execute(connection)
        .expect("Failed to delete group");
}
