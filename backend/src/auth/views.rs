use actix_web::{web};
use crate::auth::{users, groups, perms};


pub fn auth_route_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(users::views::api_list_users)
        .service(users::views::api_read_user)
        .service(users::views::api_create_user)
        .service(users::views::api_update_user)
        .service(users::views::api_delete_user)

        // .service(users::views::api_list_user_groups)
        .service(users::views::api_add_user_group)
        .service(users::views::api_delete_user_group)

        .service(groups::views::api_list_groups)
        .service(groups::views::api_read_group)
        .service(groups::views::api_create_group)
        .service(groups::views::api_delete_group)

        // .service(groups::views::api_list_group_users)

        .service(groups::views::api_list_group_permissions)
        .service(groups::views::api_add_group_permission)
        // .service(groups::views::api_delete_group_permission)

        .service(perms::views::api_list_perms)
        .service(perms::views::api_read_perm)
        .service(perms::views::api_create_perm);
    // .service(perms::views::api_delete_perm)

    // .service(perms::views::api_user_has_perm)
}
