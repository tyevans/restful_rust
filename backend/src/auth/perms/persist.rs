use diesel::prelude::*;

use super::models::{Permission, NewPermission};
use crate::common::models::{IdRequest, ListPage, ObjectList, Page};
use crate::database::establish_connection;
use crate::schema::auth_permissions;
use crate::schema::auth_permissions::table as perms_query;

pub async fn list_perms(query: ListPage) -> ObjectList<Permission> {
    let connection = &mut establish_connection();

    let page = query.page;
    let per_page = query.per_page;
    let offset = (page - 1) * per_page;
    let limit = per_page;

    let results = perms_query
        .offset(offset)
        .limit(limit)
        .load::<Permission>(connection)
        .expect("Error loading permissions");

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


pub async fn read_perm(query: IdRequest) -> Permission {
    let connection = &mut establish_connection();

    let result = perms_query
        .filter(auth_permissions::id.eq(query.id))
        .load::<Permission>(connection)
        .expect("Error loading permission");

    return result[0].clone();
}


pub async fn create_perm(new_perm: NewPermission) -> Permission {
    let connection = &mut establish_connection();

    diesel::insert_into(auth_permissions::table)
        .values(&new_perm)
        .get_result::<Permission>(connection)
        .expect("Error saving new user")
}


pub async fn delete_perm(perm_id: uuid::Uuid) {
    let connection = &mut establish_connection();

    diesel::delete(auth_permissions::table)
        .filter(auth_permissions::id.eq(perm_id))
        .execute(connection)
        .expect("Failed to delete permission");
}
