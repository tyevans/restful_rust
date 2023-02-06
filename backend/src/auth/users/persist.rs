use diesel::prelude::*;
use crate::auth::groups::models::Group;
use crate::auth::users::models::{UserGroupData, UserGroup, UserPermissionData};

use super::models::{UserData, User};
use crate::common::models::{IdRequest, ListPage, ObjectList, Page};
use crate::database::establish_connection;
use crate::schema::{auth_groups, auth_user_groups, auth_users, auth_group_permissions};
use crate::schema::auth_users::table as users_query;
use crate::schema::auth_groups::table as groups_query;

pub async fn list_users(query: ListPage) -> ObjectList<User> {
    let connection = &mut establish_connection();

    let page = query.page;
    let per_page = query.per_page;
    let offset = (page - 1) * per_page;
    let limit = per_page;

    let results = users_query
        .offset(offset)
        .limit(limit)
        .load::<User>(connection)
        .expect("Error loading posts");

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

pub async fn read_user(query: IdRequest) -> User {
    let connection = &mut establish_connection();

    let result = users_query
        .filter(auth_users::id.eq(query.id))
        .load::<User>(connection)
        .expect("Error loading user");

    result[0].clone()
}


pub async fn create_user(new_user: UserData) -> User {
    let connection = &mut establish_connection();

    diesel::insert_into(auth_users::table)
        .values(&new_user)
        .get_result::<User>(connection)
        .expect("Error saving new user")
}


pub async fn update_user(user_id: uuid::Uuid, updated_user: UserData) -> User {
    let connection = &mut establish_connection();

    diesel::update(auth_users::table)
        .filter(auth_users::id.eq(user_id))
        .set(&updated_user)
        .get_result::<User>(connection)
        .expect("Error saving new user")
}


pub async fn delete_user(user_id: uuid::Uuid) {
    let connection = &mut establish_connection();

    diesel::delete(auth_users::table)
        .filter(auth_users::id.eq(user_id))
        .execute(connection)
        .expect("Failed to delete user");
}

pub async fn list_user_groups(user_id: uuid::Uuid, query: ListPage) -> ObjectList<Group> {
    let connection = &mut establish_connection();

    let page = query.page;
    let per_page = query.per_page;
    let offset = (page - 1) * per_page;
    let limit = per_page;

    let user = User {
        id: user_id,
        display_name: "".parse().unwrap(),
        email: "".parse().unwrap(),
        active: true
    };
    let group_ids = UserGroup::belonging_to(&user).select(auth_user_groups::group_id);

    let results = groups_query
        .filter(auth_groups::id.eq_any(group_ids))
        .offset(offset)
        .limit(limit)
        .load::<Group>(connection)
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


pub async fn add_user_group(user_group: UserGroupData) -> UserGroup {
    let connection = &mut establish_connection();

    diesel::insert_into(auth_user_groups::table)
        .values(&user_group)
        .get_result::<UserGroup>(connection)
        .expect("Error adding user to group")
}


pub async fn delete_user_group(user_group: UserGroupData) {
    let connection = &mut establish_connection();

    diesel::delete(auth_user_groups::table)
        .filter(auth_user_groups::user_id.eq(user_group.user_id))
        .filter(auth_user_groups::group_id.eq(user_group.group_id))
        .execute(connection)
        .expect("Failed to delete user");
}

pub async fn user_has_perm(user_perm: UserPermissionData) -> bool {
    let connection = &mut establish_connection();

    let user = User {
        id: user_perm.user_id,
        display_name: "".parse().unwrap(),
        email: "".parse().unwrap(),
        active: true
    };
    let group_ids = UserGroup::belonging_to(&user).select(auth_user_groups::group_id);

    let results: i64 = auth_group_permissions::table
        .filter(auth_group_permissions::group_id.eq_any(group_ids))
        .filter(auth_group_permissions::permission_id.eq(user_perm.permission_id))
        .count()
        .first(connection)
        .expect("Failed to check user permissions");

    results > 0
}
