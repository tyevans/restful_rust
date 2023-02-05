use diesel::prelude::*;

use crate::common::models::{IdRequest, ObjectList};
use crate::auth::models::{User, NewUser, Group, UserGroup, GroupDetails, NewGroup};
use crate::database::establish_connection;
use crate::schema::{auth_groups, auth_user_groups, auth_users};
use crate::schema::auth_users::dsl::auth_users as users_query;
use crate::schema::auth_groups::dsl::auth_groups as groups_query;

pub async fn list_users() -> ObjectList<User> {
    let connection = &mut establish_connection();

    let results = users_query
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading posts");

    ObjectList {
        objects: results
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

pub async fn list_groups() -> ObjectList<Group> {
    let connection = &mut establish_connection();

    let results = groups_query
        .limit(5)
        .load::<Group>(connection)
        .expect("Error loading groups");

    ObjectList {
        objects: results
    }
}

pub async fn list_group_users(group: &Group) -> ObjectList<User> {
    let connection = &mut establish_connection();

    let user_ids = UserGroup::belonging_to(group).select(auth_user_groups::user_id);

    let results = users_query
        .filter(auth_users::id.eq_any(user_ids))
        .load::<User>(connection)
        .expect("could not load users");

    ObjectList {
        objects: results
    }
}

pub async fn read_group(query: IdRequest) -> GroupDetails {
    let connection = &mut establish_connection();

    let result = groups_query
        .filter(auth_groups::id.eq(query.id))
        .load::<Group>(connection)
        .expect("Error loading group");

    let group = result[0].clone();

    GroupDetails {
        id: group.id,
        name: group.name,
        description: group.description,
        users: list_group_users(&result[0]).await.objects
    }
}


pub async fn create_group(new_group: NewGroup) -> Group {
    let connection = &mut establish_connection();

    diesel::insert_into(auth_groups::table)
        .values(&new_group)
        .get_result::<Group>(connection)
        .expect("Error saving new group")
}
