use diesel::prelude::*;
use crate::auth::users::models::{NewUserGroup, UserGroup};

use super::models::{NewUser, User};
use crate::common::models::{IdRequest, ListPage, ObjectList, Page};
use crate::database::establish_connection;
use crate::schema::{auth_user_groups, auth_users};
use crate::schema::auth_users::table as users_query;

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


pub async fn create_user(new_user: NewUser) -> User {
    let connection = &mut establish_connection();

    diesel::insert_into(auth_users::table)
        .values(&new_user)
        .get_result::<User>(connection)
        .expect("Error saving new user")
}


pub async fn update_user(user_id: uuid::Uuid, updated_user: NewUser) -> User {
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


pub async fn add_user_group(user_group: NewUserGroup) -> UserGroup {
    let connection = &mut establish_connection();

    diesel::insert_into(auth_user_groups::table)
        .values(&user_group)
        .get_result::<UserGroup>(connection)
        .expect("Error adding user to group")
}


pub async fn delete_user_group(user_group: NewUserGroup) {
    let connection = &mut establish_connection();

    diesel::delete(auth_user_groups::table)
        .filter(auth_user_groups::user_id.eq(user_group.user_id))
        .filter(auth_user_groups::group_id.eq(user_group.group_id))
        .execute(connection)
        .expect("Failed to delete user");
}
